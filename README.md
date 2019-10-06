# Hephaestus

[![Build Status](https://travis-ci.org/expobrain/hephaestus.svg?branch=master)](https://travis-ci.org/expobrain/hephaestus)

## Introduction

Hephaestus is a library to statically which parses a SQL query and return an AST.

This is useful for different purposes like:

- statistic informations, i.e. table or field used, tipe of joins, filter conditions, etc
- partial or full query comparison
- query type detection

The SQL grammar is specific for [Exasol](https://www.exasol.com/en/), but it can be easily ported to support other grammars as well.

Hephaestus's parser is built in [Rust](https://www.rust-lang.org/) using the [lalrpop](https://github.com/lalrpop/lalrpop) crate for grammar's definition and [vervolg](https://github.com/hmwill/vervolg) used as a base for the SQL grammar.

## Installation

To install the package you need to install the `nightly` version of the Rust compiler:

```shell
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=nightly
```

then install the package straight from Github:

```shell
pip install git+https://github.com/expobrain/hephaestus@develop
```

> Note: until version 0.8 of `pyo3` you need to have the `python` binary available in your `PATH`; on some systems it will fail because only `python3` is present. In this case install the Python 2 package.

## Usage

### Parsing

Hephaestus exposes a single `parse()` function which given a SQl string returns a dictionary representing the SQL in an AST format:

```python
>>> import hephaestus
>>> hephaestus.parse("SELECT 1")
{'_type': 'SelectStatement',
 'common': [],
 'expr': {'_type': 'QuerySetExpression',
  'mode': {'_type': 'SelectAll'},
  'columns': {'_type': 'ListResultColumn',
   'values': [{'_type': 'ExprResultColumn',
     'expr': {'_type': 'NumericLiteral', 'num': '1'},
     'rename': None}]},
  'from': [],
  'where_expr': None,
  'group_by': None},
 'order_by': [],
 'limit': None}
```

Every node of the dictionary has a `_type` key which indicate the type of the node, i.e. `SelectStatement`, and a series of keys specific for the node's type which can contains scalars values like `str`, `int`, `None`, a dictionary representing a node or a list of scalar values or nodes.

> For a full list of node types see `src/ast.rs` and `src/ast_py.rs`.

### Traversing

Parsing the incoming SQL expression is only the first step and its not very useful without the ability to traverse the tree and inspect the nodes.

Instead of writing from scratch the code to traverse the dictionary nested structure by yourself Hephaestus provides you a class to be used to walk through the tree and call a method for every node it encouters during the traverse.

Just inherith from the `Visitor` class and implement methods in the form of `visit_<node_type>` where `<node_type>` is the type of the node in the `_type` key.

This example shows how to check if the given SQL statement is querying from the `DUAL` table:

```python
from typing import Dict

from hephaestus.walk import Visitor


class MyVisitor(Visitor):
    has_dual_table = False

    def visit_NamedTableExpression(self, attr: str, node: Dict):
        self.has_dual_table = node["name"] == ["dual"]


visitor = MyVisitor("SELECT 1 FROM dual")
visitor.walk()

assert visitor.has_dual_table is True
```

When the `NamedTableExpression` node is reached the `visit_NamedTableExpression` method is called and receives the parent node's attribute name `attr` and the node itself.

## Testing

There are two set of tests, one for the Rust and one for the Python code.

> Note: Unfortunately at the moment it's not possible to execute both test suites with the same `Cargo.toml` (looks like that the `extension-module` feature in `pyo3` interfere with the way the tests are linked with the Rust library), so it's necessary to switch to a different config before running the them.

To run the Rust tests:

```shell
make test_rust
```

To run the Python tests:

```shell
make test_py
```

## Contribute

If you want to contribute to the Hephaestus project to extend the grammar follow this steps:

1. switch to a pure Rust `Cargo.toml` with `make rust`
1. write a test in `src/tests` to cover the cases
1. extend the grammar in `src/sql.lalrpop`; please refer to the [LALR Book](http://lalrpop.github.io/lalrpop/)
1. add the relative AST note to `src/ast.rs`
1. in some cases, like defining a new Rust enumerator, it's necessary to add an explicit conversion form the Rust type to a Python-compatible type; in this add the relevant code into `src/ast_py.rs`

## Caveats

- not all the grammar of Exasol is supported, most of the functions and statements needs to be implemented
- despite the fact that the AST returned by the `parse()` function can be manipulated at will at the moment there's no way to get back a SQL string
