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

#[macro_use]
mod macros;

use hephaestus::ast::*;
use hephaestus::parse;
use hephaestus::symbols;

test_builder!(
    select_from_table_with_schema,
    "select * from a.b",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::All,
            from: vec![TableExpression::Named(NamedTableExpression {
                name: vec![
                    symbols::Name::new("a".to_string()),
                    symbols::Name::new("b".to_string())
                ],
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
    select_from_aliased_table,
    "select * from a as b",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::All,
            from: vec![TableExpression::Named(NamedTableExpression {
                name: vec![symbols::Name::new("a".to_string())],
                alias: Some(symbols::Name::new("b".to_string()))
            })],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_from_aliased_table_with_schema,
    "select * from a.b as c",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::All,
            from: vec![TableExpression::Named(NamedTableExpression {
                name: vec![
                    symbols::Name::new("a".to_string()),
                    symbols::Name::new("b".to_string())
                ],
                alias: Some(symbols::Name::new("c".to_string()))
            })],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);

test_builder!(
    select_from_aliased_table_with_schema_short,
    "select * from a.b c",
    SqlStatement::Statement(Statement::Select(SelectStatement {
        common: vec![],
        expr: Box::new(SetExpression::Query(QuerySetExpression {
            mode: SelectMode::All,
            columns: ResultColumns::All,
            from: vec![TableExpression::Named(NamedTableExpression {
                name: vec![
                    symbols::Name::new("a".to_string()),
                    symbols::Name::new("b".to_string())
                ],
                alias: Some(symbols::Name::new("c".to_string()))
            })],
            where_expr: None,
            group_by: None,
        })),
        order_by: vec![],
        limit: None
    }))
);
