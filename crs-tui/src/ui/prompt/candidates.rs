//! Displays the selection part of the prompt

use core::fmt::Display;

/// List of possible candidates to display
pub struct Candidates<T: Display> {
    /// List of all the possible candidates, whether they correspond to the
    /// search or not
    all_candidates: Vec<T>,
    /// Currently selected item, if used with entries
    cursor:         Option<usize>,
}

impl<T: Display> Candidates<T> {
    /// Decrement the cursor position after pressing tab with the new
    /// position, if it is valid.
    #[expect(clippy::arithmetic_side_effects, reason = "explicitly checked")]
    const fn cursor_decrement(&mut self) {
        if self.all_candidates.is_empty() {
            return;
        }
        self.cursor = Some(match self.cursor {
            None | Some(0) => self.all_candidates.len() - 1,
            Some(cursor) => cursor - 1,
        });
    }

    /// Increment the cursor position after pressing tab with the new
    /// position, if it is valid.
    const fn cursor_increment(&mut self) {
        if self.all_candidates.is_empty() {
            return;
        }
        self.cursor = Some(match self.cursor {
            None => 0,
            Some(cursor) => {
                let incremented = cursor.saturating_add(1);
                if incremented == self.all_candidates.len() {
                    0
                } else {
                    incremented
                }
            }
        });
    }

    /// Returns the first possible entries that match the search
    pub fn get_possibilites(
        &self,
        max_number: usize,
        input: &str,
    ) -> Vec<String> {
        self.all_candidates
            .iter()
            .filter_map(|entry| {
                let formatted = format!("{entry}");
                formatted.contains(input).then_some(formatted)
            })
            .take(max_number)
            .collect()
    }

    /// Returns a new empty [`Candidates`]
    pub const fn new() -> Self {
        Self { cursor: None, all_candidates: vec![] }
    }

    /// Returns a new empty [`Candidates`]
    pub const fn new_with_list(list: Vec<T>) -> Self {
        Self { cursor: None, all_candidates: list }
    }
}
