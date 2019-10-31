// MIT License
//
// Copyright (c) 2019 Daniele Esposti
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
use hephaestus::sql::CaseExpressionParser;
use hephaestus::symbols;

macro_rules! test_case_builder {
    ($name:ident, $sql:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let result = CaseExpressionParser::new().parse($sql).unwrap();

            assert_eq!(result, $expected);
        }
    };
}

test_case_builder!(
    simple_case,
    "CASE a WHEN 1 THEN 'one' END",
    Expression::Case(CaseExpression {
        expr: Some(Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        ))),
        when_part: vec![WhenClause {
            guard: Expression::Literal(Literal::Numeric("1".to_string())),
            body: Expression::Literal(Literal::String("one".to_string())),
        }],
        else_part: None,
    })
);

test_case_builder!(
    simple_case_multiple_when,
    "CASE a WHEN 1 THEN 'one' WHEN 2 THEN 'two' END",
    Expression::Case(CaseExpression {
        expr: Some(Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        ))),
        when_part: vec![
            WhenClause {
                guard: Expression::Literal(Literal::Numeric("1".to_string())),
                body: Expression::Literal(Literal::String("one".to_string())),
            },
            WhenClause {
                guard: Expression::Literal(Literal::Numeric("2".to_string())),
                body: Expression::Literal(Literal::String("two".to_string())),
            }
        ],
        else_part: None,
    })
);

test_case_builder!(
    simple_case_with_else,
    "CASE a WHEN 1 THEN 'one' ELSE 'none' END",
    Expression::Case(CaseExpression {
        expr: Some(Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        ))),
        when_part: vec![WhenClause {
            guard: Expression::Literal(Literal::Numeric("1".to_string())),
            body: Expression::Literal(Literal::String("one".to_string())),
        }],
        else_part: Some(Box::new(Expression::Literal(Literal::String(
            "none".to_string()
        )))),
    })
);

test_case_builder!(
    simple_case_multiple_when_with_else,
    "CASE a WHEN 1 THEN 'one' WHEN 2 THEN 'two' ELSE 'none' END",
    Expression::Case(CaseExpression {
        expr: Some(Box::new(Expression::QualifiedIdentifier(
            QualifiedIdentifierExpression {
                identifiers: vec![symbols::Name::new("a".to_string())]
            }
        ))),
        when_part: vec![
            WhenClause {
                guard: Expression::Literal(Literal::Numeric("1".to_string())),
                body: Expression::Literal(Literal::String("one".to_string())),
            },
            WhenClause {
                guard: Expression::Literal(Literal::Numeric("2".to_string())),
                body: Expression::Literal(Literal::String("two".to_string())),
            }
        ],
        else_part: Some(Box::new(Expression::Literal(Literal::String(
            "none".to_string()
        )))),
    })
);

test_case_builder!(
    searched_case,
    "CASE WHEN 1 THEN 'one' END",
    Expression::Case(CaseExpression {
        expr: None,
        when_part: vec![WhenClause {
            guard: Expression::Literal(Literal::Numeric("1".to_string())),
            body: Expression::Literal(Literal::String("one".to_string())),
        }],
        else_part: None,
    })
);

test_case_builder!(
    searched_case_multiple_cases,
    "CASE WHEN 1 THEN 'one' WHEN 2 THEN 'two' END",
    Expression::Case(CaseExpression {
        expr: None,
        when_part: vec![
            WhenClause {
                guard: Expression::Literal(Literal::Numeric("1".to_string())),
                body: Expression::Literal(Literal::String("one".to_string())),
            },
            WhenClause {
                guard: Expression::Literal(Literal::Numeric("2".to_string())),
                body: Expression::Literal(Literal::String("two".to_string())),
            }
        ],
        else_part: None,
    })
);

test_case_builder!(
    searched_case_with_else,
    "CASE WHEN 1 THEN 'one' ELSE 'none' END",
    Expression::Case(CaseExpression {
        expr: None,
        when_part: vec![WhenClause {
            guard: Expression::Literal(Literal::Numeric("1".to_string())),
            body: Expression::Literal(Literal::String("one".to_string())),
        }],
        else_part: Some(Box::new(Expression::Literal(Literal::String(
            "none".to_string()
        )))),
    })
);

test_case_builder!(
    searched_case_multiple_when_with_else,
    "CASE WHEN 1 THEN 'one' WHEN 2 THEN 'two' ELSE 'none' END",
    Expression::Case(CaseExpression {
        expr: None,
        when_part: vec![
            WhenClause {
                guard: Expression::Literal(Literal::Numeric("1".to_string())),
                body: Expression::Literal(Literal::String("one".to_string())),
            },
            WhenClause {
                guard: Expression::Literal(Literal::Numeric("2".to_string())),
                body: Expression::Literal(Literal::String("two".to_string())),
            }
        ],
        else_part: Some(Box::new(Expression::Literal(Literal::String(
            "none".to_string()
        )))),
    })
);

test_case_builder!(
    searched_case_with_function,
    "CASE WHEN 1 THEN SUBSTR('abc', 1) END",
    Expression::Case(CaseExpression {
        expr: None,
        when_part: vec![WhenClause {
            guard: Expression::Literal(Literal::Numeric("1".to_string())),
            body: Expression::Substring(SubstringExpression {
                string: Box::new(Expression::Literal(Literal::String("abc".to_string()))),
                position: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
                length: None,
            }),
        }],
        else_part: None,
    })
);

test_case_builder!(
    searched_case_with_function_in_else,
    "CASE WHEN 1 THEN 'one' ELSE SUBSTR('abc', 1) END",
    Expression::Case(CaseExpression {
        expr: None,
        when_part: vec![WhenClause {
            guard: Expression::Literal(Literal::Numeric("1".to_string())),
            body: Expression::Literal(Literal::String("one".to_string())),
        }],
        else_part: Some(Box::new(Expression::Substring(SubstringExpression {
            string: Box::new(Expression::Literal(Literal::String("abc".to_string()))),
            position: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
            length: None,
        }))),
    })
);
