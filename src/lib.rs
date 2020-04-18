#[macro_use]
extern crate pest_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate pest;
extern crate dict_derive;

use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::Python;

mod ast;
mod ast_py;
mod parser;
mod parser_pest;

#[pyfunction]
pub fn parse(sql_str: &str) -> PyResult<ast::AstNode> {
    let result = parser::parse(sql_str);

    match result {
        Ok(r) => Ok(r),
        Err(e) => exceptions::TypeError::into(format!("{}", e)),
    }
}

#[pymodule]
fn hephaestus(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(parse))?;

    Ok(())
}
