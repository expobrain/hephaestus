from typing import Dict, Iterator


def iter_node_attributes(root_node: object) -> Iterator[object]:
    nodes = [root_node]

    for node in nodes:
        if isinstance(node, dict):
            yield node

        if isinstance(node, list):
            nodes.extend(node)
        elif isinstance(node, dict):
            nodes.extend(v for k, v in node.items() if not k.startswith("_"))


def iter_tables(sql_ast: Dict) -> Iterator[str]:
    for node in iter_node_attributes(sql_ast):
        if node["_type"] == "NamedTableExpression":
            for name in node["name"]:
                yield name


def iter_ctes(sql_ast: Dict) -> Iterator[str]:
    for node in iter_node_attributes(sql_ast):
        if node["_type"] == "CommonTableExpression":
            yield node["identifier"]
