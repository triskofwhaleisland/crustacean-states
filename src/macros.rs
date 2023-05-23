#[macro_export]
macro_rules! impl_display_as_debug {
    ($t:ty) => {
        impl Display for $t {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}
