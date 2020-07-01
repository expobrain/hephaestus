use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::prec_climber::{Assoc, Operator, PrecClimber};
use pest::Parser;

use crate::ast::*;
use crate::parser_pest::*;

lazy_static! {
    static ref PREC_CLIMBER: PrecClimber<Rule> = {
        use Rule::*;

        PrecClimber::new(vec![
            // Binary algebra
            Operator::new(operation_or, Assoc::Left),
            Operator::new(operation_and, Assoc::Left),

            // Comparison
            Operator::new(operation_equal, Assoc::Left)
            | Operator::new(operation_not_equal, Assoc::Left)
            | Operator::new(operation_greater_or_equal, Assoc::Left)
            | Operator::new(operation_greater_than, Assoc::Left)
            | Operator::new(operation_less_or_equal, Assoc::Left)
            | Operator::new(operation_less_than, Assoc::Left),

            // Algebric
            Operator::new(operation_add, Assoc::Left)
            | Operator::new(operation_subtract, Assoc::Left),
            Operator::new(operation_concat, Assoc::Left),
            Operator::new(operation_multiply, Assoc::Left)
            | Operator::new(operation_divide, Assoc::Left),

        ])
    };
}

fn primary(pair: Pair<Rule>) -> AstNode {
    let mut inner_iter = pair.clone().into_inner().filter(|p| match p.as_rule() {
        Rule::COMMENT => false,
        _ => true,
    });

    match pair.as_rule() {
        // ------------------------------------------------------------------
        // Columns
        // ------------------------------------------------------------------
        Rule::all_columns => AstNode::AllColumns,
        Rule::column => AstNode::Column {
            expr: Box::new(parse_value(pair.into_inner())),
        },
        Rule::all_columns_from => AstNode::AllColumnsFrom {
            schema: Box::new(parse_value(pair.into_inner())),
        },
        Rule::named_column => AstNode::NamedColumn {
            expr: Box::new(parse_value(pair.into_inner())),
            alias: inner_iter.nth(1).map(|v| v.as_str().to_string()),
        },

        // ------------------------------------------------------------------
        // Identifiers
        // ------------------------------------------------------------------
        Rule::identifier => AstNode::Identifier {
            s: pair.as_str().to_string(),
        },
        Rule::qualified_identifier => AstNode::QualifiedIdentifier {
            s: inner_iter.map(Pairs::single).map(parse_value).collect(),
        },

        // ------------------------------------------------------------------
        // Literals
        // ------------------------------------------------------------------
        Rule::integer_literal => AstNode::IntegerLiteral {
            s: pair.as_str().to_string(),
        },
        Rule::decimal_literal => AstNode::DecimalLiteral {
            s: pair.as_str().to_string(),
        },
        Rule::float_literal => AstNode::FloatLiteral {
            s: pair.as_str().to_string(),
        },
        Rule::string_literal => AstNode::StringLiteral {
            // Necessary to strip the single quotes around the string
            s: inner_iter.next().unwrap().as_str().to_string(),
        },
        Rule::boolean_literal => AstNode::BooleanLiteral {
            s: pair.as_str().to_string(),
        },
        Rule::interval_year_to_month => AstNode::IntervalLiteral {
            interval: Box::new(parse_value(pair.into_inner())),
            period: Interval::from_str(inner_iter.clone().nth(1).unwrap().as_str()),
            precision: inner_iter
                .clone()
                .find(|p| match p.as_rule() {
                    Rule::interval_precision | Rule::interval_precision_with_fractional => true,
                    _ => false,
                })
                .map(|p| p.into_inner().map(Pairs::single).map(parse_value).collect())
                .unwrap_or_else(|| Vec::with_capacity(0)),
            convert_to: inner_iter
                .clone()
                .skip(2)
                .find(|p| match p.as_rule() {
                    Rule::interval_year | Rule::interval_month => true,
                    _ => false,
                })
                .map(|p| Interval::from_str(p.as_str())),
            convert_precision: None,
        },
        Rule::interval_day_to_second => {
            let conversion = inner_iter
                .clone()
                .find(|p| match p.as_rule() {
                    Rule::interval_day_conversion => true,
                    _ => false,
                })
                .map(|p| p.into_inner());

            AstNode::IntervalLiteral {
                interval: Box::new(parse_value(pair.into_inner())),
                period: Interval::from_str(inner_iter.clone().nth(1).unwrap().as_str()),
                precision: inner_iter
                    .clone()
                    .find(|p| match p.as_rule() {
                        Rule::interval_precision | Rule::interval_precision_with_fractional => true,
                        _ => false,
                    })
                    .map(|p| p.into_inner().map(Pairs::single).map(parse_value).collect())
                    .unwrap_or_else(|| Vec::with_capacity(0)),
                convert_to: conversion
                    .clone()
                    .map(|c| c.clone().next().unwrap().as_str())
                    .map(Interval::from_str),
                convert_precision: conversion
                    .clone()
                    .map(|c| {
                        c.clone()
                            .nth(1)
                            .map(Pairs::single)
                            .map(parse_value)
                            .map(Box::new)
                    })
                    .unwrap_or(None),
            }
        }

        // ------------------------------------------------------------------
        // Select statement
        // ------------------------------------------------------------------
        Rule::select_statement => AstNode::SelectStatement {
            common: inner_iter
                .clone()
                .find(|p| match p.as_rule() {
                    Rule::ctes => true,
                    _ => false,
                })
                .map(|p| p.into_inner().map(Pairs::single).map(parse_value).collect())
                .unwrap_or_else(Vec::new),
            mode: inner_iter
                .clone()
                .find(|p| match p.as_rule() {
                    Rule::select_mode => true,
                    _ => false,
                })
                .map(|p| SelectMode::from_str(p.as_str()))
                .unwrap_or(SelectMode::All),
            columns: inner_iter
                .clone()
                .find(|p| match p.as_rule() {
                    Rule::columns => true,
                    _ => false,
                })
                .map(|p| p.into_inner().map(Pairs::single).map(parse_value).collect())
                .unwrap_or_else(Vec::new),
            table_exprs: inner_iter
                .clone()
                .find(|p| match p.as_rule() {
                    Rule::table_expressions => true,
                    _ => false,
                })
                .map(|p| p.into_inner().map(Pairs::single).map(parse_value).collect())
                .unwrap_or_else(Vec::new),
            where_expr: inner_iter
                .clone()
                .find(|p| match p.as_rule() {
                    Rule::where_clause => true,
                    _ => false,
                })
                .map(Pairs::single)
                .map(parse_value)
                .map(Box::new),
            group_by: inner_iter
                .clone()
                .find(|p| match p.as_rule() {
                    Rule::group_by => true,
                    _ => false,
                })
                .map(Pairs::single)
                .map(parse_value)
                .map(Box::new),
        },
        Rule::select_mode => AstNode::SelectMode {
            mode: SelectMode::from_str(pair.as_str()),
        },
        Rule::where_clause => {
            // eprintln!("{:#?}", pair.clone());
            parse_value(pair.into_inner())
        }
        Rule::group_by => AstNode::GroupBy {
            groupings: inner_iter.map(Pairs::single).map(parse_value).collect(),
            having: None,
        },

        // ------------------------------------------------------------------
        // Table expressions
        // ------------------------------------------------------------------
        Rule::named_table_expression => AstNode::NamedTableExpression {
            name: Box::new(parse_value(pair.into_inner())),
            alias: inner_iter.nth(1).map(|v| v.as_str().to_string()),
        },

        // ------------------------------------------------------------------
        // Expressions
        // ------------------------------------------------------------------
        Rule::signed_expression => AstNode::SignedExpression {
            sign: Sign::from_str(inner_iter.clone().next().unwrap().as_str()),
            expr: Box::new(parse_value(inner_iter.nth(1).unwrap().into_inner())),
        },
        Rule::expression => parse_value(pair.into_inner()),
        // Rule::comparative_expression => AstNode::Expression {
        //     left: Box::new(parse_value(pair.into_inner())),
        //     op: Operation::from_str(inner_iter.clone().nth(1).unwrap().as_str()),
        //     right: Box::new(parse_value(inner_iter.clone().nth(2).unwrap().into_inner())),
        // },
        Rule::is_null_expression => AstNode::IsNullExpression {
            expr: Box::new(parse_value(pair.into_inner())),
            is_null: inner_iter
                .find_map(|p| match p.as_rule() {
                    Rule::negate => Some(false),
                    _ => None,
                })
                .unwrap_or(true),
        },
        Rule::case_clause => parse_value(pair.into_inner()),
        Rule::when_clause => {
            let nodes: Vec<AstNode> = inner_iter.map(Pairs::single).map(parse_value).collect();

            AstNode::WhenClause {
                guard: Box::new(nodes[0].clone()),
                body: Box::new(nodes[1].clone()),
            }
        }
        Rule::else_clause => parse_value(pair.into_inner()),
        Rule::case_expression => AstNode::CaseExpression {
            expr: inner_iter
                .clone()
                .find(|p| match p.as_rule() {
                    Rule::case_clause => true,
                    _ => false,
                })
                .map(Pairs::single)
                .map(parse_value)
                .map(Box::new),
            when_expr: inner_iter
                .clone()
                .filter(|p| match p.as_rule() {
                    Rule::when_clause => true,
                    _ => false,
                })
                .map(Pairs::single)
                .map(parse_value)
                .collect(),
            else_expr: inner_iter
                .clone()
                .find(|p| match p.as_rule() {
                    Rule::else_clause => true,
                    _ => false,
                })
                .map(Pairs::single)
                .map(parse_value)
                .map(Box::new),
        },

        // ------------------------------------------------------------------
        // Join clause
        // ------------------------------------------------------------------
        Rule::join_constraint_on => AstNode::JoinConstraintOn {
            expr: Box::new(parse_value(pair.into_inner())),
        },
        Rule::join_constraint_using => AstNode::JoinConstraintUsing {
            columns: inner_iter.map(Pairs::single).map(parse_value).collect(),
        },

        // ------------------------------------------------------------------
        // Function expressions
        // ------------------------------------------------------------------
        Rule::coalesce_function => AstNode::CoalesceFunction {
            exprs: inner_iter
                .next()
                .unwrap()
                .into_inner()
                .map(Pairs::single)
                .map(parse_value)
                .collect(),
        },
        Rule::replace_function => AstNode::ReplaceFunction {
            string: Box::new(parse_value(pair.into_inner())),
            search_string: Box::new(parse_value(inner_iter.clone().nth(1).unwrap().into_inner())),
            replace_string: inner_iter
                .nth(2)
                .map(Pairs::single)
                .map(parse_value)
                .map(Box::new),
        },
        Rule::substring_function => AstNode::SubstringFunction {
            string: Box::new(parse_value(pair.into_inner())),
            position: Box::new(parse_value(inner_iter.clone().nth(1).unwrap().into_inner())),
            length: inner_iter
                .nth(2)
                .map(Pairs::single)
                .map(parse_value)
                .map(Box::new),
        },
        Rule::to_date_function => AstNode::ToDateFunction {
            string: Box::new(parse_value(pair.into_inner())),
            format: inner_iter
                .nth(1)
                .map(Pairs::single)
                .map(parse_value)
                .map(Box::new),
        },
        Rule::right_function => {
            let nodes: Vec<AstNode> = inner_iter.map(Pairs::single).map(parse_value).collect();

            AstNode::RightFunction {
                string: Box::new(nodes[0].clone()),
                length: Box::new(nodes[1].clone()),
            }
        }
        Rule::max_function => {
            let nodes: Vec<AstNode> = inner_iter.map(Pairs::single).map(parse_value).collect();

            match nodes.len() {
                2 => AstNode::MaxFunction {
                    mode: Box::new(nodes[0].clone()),
                    expr: Box::new(nodes[1].clone()),
                },
                _ => AstNode::MaxFunction {
                    mode: Box::new(AstNode::SelectMode {
                        mode: SelectMode::All,
                    }),
                    expr: Box::new(nodes[0].clone()),
                },
            }
        }
        Rule::min_function => {
            let nodes: Vec<AstNode> = inner_iter.map(Pairs::single).map(parse_value).collect();

            match nodes.len() {
                2 => AstNode::MinFunction {
                    mode: Box::new(nodes[0].clone()),
                    expr: Box::new(nodes[1].clone()),
                },
                _ => AstNode::MinFunction {
                    mode: Box::new(AstNode::SelectMode {
                        mode: SelectMode::All,
                    }),
                    expr: Box::new(nodes[0].clone()),
                },
            }
        }
        Rule::sum_function => {
            let nodes: Vec<AstNode> = inner_iter.map(Pairs::single).map(parse_value).collect();

            match nodes.len() {
                2 => AstNode::SumFunction {
                    mode: Box::new(nodes[0].clone()),
                    expr: Box::new(nodes[1].clone()),
                },
                _ => AstNode::SumFunction {
                    mode: Box::new(AstNode::SelectMode {
                        mode: SelectMode::All,
                    }),
                    expr: Box::new(nodes[0].clone()),
                },
            }
        }
        Rule::power_function => {
            let nodes: Vec<AstNode> = inner_iter.map(Pairs::single).map(parse_value).collect();

            AstNode::PowerFunction {
                base: Box::new(nodes[0].clone()),
                exponent: Box::new(nodes[1].clone()),
            }
        }
        Rule::count_function => {
            let nodes: Vec<AstNode> = inner_iter.map(Pairs::single).map(parse_value).collect();

            match nodes.get(0) {
                Some(AstNode::SelectMode { .. }) => AstNode::CountFunction {
                    mode: Box::new(nodes[0].clone()),
                    columns: nodes.iter().skip(1).cloned().collect(),
                },
                _ => AstNode::CountFunction {
                    mode: Box::new(AstNode::SelectMode {
                        mode: SelectMode::All,
                    }),
                    columns: nodes,
                },
            }
        }
        Rule::concat_function => AstNode::ConcatFunction {
            exprs: inner_iter
                .next()
                .unwrap()
                .into_inner()
                .map(Pairs::single)
                .map(parse_value)
                .collect(),
        },
        Rule::cast_function => {
            let nodes: Vec<AstNode> = inner_iter.map(Pairs::single).map(parse_value).collect();

            AstNode::CastFunction {
                expr: Box::new(nodes[0].clone()),
                data_type: Box::new(nodes[1].clone()),
            }
        }
        Rule::date_trunc_function => {
            let nodes: Vec<AstNode> = inner_iter.map(Pairs::single).map(parse_value).collect();

            AstNode::DateTruncFunction {
                format: Box::new(nodes[0].clone()),
                datetime: Box::new(nodes[1].clone()),
            }
        }
        Rule::months_between_function => {
            let nodes: Vec<AstNode> = inner_iter.map(Pairs::single).map(parse_value).collect();

            AstNode::MonthsBetweenFunction {
                datetime1: Box::new(nodes[0].clone()),
                datetime2: Box::new(nodes[1].clone()),
            }
        }
        Rule::unknown_function => AstNode::UnknownFunction {
            name: Box::new(parse_value(pair.into_inner())),
            exprs: inner_iter
                .nth(1)
                .unwrap()
                .into_inner()
                .map(Pairs::single)
                .map(parse_value)
                .collect(),
        },

        // ------------------------------------------------------------------
        // Join clause
        // ------------------------------------------------------------------
        Rule::join_clause => {
            let nodes: Vec<AstNode> = inner_iter.map(Pairs::single).map(parse_value).collect();

            AstNode::JoinClause {
                join_type: Box::new(nodes[0].clone()),
                table_expr: Box::new(nodes[1].clone()),
                constraint: Box::new(nodes[2].clone()),
            }
        }
        Rule::inner_join_type => AstNode::InnerJoin,
        Rule::left_outer_join_type => AstNode::LeftOuterJoin,
        Rule::right_outer_join_type => AstNode::RightOuterJoin,
        Rule::full_outer_join_type => AstNode::FullOuterJoin,

        // ------------------------------------------------------------------
        // With clause
        // ------------------------------------------------------------------
        Rule::with_clause => {
            let identifier = Pairs::single(
                inner_iter
                    .find(|p| match p.as_rule() {
                        Rule::identifier => true,
                        _ => false,
                    })
                    .unwrap(),
            );
            let query = Pairs::single(
                inner_iter
                    .find(|p| match p.as_rule() {
                        Rule::select_statement => true,
                        _ => false,
                    })
                    .unwrap(),
            );

            AstNode::WithClause {
                identifier: Box::new(parse_value(identifier)),
                columns: vec![],
                query: Box::new(parse_value(query)),
            }
        }

        // ------------------------------------------------------------------
        // Data types
        // ------------------------------------------------------------------
        Rule::boolean_type => AstNode::BooleanType,
        Rule::char_type => AstNode::CharType {
            n: Box::new(parse_value(pair.into_inner())),
        },
        Rule::varchar_type => AstNode::VarcharType {
            n: Box::new(parse_value(pair.into_inner())),
        },
        Rule::date_type => AstNode::DateType,
        Rule::double_type => AstNode::DoubleType,
        Rule::timestamp_type => AstNode::TimestampType,
        Rule::local_timestamp_type => AstNode::LocalTimestampType,
        Rule::decimal_type => {
            let nodes: Vec<AstNode> = inner_iter.map(Pairs::single).map(parse_value).collect();

            AstNode::DecimalType {
                p: Box::new(nodes[0].clone()),
                s: Box::new(nodes[1].clone()),
            }
        }

        // ------------------------------------------------------------------
        // Comment
        // ------------------------------------------------------------------
        Rule::COMMENT => {
            // Necessary to strip the single quotes around the string
            let s = inner_iter
                .next()
                .map(|v| v.as_str().to_string())
                .unwrap_or_else(|| "".to_string());

            AstNode::Comment { s }
        }

        // ------------------------------------------------------------------
        // Unknown
        // ------------------------------------------------------------------
        _ => unreachable!(format!("Rule {:#?} not supported", pair)),
    }
}

fn infix(left: AstNode, op: Pair<Rule>, right: AstNode) -> AstNode {
    match op.as_rule() {
        Rule::operation_add
        | Rule::operation_subtract
        | Rule::operation_multiply
        | Rule::operation_divide
        | Rule::operation_concat
        | Rule::operation_or
        | Rule::operation_and
        | Rule::operation_equal
        | Rule::operation_not_equal
        | Rule::operation_greater_or_equal
        | Rule::operation_greater_than
        | Rule::operation_less_than
        | Rule::operation_less_or_equal => AstNode::Expression {
            left: Box::new(left),
            op: Operation::from_str(op.as_str()),
            right: Box::new(right),
        },
        _ => unreachable!(),
    }
}

fn parse_value(pairs: Pairs<Rule>) -> AstNode {
    PREC_CLIMBER.climb(
        pairs.filter(|p| match p.as_rule() {
            Rule::COMMENT => false,
            _ => true,
        }),
        primary,
        infix,
    )
}

fn parse_tokens(sql: &str) -> Pairs<Rule> {
    SqlParser::parse(Rule::sql_statement, sql).unwrap()
}

pub fn parse(sql: &str) -> Result<AstNode, Error<Rule>> {
    let pair = parse_tokens(sql);

    Ok(parse_value(pair))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(unused_macros)]
    macro_rules! parse_rule {
        (rule: $rule:expr, input: $sql:expr, expected: $expected:expr) => {
            let pair = SqlParser::parse($rule, $sql).unwrap();
            let node = parse_value(pair.clone());

            eprintln!("{:#?}", pair);
            eprintln!("{}", $sql);

            assert_eq!(node, $expected);
        };
    }

    // ------------------------------------------------------------------
    // Rule::data_type
    // ------------------------------------------------------------------

    #[test]
    fn data_type_boolean() {
        parse_rule! {
            rule: Rule::data_type,
            input: "BOOLEAN",
            expected: AstNode::BooleanType
        };
    }

    #[test]
    fn data_type_double() {
        parse_rule! {
            rule: Rule::data_type,
            input: "DOUBLE",
            expected: AstNode::DoubleType
        };
    }

    #[test]
    fn data_type_local_timestamp() {
        parse_rule! {
            rule: Rule::data_type,
            input: "TIMESTAMP",
            expected: AstNode::TimestampType
        };
    }

    #[test]
    fn data_type_timestamp() {
        parse_rule! {
            rule: Rule::data_type,
            input: "TIMESTAMP",
            expected: AstNode::TimestampType
        };
    }

    #[test]
    fn data_type_decimal() {
        parse_rule! {
            rule: Rule::data_type,
            input: "DECIMAL(10, 2)",
            expected: AstNode::DecimalType{
                p: Box::new(AstNode::IntegerLiteral { s: "10".to_string() }),
                s: Box::new(AstNode::IntegerLiteral { s: "2".to_string() })
            }
        };
    }

    #[test]
    fn data_type_char() {
        parse_rule! {
            rule: Rule::data_type,
            input: "CHAR(1)",
            expected: AstNode::CharType {
                n: Box::new(
                AstNode::IntegerLiteral { s: "1".to_string() }
            )}
        };
    }

    #[test]
    fn data_type_date() {
        parse_rule! {
            rule: Rule::data_type,
            input: "DATE",
            expected: AstNode::DateType
        };
    }

    // ------------------------------------------------------------------
    // Rule::coalesce_function
    // ------------------------------------------------------------------

    #[test]
    fn coalesce_function() {
        parse_rule! {
            rule: Rule::coalesce_function,
            input: "COALESCE(a, b)",
            expected: AstNode::CoalesceFunction {
                exprs: vec![
                    AstNode::Identifier { s: "a".to_string() },
                    AstNode::Identifier { s: "b".to_string() },
                ]
            }
        };
    }

    #[test]
    fn coalesce_function_3() {
        parse_rule! {
            rule: Rule::coalesce_function,
            input: "COALESCE(a, b, c)",
            expected: AstNode::CoalesceFunction {
                exprs: vec![
                    AstNode::Identifier { s: "a".to_string() },
                    AstNode::Identifier { s: "b".to_string() },
                    AstNode::Identifier { s: "c".to_string() },
                ]
            }
        };
    }

    // ------------------------------------------------------------------
    // Rule::cast_function
    // ------------------------------------------------------------------

    #[test]
    fn cast_function() {
        parse_rule! {
            rule: Rule::cast_function,
            input: "CAST(a AS BOOLEAN)",
            expected: AstNode::CastFunction{
                expr: Box::new(AstNode::Identifier { s: "a".to_string() }),
                data_type: Box::new(AstNode::BooleanType)
            }
        };
    }

    #[test]
    fn cast_function_with_expression() {
        parse_rule! {
            rule: Rule::cast_function,
            input: "CAST(a / b AS BOOLEAN)",
            expected: AstNode::CastFunction{
                expr: Box::new(AstNode::Expression {
                    left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                    op: Operation::Divide,
                    right: Box::new(AstNode::Identifier { s: "b".to_string() }),
                }),
                data_type: Box::new(AstNode::BooleanType)
            }
        };
    }

    // ------------------------------------------------------------------
    // Rule::column
    // ------------------------------------------------------------------

    #[test]
    fn column_identifier() {
        parse_rule! {
            rule: Rule::column,
            input: "a",
            expected: AstNode::Identifier { s: "a".to_string() }
        };
    }

    #[test]
    fn column_integer() {
        parse_rule! {
            rule: Rule::column,
            input: "42",
            expected: AstNode::IntegerLiteral { s: "42".to_string() }
        };
    }

    #[test]
    fn column_named_column() {
        parse_rule! {
            rule: Rule::column,
            input: "a AS b",
            expected: AstNode::NamedColumn {
                expr: Box::new(AstNode::Identifier { s: "a".to_string() }),
                alias: Some("b".to_string())
            }
        };
    }

    #[test]
    fn column_named_column_2() {
        parse_rule! {
            rule: Rule::column,
            input: "a.b AS c",
            expected: AstNode::NamedColumn {
                expr: Box::new(AstNode::QualifiedIdentifier {
                    s: vec![
                        AstNode::Identifier { s: "a".to_string() },
                        AstNode::Identifier { s: "b".to_string() }
                    ]
                }),
                alias: Some("c".to_string())
            }
        };
    }

    #[test]
    fn column_named_column_function_expression() {
        parse_rule! {
            rule: Rule::column,
            input: "COALESCE(1, 2) AS a",
            expected: AstNode::NamedColumn {
                expr: Box::new(AstNode::CoalesceFunction {
                    exprs: vec![
                        AstNode::IntegerLiteral { s: "1".to_string() },
                        AstNode::IntegerLiteral { s: "2".to_string() }
                    ]
                }),
                alias: Some("a".to_string())
            }
        };
    }

    // ------------------------------------------------------------------
    // Rule::join_clause
    // ------------------------------------------------------------------

    #[test]
    fn join_clause() {
        parse_rule! {
            rule: Rule::join_clause,
            input: "JOIN b ON a.id = b.id",
            expected: AstNode::JoinClause {
                join_type: Box::new(AstNode::InnerJoin),
                table_expr: Box::new(AstNode::NamedTableExpression {
                    name: Box::new(AstNode::Identifier { s: "b".to_string() }),
                    alias: None,
                }),
                constraint: Box::new(AstNode::JoinConstraintOn {
                    expr: Box::new(AstNode::Expression {
                        left: Box::new(AstNode::QualifiedIdentifier {
                            s: vec![
                                AstNode::Identifier { s: "a".to_string() },
                                AstNode::Identifier { s: "id".to_string() }
                            ]
                        }),
                        op: Operation::Equal,
                        right: Box::new(AstNode::QualifiedIdentifier {
                            s: vec![
                                AstNode::Identifier { s: "b".to_string() },
                                AstNode::Identifier { s: "id".to_string() }
                            ]
                        }),
                    })}
                ),
            }
        };
    }

    #[test]
    fn join_clause_named_tables() {
        parse_rule! {
            rule: Rule::join_clause,
            input: "JOIN b t2 ON t1.id = t2.id",
            expected: AstNode::JoinClause {
                join_type: Box::new(AstNode::InnerJoin),
                table_expr: Box::new(AstNode::NamedTableExpression {
                    name: Box::new(AstNode::Identifier { s: "b".to_string() }),
                    alias: Some("t2".to_string())
                }),
                constraint: Box::new(AstNode::JoinConstraintOn {
                    expr: Box::new(AstNode::Expression {
                        left: Box::new(AstNode::QualifiedIdentifier {
                            s: vec![
                                AstNode::Identifier { s: "t1".to_string() },
                                AstNode::Identifier { s: "id".to_string() }
                            ]
                        }),
                        op: Operation::Equal,
                        right: Box::new(AstNode::QualifiedIdentifier {
                            s: vec![
                                AstNode::Identifier { s: "t2".to_string() },
                                AstNode::Identifier { s: "id".to_string() }
                            ]
                        }),
                    })}
                ),
            }
        };
    }

    #[test]
    fn join_clause_with_and() {
        parse_rule! {
            rule: Rule::join_clause,
            input: "JOIN b ON a.id = b.id AND a.field = b.field",
            expected: AstNode::JoinClause {
                join_type: Box::new(AstNode::InnerJoin),
                table_expr: Box::new(AstNode::NamedTableExpression {
                    name: Box::new(AstNode::Identifier { s: "b".to_string() }),
                    alias: None,
                }),
                constraint: Box::new(AstNode::JoinConstraintOn {
                    expr: Box::new(AstNode::Expression {
                        left: Box::new(AstNode::Expression {
                            left: Box::new(AstNode::QualifiedIdentifier {
                                s: vec![
                                    AstNode::Identifier { s: "a".to_string() },
                                    AstNode::Identifier { s: "id".to_string() }
                                ]
                            }),
                            op: Operation::Equal,
                            right: Box::new(AstNode::QualifiedIdentifier {
                                s: vec![
                                    AstNode::Identifier { s: "b".to_string() },
                                    AstNode::Identifier { s: "id".to_string() }
                                ]
                            }),
                        }),
                        op: Operation::And,
                        right: Box::new(AstNode::Expression {
                            left: Box::new(AstNode::QualifiedIdentifier {
                                s: vec![
                                    AstNode::Identifier { s: "a".to_string() },
                                    AstNode::Identifier { s: "field".to_string() }
                                ]
                            }),
                            op: Operation::Equal,
                            right: Box::new(AstNode::QualifiedIdentifier {
                                s: vec![
                                    AstNode::Identifier { s: "b".to_string() },
                                    AstNode::Identifier { s: "field".to_string() }
                                ]
                            }),
                        })
                    })
                }),
            }
        };
    }

    // ------------------------------------------------------------------
    // Rule::join_type
    // ------------------------------------------------------------------

    #[test]
    fn join_type_inner_join() {
        parse_rule! {
            rule: Rule::join_type,
            input: "JOIN",
            expected: AstNode::InnerJoin
        };
    }

    #[test]
    fn join_type_inner_join_2() {
        parse_rule! {
            rule: Rule::join_type,
            input: "INNER JOIN",
            expected: AstNode::InnerJoin
        };
    }

    #[test]
    fn join_type_left_outer_join() {
        parse_rule! {
            rule: Rule::join_type,
            input: "LEFT JOIN",
            expected: AstNode::LeftOuterJoin
        };
    }

    #[test]
    fn join_type_left_outer_join_2() {
        parse_rule! {
            rule: Rule::join_type,
            input: "LEFT OUTER JOIN",
            expected: AstNode::LeftOuterJoin
        };
    }

    #[test]
    fn join_type_right_outer_join() {
        parse_rule! {
            rule: Rule::join_type,
            input: "RIGHT JOIN",
            expected: AstNode::RightOuterJoin
        };
    }

    #[test]
    fn join_type_right_outer_join_2() {
        parse_rule! {
            rule: Rule::join_type,
            input: "RIGHT OUTER JOIN",
            expected: AstNode::RightOuterJoin
        };
    }

    #[test]
    fn join_type_full_outer() {
        parse_rule! {
            rule: Rule::join_type,
            input: "FULL OUTER JOIN",
            expected: AstNode::FullOuterJoin
        };
    }

    // ------------------------------------------------------------------
    // Rule::join_constraint
    // ------------------------------------------------------------------

    #[test]
    fn join_constraint_on() {
        parse_rule! {
            rule: Rule::join_constraint,
            input: "ON a = b",
            expected: AstNode::JoinConstraintOn {
                expr: Box::new(AstNode::Expression {
                    left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                    op: Operation::Equal,
                    right: Box::new(AstNode::Identifier { s: "b".to_string() }),
                })
            }
        };
    }

    #[test]
    fn join_constraint_using() {
        parse_rule! {
            rule: Rule::join_constraint,
            input: "USING (a)",
            expected: AstNode::JoinConstraintUsing {
                columns: vec![AstNode::Identifier { s: "a".to_string() }]
            }
        };
    }

    #[test]
    fn join_constraint_using_2() {
        parse_rule! {
            rule: Rule::join_constraint,
            input: "USING (a, b)",
            expected: AstNode::JoinConstraintUsing {
                columns: vec![
                    AstNode::Identifier { s: "a".to_string() },
                    AstNode::Identifier { s: "b".to_string() },
                ]
            }
        };
    }

    // ------------------------------------------------------------------
    // Rule::expression
    // ------------------------------------------------------------------

    #[test]
    fn expression_identifier() {
        parse_rule! {
            rule: Rule::expression,
            input: "a",
            expected: AstNode::Identifier { s: "a".to_string() }
        };
    }

    #[test]
    fn expression_qualified_identifier() {
        parse_rule! {
            rule: Rule::expression,
            input: "a.b",
            expected: AstNode::QualifiedIdentifier {
                s: vec![
                    AstNode::Identifier { s: "a".to_string() },
                    AstNode::Identifier { s: "b".to_string() }
                ]
            }
        };
    }

    #[test]
    fn expression_integer() {
        parse_rule! {
            rule: Rule::expression,
            input: "42",
            expected: AstNode::IntegerLiteral { s: "42".to_string() }
        };
    }

    #[test]
    fn expression_signed_expression() {
        parse_rule! {
            rule: Rule::expression,
            input: "-42",
            expected: AstNode::SignedExpression{
                sign: Sign::Negative,
                expr: Box::new(AstNode::IntegerLiteral { s: "42".to_string() })
            }
        };
    }

    #[test]
    fn expression_with_comment() {
        parse_rule! {
            rule: Rule::expression,
            input: r#"
                a
                -- comment
                AND b
            "#.trim(),
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::And,
                right: Box::new(AstNode::Identifier { s: "b".to_string() }),
            }
        };
    }

    #[test]
    fn expression_equals() {
        parse_rule! {
            rule: Rule::expression,
            input: "a = b",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::Equal,
                right: Box::new(AstNode::Identifier { s: "b".to_string() })
            }
        };
    }

    #[test]
    fn expression_equals_literal() {
        parse_rule! {
            rule: Rule::expression,
            input: "a = 1",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::Equal,
                right: Box::new(AstNode::IntegerLiteral { s: "1".to_string() })
            }
        };
    }

    #[test]
    fn expression_not_equal() {
        parse_rule! {
            rule: Rule::expression,
            input: "a != b",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::NotEqual,
                right: Box::new(AstNode::Identifier { s: "b".to_string() })
            }
        };
    }

    #[test]
    fn expression_greater_than() {
        parse_rule! {
            rule: Rule::expression,
            input: "a > b",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::GreaterThan,
                right: Box::new(AstNode::Identifier { s: "b".to_string() })
            }
        };
    }

    #[test]
    fn expression_greater_or_equal_than() {
        parse_rule! {
            rule: Rule::expression,
            input: "a >= b",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::GreaterOrEqualThan,
                right: Box::new(AstNode::Identifier { s: "b".to_string() })
            }
        };
    }

    #[test]
    fn expression_less_than() {
        parse_rule! {
            rule: Rule::expression,
            input: "a < b",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::LessThan,
                right: Box::new(AstNode::Identifier { s: "b".to_string() })
            }
        };
    }

    #[test]
    fn expression_less_or_equal_than() {
        parse_rule! {
            rule: Rule::expression,
            input: "a <= b",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::LessOrEqualThan,
                right: Box::new(AstNode::Identifier { s: "b".to_string() })
            }
        };
    }

    #[test]
    fn expression_qualified_identifier_equals() {
        parse_rule! {
            rule: Rule::expression,
            input: "a.b = c",
            expected: AstNode::Expression {
                left: Box::new(AstNode::QualifiedIdentifier {
                    s: vec![
                        AstNode::Identifier { s: "a".to_string() },
                        AstNode::Identifier { s: "b".to_string() },
                    ]
                }),
                op: Operation::Equal,
                right: Box::new(AstNode::Identifier { s: "c".to_string() })
            }
        };
    }

    #[test]
    fn expression_case_expression() {
        parse_rule! {
            rule: Rule::expression,
            input: "a = CASE WHEN 1 THEN 'one' END",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::Equal,
                right: Box::new(AstNode::CaseExpression {
                    expr: None,
                    when_expr: vec![AstNode::WhenClause{
                        guard: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                        body: Box::new(AstNode::StringLiteral { s: "one".to_string() }),
                    }],
                    else_expr: None
                })
            }
        };
    }

    #[test]
    fn expression_prec_climber_equality() {
        parse_rule! {
            rule: Rule::expression,
            input: "a = b - c",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::Equal,
                right: Box::new(AstNode::Expression {
                    left: Box::new(AstNode::Identifier { s: "b".to_string() }),
                    op: Operation::Subtract,
                    right: Box::new(AstNode::Identifier { s: "c".to_string() })
                })
            }
        };
    }
    // ------------------------------------------------------------------
    // Rule::unary_expression
    // ------------------------------------------------------------------

    #[test]
    fn expression_is_null() {
        parse_rule! {
            rule: Rule::unary_expression,
            input: "a IS NULL",
            expected: AstNode::IsNullExpression {
                expr: Box::new(AstNode::Identifier { s: "a".to_string() }),
                is_null: true,
            }
        };
    }

    #[test]
    fn expression_is_not_null() {
        parse_rule! {
            rule: Rule::unary_expression,
            input: "a IS NOT NULL",
            expected: AstNode::IsNullExpression {
                expr: Box::new(AstNode::Identifier { s: "a".to_string() }),
                is_null: false,
            }
        };
    }

    // ------------------------------------------------------------------
    // Rule::case_expression
    // ------------------------------------------------------------------

    #[test]
    fn case_expression() {
        parse_rule! {
            rule: Rule::case_expression,
            input: "CASE a WHEN 1 THEN 'one' END",
            expected: AstNode::CaseExpression {
                expr: Some(Box::new(AstNode::Identifier { s: "a".to_string() })),
                when_expr: vec![AstNode::WhenClause{
                    guard: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                    body: Box::new(AstNode::StringLiteral { s: "one".to_string() }),
                }],
                else_expr: None
            }
        };
    }

    #[test]
    fn case_expression_when_with_function() {
        parse_rule! {
            rule: Rule::case_expression,
            input: "CASE WHEN SUBSTR('abc', 1) THEN 'one' END",
            expected: AstNode::CaseExpression {
                expr: None,
                when_expr: vec![AstNode::WhenClause{
                    guard: Box::new(AstNode::SubstringFunction {
                        string: Box::new(AstNode::StringLiteral { s: "abc".to_string() }),
                        position: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                        length: None
                    }),
                    body: Box::new(AstNode::StringLiteral { s: "one".to_string() }),
                }],
                else_expr: None
            }
        };
    }

    #[test]
    fn case_expression_no_case_expression() {
        parse_rule! {
            rule: Rule::case_expression,
            input: "CASE WHEN 1 THEN 'one' END",
            expected: AstNode::CaseExpression {
                expr: None,
                when_expr: vec![AstNode::WhenClause{
                    guard: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                    body: Box::new(AstNode::StringLiteral { s: "one".to_string() }),
                }],
                else_expr: None
            }
        };
    }

    #[test]
    fn case_expression_with_else() {
        parse_rule! {
            rule: Rule::case_expression,
            input: "CASE a WHEN 1 THEN 'one' ELSE 'zero' END",
            expected: AstNode::CaseExpression {
                expr: Some(Box::new(AstNode::Identifier { s: "a".to_string() })),
                when_expr: vec![AstNode::WhenClause {
                    guard: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                    body: Box::new(AstNode::StringLiteral { s: "one".to_string() }),
                }],
                else_expr: Some(Box::new(AstNode::StringLiteral { s: "zero".to_string() }))
            }
        };
    }

    #[test]
    fn case_expression_else_with_function() {
        parse_rule! {
            rule: Rule::case_expression,
            input: "CASE WHEN 1 THEN 'one' ELSE SUBSTR('abc', 1) END",
            expected: AstNode::CaseExpression {
                expr: None,
                when_expr: vec![AstNode::WhenClause {
                    guard: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                    body: Box::new(AstNode::StringLiteral { s: "one".to_string() }),
                }],
                else_expr: Some(Box::new(AstNode::SubstringFunction {
                    string: Box::new(AstNode::StringLiteral { s: "abc".to_string() }),
                    position: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                    length: None
                }))
            }
        };
    }

    #[test]
    fn case_expression_more_cases() {
        parse_rule! {
            rule: Rule::case_expression,
            input: "CASE a WHEN 1 THEN 'one' WHEN 2 THEN 'two' END",
            expected: AstNode::CaseExpression {
                expr: Some(Box::new(AstNode::Identifier { s: "a".to_string() })),
                when_expr: vec![
                    AstNode::WhenClause {
                        guard: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                        body: Box::new(AstNode::StringLiteral { s: "one".to_string() }),
                    },
                    AstNode::WhenClause {
                        guard: Box::new(AstNode::IntegerLiteral { s: "2".to_string() }),
                        body: Box::new(AstNode::StringLiteral { s: "two".to_string() }),
                    },
                ],
                else_expr: None
            }
        };
    }

    // ------------------------------------------------------------------
    // Rule::expression
    // ------------------------------------------------------------------

    #[test]
    fn expression_concat_operator() {
        parse_rule! {
            rule: Rule::expression,
            input: "a || b",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::Concat,
                right: Box::new(AstNode::Identifier { s: "b".to_string() })
            }
        };
    }

    #[test]
    fn expression_and_operator() {
        parse_rule! {
            rule: Rule::expression,
            input: "a AND b",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::And,
                right: Box::new(AstNode::Identifier { s: "b".to_string() })
            }
        };
    }

    #[test]
    fn expression_or_operator() {
        parse_rule! {
            rule: Rule::expression,
            input: "a OR b",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::Or,
                right: Box::new(AstNode::Identifier { s: "b".to_string() })
            }
        };
    }

    #[test]
    fn expression_binary_concat() {
        parse_rule! {
            rule: Rule::expression,
            input: "COALESCE(1) || b",
            expected: AstNode::Expression {
                left: Box::new(AstNode::CoalesceFunction{
                    exprs: vec![AstNode::IntegerLiteral { s: "1".to_string() }]
                }),
                right: Box::new(AstNode::Identifier { s: "b".to_string() }),
                op: Operation::Concat
            }
        };
    }

    #[test]
    fn expression_binary_concat_2() {
        parse_rule! {
            rule: Rule::expression,
            input: "b || COALESCE(1)",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "b".to_string() }),
                right: Box::new(AstNode::CoalesceFunction{
                    exprs: vec![AstNode::IntegerLiteral { s: "1".to_string() }]
                }),
                op: Operation::Concat
            }
        };
    }

    #[test]
    fn expression_binary_concat_3() {
        parse_rule! {
            rule: Rule::expression,
            input: "COALESCE(1) || COALESCE(2)",
            expected: AstNode::Expression {
                left: Box::new(AstNode::CoalesceFunction{
                    exprs: vec![AstNode::IntegerLiteral { s: "1".to_string() }]
                }),
                right: Box::new(AstNode::CoalesceFunction{
                    exprs: vec![AstNode::IntegerLiteral { s: "2".to_string() }]
                }),
                op: Operation::Concat
            }
        };
    }

    #[test]
    fn expression_binary_concat_4() {
        parse_rule! {
            rule: Rule::expression,
            input: "COALESCE(1) || a || COALESCE(2)",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Expression {
                    left: Box::new(AstNode::CoalesceFunction{
                        exprs: vec![AstNode::IntegerLiteral { s: "1".to_string() }]
                    }),
                    op: Operation::Concat,
                    right: Box::new(AstNode::Identifier { s: "a".to_string() }),
                }),
                op: Operation::Concat,
                right: Box::new(AstNode::CoalesceFunction{
                    exprs: vec![AstNode::IntegerLiteral { s: "2".to_string() }]
                }),
            }
        };
    }

    #[test]
    fn expression_parens() {
        parse_rule! {
            rule: Rule::expression,
            input: "a + (b + c)",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::Add,
                right: Box::new(AstNode::Expression {
                    left: Box::new(AstNode::Identifier { s: "b".to_string() }),
                    op: Operation::Add,
                    right: Box::new(AstNode::Identifier { s: "c".to_string() }),
                }),
            }
        };
    }

    #[test]
    fn expression_parens_2() {
        parse_rule! {
            rule: Rule::expression,
            input: "(a + b) + c",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Expression {
                    left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                    op: Operation::Add,
                    right: Box::new(AstNode::Identifier { s: "b".to_string() }),
                }),
                op: Operation::Add,
                right: Box::new(AstNode::Identifier { s: "c".to_string() }),
            }
        };
    }

    #[test]
    fn expression_multiply() {
        parse_rule! {
            rule: Rule::expression,
            input: "a * b",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::Multiply,
                right: Box::new(AstNode::Identifier { s: "b".to_string() }),
            }
        };
    }

    #[test]
    fn expression_divide() {
        parse_rule! {
            rule: Rule::expression,
            input: "a / b",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::Divide,
                right: Box::new(AstNode::Identifier { s: "b".to_string() }),
            }
        };
    }

    #[test]
    fn expression_subtract() {
        parse_rule! {
            rule: Rule::expression,
            input: "a - b",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::Subtract,
                right: Box::new(AstNode::Identifier { s: "b".to_string() }),
            }
        };
    }

    #[test]
    fn expression_and_boolean_literal() {
        parse_rule! {
            rule: Rule::expression,
            input: "TRUE AND a",
            expected: AstNode::Expression {
                left: Box::new(AstNode::BooleanLiteral { s: "TRUE".to_string() }),
                op: Operation::And,
                right: Box::new(AstNode::Identifier { s: "a".to_string() }),
            }
        };
    }

    #[test]
    fn expression_unary_expression() {
        parse_rule! {
            rule: Rule::expression,
            input: "a IS NULL AND TRUE",
            expected: AstNode::Expression {
                left: Box::new(AstNode::IsNullExpression {
                   expr: Box::new(AstNode::Identifier { s: "a".to_string() }),
                   is_null: true,
                }),
                op: Operation::And,
                right: Box::new(AstNode::BooleanLiteral { s: "TRUE".to_string() }),
            }
        };
    }

    // ------------------------------------------------------------------
    // Rule::boolean_literal
    // ------------------------------------------------------------------

    #[test]
    fn boolean_literal() {
        parse_rule! {
            rule: Rule::literal_value,
            input: "TRUE",
            expected: AstNode::BooleanLiteral { s: "TRUE".to_string() }
        };
    }

    #[test]
    fn boolean_literal_false() {
        parse_rule! {
            rule: Rule::literal_value,
            input: "FALSE",
            expected: AstNode::BooleanLiteral { s: "FALSE".to_string() }
        };
    }

    // ------------------------------------------------------------------
    // Rule::integer_literal
    // ------------------------------------------------------------------

    #[test]
    fn integer_literal() {
        parse_rule! {
            rule: Rule::literal_value,
            input: "42",
            expected: AstNode::IntegerLiteral { s: "42".to_string() }
        };
    }

    #[test]
    fn integer_literal_negative() {
        parse_rule! {
            rule: Rule::literal_value,
            input: "-42",
            expected: AstNode::IntegerLiteral { s: "-42".to_string() }
        };
    }

    // ------------------------------------------------------------------
    // Rule::float_literal
    // ------------------------------------------------------------------

    #[test]
    fn float_literal() {
        parse_rule! {
            rule: Rule::literal_value,
            input: "42.0E10",
            expected: AstNode::FloatLiteral { s: "42.0E10".to_string() }
        };
    }

    #[test]
    fn float_literal_no_leading_zero() {
        parse_rule! {
            rule: Rule::literal_value,
            input: ".42E10",
            expected: AstNode::FloatLiteral { s: ".42E10".to_string() }
        };
    }

    #[test]
    fn float_literal_negative_esponent() {
        parse_rule! {
            rule: Rule::literal_value,
            input: "42.0E-10",
            expected: AstNode::FloatLiteral { s: "42.0E-10".to_string() }
        };
    }

    #[test]
    fn float_literal_negative_with_negative_esponent() {
        parse_rule! {
            rule: Rule::literal_value,
            input: "-42.0E-10",
            expected: AstNode::FloatLiteral { s: "-42.0E-10".to_string() }
        };
    }

    #[test]
    fn float_literal_negative_with_negative_esponent_no_leading_zero() {
        parse_rule! {
            rule: Rule::literal_value,
            input: "-.42E-10",
            expected: AstNode::FloatLiteral { s: "-.42E-10".to_string() }
        };
    }

    // ------------------------------------------------------------------
    // Rule::decimal_literal
    // ------------------------------------------------------------------

    #[test]
    fn decimal_literal() {
        parse_rule! {
            rule: Rule::literal_value,
            input: "42.0",
            expected: AstNode::DecimalLiteral { s: "42.0".to_string() }
        };
    }

    #[test]
    fn decimal_literal_no_leading_zero() {
        parse_rule! {
            rule: Rule::literal_value,
            input: ".42",
            expected: AstNode::DecimalLiteral { s: ".42".to_string() }
        };
    }

    // ------------------------------------------------------------------
    // Rule::string_literal
    // ------------------------------------------------------------------

    #[test]
    fn string_literal() {
        parse_rule! {
            rule: Rule::literal_value,
            input: "'text'",
            expected: AstNode::StringLiteral { s: "text".to_string() }
        };
    }

    #[test]
    fn string_literal_empty() {
        parse_rule! {
            rule: Rule::literal_value,
            input: "''",
            expected: AstNode::StringLiteral { s: "".to_string() }
        };
    }

    // ------------------------------------------------------------------
    // Rule::interval_literal
    // ------------------------------------------------------------------

    #[test]
    fn interval_literal_month() {
        parse_rule! {
            rule: Rule::interval_literal,
            input: "INTERVAL '1' MONTH",
            expected: AstNode::IntervalLiteral {
                interval: Box::new(AstNode::StringLiteral { s: "1".to_string() }),
                period: Interval::Month,
                precision: vec![],
                convert_to: None,
                convert_precision: None,
            }
        };
    }

    #[test]
    fn interval_literal_month_with_precision() {
        parse_rule! {
            rule: Rule::interval_literal,
            input: "INTERVAL '1' MONTH (3)",
            expected: AstNode::IntervalLiteral {
                interval: Box::new(AstNode::StringLiteral { s: "1".to_string() }),
                period: Interval::Month,
                precision: vec![AstNode::IntegerLiteral { s: "3".to_string() }],
                convert_to: None,
                convert_precision: None,
            }
        };
    }

    #[test]
    fn interval_literal_year() {
        parse_rule! {
            rule: Rule::interval_literal,
            input: "INTERVAL '1' YEAR",
            expected: AstNode::IntervalLiteral {
                interval: Box::new(AstNode::StringLiteral { s: "1".to_string() }),
                period: Interval::Year,
                precision: vec![],
                convert_to: None,
                convert_precision: None,
            }
        };
    }

    #[test]
    fn interval_literal_year_with_precision() {
        parse_rule! {
            rule: Rule::interval_literal,
            input: "INTERVAL '1' YEAR (3)",
            expected: AstNode::IntervalLiteral {
                interval: Box::new(AstNode::StringLiteral { s: "1".to_string() }),
                period: Interval::Year,
                precision: vec![AstNode::IntegerLiteral { s: "3".to_string() }],
                convert_to: None,
                convert_precision: None,
            }
        };
    }

    #[test]
    fn interval_literal_year_to_month() {
        parse_rule! {
            rule: Rule::interval_literal,
            input: "INTERVAL '1' YEAR TO MONTH",
            expected: AstNode::IntervalLiteral {
                interval: Box::new(AstNode::StringLiteral { s: "1".to_string() }),
                period: Interval::Year,
                precision: vec![],
                convert_to: Some(Interval::Month),
                convert_precision: None,
            }
        };
    }

    #[test]
    fn interval_literal_year_to_month_with_precision() {
        parse_rule! {
            rule: Rule::interval_literal,
            input: "INTERVAL '1' YEAR (3) TO MONTH",
            expected: AstNode::IntervalLiteral {
                interval: Box::new(AstNode::StringLiteral { s: "1".to_string() }),
                period: Interval::Year,
                precision: vec![AstNode::IntegerLiteral { s: "3".to_string() }],
                convert_to: Some(Interval::Month),
                convert_precision: None,
            }
        };
    }

    #[test]
    fn interval_literal_day() {
        parse_rule! {
            rule: Rule::interval_literal,
            input: "INTERVAL '1' DAY",
            expected: AstNode::IntervalLiteral {
                interval: Box::new(AstNode::StringLiteral { s: "1".to_string() }),
                period: Interval::Day,
                precision: vec![],
                convert_to: None,
                convert_precision: None,
            }
        };
    }

    #[test]
    fn interval_literal_day_with_precision() {
        parse_rule! {
            rule: Rule::interval_literal,
            input: "INTERVAL '1' DAY (3)",
            expected: AstNode::IntervalLiteral {
                interval: Box::new(AstNode::StringLiteral { s: "1".to_string() }),
                period: Interval::Day,
                precision: vec![AstNode::IntegerLiteral { s: "3".to_string() }],
                convert_to: None,
                convert_precision: None,
            }
        };
    }

    #[test]
    fn interval_literal_day_with_precision_to_hour() {
        parse_rule! {
            rule: Rule::interval_literal,
            input: "INTERVAL '1' DAY (3) TO HOUR",
            expected: AstNode::IntervalLiteral {
                interval: Box::new(AstNode::StringLiteral { s: "1".to_string() }),
                period: Interval::Day,
                precision: vec![AstNode::IntegerLiteral { s: "3".to_string() }],
                convert_to: Some(Interval::Hour),
                convert_precision: None,
            }
        };
    }

    #[test]
    fn interval_literal_day_with_precision_to_second_with_precision() {
        parse_rule! {
            rule: Rule::interval_literal,
            input: "INTERVAL '1' DAY (3) TO SECOND (4)",
            expected: AstNode::IntervalLiteral {
                interval: Box::new(AstNode::StringLiteral { s: "1".to_string() }),
                period: Interval::Day,
                precision: vec![AstNode::IntegerLiteral { s: "3".to_string() }],
                convert_to: Some(Interval::Second),
                convert_precision:Some(Box::new(AstNode::IntegerLiteral { s: "4".to_string() }))
            }
        };
    }

    #[test]
    fn interval_literal_second_with_precision_fractional() {
        parse_rule! {
            rule: Rule::interval_literal,
            input: "INTERVAL '1' SECOND (3, 1)",
            expected: AstNode::IntervalLiteral {
                interval: Box::new(AstNode::StringLiteral { s: "1".to_string() }),
                period: Interval::Second,
                precision: vec![
                    AstNode::IntegerLiteral { s: "3".to_string() },
                    AstNode::IntegerLiteral { s: "1".to_string() },
                ],
                convert_to: None,
                convert_precision: None,
            }
        };
    }

    // ------------------------------------------------------------------
    // Rule::function_expression
    // ------------------------------------------------------------------

    #[test]
    fn function_expression_unknown() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "my_function(a)",
            expected: AstNode::UnknownFunction {
                name: Box::new(AstNode::Identifier { s: "my_function".to_string() }),
                exprs: vec![AstNode::Identifier { s: "a".to_string() }]
            }
        };
    }

    #[test]
    fn function_expression_unknown_2() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "my_function(a, b)",
            expected: AstNode::UnknownFunction {
                name: Box::new(AstNode::Identifier { s: "my_function".to_string() }),
                exprs: vec![
                    AstNode::Identifier { s: "a".to_string() },
                    AstNode::Identifier { s: "b".to_string() }
                ]
            }
        };
    }

    #[test]
    fn function_expression_unknown_3() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "SCHEMA.MY_FUNCTION(a, b)",
            expected: AstNode::UnknownFunction {
                name: Box::new(AstNode::QualifiedIdentifier {
                    s: vec![
                        AstNode::Identifier { s: "SCHEMA".to_string() },
                        AstNode::Identifier { s: "MY_FUNCTION".to_string() }
                    ]
                }),
                exprs: vec![
                    AstNode::Identifier { s: "a".to_string() },
                    AstNode::Identifier { s: "b".to_string() }
                ]
            }
        };
    }

    #[test]
    fn function_expression_coalesce() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "COALESCE(a, b)",
            expected: AstNode::CoalesceFunction {
                exprs: vec![
                    AstNode::Identifier { s: "a".to_string() },
                    AstNode::Identifier { s: "b".to_string() }
                ]
            }
        };
    }

    #[test]
    fn function_expression_replace() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "REPLACE(a, b)",
            expected: AstNode::ReplaceFunction {
                string: Box::new(AstNode::Identifier { s: "a".to_string() }),
                search_string: Box::new(AstNode::Identifier { s: "b".to_string() }),
                replace_string: None,
            }
        };
    }

    #[test]
    fn function_expression_replace_with_replace_string() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "REPLACE(a, b, c)",
            expected: AstNode::ReplaceFunction {
                string: Box::new(AstNode::Identifier { s: "a".to_string() }),
                search_string: Box::new(AstNode::Identifier { s: "b".to_string() }),
                replace_string: Some(Box::new(AstNode::Identifier { s: "c".to_string() })),
            }
        };
    }

    #[test]
    fn function_expression_right() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "RIGHT(a, 3)",
            expected: AstNode::RightFunction {
                string: Box::new(AstNode::Identifier { s: "a".to_string() }),
                length: Box::new(AstNode::IntegerLiteral { s: "3".to_string() }),
            }
        };
    }

    #[test]
    fn function_expression_count() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "COUNT(*)",
            expected: AstNode::CountFunction {
                columns: vec![AstNode::AllColumns],
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::All }),
            }
        };
    }

    #[test]
    fn function_expression_count_column() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "COUNT(a)",
            expected: AstNode::CountFunction {
                columns: vec![AstNode::Identifier { s: "a".to_string() }],
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::All }),
            }
        };
    }

    #[test]
    fn function_expression_count_column_all() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "COUNT(ALL a)",
            expected: AstNode::CountFunction {
                columns: vec![AstNode::Identifier { s: "a".to_string() }],
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::All }),
            }
        };
    }

    #[test]
    fn function_expression_count_column_distinct() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "COUNT(DISTINCT a)",
            expected: AstNode::CountFunction {
                columns: vec![AstNode::Identifier { s: "a".to_string() }],
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::Distinct }),
            }
        };
    }

    #[test]
    fn function_expression_count_columns() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "COUNT((a, b))",
            expected: AstNode::CountFunction {
                columns: vec![
                    AstNode::Identifier { s: "a".to_string() },
                    AstNode::Identifier { s: "b".to_string() }
                ],
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::All }),
            }
        };
    }

    #[test]
    fn function_expression_substring_short() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "SUBSTR(a, b)",
            expected: AstNode::SubstringFunction {
                string: Box::new(AstNode::Identifier { s: "a".to_string() }),
                position: Box::new(AstNode::Identifier { s: "b".to_string() }),
                length: None,
            }
        };
    }

    #[test]
    fn function_expression_substring() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "SUBSTRING(a, b)",
            expected: AstNode::SubstringFunction {
                string: Box::new(AstNode::Identifier { s: "a".to_string() }),
                position: Box::new(AstNode::Identifier { s: "b".to_string() }),
                length: None,
            }
        };
    }

    #[test]
    fn function_expression_substring_3() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "SUBSTRING(a, b, c)",
            expected: AstNode::SubstringFunction {
                string: Box::new(AstNode::Identifier { s: "a".to_string() }),
                position: Box::new(AstNode::Identifier { s: "b".to_string() }),
                length: Some(Box::new(AstNode::Identifier { s: "c".to_string() }))
            }
        };
    }

    #[test]
    fn function_expression_substring_from() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "SUBSTRING(a FROM b)",
            expected: AstNode::SubstringFunction {
                string: Box::new(AstNode::Identifier { s: "a".to_string() }),
                position: Box::new(AstNode::Identifier { s: "b".to_string() }),
                length: None
            }
        };
    }

    #[test]
    fn function_expression_substring_from_for() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "SUBSTRING(a FROM b FOR c)",
            expected: AstNode::SubstringFunction {
                string: Box::new(AstNode::Identifier { s: "a".to_string() }),
                position: Box::new(AstNode::Identifier { s: "b".to_string() }),
                length: Some(Box::new(AstNode::Identifier { s: "c".to_string() }))
            }
        };
    }

    #[test]
    fn function_expression_to_date() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "TO_DATE(a)",
            expected: AstNode::ToDateFunction {
                string: Box::new(AstNode::Identifier { s: "a".to_string() }),
                format: None
            }
        };
    }

    #[test]
    fn function_expression_to_date_short() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "DATE a",
            expected: AstNode::ToDateFunction {
                string: Box::new(AstNode::Identifier { s: "a".to_string() }),
                format: None
            }
        };
    }

    #[test]
    fn function_expression_to_date_format() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "TO_DATE(a, b)",
            expected: AstNode::ToDateFunction {
                string: Box::new(AstNode::Identifier { s: "a".to_string() }),
                format: Some(Box::new(AstNode::Identifier { s: "b".to_string() }))
            }
        };
    }

    #[test]
    fn function_expression_power() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "POWER(a, b)",
            expected: AstNode::PowerFunction {
                base: Box::new(AstNode::Identifier { s: "a".to_string() }),
                exponent: Box::new(AstNode::Identifier { s: "b".to_string() })
            }
        };
    }

    #[test]
    fn function_expression_concat() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "CONCAT(a, b)",
            expected: AstNode::ConcatFunction {
                exprs: vec![
                    AstNode::Identifier { s: "a".to_string() },
                    AstNode::Identifier { s: "b".to_string() }
                ]
            }
        };
    }

    #[test]
    fn function_expression_max() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "MAX(a)",
            expected: AstNode::MaxFunction {
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::All }),
                expr: Box::new(AstNode::Identifier { s: "a".to_string() })
            }
        };
    }

    #[test]
    fn function_expression_max_all() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "MAX(ALL a)",
            expected: AstNode::MaxFunction {
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::All }),
                expr: Box::new(AstNode::Identifier { s: "a".to_string() })
            }
        };
    }

    #[test]
    fn function_expression_max_distinct() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "MAX(DISTINCT a)",
            expected: AstNode::MaxFunction {
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::Distinct }),
                expr: Box::new(AstNode::Identifier { s: "a".to_string() })
            }
        };
    }

    #[test]
    fn function_expression_min() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "MIN(a)",
            expected: AstNode::MinFunction {
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::All }),
                expr: Box::new(AstNode::Identifier { s: "a".to_string() })
            }
        };
    }

    #[test]
    fn function_expression_min_all() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "MIN(ALL a)",
            expected: AstNode::MinFunction {
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::All }),
                expr: Box::new(AstNode::Identifier { s: "a".to_string() })
            }
        };
    }

    #[test]
    fn function_expression_min_distinct() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "MIN(DISTINCT a)",
            expected: AstNode::MinFunction {
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::Distinct }),
                expr: Box::new(AstNode::Identifier { s: "a".to_string() })
            }
        };
    }

    #[test]
    fn function_expression_sum() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "SUM(a)",
            expected: AstNode::SumFunction {
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::All }),
                expr: Box::new(AstNode::Identifier { s: "a".to_string() })
            }
        };
    }

    #[test]
    fn function_expression_sum_all() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "SUM(ALL a)",
            expected: AstNode::SumFunction {
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::All }),
                expr: Box::new(AstNode::Identifier { s: "a".to_string() })
            }
        };
    }

    #[test]
    fn function_expression_sum_distinct() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "SUM(DISTINCT a)",
            expected: AstNode::SumFunction {
                mode: Box::new(AstNode::SelectMode { mode:SelectMode::Distinct }),
                expr: Box::new(AstNode::Identifier { s: "a".to_string() })
            }
        };
    }

    #[test]
    fn function_expression_nested_function() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "COALESCE(POWER(1, 2), 3)",
            expected: AstNode::CoalesceFunction{
                exprs: vec![
                    AstNode::PowerFunction{
                        base: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                        exponent: Box::new(AstNode::IntegerLiteral { s: "2".to_string() })
                    },
                    AstNode::IntegerLiteral { s: "3".to_string() }
                ]
            }
        };
    }

    #[test]
    fn function_expression_date_trunc_function() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "DATE_TRUNC('month', '2020-06-01')",
            expected: AstNode::DateTruncFunction{
                format: Box::new(AstNode::StringLiteral { s: "month".to_string() }),
                datetime: Box::new(AstNode::StringLiteral { s: "2020-06-01".to_string() }),
            }
        };
    }

    #[test]
    fn function_expression_months_between_function() {
        parse_rule! {
            rule: Rule::function_expression,
            input: "MONTHS_BETWEEN('2020-05-01', '2020-06-01')",
            expected: AstNode::MonthsBetweenFunction{
                datetime1: Box::new(AstNode::StringLiteral { s: "2020-05-01".to_string() }),
                datetime2: Box::new(AstNode::StringLiteral { s: "2020-06-01".to_string() }),
            }
        };
    }

    // ------------------------------------------------------------------
    // Rule::identifier
    // ------------------------------------------------------------------

    #[test]
    fn identifier() {
        parse_rule! {
            rule: Rule::identifier,
            input: "name",
            expected: AstNode::Identifier { s: "name".to_string() }
        };
    }

    #[test]
    fn identifier_upper() {
        parse_rule! {
            rule: Rule::identifier,
            input: "NAME",
            expected: AstNode::Identifier { s: "NAME".to_string() }
        };
    }

    #[test]
    fn idenfier_alphanumeric() {
        parse_rule! {
            rule: Rule::identifier,
            input: "name4",
            expected: AstNode::Identifier { s: "name4".to_string() }
        };
    }

    #[test]
    fn idenfier_underscore() {
        parse_rule! {
            rule: Rule::identifier,
            input: "my_name",
            expected: AstNode::Identifier { s: "my_name".to_string() }
        };
    }

    // ------------------------------------------------------------------
    // Rule::with_clause
    // ------------------------------------------------------------------

    #[test]
    fn with_clause() {
        parse_rule! {
            rule: Rule::with_clause,
            input:  "my_cte AS ( SELECT 1 )",
            expected: AstNode::WithClause {
                identifier: Box::new(AstNode::Identifier { s: "my_cte".to_string() }),
                columns: vec![],
                query: Box::new(AstNode::SelectStatement {
                    common: vec![],
                    mode: SelectMode::All,
                    columns: vec![AstNode::IntegerLiteral { s: "1".to_string() }],
                    table_exprs: vec![],
                    where_expr: None,
                    group_by: None,
                }),
            }
        };
    }

    // ------------------------------------------------------------------
    // Rule::where_clause
    // ------------------------------------------------------------------

    #[test]
    fn where_with_expression() {
        parse_rule! {
            rule: Rule::where_clause,
            input: "WHERE a = 1",
            expected: AstNode::Expression {
                left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                op: Operation::Equal,
                right: Box::new(AstNode::IntegerLiteral { s: "1".to_string() })
            }
        };
    }

    #[test]
    fn where_with_expression_multi() {
        parse_rule! {
            rule: Rule::where_clause,
            input: "WHERE TRUE AND a = 1",
            expected: AstNode::Expression {
                left: Box::new(AstNode::BooleanLiteral { s: "TRUE".to_string() }),
                op: Operation::And,
                right: Box::new(AstNode::Expression {
                    left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                    op: Operation::Equal,
                    right: Box::new(AstNode::IntegerLiteral { s: "1".to_string() })
                })
            }
        };
    }

    #[test]
    fn where_with_unary_expression() {
        parse_rule! {
            rule: Rule::where_clause,
            input: "WHERE a IS NULL AND a = 1",
            expected: AstNode::Expression {
                left: Box::new(AstNode::IsNullExpression {
                    expr: Box::new(AstNode::Identifier { s: "a".to_string() }),
                    is_null: true,
                }),
                op: Operation::And,
                right: Box::new(AstNode::Expression {
                    left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                    op: Operation::Equal,
                    right: Box::new(AstNode::IntegerLiteral { s: "1".to_string() })
                })
            }
        };
    }

    // ------------------------------------------------------------------
    // Rule::select_statement
    // ------------------------------------------------------------------

    #[test]
    fn select_literal() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "SELECT 1",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![AstNode::IntegerLiteral { s: "1".to_string() }],
                table_exprs: vec![],
                where_expr: None,
                group_by: None,
            }
        };
    }

    #[test]
    fn select_with_cte() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "WITH my_cte AS ( SELECT 1 AS c ) SELECT c FROM my_cte",
            expected: AstNode::SelectStatement {
                common: vec![AstNode::WithClause {
                    identifier: Box::new(AstNode::Identifier { s: "my_cte".to_string() }),
                    columns: vec![],
                    query: Box::new(AstNode::SelectStatement {
                        common: vec![],
                        mode: SelectMode::All,
                        columns: vec![AstNode::NamedColumn {
                            expr: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                            alias: Some("c".to_string())
                        }],
                        table_exprs: vec![],
                        where_expr: None,
                        group_by: None,
                    }),
                }],
                mode: SelectMode::All,
                columns: vec![AstNode::Identifier { s: "c".to_string() }],
                table_exprs: vec![AstNode::NamedTableExpression {
                    name: Box::new(AstNode::Identifier{ s: "my_cte".to_string() }),
                    alias: None,
                }],
                where_expr: None,
                group_by: None,
            }
        };
    }

    #[test]
    fn select_with_cte_2() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "WITH cte_1 AS ( SELECT 1 AS a ), cte_2 AS ( SELECT 1 AS b ) SELECT a FROM cte_1",
            expected: AstNode::SelectStatement {
                common: vec![
                    AstNode::WithClause {
                        identifier: Box::new(AstNode::Identifier { s: "cte_1".to_string() }),
                        columns: vec![],
                        query: Box::new(AstNode::SelectStatement {
                            common: vec![],
                            mode: SelectMode::All,
                            columns: vec![AstNode::NamedColumn {
                                expr: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                                alias: Some("a".to_string())
                            }],
                            table_exprs: vec![],
                            where_expr: None,
                            group_by: None,
                        }),
                    },
                    AstNode::WithClause {
                        identifier: Box::new(AstNode::Identifier { s: "cte_2".to_string() }),
                        columns: vec![],
                        query: Box::new(AstNode::SelectStatement {
                            common: vec![],
                            mode: SelectMode::All,
                            columns: vec![AstNode::NamedColumn {
                                expr: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                                alias: Some("b".to_string())
                            }],
                            table_exprs: vec![],
                            where_expr: None,
                            group_by: None,
                        }),
                    }
                ],
                mode: SelectMode::All,
                columns: vec![AstNode::Identifier { s: "a".to_string() }],
                table_exprs: vec![AstNode::NamedTableExpression {
                    name: Box::new(AstNode::Identifier{ s: "cte_1".to_string() }),
                    alias: None,
                }],
                where_expr: None,
                group_by: None,
            }
        };
    }

    #[test]
    fn select_two_fields_from_table() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "SELECT a, b FROM table",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![
                    AstNode::Identifier { s: "a".to_string() },
                    AstNode::Identifier { s: "b".to_string() }
                ],
                table_exprs: vec![
                    AstNode::NamedTableExpression {
                        name: Box::new(AstNode::Identifier{ s: "table".to_string() }),
                        alias: None,
                    }
                ],
                where_expr: None,
                group_by: None,
            }
        };
    }

    #[test]
    fn select_two_fields_from_table_with_schema() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "SELECT a, b FROM schema.table",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![
                    AstNode::Identifier { s: "a".to_string() },
                    AstNode::Identifier { s: "b".to_string() }
                ],
                table_exprs: vec![AstNode::NamedTableExpression {
                    name: Box::new(AstNode::QualifiedIdentifier {
                        s: vec![
                            AstNode::Identifier { s: "schema".to_string() },
                            AstNode::Identifier { s: "table".to_string() }
                        ]
                    }),
                    alias: None,
                }],
                where_expr: None,
                group_by: None,
            }
        };
    }

    #[test]
    fn select_two_fields_from_named_table() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "SELECT a, b FROM table AS alias",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![
                    AstNode::Identifier { s: "a".to_string() },
                    AstNode::Identifier { s: "b".to_string() }
                ],
                table_exprs: vec![
                    AstNode::NamedTableExpression{
                        name: Box::new(AstNode::Identifier { s: "table".to_string() }),
                        alias: Some("alias".to_string())
                    },
                ],
                where_expr: None,
                group_by: None,
            }
        };
    }

    #[test]
    fn select_two_fields_from_named_table_2() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "SELECT a, b FROM schema.table AS alias",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![
                    AstNode::Identifier { s: "a".to_string() },
                    AstNode::Identifier { s: "b".to_string() }
                ],
                table_exprs: vec![
                    AstNode::NamedTableExpression{
                        name: Box::new(
                            AstNode::QualifiedIdentifier {
                                s: vec![
                                    AstNode::Identifier { s: "schema".to_string() },
                                    AstNode::Identifier { s: "table".to_string() }
                                ]
                            }
                        ),
                        alias: Some("alias".to_string())
                    },
                ],
                where_expr: None,
                group_by: None,
            }
        };
    }

    #[test]
    fn select_all_from_named_table_short() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "SELECT * FROM schema.table alias",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![AstNode::AllColumns],
                table_exprs: vec![
                    AstNode::NamedTableExpression{
                        name: Box::new(
                            AstNode::QualifiedIdentifier {
                                s: vec![
                                    AstNode::Identifier { s: "schema".to_string() },
                                    AstNode::Identifier { s: "table".to_string() }
                                ]
                            }
                        ),
                        alias: Some("alias".to_string())
                    },
                ],
                where_expr: None,
                group_by: None,
            }
        };
    }

    #[test]
    fn select_join() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "SELECT * FROM a JOIN b ON a.id = b.id",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![AstNode::AllColumns],
                table_exprs: vec![
                    AstNode::NamedTableExpression {
                        name: Box::new(AstNode::Identifier{ s: "a".to_string() }),
                        alias: None,
                    },
                    AstNode::JoinClause {
                        join_type: Box::new(AstNode::InnerJoin),
                        table_expr: Box::new(AstNode::NamedTableExpression {
                            name: Box::new(AstNode::Identifier{ s: "b".to_string() }),
                            alias: None,
                        }),
                        constraint: Box::new(AstNode::JoinConstraintOn {
                            expr: Box::new(AstNode::Expression {
                                left: Box::new(AstNode::QualifiedIdentifier {
                                    s: vec![
                                        AstNode::Identifier { s: "a".to_string() },
                                        AstNode::Identifier { s: "id".to_string() }
                                    ]
                                }),
                                op: Operation::Equal,
                                right: Box::new(AstNode::QualifiedIdentifier {
                                    s: vec![
                                        AstNode::Identifier { s: "b".to_string() },
                                        AstNode::Identifier { s: "id".to_string() }
                                    ]
                                }),
                            })
                        }),
                    }
                ],
                where_expr: None,
                group_by: None,
            }
        };
    }

    #[test]
    fn select_where_clause() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "SELECT * WHERE a = 1",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![AstNode::AllColumns],
                table_exprs: vec![],
                where_expr: Some(Box::new(AstNode::Expression {
                    left: Box::new(AstNode::Identifier { s: "a".to_string() }),
                    op: Operation::Equal,
                    right: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                })),
                group_by: None,
            }
        };
    }

    #[test]
    fn select_where_clause_from_table() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "SELECT * FROM a WHERE b = 1",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![AstNode::AllColumns],
                table_exprs: vec![AstNode::NamedTableExpression {
                    name: Box::new(AstNode::Identifier{ s: "a".to_string() }),
                    alias: None,
                }],
                where_expr: Some(Box::new(AstNode::Expression {
                    left: Box::new(AstNode::Identifier { s: "b".to_string() }),
                    op: Operation::Equal,
                    right: Box::new(AstNode::IntegerLiteral { s: "1".to_string() }),
                })),
                group_by: None,
            }
        };
    }

    #[test]
    fn select_distinct() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "SELECT DISTINCT a",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::Distinct,
                columns: vec![AstNode::Identifier { s: "a".to_string() }],
                table_exprs: vec![],
                where_expr: None,
                group_by: None,
            }
        };
    }

    #[test]
    fn select_all() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "SELECT ALL a",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![AstNode::Identifier { s: "a".to_string() }],
                table_exprs: vec![],
                where_expr: None,
                group_by: None,
            }
        };
    }

    #[test]
    fn select_group_by() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "SELECT a GROUP BY a",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![AstNode::Identifier { s: "a".to_string() }],
                table_exprs: vec![],
                where_expr: None,
                group_by: Some(Box::new(AstNode::GroupBy {
                    groupings: vec![AstNode::Identifier { s: "a".to_string() }],
                    having: None
                }))
            }
        };
    }

    #[test]
    fn select_join_clause_multiple() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "SELECT * FROM a JOIN b ON a.id = b.id JOIN c ON a.id = c.id",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![AstNode::AllColumns],
                table_exprs: vec![
                    AstNode::NamedTableExpression {
                        name: Box::new(AstNode::Identifier { s: "a".to_string() }),
                        alias: None,
                    },
                    AstNode::JoinClause {
                        join_type: Box::new(AstNode::InnerJoin),
                        table_expr: Box::new(AstNode::NamedTableExpression {
                            name: Box::new(AstNode::Identifier { s: "b".to_string() }),
                            alias: None,
                        }),
                        constraint: Box::new(AstNode::JoinConstraintOn {
                            expr: Box::new(AstNode::Expression {
                                left: Box::new(AstNode::QualifiedIdentifier {
                                    s: vec![
                                        AstNode::Identifier { s: "a".to_string() },
                                        AstNode::Identifier { s: "id".to_string() }
                                    ]
                                }),
                                op: Operation::Equal,
                                right: Box::new(AstNode::QualifiedIdentifier {
                                    s: vec![
                                        AstNode::Identifier { s: "b".to_string() },
                                        AstNode::Identifier { s: "id".to_string() }
                                    ]
                                }),
                            })
                        }),
                    },
                    AstNode::JoinClause {
                        join_type: Box::new(AstNode::InnerJoin),
                        table_expr: Box::new(AstNode::NamedTableExpression {
                            name: Box::new(AstNode::Identifier { s: "c".to_string() }),
                            alias: None,
                        }),
                        constraint: Box::new(AstNode::JoinConstraintOn {
                            expr: Box::new(AstNode::Expression {
                                left: Box::new(AstNode::QualifiedIdentifier {
                                    s: vec![
                                        AstNode::Identifier { s: "a".to_string() },
                                        AstNode::Identifier { s: "id".to_string() }
                                    ]
                                }),
                                op: Operation::Equal,
                                right: Box::new(AstNode::QualifiedIdentifier {
                                    s: vec![
                                        AstNode::Identifier { s: "c".to_string() },
                                        AstNode::Identifier { s: "id".to_string() }
                                    ]
                                }),
                            })
                        }),
                    }
                ],
                where_expr: None,
                group_by: None,
            }
        };
    }

    // ------------------------------------------------------------------
    // Rule::comment
    // ------------------------------------------------------------------

    #[test]
    fn comment_before_statement() {
        parse_rule! {
            rule: Rule::select_statement,
            input:
            r#"
                -- comment
                select 1
            "#.trim(),
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![AstNode::IntegerLiteral { s: "1".to_string() }],
                table_exprs: vec![],
                where_expr: None,
                group_by: None,
            }
        };
    }

    #[test]
    fn comment_inline() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "select 1 -- comment",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![AstNode::IntegerLiteral { s: "1".to_string() }],
                table_exprs: vec![],
                where_expr: None,
                group_by: None,
            }
        };
    }

    #[test]
    fn not_a_comment() {
        parse_rule! {
            rule: Rule::select_statement,
            input: "select '--'",
            expected: AstNode::SelectStatement {
                common: vec![],
                mode: SelectMode::All,
                columns: vec![AstNode::StringLiteral { s: "--".to_string() }],
                table_exprs: vec![],
                where_expr: None,
                group_by: None,
            }
        };
    }
}
