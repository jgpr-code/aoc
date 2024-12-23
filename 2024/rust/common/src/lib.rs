mod answer;
pub mod point;

#[cfg(feature = "test-utils")]
pub mod test_utils;

pub use answer::Answer;
pub use regex;

#[macro_export]
macro_rules! regx {
    ($re:literal) => {{
        static RE: std::sync::LazyLock<$crate::regex::Regex> =
            std::sync::LazyLock::new(|| $crate::regex::Regex::new($re).unwrap());
        &RE
    }};
}
