use crate::color::blend;
use crate::color::is_light;
use crate::terminal_palette::best_color;
use crate::terminal_palette::default_bg;
use codex_config::types::TuiBackgroundMode;
use ratatui::style::Color;
use ratatui::style::Style;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

const TUI_BACKGROUND_MODE_AUTO: u8 = 0;
const TUI_BACKGROUND_MODE_OPAQUE: u8 = 1;
const TUI_BACKGROUND_MODE_CLASSIC: u8 = 2;
const TUI_BACKGROUND_MODE_TRANSPARENT: u8 = 3;

static TUI_BACKGROUND_MODE: AtomicU8 = AtomicU8::new(TUI_BACKGROUND_MODE_AUTO);

fn encode_tui_background_mode(mode: TuiBackgroundMode) -> u8 {
    match mode {
        TuiBackgroundMode::Auto => TUI_BACKGROUND_MODE_AUTO,
        TuiBackgroundMode::Opaque => TUI_BACKGROUND_MODE_OPAQUE,
        TuiBackgroundMode::Classic => TUI_BACKGROUND_MODE_CLASSIC,
        TuiBackgroundMode::Transparent => TUI_BACKGROUND_MODE_TRANSPARENT,
    }
}

fn decode_tui_background_mode(mode: u8) -> TuiBackgroundMode {
    match mode {
        TUI_BACKGROUND_MODE_OPAQUE => TuiBackgroundMode::Opaque,
        TUI_BACKGROUND_MODE_CLASSIC => TuiBackgroundMode::Classic,
        TUI_BACKGROUND_MODE_TRANSPARENT => TuiBackgroundMode::Transparent,
        _ => TuiBackgroundMode::Auto,
    }
}

pub(crate) fn set_tui_background_mode(mode: TuiBackgroundMode) {
    TUI_BACKGROUND_MODE.store(encode_tui_background_mode(mode), Ordering::Relaxed);
}

pub(crate) fn tui_background_mode() -> TuiBackgroundMode {
    decode_tui_background_mode(TUI_BACKGROUND_MODE.load(Ordering::Relaxed))
}

fn auto_uses_transparent_background() -> bool {
    false
}

pub(crate) fn surface_background_transparency_enabled() -> bool {
    match tui_background_mode() {
        TuiBackgroundMode::Transparent => true,
        TuiBackgroundMode::Classic | TuiBackgroundMode::Opaque => false,
        TuiBackgroundMode::Auto => auto_uses_transparent_background(),
    }
}

pub(crate) fn diff_background_transparency_enabled() -> bool {
    match tui_background_mode() {
        TuiBackgroundMode::Transparent | TuiBackgroundMode::Classic => true,
        TuiBackgroundMode::Opaque => false,
        TuiBackgroundMode::Auto => auto_uses_transparent_background(),
    }
}

pub(crate) fn classic_diff_style_enabled() -> bool {
    matches!(tui_background_mode(), TuiBackgroundMode::Classic)
}

pub fn user_message_style() -> Style {
    user_message_style_for(default_bg(), surface_background_transparency_enabled())
}

pub fn proposed_plan_style() -> Style {
    proposed_plan_style_for(default_bg(), surface_background_transparency_enabled())
}

/// Returns the style for a user-authored message using the provided terminal background.
pub fn user_message_style_for(
    terminal_bg: Option<(u8, u8, u8)>,
    transparent_background: bool,
) -> Style {
    surface_style_for(terminal_bg, transparent_background, user_message_bg)
}

pub fn proposed_plan_style_for(
    terminal_bg: Option<(u8, u8, u8)>,
    transparent_background: bool,
) -> Style {
    surface_style_for(terminal_bg, transparent_background, proposed_plan_bg)
}

fn surface_style_for(
    terminal_bg: Option<(u8, u8, u8)>,
    transparent_background: bool,
    bg_fn: impl FnOnce((u8, u8, u8)) -> Color,
) -> Style {
    if transparent_background {
        return Style::default();
    }
    match terminal_bg {
        Some(bg) => Style::default().bg(bg_fn(bg)),
        None => Style::default(),
    }
}

#[allow(clippy::disallowed_methods)]
pub fn user_message_bg(terminal_bg: (u8, u8, u8)) -> Color {
    let (top, alpha) = if is_light(terminal_bg) {
        ((0, 0, 0), 0.04)
    } else {
        ((255, 255, 255), 0.12)
    };
    best_color(blend(top, terminal_bg, alpha))
}

#[allow(clippy::disallowed_methods)]
pub fn proposed_plan_bg(terminal_bg: (u8, u8, u8)) -> Color {
    user_message_bg(terminal_bg)
}

#[cfg(test)]
pub(crate) fn with_tui_background_mode_for_test<T>(
    mode: TuiBackgroundMode,
    f: impl FnOnce() -> T,
) -> T {
    use std::sync::Mutex;
    use std::sync::OnceLock;

    static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    struct ResetGuard {
        previous: TuiBackgroundMode,
    }

    impl Drop for ResetGuard {
        fn drop(&mut self) {
            set_tui_background_mode(self.previous);
        }
    }

    let _guard = TEST_MUTEX
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let reset_guard = ResetGuard {
        previous: tui_background_mode(),
    };
    set_tui_background_mode(mode);
    let result = f();
    drop(reset_guard);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn transparent_user_message_style_omits_background_fill() {
        assert_eq!(
            user_message_style_for(Some((12, 34, 56)), /*transparent_background*/ true),
            Style::default(),
        );
    }

    #[test]
    fn transparent_proposed_plan_style_omits_background_fill() {
        assert_eq!(
            proposed_plan_style_for(Some((12, 34, 56)), /*transparent_background*/ true),
            Style::default(),
        );
    }

    #[test]
    fn classic_mode_keeps_user_message_background_fill() {
        with_tui_background_mode_for_test(TuiBackgroundMode::Classic, || {
            let bg = user_message_style_for(
                Some((12, 34, 56)),
                surface_background_transparency_enabled(),
            );
            assert!(bg.bg.is_some());
        });
    }

    #[test]
    fn surface_background_transparency_enabled_follows_mode() {
        with_tui_background_mode_for_test(TuiBackgroundMode::Auto, || {
            assert!(!surface_background_transparency_enabled());
        });
        with_tui_background_mode_for_test(TuiBackgroundMode::Opaque, || {
            assert!(!surface_background_transparency_enabled());
        });
        with_tui_background_mode_for_test(TuiBackgroundMode::Classic, || {
            assert!(!surface_background_transparency_enabled());
        });
        with_tui_background_mode_for_test(TuiBackgroundMode::Transparent, || {
            assert!(surface_background_transparency_enabled());
        });
    }

    #[test]
    fn diff_background_transparency_enabled_follows_mode() {
        with_tui_background_mode_for_test(TuiBackgroundMode::Auto, || {
            assert!(!diff_background_transparency_enabled());
        });
        with_tui_background_mode_for_test(TuiBackgroundMode::Opaque, || {
            assert!(!diff_background_transparency_enabled());
        });
        with_tui_background_mode_for_test(TuiBackgroundMode::Classic, || {
            assert!(diff_background_transparency_enabled());
        });
        with_tui_background_mode_for_test(TuiBackgroundMode::Transparent, || {
            assert!(diff_background_transparency_enabled());
        });
    }

    #[test]
    fn classic_diff_style_enabled_only_for_classic_mode() {
        with_tui_background_mode_for_test(TuiBackgroundMode::Auto, || {
            assert!(!classic_diff_style_enabled());
        });
        with_tui_background_mode_for_test(TuiBackgroundMode::Opaque, || {
            assert!(!classic_diff_style_enabled());
        });
        with_tui_background_mode_for_test(TuiBackgroundMode::Classic, || {
            assert!(classic_diff_style_enabled());
        });
        with_tui_background_mode_for_test(TuiBackgroundMode::Transparent, || {
            assert!(!classic_diff_style_enabled());
        });
    }
}
