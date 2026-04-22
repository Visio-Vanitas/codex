//! Builds the `/theme` top-level settings menu for the TUI.

use codex_config::types::TuiBackgroundMode;

use crate::app_event::AppEvent;
use crate::bottom_pane::SelectionItem;
use crate::bottom_pane::SelectionViewParams;
use crate::bottom_pane::popup_consts::standard_popup_hint_line;

fn syntax_theme_description(current_theme: Option<&str>) -> String {
    match current_theme {
        Some(theme) => format!("Choose the syntax highlighting theme. Current: {theme}."),
        None => "Choose the syntax highlighting theme.".to_string(),
    }
}

fn background_mode_description(mode: TuiBackgroundMode) -> String {
    format!("Control panel and diff backgrounds. Current: {mode}.")
}

pub(crate) fn build_theme_menu_params(
    current_theme: Option<&str>,
    current_background_mode: TuiBackgroundMode,
) -> SelectionViewParams {
    let items = vec![
        SelectionItem {
            name: "Syntax Highlighting".to_string(),
            description: Some(syntax_theme_description(current_theme)),
            actions: vec![Box::new(|tx| {
                tx.send(AppEvent::OpenThemeSyntaxPicker);
            })],
            dismiss_on_select: true,
            ..Default::default()
        },
        SelectionItem {
            name: "Background Mode".to_string(),
            description: Some(background_mode_description(current_background_mode)),
            actions: vec![Box::new(|tx| {
                tx.send(AppEvent::OpenThemeBackgroundPicker);
            })],
            dismiss_on_select: true,
            ..Default::default()
        },
    ];

    SelectionViewParams {
        title: Some("Theme Settings".to_string()),
        subtitle: Some(
            "Syntax colors and TUI background rendering are configured separately.".to_string(),
        ),
        footer_hint: Some(standard_popup_hint_line()),
        items,
        ..Default::default()
    }
}
