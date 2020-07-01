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
            name_node = node["name"]
            name_type = name_node["_type"]

            if name_type == "Identifier":
                yield name_node["s"]
            elif name_type == "QualifiedIdentifier":
                yield from name_node["s"]
            else:
                raise NotImplementedError(name_node)


def iter_ctes(sql_ast: Dict) -> Iterator[str]:
    for node in iter_node_attributes(sql_ast):
        if node["_type"] == "WithClause":
            yield node["identifier"]["s"]
