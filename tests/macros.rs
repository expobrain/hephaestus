#[allow(unused_macros)]
macro_rules! test_builder {
    ($name:ident, $sql:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let result = parse($sql).unwrap();

            assert_eq!(result, $expected);
        }
    };
}

#[allow(unused_macros)]
macro_rules! test_formatter {
    ($name:ident, $sql:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let result = format($sql).unwrap();

            assert_eq!(result, $expected);
        }
    };
}
