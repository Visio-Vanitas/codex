//! Builds the `/theme background` picker dialog for the TUI.

use codex_config::types::TuiBackgroundMode;

use crate::app_event::AppEvent;
use crate::bottom_pane::SelectionItem;
use crate::bottom_pane::SelectionViewParams;
use crate::bottom_pane::popup_consts::standard_popup_hint_line;

#[derive(Clone, Copy)]
struct BackgroundModeOption {
    mode: TuiBackgroundMode,
    label: &'static str,
    description: &'static str,
}

const BACKGROUND_MODE_OPTIONS: [BackgroundModeOption; 4] = [
    BackgroundModeOption {
        mode: TuiBackgroundMode::Auto,
        label: "Auto",
        description: "Use Codex default background rendering behavior.",
    },
    BackgroundModeOption {
        mode: TuiBackgroundMode::Opaque,
        label: "Opaque",
        description: "Always paint Codex panel and diff backgrounds.",
    },
    BackgroundModeOption {
        mode: TuiBackgroundMode::Classic,
        label: "Classic",
        description: "Keep standard UI backgrounds and restore the classic diff style.",
    },
    BackgroundModeOption {
        mode: TuiBackgroundMode::Transparent,
        label: "Transparent",
        description: "Do not paint panel and diff backgrounds.",
    },
];

pub(crate) fn build_theme_background_picker_params(
    current_mode: TuiBackgroundMode,
) -> SelectionViewParams {
    let mut initial_selected_idx: Option<usize> = None;
    let mut items: Vec<SelectionItem> = Vec::with_capacity(BACKGROUND_MODE_OPTIONS.len());

    for (idx, option) in BACKGROUND_MODE_OPTIONS.into_iter().enumerate() {
        if option.mode == current_mode {
            initial_selected_idx = Some(idx);
        }
        items.push(SelectionItem {
            name: option.label.to_string(),
            description: Some(option.description.to_string()),
            is_current: option.mode == current_mode,
            is_default: option.mode == TuiBackgroundMode::Auto,
            actions: vec![Box::new(move |tx| {
                tx.send(AppEvent::TuiBackgroundModeSelected { mode: option.mode });
            })],
            dismiss_on_select: true,
            ..Default::default()
        });
    }

    SelectionViewParams {
        title: Some("Choose a background mode".to_string()),
        subtitle: Some("Controls how Codex paints panel and diff backgrounds.".to_string()),
        footer_hint: Some(standard_popup_hint_line()),
        items,
        initial_selected_idx,
        ..Default::default()
    }
}
