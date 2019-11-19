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

// test_builder!(
//     create_table,
//     "CREATE TABLE t (a VARCHAR(10))",
//     SqlStatement::Statement(Statement::Create(CreateStatement {
//         replace: false,
//         if_not_exists: false,
//         table_name: vec![symbols::Name::new("t".to_string())],
//         content: CreateTableContent::ColumnDefinitions(vec![ColumnDefinition {
//             id: symbols::Name::new("a".to_string()),
//             data_type: DataType::Varchar(Literal::Numeric("10".to_string()))
//         }])
//     }))
// );

// test_builder!(
//     create_table_with_replace,
//     "CREATE OR REPLACE TABLE t (a VARCHAR(10))",
//     SqlStatement::Statement(Statement::Create(CreateStatement {
//         replace: true,
//         if_not_exists: false,
//         table_name: vec![symbols::Name::new("t".to_string())],
//         content: CreateTableContent::ColumnDefinitions(vec![ColumnDefinition {
//             id: symbols::Name::new("a".to_string()),
//             data_type: DataType::Varchar(Literal::Numeric("10".to_string()))
//         }])
//     }))
// );

// test_builder!(
//     create_table_if_not_exists,
//     "CREATE TABLE IF NOT EXISTS t (a VARCHAR(10))",
//     SqlStatement::Statement(Statement::Create(CreateStatement {
//         replace: false,
//         if_not_exists: true,
//         table_name: vec![symbols::Name::new("t".to_string())],
//         content: CreateTableContent::ColumnDefinitions(vec![ColumnDefinition {
//             id: symbols::Name::new("a".to_string()),
//             data_type: DataType::Varchar(Literal::Numeric("10".to_string()))
//         }])
//     }))
// );

test_builder!(
    create_table_two_columns,
    "CREATE TABLE IF NOT EXISTS t (a VARCHAR(10), b BOOL)",
    SqlStatement::Statement(Statement::Create(CreateStatement {
        replace: false,
        if_not_exists: true,
        table_name: vec![symbols::Name::new("t".to_string())],
        content: CreateTableContent::ColumnDefinitions(vec![])
            // ColumnDefinition {
            //     id: symbols::Name::new("a".to_string()),
            //     data_type: DataType::Varchar(Literal::Numeric("10".to_string()))
            // },
            // ColumnDefinition {
            //     id: symbols::Name::new("b".to_string()),
            //     data_type: DataType::Boolean
            // }
        // ])
    }))
);
