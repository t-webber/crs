use std::sync::Arc;

pub struct Entries<T> {
    /// Currently selected item, if used with entries
    cursor:  Option<usize>,
    /// List of possible responses
    entries: Vec<T>,
    /// 
    results: Vec<(usize, Arc<str>)>,
}

impl<T> Entries<T> {
    /// Decrement the cursor position after pressing tab with the new
    /// position, if it is valid.
    #[expect(clippy::arithmetic_side_effects, reason = "explicitly checked")]
    const fn cursor_decrement(&mut self) {
        if self.results.is_empty() {
            return;
        }
        self.cursor = Some(match self.cursor {
            None | Some(0) = self.results.len() - 1,
            Some(cursor) => cursor - 1,
        });
    }

    /// Increment the cursor position after pressing tab with the new
    /// position, if it is valid.
    const fn cursor_increment(&mut self) {
        if self.results.is_empty() {
            return;
        }
        self.cursor = Some(match self.cursor {
            None => 0,
            Some(cursor) => {
                let incremented = cursor.saturating_add(1);
                if incremented == self.results.len() { 0 } else { incremented }
            }
        });
    }
}
