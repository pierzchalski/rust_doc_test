//!
//! Module documentation for magic inside of magic/mod.rs!

/// Documentation for `things` inside of magic/mod.rs!
///
/// This shouldn't be visible, since things isn't public.
mod things;

pub use self::things::my_magic_library_function;

/// Documentation for struct Thing!
pub struct Thing {
    pub thing: usize,
    otherthing: usize,
}

impl Thing {
    /// Documentation for Thing::new!
    pub fn new(otherthing: usize) -> Self {
        Thing {
            thing: 0,
            otherthing: otherthing,
        }
    }

    pub fn otherthing(&self) -> usize {
        self.otherthing
    }
}
