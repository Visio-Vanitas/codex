use crate::color::blend;
use crate::color::is_light;
use crate::terminal_palette::best_color;
use crate::terminal_palette::default_bg;
use ratatui::style::Color;
use ratatui::style::Style;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

static TRANSPARENT_BACKGROUND: AtomicBool = AtomicBool::new(false);

pub(crate) fn set_transparent_background(enabled: bool) {
    TRANSPARENT_BACKGROUND.store(enabled, Ordering::Relaxed);
}

pub(crate) fn transparent_background_enabled() -> bool {
    TRANSPARENT_BACKGROUND.load(Ordering::Relaxed)
}

pub fn user_message_style() -> Style {
    user_message_style_for(default_bg(), transparent_background_enabled())
}

pub fn proposed_plan_style() -> Style {
    proposed_plan_style_for(default_bg(), transparent_background_enabled())
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
pub(crate) fn with_transparent_background_for_test<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
    use std::sync::Mutex;
    use std::sync::OnceLock;

    static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

    struct ResetGuard {
        previous: bool,
    }

    impl Drop for ResetGuard {
        fn drop(&mut self) {
            set_transparent_background(self.previous);
        }
    }

    let _guard = TEST_MUTEX
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let reset_guard = ResetGuard {
        previous: transparent_background_enabled(),
    };
    set_transparent_background(enabled);
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
}
