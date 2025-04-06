use color_eyre::eyre::{eyre, Context, Result};
use ratatui::Terminal;


#[cfg(feature = "web-backend")]
pub fn terminal() -> Result<Terminal<ratzilla::DomBackend>> {
    let backend = ratzilla::DomBackend::new_by_id("content")
        .map_err(|e| eyre!("{e}"))?;

    Terminal::new(backend)
        .wrap_err("failed to initialize terminal")
}

#[cfg(feature = "crossterm-backend")]
pub fn terminal() -> Result<ratatui::DefaultTerminal> {
    Ok(ratatui::try_init()
        .wrap_err("failed to initialize terminal")?)
}