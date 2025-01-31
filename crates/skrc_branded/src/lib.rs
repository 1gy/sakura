pub use skrc_branded_derive::branded;

pub trait Tag {
    const NAME: &'static str;
}

#[repr(transparent)]
pub struct Branded<Tag, Inner>(std::marker::PhantomData<Tag>, Inner);

impl<Tag, Inner> From<Inner> for Branded<Tag, Inner> {
    fn from(inner: Inner) -> Self {
        Branded(std::marker::PhantomData, inner)
    }
}

impl<Tag, Inner> std::ops::Deref for Branded<Tag, Inner> {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<Tag, Inner> AsRef<Inner> for Branded<Tag, Inner> {
    fn as_ref(&self) -> &Inner {
        &self.1
    }
}

impl<Tag, Inner: PartialEq> PartialEq for Branded<Tag, Inner> {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl<Tag, Inner: Clone> Clone for Branded<Tag, Inner> {
    fn clone(&self) -> Self {
        Branded(std::marker::PhantomData, self.1.clone())
    }
}

impl<Tag, Inner: Copy> Copy for Branded<Tag, Inner> {}

impl<Tag: crate::Tag, Inner: std::fmt::Debug> std::fmt::Debug for Branded<Tag, Inner> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple(Tag::NAME).field(&self.1).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestNumberTag;

    impl Tag for TestNumberTag {
        const NAME: &'static str = "TestNumber";
    }

    type TestNumber = Branded<TestNumberTag, u8>;

    #[test]
    fn test_branded_from() {
        let value = TestNumber::from(42);
        assert_eq!(value, Branded(std::marker::PhantomData, 42));
    }

    #[test]
    fn test_branded_into() {
        let value: TestNumber = 42.into();
        assert_eq!(value, Branded(std::marker::PhantomData, 42));
    }

    #[test]
    fn test_branded_deref() {
        let value = TestNumber::from(42);
        assert_eq!(*value, 42);
    }

    #[test]
    fn test_branded_as_ref() {
        let value = TestNumber::from(42);
        assert_eq!(value.as_ref(), &42);
    }

    #[test]
    fn test_branded_eq() {
        let value = TestNumber::from(42);
        let other = TestNumber::from(42);
        let different = TestNumber::from(43);
        assert_eq!(value, other);
        assert_ne!(value, different);
    }

    #[test]
    #[allow(clippy::clone_on_copy)]
    fn test_branded_clone() {
        let value = TestNumber::from(42);
        let cloned = value.clone();
        assert_eq!(value, cloned);
    }

    #[test]
    fn test_branded_copy() {
        let value = TestNumber::from(42);
        let copied = value;
        assert_eq!(value, copied);
    }

    #[test]
    fn test_branded_debug() {
        let value = TestNumber::from(42);
        assert_eq!(format!("{:?}", value), "TestNumber(42)");
    }
}
