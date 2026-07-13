//! Defines internal enum, regular-expression, and colocated-test macros.

/// A macro which allows the ability to easily define an enum with variants which can easily be
/// converted to and from strings.
macro_rules! convertable_enum {
    ($(#[$attr:meta])* $name:ident, $($(#[$vattr:meta])* $variant:ident => $value:expr),* $(,)?) => {
        $(#[$attr])*
        #[non_exhaustive]
        #[derive(Debug, PartialEq, Eq, Clone)]

        pub enum $name {
            $($(#[$vattr])* $variant),*
        }

        impl std::str::FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $($value => Ok(Self::$variant),)*
                    _ => Err(()),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s = match self {
                    $(Self::$variant => $value,)*
                };
                f.write_str(s)
            }
        }
    };
}
pub(crate) use convertable_enum;

/// A macro which allows the ability to easily aggregate a list of strings into a single regular
/// expression.
macro_rules! recat {
    ($first:expr, $($rest:expr),* $(,)?) => {
        const_format::concatcp!(
            "(?:",
            $first,
            $(const_format::concatcp!("|",$rest)),*,
            ")"
        )
    };
}
pub(crate) use recat;

/// Includes a colocated unit-test module only when compiling tests.
macro_rules! unit_tests {
    ($path:literal) => {
        #[cfg(test)]
        #[path = $path]
        mod tests;
    };
}
pub(crate) use unit_tests;
