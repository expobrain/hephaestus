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

extern crate lalrpop_util;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate dict_derive;
extern crate serde_json;

use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub mod ast;
pub mod error;
pub mod sql;
pub mod symbols;
pub mod types;

mod ast_py;

fn strip_inline_comments(s: &str) -> String {
    match s.rfind("--") {
        Some(pos) => match s[..pos].matches('\'').count() % 2 {
            1 => s.to_owned(),
            _ => s[..pos].to_owned(),
        },
        None => s.to_owned(),
    }
}

#[pyfunction]
pub fn parse(sql_str: &str) -> PyResult<ast::SqlStatement> {
    // This is an hack to strip comments from the original SQL
    // because LALRPOP doesn't support that yet.
    // See https://github.com/lalrpop/lalrpop/issues/10.

    let stripped_sql: String = sql_str
        .lines()
        .filter(|s| !s.trim_start().starts_with("--"))
        .map(strip_inline_comments)
        .collect::<String>();

    let result = sql::SqlStatementParser::new().parse(&stripped_sql);

    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(PyErr::new::<exceptions::ValueError, _>(format!("{}", e))),
    }
}

#[pymodule]
fn hephaestus(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(parse))?;

    Ok(())
}
