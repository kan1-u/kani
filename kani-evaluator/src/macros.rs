#[doc(hidden)]
#[macro_export]
macro_rules! impl_from_enum {
    ($e:tt::$t:tt, $f:ty) => {
        impl From<$f> for $e {
            fn from(expression: $f) -> Self {
                Self::$t(expression)
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_deref {
    ($ty:ty, $ta:ty) => {
        impl std::ops::Deref for $ty {
            type Target = $ta;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}
