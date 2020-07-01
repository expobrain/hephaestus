use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use pyo3::{IntoPy, PyObject, Python};

use crate::ast::*;

macro_rules! py_attr {
    // Accepts a pair of attribute and key names to build a Python dictionary
    ( $py:ident; $dict:ident; $attr:ident => $key:expr ) => {
        $dict
            .set_item(
                $key,
                $attr
                    .iter()
                    .map(|e| e.clone().into_py($py))
                    .collect::<Vec<PyObject>>(),
            )
            .unwrap();
    };

    // Converts a Box<T> into a Pytohn object
    ( $py:ident; $dict:ident; $attr:ident -> $key:expr ) => {
        $dict
            .set_item::<&str, PyObject>($key, (*$attr).into_py($py))
            .unwrap();
    };

    // Converts a T into a Python object
    ( $py:ident; $dict:ident; $attr:ident >> $key:expr ) => {
        $dict
            .set_item::<&str, PyObject>($key, $attr.into_py($py))
            .unwrap();
    };

    // Convers an Option<Box<T>> into a Python object
    ( $py:ident; $dict:ident; $attr:ident ? $key:expr ) => {
        if let Some(value) = $attr {
            $dict
                .set_item::<&str, PyObject>($key, (*value).into_py($py))
                .unwrap();
        }
    };

    // Converts an Option<T> into a Python object
    ( $py:ident; $dict:ident; $attr:ident - $key:expr ) => {
        if let Some(value) = $attr {
            $dict
                .set_item::<&str, PyObject>($key, value.into_py($py))
                .unwrap();
        }
    };
}

macro_rules! py_dict {
    ( $py:expr; $type:expr; $( $key:tt $op:tt $attr:tt ),* ) => {
        {
            let py = $py;
            let dict = PyDict::new($py);
            dict.set_item("_type", $type).unwrap();

            $(
                py_attr![py; dict; $key $op $attr];
            )*

            dict
        }
    };
    ( $py:expr; $type:expr ) => {
        {
            let dict = PyDict::new($py);
            dict.set_item("_type", $type).unwrap();

            dict
        }
    }
}

impl IntoPy<PyObject> for SelectMode {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Self::Distinct => PyString::new(py, "DISTINCT"),
            Self::All => PyString::new(py, "ALL"),
        }
        .to_object(py)
    }
}

impl IntoPy<PyObject> for Operation {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Self::Multiply => PyString::new(py, "*"),
            Self::Divide => PyString::new(py, "/"),
            Self::Add => PyString::new(py, "+"),
            Self::Subtract => PyString::new(py, "-"),
            Self::Concat => PyString::new(py, "||"),
            Self::And => PyString::new(py, "AND"),
            Self::Or => PyString::new(py, "OR"),
            Self::Equal => PyString::new(py, "="),
            Self::NotEqual => PyString::new(py, "!="),
            Self::GreaterThan => PyString::new(py, ">"),
            Self::GreaterOrEqualThan => PyString::new(py, ">="),
            Self::LessThan => PyString::new(py, "<"),
            Self::LessOrEqualThan => PyString::new(py, "<="),
        }
        .to_object(py)
    }
}

impl IntoPy<PyObject> for Sign {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Self::Positive => PyString::new(py, "+"),
            Self::Negative => PyString::new(py, "-"),
        }
        .to_object(py)
    }
}

impl IntoPy<PyObject> for Interval {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Self::Year => PyString::new(py, "YEAR"),
            Self::Month => PyString::new(py, "MONTH"),
            Self::Day => PyString::new(py, "DAY"),
            Self::Hour => PyString::new(py, "HOUR"),
            Self::Minute => PyString::new(py, "MINUTE"),
            Self::Second => PyString::new(py, "SECOND"),
        }
        .to_object(py)
    }
}

impl IntoPy<PyObject> for AstNode {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            // ------------------------------------------------------------------
            // Comments
            // ------------------------------------------------------------------
            AstNode::Comment { s } => py_dict![py; "Comment"; s -> "s"],

            // ------------------------------------------------------------------
            // Select statement
            // ------------------------------------------------------------------
            AstNode::SelectStatement {
                common,
                mode,
                columns,
                table_exprs,
                where_expr,
                group_by,
            } => py_dict![py;
                "SelectStatement";
                common => "common",
                mode >> "mode",
                columns => "columns",
                table_exprs => "table_exprs",
                where_expr ? "where_expr",
                group_by ? "group_by"
            ],
            AstNode::SelectMode { mode } => py_dict![py; "SelectMode"; mode >> "mode"],
            AstNode::WithClause {
                identifier,
                columns,
                query,
            } => py_dict![
                py;
                "WithClause";
                identifier -> "identifier", columns => "columns", query -> "query"
            ],
            AstNode::GroupBy { groupings, having } => py_dict![
                py;
                "GroupBy";
                groupings >> "groupings", having ? "having"
            ],

            // ------------------------------------------------------------------
            // Columns
            // ------------------------------------------------------------------
            AstNode::Column { expr } => py_dict![py; "Column"; expr -> "expr"],
            AstNode::AllColumns => py_dict![py; "AllColumns"],
            AstNode::AllColumnsFrom { schema } => {
                py_dict![py; "AllColumnsFrom"; schema -> "schema"]
            }
            AstNode::NamedColumn { expr, alias } => py_dict![
                py;
                "NamedColumn";
                expr -> "expr", alias ? "alias"
            ],

            // ------------------------------------------------------------------
            // Identifier
            // ------------------------------------------------------------------
            AstNode::Identifier { s } => py_dict![py; "Identifier"; s >> "s"],
            AstNode::QualifiedIdentifier { s } => py_dict![py; "QualifiedIdentifier"; s => "s"],

            // ------------------------------------------------------------------
            // Literals
            // ------------------------------------------------------------------
            AstNode::IntegerLiteral { s } => py_dict![py; "IntegerLiteral"; s -> "s"],
            AstNode::DecimalLiteral { s } => py_dict![py; "DecimalLiteral"; s -> "s"],
            AstNode::FloatLiteral { s } => py_dict![py; "FloatLiteral"; s -> "s"],
            AstNode::StringLiteral { s } => py_dict![py; "StringLiteral"; s -> "s"],
            AstNode::BooleanLiteral { s } => py_dict![py; "BooleanLiteral"; s -> "s"],
            AstNode::IntervalLiteral {
                interval,
                period,
                precision,
                convert_to,
                convert_precision,
            } => py_dict![
                py;
                "IntervalLiteral";
                interval -> "interval",
                period >> "period",
                precision >> "precision",
                convert_to - "convert_to",
                convert_precision ? "convert_precision"
            ],

            // ------------------------------------------------------------------
            // Join
            // ------------------------------------------------------------------
            AstNode::JoinClause {
                join_type,
                table_expr,
                constraint,
            } => py_dict![
                py;
                "JoinClause";
                join_type -> "join_type", table_expr -> "table_expr", constraint -> "constraint"
            ],
            AstNode::InnerJoin => py_dict![py; "InnerJoin"],
            AstNode::LeftOuterJoin => py_dict![py; "LeftOuterJoin"],
            AstNode::RightOuterJoin => py_dict![py; "RightOuterJoin"],
            AstNode::FullOuterJoin => py_dict![py; "FullOuterJoin"],
            AstNode::JoinConstraintOn { expr } => {
                py_dict![ py; "JoinConstraintOn"; expr -> "expr" ]
            }
            AstNode::JoinConstraintUsing { columns } => py_dict![
                py;
                "JoinConstraintUsing";
                columns => "columns"
            ],

            // ------------------------------------------------------------------
            // Tables
            // ------------------------------------------------------------------
            AstNode::NamedTableExpression { name, alias } => {
                py_dict![py; "NamedTableExpression"; name -> "name", alias ? "alias"]
            }

            // ------------------------------------------------------------------
            // Expressions
            // ------------------------------------------------------------------
            AstNode::SignedExpression { sign, expr } => py_dict![
                py;
                "SignedExpression";
                sign >> "sign", expr -> "expr"
            ],
            AstNode::Expression { left, op, right } => {
                py_dict![py; "Expression"; left -> "left", op >> "op", right -> "right"]
            }
            AstNode::CaseExpression {
                expr,
                when_expr,
                else_expr,
            } => py_dict![
                py;
                "CaseExpression";
                expr ? "expr", when_expr => "when_expr", else_expr ? "else_expr"
            ],
            AstNode::IsNullExpression { expr, is_null } => py_dict![
                py;
                "IsNullExpression";
                expr -> "expr", is_null >> "is_null"
            ],
            AstNode::WhenClause { guard, body } => py_dict![
                py;
                "WhenClause";
                guard -> "guard", body -> "body"
            ],

            // ------------------------------------------------------------------
            // Data types
            // ------------------------------------------------------------------
            AstNode::BooleanType => py_dict![py; "BooleanType"],
            AstNode::CharType { n } => py_dict![py; "CharType"; n -> "n"],
            AstNode::DateType => py_dict![py; "DateType"],
            AstNode::DecimalType { p, s } => py_dict![py; "DecimalType"; p -> "p", s -> "s"],
            AstNode::DoubleType => py_dict![py; "DoubleType"],
            AstNode::TimestampType => py_dict![py; "TimestampType"],
            AstNode::LocalTimestampType => py_dict![py; "LocalTimestampType"],
            AstNode::VarcharType { n } => py_dict![py; "VarcharType"; n -> "n"],

            // ------------------------------------------------------------------
            // Function expressions
            // ------------------------------------------------------------------
            AstNode::CoalesceFunction { exprs } => {
                py_dict![py; "CoalesceFunction"; exprs => "exprs"]
            }
            AstNode::CastFunction { expr, data_type } => {
                py_dict![py; "CastFunction"; expr -> "expr", data_type -> "data_type"]
            }
            AstNode::RightFunction { string, length } => {
                py_dict![py; "RightFunction"; string -> "string", length -> "length"]
            }
            AstNode::ReplaceFunction {
                string,
                search_string,
                replace_string,
            } => py_dict![
                py;
                "ReplaceFunction";
                string -> "string",
                search_string -> "search_string",
                replace_string ? "replace_string"
            ],
            AstNode::SubstringFunction {
                string,
                position,
                length,
            } => py_dict![
                py;
                "SubstringFunction";
                string -> "string", position -> "position", length ? "length"
            ],
            AstNode::ConcatFunction { exprs } => py_dict![py; "ConcatFunction"; exprs => "exprs"],
            AstNode::MaxFunction { mode, expr } => py_dict![
                py;
                "MaxFunction";
                mode -> "mode", expr -> "expr"
            ],
            AstNode::MinFunction { mode, expr } => py_dict![
                py;
                "MinFunction";
                mode -> "mode", expr -> "expr"
            ],
            AstNode::SumFunction { mode, expr } => py_dict![
                py;
                "SumFunction";
                mode -> "mode", expr -> "expr"
            ],
            AstNode::ToDateFunction { string, format } => py_dict![
                py;
                "ToDateFunction";
                string -> "string", format ? "format"
            ],
            AstNode::PowerFunction { base, exponent } => py_dict![
                py;
                "PowerFunction";
                base -> "base", exponent -> "exponent"
            ],
            AstNode::CountFunction { mode, columns } => py_dict![
                py;
                "CountFunction";
                mode -> "mode", columns => "columns"
            ],
            AstNode::DateTruncFunction { format, datetime } => py_dict![
                py;
                "DateTruncFunction";
                format -> "format", datetime -> "datetime"
            ],
            AstNode::MonthsBetweenFunction {
                datetime1,
                datetime2,
            } => py_dict![
                py;
                "MonthsBetweenFunction";
                datetime1 -> "datetime1", datetime2 -> "datetime2"
            ],
            AstNode::UnknownFunction { name, exprs } => py_dict![
                py;
                "UnknownFunction";
                name -> "name", exprs => "exprs"
            ],
        }
        .to_object(py)
    }
}
