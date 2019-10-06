from hephaestus import parse


class Visitor:
    def __init__(self, sql: str):
        self.sql = sql

    def walk(self):
        root_node = parse(self.sql)
        nodes = [(None, root_node)]

        for (attr, value) in nodes:
            if isinstance(value, dict):
                # Call visitor function for this node
                node_type = value["_type"]
                fn_name = f"visit_{node_type}"
                visitor_fn = getattr(self, fn_name, None)

                if visitor_fn:
                    visitor_fn(attr, value)

                # Push children nodes back to the visiting buffer
                nodes.extend(
                    (k, v)
                    for k, v in value.items()
                    if not k.startswith("_") and isinstance(v, (dict, list))
                )

            elif isinstance(value, list):
                # Iterate over nodes in list keeping the item's index
                nodes.extend((attr, v) for v in value if isinstance(v, dict))
