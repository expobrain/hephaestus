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

use efesto::ast::*;

use efesto::sql::ExpressionParser;
use efesto::symbols;

macro_rules! test_expression_builder {
    ($name:ident, $sql:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let result = ExpressionParser::new().parse($sql).unwrap();

            assert_eq!(result, $expected);
        }
    };
}

test_expression_builder!(
    select_where_equality_trivial,
    "1 = 1",
    Expression::Comparison(ComparisonExpression {
        op: ComparisonOperator::Equal,
        left: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
        right: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
    })
);

test_expression_builder!(
    select_where_equality_column_string_literal,
    "a = 'b'",
    Expression::Comparison(ComparisonExpression {
        op: ComparisonOperator::Equal,
        left: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
        right: Box::new(Expression::Literal(Literal::String("b".to_string()))),
    })
);

test_expression_builder!(
    select_where_equality_column_unsigned_integer,
    "a = 1",
    Expression::Comparison(ComparisonExpression {
        op: ComparisonOperator::Equal,
        left: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
        right: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
    })
);

test_expression_builder!(
    select_where_equality_column_with_schema_unsigned_integer,
    "a.b = 1",
    Expression::Comparison(ComparisonExpression {
        op: ComparisonOperator::Equal,
        left: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![
                    symbols::Name::new("a".to_string()),
                    symbols::Name::new("b".to_string()),
                ]
            }
        )),
        right: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
    })
);

test_expression_builder!(
    select_where_equality_column_with_column,
    "a = b",
    Expression::Comparison(ComparisonExpression {
        op: ComparisonOperator::Equal,
        left: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
        right: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("b".to_string())]
            }
        )),
    })
);

test_expression_builder!(
    select_where_equality_column_case,
    "a = CASE WHEN 1 THEN 'one' END",
    Expression::Comparison(ComparisonExpression {
        op: ComparisonOperator::Equal,
        left: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        )),
        right: Box::new(Expression::Case(CaseExpression {
            expr: None,
            when_part: vec![WhenClause {
                guard: Expression::Literal(Literal::Numeric("1".to_string())),
                body: Expression::Literal(Literal::String("one".to_string())),
            }],
            else_part: None,
        })),
    })
);

test_expression_builder!(
    select_where_equality_with_function,
    "b = TO_DATE(c)",
    Expression::Comparison(ComparisonExpression {
        op: ComparisonOperator::Equal,
        left: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("b".to_string())]
            }
        )),
        right: Box::new(Expression::ToDate(ToDateExpression {
            string: Box::new(Expression::QualifiedIdentifier(
                QualifiedIdentifierExpression {
                    identifiers: vec![symbols::Name::new("c".to_string())]
                }
            )),
            format: None,
        }))
    })
);

test_expression_builder!(
    select_where_unequality_with_function,
    "b != TO_DATE(c)",
    Expression::Comparison(ComparisonExpression {
        op: ComparisonOperator::NotEqual,
        left: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("b".to_string())]
            }
        )),
        right: Box::new(Expression::ToDate(ToDateExpression {
            string: Box::new(Expression::QualifiedIdentifier(
                QualifiedIdentifierExpression {
                    identifiers: vec![symbols::Name::new("c".to_string())]
                }
            )),
            format: None,
        }))
    })
);

test_expression_builder!(
    select_where_unequality_with_function_reverse,
    "TO_DATE(c) != b",
    Expression::Comparison(ComparisonExpression {
        op: ComparisonOperator::NotEqual,
        left: Box::new(Expression::ToDate(ToDateExpression {
            string: Box::new(Expression::QualifiedIdentifier(
                QualifiedIdentifierExpression {
                    identifiers: vec![symbols::Name::new("c".to_string())]
                }
            )),
            format: None,
        })),
        right: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("b".to_string())]
            }
        )),
    })
);

test_expression_builder!(
    select_where_unequality_with_function_both_sides,
    "TO_DATE(b) != TO_DATE(c)",
    Expression::Comparison(ComparisonExpression {
        op: ComparisonOperator::NotEqual,
        left: Box::new(Expression::ToDate(ToDateExpression {
            string: Box::new(Expression::QualifiedIdentifier(
                QualifiedIdentifierExpression {
                    identifiers: vec![symbols::Name::new("b".to_string())]
                }
            )),
            format: None,
        })),
        right: Box::new(Expression::ToDate(ToDateExpression {
            string: Box::new(Expression::QualifiedIdentifier(
                QualifiedIdentifierExpression {
                    identifiers: vec![symbols::Name::new("c".to_string())]
                }
            )),
            format: None,
        })),
    })
);

test_expression_builder!(
    select_where_multiplication_with_function,
    "b * POWER(1, 2)",
    Expression::Binary(BinaryExpression {
        op: BinaryOperator::Multiply,
        left: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("b".to_string())]
            }
        )),
        right: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
    })
);

test_expression_builder!(
    select_where_multiplication_with_function_both_sides,
    "POWER(1, 2) * POWER(1, 2)",
    Expression::Binary(BinaryExpression {
        op: BinaryOperator::Multiply,
        left: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
        right: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
    })
);

test_expression_builder!(
    select_where_multiplication_with_function_reverse,
    "POWER(1, 2) * b",
    Expression::Binary(BinaryExpression {
        op: BinaryOperator::Multiply,
        left: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
        right: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("b".to_string())]
            }
        )),
    })
);

test_expression_builder!(
    select_where_division_with_function,
    "b / POWER(1, 2)",
    Expression::Binary(BinaryExpression {
        op: BinaryOperator::Divide,
        left: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("b".to_string())]
            }
        )),
        right: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
    })
);

test_expression_builder!(
    select_where_division_with_function_reverse,
    "POWER(1, 2) / b",
    Expression::Binary(BinaryExpression {
        op: BinaryOperator::Divide,
        left: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
        right: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("b".to_string())]
            }
        )),
    })
);

test_expression_builder!(
    select_where_division_with_function_both_sides,
    "POWER(1, 2) / POWER(1, 2)",
    Expression::Binary(BinaryExpression {
        op: BinaryOperator::Divide,
        left: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
        right: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
    })
);

test_expression_builder!(
    select_where_addition_with_function,
    "b + POWER(1, 2)",
    Expression::Binary(BinaryExpression {
        op: BinaryOperator::Add,
        left: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("b".to_string())]
            }
        )),
        right: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
    })
);

test_expression_builder!(
    select_where_addition_with_function_reverse,
    "b + POWER(1, 2)",
    Expression::Binary(BinaryExpression {
        op: BinaryOperator::Add,
        left: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("b".to_string())]
            }
        )),
        right: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
    })
);

test_expression_builder!(
    select_where_addition_with_function_both_sides,
    "POWER(1, 2) + POWER(1, 2)",
    Expression::Binary(BinaryExpression {
        op: BinaryOperator::Add,
        left: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
        right: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
    })
);

test_expression_builder!(
    select_where_substract_with_function,
    "b - POWER(1, 2)",
    Expression::Binary(BinaryExpression {
        op: BinaryOperator::Subtract,
        left: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("b".to_string())]
            }
        )),
        right: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
    })
);

test_expression_builder!(
    select_where_substract_with_function_reverse,
    "b - POWER(1, 2)",
    Expression::Binary(BinaryExpression {
        op: BinaryOperator::Subtract,
        left: Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("b".to_string())]
            }
        )),
        right: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
    })
);

test_expression_builder!(
    select_where_substract_with_function_both_sides,
    "POWER(1, 2) - POWER(1, 2)",
    Expression::Binary(BinaryExpression {
        op: BinaryOperator::Subtract,
        left: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
        right: Box::new(Expression::Power(PowerExpression {
            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
        })),
    })
);

test_expression_builder!(
    select_where_nested_functions,
    "COALESCE(POWER(1, 2), 2)",
    Expression::Coalesce(CoalesceExpression {
        exprs: vec![
            Expression::Power(PowerExpression {
                base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
                exponent: Box::new(Expression::Literal(Literal::Numeric("2".to_string()))),
            }),
            Expression::Literal(Literal::Numeric("2".to_string())),
        ]
    })
);
