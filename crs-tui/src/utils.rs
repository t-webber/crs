//! Defines reusable functions for lousy redundant tasks

use std::sync::{Mutex, MutexGuard};

/// Safely unlock a mutex without panicking.
///
/// If the mutex is poisened, the data continues to be read and written.
///
/// If compiled in debug mode, a poised mutex will crash the app.
pub fn safe_unlock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poison) => {
            debug_assert!(false, "Mutex was poisoned");
            poison.into_inner()
        }
    }
}
