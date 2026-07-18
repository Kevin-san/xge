//! Deref 宏 — 为 newtype 自动实现 Deref/DerefMut

/// 为 newtype 自动实现 Deref 和 DerefMut
///
/// # Example
/// ```ignore
/// struct Wrapper(Vec<u8>);
/// impl_deref!(Wrapper, Vec<u8>);
/// ```
#[macro_export]
macro_rules! impl_deref {
    ($type:ty, $target:ty) => {
        impl core::ops::Deref for $type {
            type Target = $target;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for $type {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[cfg(test)]
mod tests {
    struct Wrapper(Vec<u8>);

    impl_deref!(Wrapper, Vec<u8>);

    #[test]
    fn test_deref_macro() {
        let mut w = Wrapper(vec![1, 2, 3]);
        assert_eq!(w.len(), 3);
        w.push(4);
        assert_eq!(w[3], 4);
    }
}
