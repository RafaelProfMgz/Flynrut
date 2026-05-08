use std::{
    env, fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use anyhow::Result;
use git2::{BranchType, Repository, Status, StatusOptions};

use crate::config::{AppConfig, ToolCommands};

#[derive(Debug, Clone)]
pub struct ToolStatus {
    pub name: &'static str,
    pub command: Option<String>,
    pub available: bool,
}

#[derive(Debug, Clone)]
pub struct GitStatusSnapshot {
    pub repository_root: PathBuf,
    pub branch: String,
    pub staged: usize,
    pub unstaged: usize,
    pub untracked: usize,
    pub ahead: usize,
    pub behind: usize,
}

#[derive(Debug)]
pub struct IntegrationState {
    pub tools: Vec<ToolStatus>,
    pub git: Option<GitStatusSnapshot>,
    pub config_path: PathBuf,
    last_git_refresh: Instant,
}

impl IntegrationState {
    #[must_use]
    pub fn discover(workspace_root: &Path, config: &AppConfig) -> Self {
        let tools = detect_tools(&config.tools);
        let git = git_status(workspace_root).ok();

        Self {
            tools,
            git,
            config_path: config.config_path.clone(),
            last_git_refresh: Instant::now(),
        }
    }

    pub fn refresh(&mut self, workspace_root: &Path) {
        if self.last_git_refresh.elapsed() >= Duration::from_secs(2) {
            self.git = git_status(workspace_root).ok();
            self.last_git_refresh = Instant::now();
        }
    }

    pub fn refresh_now(&mut self, workspace_root: &Path, tools: &ToolCommands) {
        self.tools = detect_tools(tools);
        self.git = git_status(workspace_root).ok();
        self.last_git_refresh = Instant::now();
    }
}

fn detect_tools(commands: &ToolCommands) -> Vec<ToolStatus> {
    vec![
        ToolStatus {
            name: "git",
            command: Some("git".to_string()),
            available: binary_on_path("git"),
        },
        ToolStatus {
            name: "lazygit",
            command: Some(commands.lazygit.clone()),
            available: binary_for_command(commands.lazygit.as_str()).is_some_and(binary_on_path),
        },
        ToolStatus {
            name: "lazydocker",
            command: Some(commands.lazydocker.clone()),
            available: binary_for_command(commands.lazydocker.as_str()).is_some_and(binary_on_path),
        },
        ToolStatus {
            name: "ai",
            command: commands.ai.clone(),
            available: commands
                .ai
                .as_deref()
                .and_then(binary_for_command)
                .is_some_and(binary_on_path),
        },
        ToolStatus {
            name: "mcp",
            command: commands.mcp.clone(),
            available: commands
                .mcp
                .as_deref()
                .and_then(binary_for_command)
                .is_some_and(binary_on_path),
        },
    ]
}

fn git_status(workspace_root: &Path) -> Result<GitStatusSnapshot> {
    let repository = Repository::discover(workspace_root)?;
    let repository_root = repository
        .workdir()
        .unwrap_or_else(|| repository.path())
        .to_path_buf();

    let head = repository.head().ok();
    let branch = head
        .as_ref()
        .and_then(|head| head.shorthand())
        .unwrap_or("detached")
        .to_string();

    let mut status_options = StatusOptions::new();
    status_options
        .include_untracked(true)
        .renames_head_to_index(true)
        .recurse_untracked_dirs(true);

    let statuses = repository.statuses(Some(&mut status_options))?;
    let mut staged = 0;
    let mut unstaged = 0;
    let mut untracked = 0;

    for entry in statuses.iter() {
        let status = entry.status();
        if status.contains(Status::WT_NEW) {
            untracked += 1;
        }
        if status.intersects(
            Status::INDEX_NEW
                | Status::INDEX_MODIFIED
                | Status::INDEX_DELETED
                | Status::INDEX_RENAMED
                | Status::INDEX_TYPECHANGE,
        ) {
            staged += 1;
        }
        if status.intersects(
            Status::WT_MODIFIED | Status::WT_DELETED | Status::WT_RENAMED | Status::WT_TYPECHANGE,
        ) {
            unstaged += 1;
        }
    }

    let (ahead, behind) = upstream_delta(&repository, &branch).unwrap_or_default();

    Ok(GitStatusSnapshot {
        repository_root,
        branch,
        staged,
        unstaged,
        untracked,
        ahead,
        behind,
    })
}

fn upstream_delta(repository: &Repository, branch_name: &str) -> Result<(usize, usize)> {
    let branch = repository.find_branch(branch_name, BranchType::Local)?;
    let upstream = branch.upstream()?;
    let branch_target = branch.get().target();
    let upstream_target = upstream.get().target();

    match (branch_target, upstream_target) {
        (Some(branch_target), Some(upstream_target)) => {
            let (ahead, behind) = repository.graph_ahead_behind(branch_target, upstream_target)?;
            Ok((ahead, behind))
        }
        _ => Ok((0, 0)),
    }
}

fn binary_on_path(binary: &str) -> bool {
    if binary.contains('/') {
        return fs::metadata(binary).is_ok();
    }

    env::var_os("PATH")
        .into_iter()
        .flat_map(|value| env::split_paths(&value).collect::<Vec<_>>())
        .map(|dir| dir.join(binary))
        .any(|candidate| candidate.exists())
}

pub(crate) fn binary_for_command(command: &str) -> Option<&str> {
    command.split_whitespace().next()
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    // --- binary_for_command ---

    #[test]
    fn binary_for_simple_command() {
        assert_eq!(binary_for_command("lazygit"), Some("lazygit"));
    }

    #[test]
    fn binary_for_command_with_args() {
        assert_eq!(
            binary_for_command("npx @modelcontextprotocol/inspector"),
            Some("npx")
        );
    }

    #[test]
    fn binary_for_empty_command_is_none() {
        assert_eq!(binary_for_command(""), None);
    }

    #[test]
    fn binary_for_command_with_path() {
        assert_eq!(
            binary_for_command("/usr/local/bin/lazygit --no-ui"),
            Some("/usr/local/bin/lazygit")
        );
    }

    // --- detect_tools detects git when present ---

    #[test]
    fn git_tool_detected_when_on_path() {
        use crate::config::ToolCommands;
        let tools = ToolCommands {
            lazygit: "lazygit".to_string(),
            lazydocker: "lazydocker".to_string(),
            ai: None,
            mcp: None,
        };
        let detected = detect_tools(&tools);
        let git_entry = detected.iter().find(|t| t.name == "git");
        assert!(git_entry.is_some(), "git tool entry must exist");
        // git is universally available in CI/dev environments
        assert!(
            git_entry.unwrap().available,
            "git binary should be on PATH in this environment"
        );
    }

    // --- IntegrationState tool list ---

    #[test]
    fn integration_state_has_all_required_tool_entries() {
        use crate::config::AppConfig;
        use std::path::Path;

        let config = AppConfig::load(Path::new("/tmp")).expect("config load");
        let state = IntegrationState::discover(Path::new("/tmp"), &config);

        let tool_names: Vec<_> = state.tools.iter().map(|t| t.name).collect();
        for required in ["git", "lazygit", "lazydocker", "ai", "mcp"] {
            assert!(
                tool_names.contains(&required),
                "missing tool entry: {required}"
            );
        }
    }
}
