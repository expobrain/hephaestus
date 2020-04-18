from hephaestus.walk import Visitor


def test_traverse_node():
    class MyVisitor(Visitor):
        visited = False

        def visit_SelectStatement(self, attr, node):
            self.visited = True

    visitor = MyVisitor("SELECT 1")
    visitor.walk()

    assert visitor.visited is True


def test_traverse_node_in_list():
    class MyVisitor(Visitor):
        has_dual_table = False

        def visit_NamedTableExpression(self, attr, node):
            self.has_dual_table = node["name"]["s"] == "dual"

    visitor = MyVisitor("SELECT 1 FROM dual")
    visitor.walk()

    assert visitor.has_dual_table is True
