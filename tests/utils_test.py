from hephaestus import parse
from hephaestus.utils import iter_ctes, iter_node_attributes, iter_tables
import pytest


def test_iter_node_attributes():
    result = list(iter_node_attributes(parse("SELECT 1")))
    result_types = [n["_type"] for n in result]

    expected = [
        "SelectStatement",
        "QuerySetExpression",
        "SelectAll",
        "ListResultColumn",
        "ExprResultColumn",
        "NumericLiteral",
    ]

    assert result_types == expected


@pytest.mark.parametrize(
    "sql, expected",
    [
        ["SELECT 1", []],
        ["SELECT 1 FROM dual", ["dual"]],
        ["SELECT 1 FROM dual d", ["dual"]],
        ["SELECT * FROM a JOIN b USING (c)", ["a", "b"]],
        ["WITH a AS (SELECT 1) SELECT * FROM a", ["a"]],
    ],
)
def test_iter_tables(sql, expected):
    sql_ast = parse(sql)
    results = list(iter_tables(sql_ast))

    assert results == expected


@pytest.mark.parametrize(
    "sql, expected",
    [
        ["SELECT 1", []],
        ["WITH a AS (SELECT 1) SELECT * FROM a", ["a"]],
        ["WITH a AS (SELECT 1) SELECT * FROM a JOIN B USING (c)", ["a"]],
    ],
)
def test_iter_ctes(sql, expected):
    sql_ast = parse(sql)
    results = list(iter_ctes(sql_ast))

    assert results == expected
