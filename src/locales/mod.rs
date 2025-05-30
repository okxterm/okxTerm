use std::fs;

use anyhow::anyhow;
use log::error;
use once_cell::sync::OnceCell;

pub mod ui;

static UI_LANG: OnceCell<ui::Ui> = OnceCell::new();

pub fn load(lang: &str) -> anyhow::Result<()> {
    // load ui
    let ui_content = fs::read_to_string(format!("./locales/{}/ui.toml", lang)).map_err(|err| {
        error!("{}", err);
        anyhow!(err)
    })?;

    let ui_lang: ui::Ui = toml::from_str(&ui_content).map_err(|err| {
        error!("{}", err);
        anyhow!(err)
    })?;

    UI_LANG
        .set(ui_lang)
        .map_err(|_| anyhow!("Failed to set UI LANG")).map_err(|err| {
            error!("{}", err);
            anyhow!(err)
        })?;
    Ok(())
}

pub fn ui() -> ui::Ui {
    let ui_lang = UI_LANG.get().expect("Failed to get UI LANG");
    ui_lang.clone()
}
