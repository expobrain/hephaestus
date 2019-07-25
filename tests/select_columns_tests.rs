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

#[macro_use]
mod macros;

use efesto::ast::*;
use efesto::parse;
use efesto::symbols;

test_builder!(
    select_minimum_no_table,
    "select 1",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::Literal(Literal::Numeric("1".to_string())),
                rename: None
            })]),
            from: vec![],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_minimum,
    "select 1 from dual",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::Literal(Literal::Numeric("1".to_string())),
                rename: None
            })]),
            from: vec![TableExpression::Named(NamedTableExpression {
                name: vec![symbols::Name::new("dual".to_string())],
                alias: None
            })],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_all_columns,
    "select *",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::All,
            from: vec![],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_with_two_fields,
    "select a, b from dual",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![
                ResultColumn::Expr(ExprResultColumn {
                    expr: Expression::QualifiedIdentifier(QualifiedIdentifierExpression {
                        identifiers: vec![symbols::Name::new("a".to_string())]
                    }),
                    rename: None
                }),
                ResultColumn::Expr(ExprResultColumn {
                    expr: Expression::QualifiedIdentifier(QualifiedIdentifierExpression {
                        identifiers: vec![symbols::Name::new("b".to_string())]
                    }),
                    rename: None
                })
            ]),
            from: vec![TableExpression::Named(NamedTableExpression {
                name: vec![symbols::Name::new("dual".to_string())],
                alias: None
            })],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_with_column_with_schema,
    "select a.b",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::QualifiedIdentifier(QualifiedIdentifierExpression {
                    identifiers: vec![
                        symbols::Name::new("a".to_string()),
                        symbols::Name::new("b".to_string())
                    ]
                }),
                rename: None
            })]),
            from: vec![],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_with_aliased_column,
    "select a as b",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::QualifiedIdentifier(QualifiedIdentifierExpression {
                    identifiers: vec![symbols::Name::new("a".to_string())]
                }),
                rename: Some(symbols::Name::new("b".to_string()))
            })]),
            from: vec![],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_with_aliased_column_with_schema,
    "select a.b as c",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::QualifiedIdentifier(QualifiedIdentifierExpression {
                    identifiers: vec![
                        symbols::Name::new("a".to_string()),
                        symbols::Name::new("b".to_string())
                    ]
                }),
                rename: Some(symbols::Name::new("c".to_string()))
            })]),
            from: vec![],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_binary_concat,
    "SELECT a || b",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::Binary(BinaryExpression {
                    op: BinaryOperator::Concat,
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
                }),
                rename: None
            })]),
            from: vec![],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_binary_concat_function,
    "SELECT COALESCE(1) || b",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::Binary(BinaryExpression {
                    op: BinaryOperator::Concat,
                    left: Box::new(Expression::Coalesce(CoalesceExpression {
                        exprs: vec![Expression::Literal(Literal::Numeric("1".to_string()))]
                    })),
                    right: Box::new(Expression::QualifiedIdentifier(
                        QualifiedIdentifierExpression {
                            identifiers: vec![symbols::Name::new("b".to_string())]
                        }
                    )),
                }),
                rename: None
            })]),
            from: vec![],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_binary_concat_function_reverse,
    "SELECT b || COALESCE(1)",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::Binary(BinaryExpression {
                    op: BinaryOperator::Concat,
                    left: Box::new(Expression::QualifiedIdentifier(
                        QualifiedIdentifierExpression {
                            identifiers: vec![symbols::Name::new("b".to_string())]
                        }
                    )),
                    right: Box::new(Expression::Coalesce(CoalesceExpression {
                        exprs: vec![Expression::Literal(Literal::Numeric("1".to_string()))]
                    })),
                }),
                rename: None
            })]),
            from: vec![],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_binary_concat_function_both_sides,
    "SELECT COALESCE(1) || COALESCE(2)",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::Binary(BinaryExpression {
                    op: BinaryOperator::Concat,
                    left: Box::new(Expression::Coalesce(CoalesceExpression {
                        exprs: vec![Expression::Literal(Literal::Numeric("1".to_string()))]
                    })),
                    right: Box::new(Expression::Coalesce(CoalesceExpression {
                        exprs: vec![Expression::Literal(Literal::Numeric("2".to_string()))]
                    })),
                }),
                rename: None
            })]),
            from: vec![],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_binary_concat_function_mixed_with_scalars,
    "SELECT COALESCE(1) || 'a' || COALESCE(2)",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::Binary(BinaryExpression {
                    op: BinaryOperator::Concat,
                    left: Box::new(Expression::Binary(BinaryExpression {
                        op: BinaryOperator::Concat,
                        left: Box::new(Expression::Coalesce(CoalesceExpression {
                            exprs: vec![Expression::Literal(Literal::Numeric("1".to_string()))]
                        })),
                        right: Box::new(Expression::Literal(Literal::String("a".to_string()))),
                    })),
                    right: Box::new(Expression::Coalesce(CoalesceExpression {
                        exprs: vec![Expression::Literal(Literal::Numeric("2".to_string()))]
                    })),
                }),
                rename: None
            })]),
            from: vec![],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_nested_functions,
    "SELECT COALESCE(POWER(1, 2), 3)",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::Coalesce(CoalesceExpression {
                    exprs: vec![
                        Expression::Power(PowerExpression {
                            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
                            exponent: Box::new(Expression::Literal(Literal::Numeric(
                                "2".to_string()
                            ))),
                        }),
                        Expression::Literal(Literal::Numeric("3".to_string())),
                    ]
                }),
                rename: None,
            })]),
            from: vec![],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_nested_functions_as_aliased_column,
    "SELECT COALESCE(POWER(1, 2), 2) as a",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::List(vec![ResultColumn::Expr(ExprResultColumn {
                expr: Expression::Coalesce(CoalesceExpression {
                    exprs: vec![
                        Expression::Power(PowerExpression {
                            base: Box::new(Expression::Literal(Literal::Numeric("1".to_string()))),
                            exponent: Box::new(Expression::Literal(Literal::Numeric(
                                "2".to_string()
                            ))),
                        }),
                        Expression::Literal(Literal::Numeric("2".to_string())),
                    ]
                }),
                rename: Some(symbols::Name::new("a".to_string()))
            })]),
            from: vec![],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);
