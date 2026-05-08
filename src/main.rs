use rust_ide::{app, config, ui};

use std::{
    io::{self, Write},
    process::Command,
    time::{Duration, Instant},
};

use anyhow::Result;
use app::{App, AppAction};
use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

fn main() -> Result<()> {
    let workspace_root = std::env::current_dir()?;
    let config = config::AppConfig::load(&workspace_root)?;
    let mut app = App::new(workspace_root, config)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    let result = loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match app.handle_key(key)? {
                AppAction::Continue => {}
                AppAction::Quit => break Ok(()),
                AppAction::RunCommand(command) => {
                    suspend_terminal(&mut terminal)?;
                    let status = run_interactive_command(&command);
                    resume_terminal(&mut terminal)?;
                    app.after_command(status)?;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick()?;
            last_tick = Instant::now();
        }
    };

    restore_terminal(&mut terminal)?;
    result
}

fn suspend_terminal(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn resume_terminal(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    terminal.clear()?;
    Ok(())
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_interactive_command(command: &str) -> Result<String> {
    let status = Command::new("sh").arg("-lc").arg(command).status()?;

    println!();
    if status.success() {
        println!("Comando finalizado: {command}");
    } else {
        println!("Comando falhou ({status}): {command}");
    }
    print!("Pressione Enter para voltar para a IDE...");
    io::stdout().flush()?;

    let mut line = String::new();
    io::stdin().read_line(&mut line)?;

    Ok(if status.success() {
        format!("Comando executado: {command}")
    } else {
        format!("Falha ao executar: {command}")
    })
}
