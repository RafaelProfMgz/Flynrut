use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
};

use anyhow::{Context, Result, anyhow};
use serde_json::{Value, json};

#[derive(Debug, Clone)]
pub struct DiagnosticItem {
    pub line: u64,
    pub severity: String,
    pub message: String,
}

#[derive(Debug)]
pub struct LspClient {
    command: String,
    workspace_root: PathBuf,
    child: Option<Child>,
    stdin: Option<ChildStdin>,
    receiver: Option<Receiver<LspEvent>>,
    open_documents: HashMap<PathBuf, i32>,
    diagnostics: HashMap<PathBuf, Vec<DiagnosticItem>>,
    next_id: u64,
    pub status: String,
}

#[derive(Debug)]
enum LspEvent {
    Diagnostics {
        path: PathBuf,
        diagnostics: Vec<DiagnosticItem>,
    },
    Exited(String),
}

impl LspClient {
    #[must_use]
    pub fn new(workspace_root: PathBuf, command: String) -> Self {
        Self {
            command,
            workspace_root,
            child: None,
            stdin: None,
            receiver: None,
            open_documents: HashMap::new(),
            diagnostics: HashMap::new(),
            next_id: 1,
            status: "desconectado".to_string(),
        }
    }

    /// # Errors
    /// Returns an error if the LSP process cannot be spawned or initialized.
    pub fn start(&mut self) -> Result<()> {
        if self.is_running() {
            return Ok(());
        }

        let mut child = Command::new("sh")
            .arg("-lc")
            .arg(&self.command)
            .current_dir(&self.workspace_root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .with_context(|| format!("não foi possível iniciar {}", self.command))?;

        let stdin = child.stdin.take().context("stdin indisponível no LSP")?;
        let stdout = child.stdout.take().context("stdout indisponível no LSP")?;
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            let mut reader = BufReader::new(stdout);
            loop {
                match read_message(&mut reader) {
                    Ok(Some(message)) => {
                        if let Some(event) = parse_event(&message) {
                            let _ = sender.send(event);
                        }
                    }
                    Ok(None) => {
                        let _ = sender.send(LspEvent::Exited("LSP encerrado".to_string()));
                        return;
                    }
                    Err(error) => {
                        let _ = sender.send(LspEvent::Exited(format!("falha no LSP: {error}")));
                        return;
                    }
                }
            }
        });

        self.child = Some(child);
        self.stdin = Some(stdin);
        self.receiver = Some(receiver);
        self.send_request(
            "initialize",
            json!({
                "processId": std::process::id(),
                "rootUri": file_uri(&self.workspace_root),
                "capabilities": {
                    "textDocument": {
                        "publishDiagnostics": {}
                    }
                },
                "workspaceFolders": [{
                    "uri": file_uri(&self.workspace_root),
                    "name": self.workspace_root.file_name().and_then(|name| name.to_str()).unwrap_or("workspace")
                }]
            }),
        )?;
        self.send_notification("initialized", json!({}))?;
        self.status = "conectado".to_string();
        Ok(())
    }

    #[must_use]
    pub fn is_running(&self) -> bool {
        self.child.is_some()
    }

    /// # Errors
    /// Returns an error if sending the LSP notification fails.
    pub fn sync_document(&mut self, path: &Path, contents: &str) -> Result<()> {
        if !self.is_running() || path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            return Ok(());
        }

        let uri = file_uri(path);
        let version = {
            let version = self.open_documents.entry(path.to_path_buf()).or_insert(0);
            if *version == 0 {
                *version = 1;
            } else {
                *version += 1;
            }
            *version
        };

        if version == 1 {
            self.send_notification(
                "textDocument/didOpen",
                json!({
                    "textDocument": {
                        "uri": uri,
                        "languageId": "rust",
                        "version": version,
                        "text": contents,
                    }
                }),
            )?;
        } else {
            self.send_notification(
                "textDocument/didChange",
                json!({
                    "textDocument": {
                        "uri": uri,
                        "version": version,
                    },
                    "contentChanges": [{
                        "text": contents
                    }]
                }),
            )?;
        }
        Ok(())
    }

    /// # Errors
    /// Returns an error if the LSP save notification cannot be sent.
    pub fn save_document(&mut self, path: &Path, contents: &str) -> Result<()> {
        if !self.is_running() || path.extension().and_then(|ext| ext.to_str()) != Some("rs") {
            return Ok(());
        }

        self.sync_document(path, contents)?;
        self.send_notification(
            "textDocument/didSave",
            json!({
                "textDocument": {
                    "uri": file_uri(path)
                },
                "text": contents
            }),
        )?;
        Ok(())
    }

    pub fn diagnostics_for(&self, path: Option<&Path>) -> &[DiagnosticItem] {
        static EMPTY: [DiagnosticItem; 0] = [];
        path.and_then(|path| self.diagnostics.get(path))
            .map_or(&EMPTY, Vec::as_slice)
    }

    pub fn drain(&mut self) {
        let mut disconnected = false;

        while let Some(receiver) = &self.receiver {
            match receiver.try_recv() {
                Ok(LspEvent::Diagnostics { path, diagnostics }) => {
                    self.diagnostics.insert(path, diagnostics);
                }
                Ok(LspEvent::Exited(message)) => {
                    self.status = message;
                    disconnected = true;
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => {
                    disconnected = true;
                    break;
                }
            }
        }

        if disconnected {
            self.child = None;
            self.stdin = None;
            self.receiver = None;
            self.open_documents.clear();
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    fn send_request(&mut self, method: &str, params: Value) -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": self.next_id,
            "method": method,
            "params": params,
        });
        self.next_id += 1;
        self.write_message(payload)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn send_notification(&mut self, method: &str, params: Value) -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        });
        self.write_message(payload)
    }

    #[allow(clippy::needless_pass_by_value)]
    fn write_message(&mut self, payload: Value) -> Result<()> {
        let message = serde_json::to_vec(&payload)?;
        let stdin = self
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow!("LSP não iniciado"))?;
        write!(stdin, "Content-Length: {}\r\n\r\n", message.len())?;
        stdin.write_all(&message)?;
        stdin.flush()?;
        Ok(())
    }
}

fn read_message(reader: &mut BufReader<impl Read>) -> Result<Option<String>> {
    let mut content_length = None;
    let mut line = String::new();

    loop {
        line.clear();
        if reader.read_line(&mut line)? == 0 {
            return Ok(None);
        }

        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }

        if let Some(value) = trimmed.strip_prefix("Content-Length: ") {
            content_length = Some(value.parse::<usize>()?);
        }
    }

    let content_length = content_length.context("Content-Length ausente")?;
    let mut body = vec![0; content_length];
    reader.read_exact(&mut body)?;
    Ok(Some(String::from_utf8(body)?))
}

fn parse_event(message: &str) -> Option<LspEvent> {
    let value = serde_json::from_str::<Value>(message).ok()?;
    let method = value.get("method")?.as_str()?;
    if method != "textDocument/publishDiagnostics" {
        return None;
    }

    let params = value.get("params")?;
    let uri = params.get("uri")?.as_str()?;
    let path = uri.strip_prefix("file://")?.replace("%20", " ");
    let diagnostics = params
        .get("diagnostics")?
        .as_array()?
        .iter()
        .map(|item| DiagnosticItem {
            line: item
                .get("range")
                .and_then(|range| range.get("start"))
                .and_then(|start| start.get("line"))
                .and_then(Value::as_u64)
                .unwrap_or_default()
                + 1,
            severity: match item.get("severity").and_then(Value::as_u64) {
                Some(1) => "erro",
                Some(2) => "alerta",
                Some(3) => "info",
                Some(4) => "hint",
                _ => "desconhecido",
            }
            .to_string(),
            message: item
                .get("message")
                .and_then(Value::as_str)
                .unwrap_or("sem mensagem")
                .to_string(),
        })
        .collect::<Vec<_>>();

    Some(LspEvent::Diagnostics {
        path: PathBuf::from(path),
        diagnostics,
    })
}

fn file_uri(path: &Path) -> String {
    format!("file://{}", path.display())
}
