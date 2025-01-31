mod integration_test {
    #[test]
    fn test_branded_derive() {
        #[skrc_branded::branded]
        type TestNumber = u8;

        let value = TestNumber::from(42);
        let other = 42.into();
        let different = 43.into();
        assert_eq!(value, other);
        assert_ne!(value, different);
        assert_eq!(*value, 42);
        assert_eq!(value.as_ref(), &42);
    }
}
