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

use dict_derive::IntoPyObject;
use pyo3::{IntoPy, PyObject, Python};

use super::ast::*;

impl IntoPy<PyObject> for SqlStatement {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            SqlStatement::Statement(v) => IntoPy::<PyObject>::into_py(v, py),
            SqlStatement::ExplainQueryPlan(v) => IntoPy::<PyObject>::into_py(v, py),
            SqlStatement::Attach(v) => IntoPy::<PyObject>::into_py(v, py),
            SqlStatement::Describe(v) => IntoPy::<PyObject>::into_py(v, py),
        }
    }
}

impl IntoPy<PyObject> for Statement {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Statement::Select(v) => IntoPy::<PyObject>::into_py(v, py),
            Statement::Insert(v) => IntoPy::<PyObject>::into_py(v, py),
            Statement::Delete(v) => IntoPy::<PyObject>::into_py(v, py),
            Statement::Update(v) => IntoPy::<PyObject>::into_py(v, py),
        }
    }
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
struct SelectAll {}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
struct SelectDistinct {}

impl IntoPy<PyObject> for SelectMode {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            SelectMode::All => IntoPy::<PyObject>::into_py(SelectAll {}, py),
            SelectMode::Distinct => IntoPy::<PyObject>::into_py(SelectDistinct {}, py),
        }
    }
}

impl IntoPy<PyObject> for SetExpression {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            SetExpression::Values(v) => IntoPy::<PyObject>::into_py(v, py),
            SetExpression::Query(v) => IntoPy::<PyObject>::into_py(v, py),
            SetExpression::Op(v) => IntoPy::<PyObject>::into_py(v, py),
        }
    }
}

impl IntoPy<PyObject> for TableExpression {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            TableExpression::Named(v) => IntoPy::<PyObject>::into_py(v, py),
            TableExpression::Select(v) => IntoPy::<PyObject>::into_py(v, py),
            TableExpression::Join(v) => IntoPy::<PyObject>::into_py(v, py),
        }
    }
}

impl IntoPy<PyObject> for JoinConstraint {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            JoinConstraint::Expr(v) => IntoPy::<PyObject>::into_py(v, py),
            JoinConstraint::Columns(v) => IntoPy::<PyObject>::into_py(v, py),
        }
    }
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
struct RegularJoin {
    pub join: JoinType,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
struct NaturalJoin {
    pub join: JoinType,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
struct CrossJoin {}

impl IntoPy<PyObject> for JoinOperator {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            JoinOperator::Join(join) => IntoPy::<PyObject>::into_py(RegularJoin { join }, py),
            JoinOperator::Natural(join) => IntoPy::<PyObject>::into_py(NaturalJoin { join }, py),
            JoinOperator::Cross => IntoPy::<PyObject>::into_py(CrossJoin {}, py),
        }
    }
}

impl IntoPy<PyObject> for JoinType {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            JoinType::Inner => IntoPy::<PyObject>::into_py("inner", py),
            JoinType::Left => IntoPy::<PyObject>::into_py("left", py),
            JoinType::Right => IntoPy::<PyObject>::into_py("right", py),
            JoinType::Full => IntoPy::<PyObject>::into_py("full", py),
        }
    }
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
struct ListResultColumn {
    pub values: Vec<ResultColumn>,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
struct AllResultColumn {}

impl IntoPy<PyObject> for ResultColumns {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            ResultColumns::All => IntoPy::<PyObject>::into_py(AllResultColumn {}, py),
            ResultColumns::List(values) => {
                IntoPy::<PyObject>::into_py(ListResultColumn { values }, py)
            }
        }
    }
}

impl IntoPy<PyObject> for ResultColumn {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            ResultColumn::AllFrom(v) => IntoPy::<PyObject>::into_py(v, py),
            ResultColumn::Expr(v) => IntoPy::<PyObject>::into_py(v, py),
        }
    }
}

impl IntoPy<PyObject> for SetOperator {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            SetOperator::Intersect => IntoPy::<PyObject>::into_py("intersect", py),
            SetOperator::Except => IntoPy::<PyObject>::into_py("except", py),
            SetOperator::Union => IntoPy::<PyObject>::into_py("union", py),
            SetOperator::UnionAll => IntoPy::<PyObject>::into_py("union_all", py),
        }
    }
}

impl IntoPy<PyObject> for UnaryOperator {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            UnaryOperator::Negate => IntoPy::<PyObject>::into_py("negate", py),
            UnaryOperator::Not => IntoPy::<PyObject>::into_py("not", py),
            UnaryOperator::IsNull => IntoPy::<PyObject>::into_py("isnull", py),
        }
    }
}

impl IntoPy<PyObject> for BinaryOperator {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            BinaryOperator::Multiply => IntoPy::<PyObject>::into_py("multiply", py),
            BinaryOperator::Divide => IntoPy::<PyObject>::into_py("divide", py),
            BinaryOperator::Add => IntoPy::<PyObject>::into_py("add", py),
            BinaryOperator::Subtract => IntoPy::<PyObject>::into_py("subtract", py),
            BinaryOperator::Concat => IntoPy::<PyObject>::into_py("concat", py),
            BinaryOperator::And => IntoPy::<PyObject>::into_py("and", py),
            BinaryOperator::Or => IntoPy::<PyObject>::into_py("or", py),
        }
    }
}

impl IntoPy<PyObject> for ComparisonOperator {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            ComparisonOperator::Equal => IntoPy::<PyObject>::into_py("equal", py),
            ComparisonOperator::NotEqual => IntoPy::<PyObject>::into_py("not_equal", py),
            ComparisonOperator::LessThan => IntoPy::<PyObject>::into_py("less_than", py),
            ComparisonOperator::LessEqual => IntoPy::<PyObject>::into_py("less_equal", py),
            ComparisonOperator::GreaterThan => IntoPy::<PyObject>::into_py("greater_than", py),
            ComparisonOperator::GreaterEqual => IntoPy::<PyObject>::into_py("greater_equal", py),
            ComparisonOperator::Like => IntoPy::<PyObject>::into_py("like", py),
        }
    }
}

impl IntoPy<PyObject> for Expression {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Expression::Literal(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::QualifiedIdentifier(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::MakeTuple(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Select(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Unary(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Binary(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Comparison(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::In(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Between(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Case(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Coalesce(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Replace(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Substring(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::ToDate(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Power(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Concat(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Max(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Min(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Sum(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Cast(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Right(v) => IntoPy::<PyObject>::into_py(v, py),
            Expression::Count(v) => IntoPy::<PyObject>::into_py(v, py),

            Expression::Unknown(v) => IntoPy::<PyObject>::into_py(v, py),
        }
    }
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct ListSetSpecification {
    pub exprs: Vec<Expression>,
}

impl IntoPy<PyObject> for SetSpecification {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            SetSpecification::Select(v) => IntoPy::<PyObject>::into_py(v, py),
            SetSpecification::List(exprs) => {
                IntoPy::<PyObject>::into_py(ListSetSpecification { exprs }, py)
            }
            SetSpecification::Name(v) => IntoPy::<PyObject>::into_py(v, py),
        }
    }
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct StringLiteral {
    pub str: String,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct NumericLiteral {
    pub num: String,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct NullLiteral {}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct CurrentTimeLiteral {}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct CurrentDateLiteral {}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct CurrentTimestampLiteral {}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct TimeLiteral {
    time: String,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct DateLiteral {
    date: String,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct TimestampLiteral {
    ts: String,
}

impl IntoPy<PyObject> for Literal {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            Literal::String(str) => IntoPy::<PyObject>::into_py(StringLiteral { str }, py),
            Literal::Numeric(num) => IntoPy::<PyObject>::into_py(NumericLiteral { num }, py),
            Literal::Null => IntoPy::<PyObject>::into_py(NullLiteral {}, py),
            Literal::CurrentTime => IntoPy::<PyObject>::into_py(CurrentTimeLiteral {}, py),
            Literal::CurrentDate => IntoPy::<PyObject>::into_py(CurrentDateLiteral {}, py),
            Literal::CurrentTimestamp => {
                IntoPy::<PyObject>::into_py(CurrentTimestampLiteral {}, py)
            }
            Literal::Time(time) => IntoPy::<PyObject>::into_py(TimeLiteral { time }, py),
            Literal::Date(date) => IntoPy::<PyObject>::into_py(DateLiteral { date }, py),
            Literal::Timestamp(ts) => IntoPy::<PyObject>::into_py(TimestampLiteral { ts }, py),
        }
    }
}

impl IntoPy<PyObject> for OrderingDirection {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            OrderingDirection::Ascending => IntoPy::<PyObject>::into_py("Ascending", py),
            OrderingDirection::Descending => IntoPy::<PyObject>::into_py("Descending", py),
        }
    }
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct BooleanDataType {}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct CharDataType {
    pub s: Literal,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct DecimalDataType {
    pub p: Literal,
    pub s: Literal,
}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct DateDataType {}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct DoublePrecisionDataType {}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct TimestampDataType {}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct LocalTimestampDataType {}

#[derive(IntoPyObject, Debug, PartialEq, Eq, Clone)]
pub struct VarcharDataType {
    pub s: Literal,
}

impl IntoPy<PyObject> for DataType {
    fn into_py(self, py: Python) -> PyObject {
        match self {
            DataType::Boolean => IntoPy::<PyObject>::into_py(BooleanDataType {}, py),
            DataType::Char(s) => IntoPy::<PyObject>::into_py(CharDataType { s }, py),
            DataType::Date => IntoPy::<PyObject>::into_py(DateDataType {}, py),
            DataType::Decimal { p, s } => IntoPy::<PyObject>::into_py(DecimalDataType { p, s }, py),
            DataType::DoublePrecision => {
                IntoPy::<PyObject>::into_py(DoublePrecisionDataType {}, py)
            }
            DataType::Timestamp => IntoPy::<PyObject>::into_py(TimestampDataType {}, py),
            DataType::LocalTimestamp => IntoPy::<PyObject>::into_py(LocalTimestampDataType {}, py),
            DataType::Varchar(s) => IntoPy::<PyObject>::into_py(VarcharDataType { s }, py),
        }
    }
}
