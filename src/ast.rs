#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Sign {
    Positive,
    Negative,
}

impl Sign {
    pub fn from_str(s: &str) -> Self {
        match s {
            "+" => Self::Positive,
            "-" => Self::Negative,
            _ => unreachable!(format!("Sign: symbol {} not supported", s)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SelectMode {
    All,
    Distinct,
}

impl SelectMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_ref() {
            "DISTINCT" => Self::Distinct,
            "ALL" => Self::All,
            _ => unreachable!(format!("SelectMode: symbol {} not supported", s)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Interval {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
}

impl Interval {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_ref() {
            "YEAR" => Self::Year,
            "MONTH" => Self::Month,
            "DAY" => Self::Day,
            "HOUR" => Self::Hour,
            "MINUTE" => Self::Minute,
            "SECOND" => Self::Second,
            _ => unreachable!(format!("Interval: symbol {} not supported", s)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Union {
    UnionAll,
    Intersect,
    Minus,
    Except,
}

impl Union {
    pub fn from_str(s: &str) -> Self {
        match s
            .to_uppercase()
            .split(' ')
            .filter(|v| !v.is_empty())
            .collect::<Vec<_>>()[..]
        {
            ["UNION", "ALL"] | ["UNION"] => Self::UnionAll,
            ["INTERSECT"] => Self::Intersect,
            ["MINUS"] => Self::Minus,
            ["EXCEPT"] => Self::Except,
            _ => unreachable!(format!("Union: symbol '{}' not supported", s)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Operation {
    /// Numeric multiplication
    Multiply,

    /// Numeric division
    Divide,

    /// Numeric addition
    Add,

    /// Numeric subtraction
    Subtract,

    /// Concatenation of character sequences
    Concat,

    /// Logical and
    And,

    /// Logical or
    Or,

    /// Equality
    Equal,

    /// Inequality
    NotEqual,

    /// Greater than
    GreaterThan,

    // Grater or equal than
    GreaterOrEqualThan,

    /// Less than
    LessThan,

    /// Less or equal than
    LessOrEqualThan,
}

impl Operation {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "*" => Self::Multiply,
            "/" => Self::Divide,
            "+" => Self::Add,
            "-" => Self::Subtract,
            "||" => Self::Concat,
            "AND" => Self::And,
            "OR" => Self::Or,
            "=" => Self::Equal,
            "!=" => Self::NotEqual,
            ">" => Self::GreaterThan,
            ">=" => Self::GreaterOrEqualThan,
            "<" => Self::LessThan,
            "<=" => Self::LessOrEqualThan,
            _ => unreachable!(format!("Operation: symbol {} not supported", s)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AstNode {
    // ------------------------------------------------------------------
    // Empty
    // ------------------------------------------------------------------
    Empty,

    // ------------------------------------------------------------------
    // Comment
    // ------------------------------------------------------------------
    Comment {
        s: String,
    },

    // ------------------------------------------------------------------
    // Select statement
    // ------------------------------------------------------------------
    SelectStatement {
        common: Vec<AstNode>,
        mode: SelectMode,
        columns: Vec<AstNode>,
        table_exprs: Vec<AstNode>,
        where_expr: Option<Box<AstNode>>,
        group_by: Option<Box<AstNode>>,
    },
    SelectUnionStatement {
        left: Box<AstNode>,
        op: Union,
        right: Box<AstNode>,
    },
    SelectMode {
        mode: SelectMode,
    },
    WithClause {
        identifier: Box<AstNode>,
        columns: Vec<AstNode>,
        query: Box<AstNode>,
    },
    GroupBy {
        groupings: Vec<AstNode>,
        having: Option<Box<AstNode>>,
    },

    // ------------------------------------------------------------------
    // Columns
    // ------------------------------------------------------------------
    Column {
        expr: Box<AstNode>,
    },
    AllColumns,
    AllColumnsFrom {
        schema: Box<AstNode>,
    },
    NamedColumn {
        expr: Box<AstNode>,
        alias: String,
    },

    // ------------------------------------------------------------------
    // Identifier
    // ------------------------------------------------------------------
    Identifier {
        s: String,
    },
    QualifiedIdentifier {
        s: Vec<AstNode>,
    },

    // ------------------------------------------------------------------
    // Literals
    // ------------------------------------------------------------------
    IntegerLiteral {
        s: String,
    },
    DecimalLiteral {
        s: String,
    },
    FloatLiteral {
        s: String,
    },
    StringLiteral {
        s: String,
    },
    BooleanLiteral {
        s: String,
    },
    IntervalLiteral {
        interval: Box<AstNode>,
        period: Interval,
        precision: Vec<AstNode>,
        convert_to: Option<Interval>,
        convert_precision: Option<Box<AstNode>>,
    },

    // ------------------------------------------------------------------
    // Join
    // ------------------------------------------------------------------
    JoinClause {
        join_type: Box<AstNode>,
        table_expr: Box<AstNode>,
        constraint: Box<AstNode>,
    },
    InnerJoin,
    LeftOuterJoin,
    RightOuterJoin,
    FullOuterJoin,
    JoinConstraintOn {
        expr: Box<AstNode>,
    },
    JoinConstraintUsing {
        columns: Vec<AstNode>,
    },

    // ------------------------------------------------------------------
    // Tables
    // ------------------------------------------------------------------
    NamedTableExpression {
        name: Box<AstNode>,
        alias: Option<String>,
    },

    // ------------------------------------------------------------------
    // Expressions
    // ------------------------------------------------------------------
    SignedExpression {
        sign: Sign,
        expr: Box<AstNode>,
    },
    Expression {
        left: Box<AstNode>,
        op: Operation,
        right: Box<AstNode>,
    },
    CaseExpression {
        expr: Option<Box<AstNode>>,
        when_expr: Vec<AstNode>,
        else_expr: Option<Box<AstNode>>,
    },
    IsNullExpression {
        expr: Box<AstNode>,
        is_null: bool,
    },
    InExpression {
        expr: Box<AstNode>,
        exprs: Vec<AstNode>,
        not_in: bool,
    },
    WhenClause {
        guard: Box<AstNode>,
        body: Box<AstNode>,
    },

    // ------------------------------------------------------------------
    // Data types
    // ------------------------------------------------------------------
    /// boolean data type
    BooleanType,

    /// char
    CharType {
        n: Box<AstNode>,
    },

    /// date
    DateType,

    /// decimal
    DecimalType {
        p: Box<AstNode>,
        s: Box<AstNode>,
    },

    /// double precision
    DoubleType,

    /// timestamp
    TimestampType,

    /// local timestamp
    LocalTimestampType,

    /// varchar
    VarcharType {
        n: Box<AstNode>,
    },

    // ------------------------------------------------------------------
    // Function expressions
    // ------------------------------------------------------------------
    CoalesceFunction {
        exprs: Vec<AstNode>,
    },
    CastFunction {
        expr: Box<AstNode>,
        data_type: Box<AstNode>,
    },
    RightFunction {
        string: Box<AstNode>,
        length: Box<AstNode>,
    },
    ReplaceFunction {
        string: Box<AstNode>,
        search_string: Box<AstNode>,
        replace_string: Option<Box<AstNode>>,
    },
    SubstringFunction {
        string: Box<AstNode>,
        position: Box<AstNode>,
        length: Option<Box<AstNode>>,
    },
    MaxFunction {
        mode: Box<AstNode>,
        expr: Box<AstNode>,
    },
    MinFunction {
        mode: Box<AstNode>,
        expr: Box<AstNode>,
    },
    SumFunction {
        mode: Box<AstNode>,
        expr: Box<AstNode>,
    },
    ToDateFunction {
        string: Box<AstNode>,
        format: Option<Box<AstNode>>,
    },
    PowerFunction {
        base: Box<AstNode>,
        exponent: Box<AstNode>,
    },
    ConcatFunction {
        exprs: Vec<AstNode>,
    },
    CountFunction {
        mode: Box<AstNode>,
        columns: Vec<AstNode>,
    },
    DateTruncFunction {
        format: Box<AstNode>,
        datetime: Box<AstNode>,
    },
    MonthsBetweenFunction {
        datetime1: Box<AstNode>,
        datetime2: Box<AstNode>,
    },
    UnknownFunction {
        name: Box<AstNode>,
        exprs: Vec<AstNode>,
    },
}
