#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct SqlParser;

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------
    // Identifiers
    // ------------------------------------------------------------------

    #[test]
    fn qualified_identifier_3() {
        parses_to! {
            parser: SqlParser,
            input: "a.b.c",
            rule: Rule::qualified_identifier,
            tokens: [
                qualified_identifier(0, 5, [
                    identifier(0, 1),
                    identifier(2, 3),
                    identifier(4, 5)
                ])
            ]
        };
    }

    // ------------------------------------------------------------------
    // Columns
    // ------------------------------------------------------------------

    #[test]
    fn columns_all_columns() {
        parses_to! {
            parser: SqlParser,
            input: "*",
            rule: Rule::columns,
            tokens: [
                columns(0, 1, [
                    all_columns(0, 1)
                ])
            ]
        };
    }

    #[test]
    fn columns_all_columns_from() {
        parses_to! {
            parser: SqlParser,
            input: "a.*",
            rule: Rule::columns,
            tokens: [
                columns(0, 3, [
                    all_columns_from(0, 3, [
                        identifier(0, 1)
                    ])
                ])
            ]
        };
    }
}
