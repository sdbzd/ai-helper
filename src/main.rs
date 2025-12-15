//! Entry point placeholder. The real Tauri bootstrap would live here.

use ai_helper::AppContext;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ctx = AppContext::new();
    // TODO: wire Tauri here; for now we just log the skeleton init.
    println!("ai-helper skeleton initialized: {ctx:?}");
    Ok(())
}

