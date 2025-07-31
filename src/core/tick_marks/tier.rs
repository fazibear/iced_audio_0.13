/// Tier of sizes for a tick mark.
///
/// * One - large-sized tick mark
/// * Two - medium-sized tick mark
/// * Small - small-sized tick mark
#[derive(Debug, Copy, Clone, Eq, PartialEq, std::hash::Hash)]
pub enum Tier {
    /// large-sized tick mark
    One,
    /// medium-sized tick mark
    Two,
    /// small-sized tick mark
    Three,
}

impl Default for Tier {
    fn default() -> Self {
        Tier::One
    }
}
