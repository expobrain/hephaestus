// MIT License
//
// Copyright (c) 2018 Hans-Martin Will
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use hephaestus::ast::*;
use hephaestus::sql::FunctionExpressionParser;
use hephaestus::symbols;

macro_rules! test_function_builder {
    ($name:ident, $sql:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let result = FunctionExpressionParser::new().parse($sql).unwrap();

            assert_eq!(result, $expected);
        }
    };
}

test_function_builder!(
    unknown_function,
    "unknown_function(1, 2)",
    Expression::Unknown(UnknownExpression {
        name: vec![symbols::Name::new("unknown_function".to_string())],
        exprs: vec![
            Expression::Literal(Literal::Numeric("1".to_string())),
            Expression::Literal(Literal::Numeric("2".to_string())),
        ]
    })
);

test_function_builder!(
    unknown_function_with_schema,
    "schema.unknown_function(1, 2)",
    Expression::Unknown(UnknownExpression {
        name: vec![
            symbols::Name::new("schema".to_string()),
            symbols::Name::new("unknown_function".to_string())
        ],
        exprs: vec![
            Expression::Literal(Literal::Numeric("1".to_string())),
            Expression::Literal(Literal::Numeric("2".to_string())),
        ]
    })
);

test_function_builder!(
    coalesce_function,
    "coalesce(1, 2)",
    Expression::Coalesce(CoalesceExpression {
        exprs: vec![
            Expression::Literal(Literal::Numeric("1".to_string())),
            Expression::Literal(Literal::Numeric("2".to_string())),
        ]
    })
);

test_function_builder!(
    replace_function,
    "replace('a', 'b')",
    Expression::Replace(ReplaceExpression {
        string: Box::new(Expression::Literal(Literal::String("a".to_string()))),
        search_string: Box::new(Expression::Literal(Literal::String("b".to_string()))),
        replace_string: None
    })
);

test_function_builder!(
    replace_function_from_column,
    "replace(a, 'b')",
    Expression::Replace(ReplaceExpression {
        string: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
        search_string: Box::new(Expression::Literal(Literal::String("b".to_string()))),
        replace_string: None
    })
);

test_function_builder!(
    replace_function_with_replace_string,
    "replace('a', 'b', 'c')",
    Expression::Replace(ReplaceExpression {
        string: Box::new(Expression::Literal(Literal::String("a".to_string()))),
        search_string: Box::new(Expression::Literal(Literal::String("b".to_string()))),
        replace_string: Some(Box::new(Expression::Literal(Literal::String(
            "c".to_string()
        ))))
    })
);

test_function_builder!(
    replace_function_from_column_with_replace_string,
    "replace(a, 'b', 'c')",
    Expression::Replace(ReplaceExpression {
        string: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
        search_string: Box::new(Expression::Literal(Literal::String("b".to_string()))),
        replace_string: Some(Box::new(Expression::Literal(Literal::String(
            "c".to_string()
        )))),
    })
);

test_function_builder!(
    substr_function,
    "SUBSTR('abc', 1)",
    Expression::Substring(SubstringExpression {
        string: Box::new(Expression::Literal(Literal::String("abc".to_string()))),
        position: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
        length: None,
    })
);

test_function_builder!(
    substr_function_with_length,
    "SUBSTR('abc', 1, 2)",
    Expression::Substring(SubstringExpression {
        string: Box::new(Expression::Literal(Literal::String("abc".to_string()))),
        position: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
        length: Some(Box::new(Expression::Literal(Literal::Numeric(
            "2".to_string()
        )))),
    })
);

test_function_builder!(
    substring_function,
    "SUBSTRING('abc' FROM 1)",
    Expression::Substring(SubstringExpression {
        string: Box::new(Expression::Literal(Literal::String("abc".to_string()))),
        position: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
        length: None,
    })
);

test_function_builder!(
    substring_function_with_length,
    "SUBSTRING('abc' FROM 1 FOR 2)",
    Expression::Substring(SubstringExpression {
        string: Box::new(Expression::Literal(Literal::String("abc".to_string()))),
        position: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
        length: Some(Box::new(Expression::Literal(Literal::Numeric(
            "2".to_string()
        )))),
    })
);

test_function_builder!(
    substring_function_on_column,
    "SUBSTR(a, 1)",
    Expression::Substring(SubstringExpression {
        string: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
        position: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
        length: None,
    })
);

test_function_builder!(
    substring_function_on_column_with_schema,
    "SUBSTR(a.b, 1)",
    Expression::Substring(SubstringExpression {
        string: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![
                    symbols::Name::new("a".to_string()),
                    symbols::Name::new("b".to_string())
                ]
            }
        )),
        position: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
        length: None,
    })
);

test_function_builder!(
    undocumented_substring_function,
    "SUBSTRING('abc', 1)",
    Expression::Substring(SubstringExpression {
        string: Box::new(Expression::Literal(Literal::String("abc".to_string()))),
        position: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
        length: None,
    })
);

test_function_builder!(
    to_date_function,
    "TO_DATE(a)",
    Expression::ToDate(ToDateExpression {
        string: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
        format: None,
    })
);

test_function_builder!(
    to_date_function_with_format,
    "TO_DATE(a, 'YYYY-MM-DD')",
    Expression::ToDate(ToDateExpression {
        string: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
        format: Some(Box::new(Expression::Literal(Literal::String(
            "YYYY-MM-DD".to_string()
        )))),
    })
);

test_function_builder!(
    power_function,
    "POWER(1, 2)",
    Expression::Power(PowerExpression {
        base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
        exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
    })
);

test_function_builder!(
    concat_function,
    "CONCAT('a', 'b')",
    Expression::Concat(ConcatExpression {
        exprs: vec![
            Expression::Literal(Literal::String("a".to_string())),
            Expression::Literal(Literal::String("b".to_string())),
        ],
    })
);

test_function_builder!(
    max_function,
    "MAX(a)",
    Expression::Max(MaxExpression {
        mode: SelectMode::All,
        expr: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
    })
);

test_function_builder!(
    max_function_all,
    "MAX(ALL a)",
    Expression::Max(MaxExpression {
        mode: SelectMode::All,
        expr: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
    })
);

test_function_builder!(
    max_function_distinct,
    "MAX(DISTINCT a)",
    Expression::Max(MaxExpression {
        mode: SelectMode::Distinct,
        expr: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
    })
);

test_function_builder!(
    min_function,
    "MIN(a)",
    Expression::Min(MinExpression {
        mode: SelectMode::All,
        expr: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
    })
);

test_function_builder!(
    sum_function_all,
    "SUM(ALL a)",
    Expression::Sum(SumExpression {
        mode: SelectMode::All,
        expr: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
    })
);

test_function_builder!(
    sum_function_distinct,
    "SUM(DISTINCT a)",
    Expression::Sum(SumExpression {
        mode: SelectMode::Distinct,
        expr: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
    })
);

test_function_builder!(
    cast_function_distinct,
    "CAST(a AS BOOLEAN)",
    Expression::Cast(CastExpression {
        expr: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
        data_type: DataType::Boolean,
    })
);

test_function_builder!(
    right_function,
    "RIGHT(a, 3)",
    Expression::Right(RightExpression {
        string: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
        length: Box::new(Expression::Literal(Literal::Numeric("3".to_string()))),
    })
);

test_function_builder!(
    count_function,
    "COUNT(*)",
    Expression::Count(CountExpression {
        columns: ResultColumns::All,
        mode: SelectMode::All,
    })
);

test_function_builder!(
    count_function_column,
    "COUNT(a)",
    Expression::Count(CountExpression {
        columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
            expr: Expression::QualifiedIdentifier(QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }),
            rename: None,
        })]),
        mode: SelectMode::All,
    })
);

test_function_builder!(
    count_function_column_all,
    "COUNT(ALL a)",
    Expression::Count(CountExpression {
        columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
            expr: Expression::QualifiedIdentifier(QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }),
            rename: None,
        })]),
        mode: SelectMode::All,
    })
);

test_function_builder!(
    count_function_column_distinct,
    "COUNT(DISTINCT a)",
    Expression::Count(CountExpression {
        columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
            expr: Expression::QualifiedIdentifier(QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }),
            rename: None,
        })]),
        mode: SelectMode::Distinct,
    })
);

test_function_builder!(
    count_function_columns,
    "COUNT(a, b)",
    Expression::Count(CountExpression {
        columns: ResultColumns::List(vec![
            ResultColumn::Expr(ExprResultColumn {
                expr: Expression::QualifiedIdentifier(QualifiedIdentifierExpression {
                    identifiers: vec![symbols::Name::new("a".to_string())]
                }),
                rename: None,
            }),
            ResultColumn::Expr(ExprResultColumn {
                expr: Expression::QualifiedIdentifier(QualifiedIdentifierExpression {
                    identifiers: vec![symbols::Name::new("b".to_string())]
                }),
                rename: None,
            }),
        ]),
        mode: SelectMode::All,
    })
);
