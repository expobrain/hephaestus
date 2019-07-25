#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/build/parse_into.rs");
    t.compile_fail("tests/build/enum_into.rs");
    t.compile_fail("tests/build/unsupported_into.rs");
}
