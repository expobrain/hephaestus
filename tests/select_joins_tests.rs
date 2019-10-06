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

use hephaestus::ast::*;
use hephaestus::parse;
use hephaestus::symbols;

test_builder!(
    select_join,
    "select * from a join b on a.id = b.id",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::All,
            from: vec![TableExpression::Join(JoinTableExpression {
                left: Box::new(TableExpression::Named(NamedTableExpression {
                    name: vec![symbols::Name::new("a".to_string())],
                    alias: None
                })),
                right: Box::new(TableExpression::Named(NamedTableExpression {
                    name: vec![symbols::Name::new("b".to_string())],
                    alias: None
                })),
                op: JoinOperator::Join(JoinType::Inner),
                constraint: JoinConstraint::Expr(Expression::Comparison(ComparisonExpression {
                    op: ComparisonOperator::Equal,
                    left: Box::new(Expression::QualifiedIdentifier(
                        QualifiedIdentifierExpression {
                            identifiers: vec![
                                symbols::Name::new("a".to_string()),
                                symbols::Name::new("id".to_string()),
                            ]
                        }
                    )),
                    right: Box::new(Expression::QualifiedIdentifier(
                        QualifiedIdentifierExpression {
                            identifiers: vec![
                                symbols::Name::new("b".to_string()),
                                symbols::Name::new("id".to_string()),
                            ]
                        }
                    )),
                }))
            })],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_join_with_aliases,
    "select * from a t1 join b t2 on t1.id = t2.id",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::All,
            from: vec![TableExpression::Join(JoinTableExpression {
                left: Box::new(TableExpression::Named(NamedTableExpression {
                    name: vec![symbols::Name::new("a".to_string())],
                    alias: Some(symbols::Name::new("t1".to_string())),
                })),
                right: Box::new(TableExpression::Named(NamedTableExpression {
                    name: vec![symbols::Name::new("b".to_string())],
                    alias: Some(symbols::Name::new("t2".to_string())),
                })),
                op: JoinOperator::Join(JoinType::Inner),
                constraint: JoinConstraint::Expr(Expression::Comparison(ComparisonExpression {
                    op: ComparisonOperator::Equal,
                    left: Box::new(Expression::QualifiedIdentifier(
                        QualifiedIdentifierExpression {
                            identifiers: vec![
                                symbols::Name::new("t1".to_string()),
                                symbols::Name::new("id".to_string()),
                            ]
                        }
                    )),
                    right: Box::new(Expression::QualifiedIdentifier(
                        QualifiedIdentifierExpression {
                            identifiers: vec![
                                symbols::Name::new("t2".to_string()),
                                symbols::Name::new("id".to_string()),
                            ]
                        }
                    )),
                }))
            })],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_left_join,
    "select * from a left join b on a.id = b.id",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::All,
            from: vec![TableExpression::Join(JoinTableExpression {
                left: Box::new(TableExpression::Named(NamedTableExpression {
                    name: vec![symbols::Name::new("a".to_string())],
                    alias: None
                })),
                right: Box::new(TableExpression::Named(NamedTableExpression {
                    name: vec![symbols::Name::new("b".to_string())],
                    alias: None
                })),
                op: JoinOperator::Join(JoinType::Left),
                constraint: JoinConstraint::Expr(Expression::Comparison(ComparisonExpression {
                    op: ComparisonOperator::Equal,
                    left: Box::new(Expression::QualifiedIdentifier(
                        QualifiedIdentifierExpression {
                            identifiers: vec![
                                symbols::Name::new("a".to_string()),
                                symbols::Name::new("id".to_string()),
                            ]
                        }
                    )),
                    right: Box::new(Expression::QualifiedIdentifier(
                        QualifiedIdentifierExpression {
                            identifiers: vec![
                                symbols::Name::new("b".to_string()),
                                symbols::Name::new("id".to_string()),
                            ]
                        }
                    )),
                }))
            })],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_join_complex_constraint,
    "select * from a join b on a.f1 = b.f1 and a.f2 = b.f2",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::All,
            from: vec![TableExpression::Join(JoinTableExpression {
                left: Box::new(TableExpression::Named(NamedTableExpression {
                    name: vec![symbols::Name::new("a".to_string())],
                    alias: None
                })),
                right: Box::new(TableExpression::Named(NamedTableExpression {
                    name: vec![symbols::Name::new("b".to_string())],
                    alias: None
                })),
                op: JoinOperator::Join(JoinType::Inner),
                constraint: JoinConstraint::Expr(Expression::Binary(BinaryExpression {
                    op: BinaryOperator::And,
                    left: Box::new(Expression::Comparison(ComparisonExpression {
                        op: ComparisonOperator::Equal,
                        left: Box::new(Expression::QualifiedIdentifier(
                            QualifiedIdentifierExpression {
                                identifiers: vec![
                                    symbols::Name::new("a".to_string()),
                                    symbols::Name::new("f1".to_string()),
                                ]
                            }
                        )),
                        right: Box::new(Expression::QualifiedIdentifier(
                            QualifiedIdentifierExpression {
                                identifiers: vec![
                                    symbols::Name::new("b".to_string()),
                                    symbols::Name::new("f1".to_string()),
                                ]
                            }
                        ))
                    })),
                    right: Box::new(Expression::Comparison(ComparisonExpression {
                        op: ComparisonOperator::Equal,
                        left: Box::new(Expression::QualifiedIdentifier(
                            QualifiedIdentifierExpression {
                                identifiers: vec![
                                    symbols::Name::new("a".to_string()),
                                    symbols::Name::new("f2".to_string()),
                                ]
                            }
                        )),
                        right: Box::new(Expression::QualifiedIdentifier(
                            QualifiedIdentifierExpression {
                                identifiers: vec![
                                    symbols::Name::new("b".to_string()),
                                    symbols::Name::new("f2".to_string()),
                                ]
                            }
                        ))
                    })),
                }))
            })],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);
