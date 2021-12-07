//! The HideState enum allows for coordinating the CSS animations for transitioning an
//! HTML elements state between "hidden" and "visible" with setting their "display"
//! attribute to "none".
//!
//! This is intended to be used as a state-machine as follows:
//!     HideState::Visible(display_none: false)
//!                     ↓  <- Element marked as hidden but needs to fade out
//!     HideState::Hidden(display_none: false)
//!                     ↓  <- Element marked as hidden and has complete fade out
//!     HideState::Hidden(display_none: true)
//!                     ↓  <- Element marked as visible but another element needs to fade out
//!     HideState::Visible(display_none: true)
//!                     ↓  <- Element marked as visible and can now fade in
//!     HideState::Visible(display_none: false)
//!                     ↓
//!                    ...
//!
//! The additional "initial" field allows for elements to be marked such that the
//! CSS animations should not be played, i.e. for when the page is initially loaded.
use crate::utils::some_if;


#[derive(Clone, PartialEq)]
pub struct HideStateInner {
    display_none: bool,
    initial: bool,
}

#[derive(Clone, PartialEq)]
pub enum HideState {
    Hidden(HideStateInner),
    Visible(HideStateInner),
}

impl HideState {

    pub fn new_hidden() -> Self {
        Self::Hidden(HideStateInner { display_none: true, initial: true })
    }

    pub fn new_visible() -> Self {
        Self::Visible(HideStateInner { display_none: false, initial: true })
    }

    pub fn display_none(&self) -> Option<String> {
        match self {
            Self::Hidden(inner) |  Self::Visible(inner) => {
                some_if!(inner.display_none, "display: none;".into())
            },
        }
    }

    pub fn is_hidden(&self) -> bool {
        matches!(self, Self::Hidden(_))
    }

    pub fn toggle(&mut self) {
        match self {
            Self::Hidden(inner) => {
                *self = Self::Visible(
                    HideStateInner{display_none: inner.display_none, initial: false}
                );
            },
            Self::Visible(inner) => {
                *self = Self::Hidden(
                    HideStateInner{display_none: inner.display_none, initial: false}
                );
            }
        }
    }

    pub fn toggle_display_none(&mut self) {
        match self {
            Self::Hidden(inner) | Self::Visible(inner) => {
                inner.display_none = !inner.display_none;
            }
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            Self::Hidden(HideStateInner{ display_none: _, initial: true })
            | Self::Visible(HideStateInner{ display_none: _, initial: true }) => {
                "Initial".into()
            },
            Self::Hidden(_inner) => {
                "Hidden".into()
            }
            Self::Visible(_inner) => {
                "Visible".into()
            }
        }
    }
}
