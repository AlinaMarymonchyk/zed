use crate::{Anchor, Editor};
use gpui::{Context, Window};

/// An active type-to-accept session attached to an editor.
///
/// When the AI agent proposes a change, instead of accepting it immediately
/// the user must type each character of the new text. The buffer already
/// contains the agent's text; the session tracks the user's progress through
/// that text and gates the final acceptance on full completion.
pub struct TypeToAcceptSession {
    /// The text the user must type, in UTF-8 order.
    pub(crate) target_text: String,
    /// Byte offset into `target_text` of how far the user has typed.
    pub(crate) bytes_typed: usize,
    /// Number of wrong keystrokes made during the session.
    pub(crate) error_count: usize,
    /// MultiBuffer anchor at the start of the hunk (where typing begins).
    pub(crate) start_anchor: Anchor,
    /// MultiBuffer anchor at the end of the hunk (where typing ends).
    pub(crate) end_anchor: Anchor,
    /// Callback invoked when the session completes or is skipped.
    pub(crate) on_complete: Option<Box<dyn FnOnce(&mut Window, &mut Context<Editor>)>>,
}

impl TypeToAcceptSession {
    pub fn new(
        target_text: String,
        start_anchor: Anchor,
        end_anchor: Anchor,
        on_complete: Box<dyn FnOnce(&mut Window, &mut Context<Editor>)>,
    ) -> Self {
        Self {
            target_text,
            bytes_typed: 0,
            error_count: 0,
            start_anchor,
            end_anchor,
            on_complete: Some(on_complete),
        }
    }

    /// Returns the expected next character, or `None` if the session is complete.
    pub fn next_expected_char(&self) -> Option<char> {
        self.target_text[self.bytes_typed..].chars().next()
    }

    /// Returns true if the user has typed all characters.
    pub fn is_complete(&self) -> bool {
        self.bytes_typed >= self.target_text.len()
    }

    /// Progress as a value in `[0.0, 1.0]`.
    pub fn progress(&self) -> f32 {
        if self.target_text.is_empty() {
            1.0
        } else {
            self.bytes_typed as f32 / self.target_text.len() as f32
        }
    }

    /// Total number of logical characters in the target text.
    pub fn total_chars(&self) -> usize {
        self.target_text.chars().count()
    }

    /// Number of characters typed so far.
    pub fn typed_chars(&self) -> usize {
        self.target_text[..self.bytes_typed].chars().count()
    }

    pub fn error_count(&self) -> usize {
        self.error_count
    }
}
