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

use super::ast::*;

#[derive(Default)]
pub struct FormatOptions {}

pub trait FormatSql {
    fn format(&self, opts: &FormatOptions) -> String;
}

impl FormatSql for SqlStatement {
    fn format(&self, opts: &FormatOptions) -> String {
        match self {
            Self::Statement(s) => s.format(opts),
            _ => unimplemented!("SqlStatemnt"),
        }
    }
}

impl FormatSql for Statement {
    fn format(&self, opts: &FormatOptions) -> String {
        match self {
            Self::Select(s) => format!("SELECT {}", s.expr.format(opts)),
            _ => unimplemented!("Statemnt"),
        }
    }
}

impl FormatSql for SetExpression {
    fn format(&self, opts: &FormatOptions) -> String {
        match self {
            Self::Query(q) => q.format(opts),
            _ => unimplemented!("SetExpression"),
        }
    }
}

impl FormatSql for QuerySetExpression {
    fn format(&self, opts: &FormatOptions) -> String {
        let columns = self.columns.format(opts);
        let from = self
            .from
            .iter()
            .map(|f| f.format(opts))
            .collect::<Vec<String>>()
            .join(" ");

        if from.len() > 0 {
            format!("{} FROM {}", columns, from)
        } else {
            format!("{}", columns)
        }
    }
}

impl FormatSql for ResultColumns {
    fn format(&self, opts: &FormatOptions) -> String {
        match self {
            Self::List(l) => l
                .iter()
                .map(|c| c.format(opts))
                .collect::<Vec<String>>()
                .join(" "),
            _ => unimplemented!("ResultColumns"),
        }
    }
}

impl FormatSql for ResultColumn {
    fn format(&self, opts: &FormatOptions) -> String {
        match self {
            Self::Expr(e) => e.format(opts),
            _ => unimplemented!("ResultColumn"),
        }
    }
}

impl FormatSql for ExprResultColumn {
    fn format(&self, opts: &FormatOptions) -> String {
        self.expr.format(opts)
    }
}

impl FormatSql for Expression {
    fn format(&self, opts: &FormatOptions) -> String {
        match self {
            Self::Literal(l) => l.format(opts),
            _ => unimplemented!("Expression"),
        }
    }
}

impl FormatSql for Literal {
    fn format(&self, _opts: &FormatOptions) -> String {
        match self {
            Self::Numeric(n) => n.clone(),
            _ => unimplemented!("Literal"),
        }
    }
}

impl FormatSql for TableExpression {
    fn format(&self, opts: &FormatOptions) -> String {
        match self {
            Self::Named(n) => n.format(opts),
            _ => unimplemented!("TableExpression"),
        }
    }
}
impl FormatSql for NamedTableExpression {
    fn format(&self, _opts: &FormatOptions) -> String {
        self.name
            .iter()
            .map(|n| n.as_str().to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
