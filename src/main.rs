use anyhow::Result;
use rust_ide::{app::App, config, gui::IdeApp};

fn main() -> Result<()> {
    let workspace_root = std::env::current_dir()?;
    let config = config::AppConfig::load(&workspace_root)?;
    let app = App::new(workspace_root, config)?;
    IdeApp::run(app)
}
