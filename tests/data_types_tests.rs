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

use hephaestus::sql;

#[allow(unused_macros)]
macro_rules! test_data_types {
    ($name:ident, $sql:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let result = sql::DataTypeParser::new().parse($sql).unwrap();

            assert_eq!(result, $expected);
        }
    };
}

test_data_types!(boolean_type, "BOOLEAN", DataType::Boolean);

test_data_types!(
    char_type,
    "CHAR(10)",
    DataType::Char(Literal::Numeric("10".to_string()))
);

test_data_types!(date_type, "DATE", DataType::Date);

test_data_types!(
    decimal_type,
    "DECIMAL(10, 2)",
    DataType::Decimal {
        p: Literal::Numeric("10".to_string()),
        s: Literal::Numeric("2".to_string()),
    }
);

test_data_types!(timestamp_type, "TIMESTAMP", DataType::Timestamp);

test_data_types!(
    timestamp_local_type,
    "TIMESTAMP WITH LOCAL TIME ZONE",
    DataType::LocalTimestamp
);

test_data_types!(
    varchar_type,
    "VARCHAR(10)",
    DataType::Varchar(Literal::Numeric("10".to_string()))
);
