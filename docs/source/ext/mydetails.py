from docutils import nodes
from docutils.parsers.rst import Directive, directives
from sphinx.transforms.post_transforms import SphinxPostTransform
from sphinx.util.nodes import NodeMatcher


class details(nodes.Element, nodes.General):
    pass

class summary(nodes.TextElement, nodes.General):
    pass

def visit_details(self, node):
    self.body.append("<details>")

def depart_details(self, node):
    self.body.append("</details>")

def visit_summary(self, node):
    self.body.append("<summary>")

def depart_summary(self, node):
    self.body.append("</summary>")

class MyDetails(Directive):
    required_arguments = 0
    optional_arguments = 1
    final_argument_whitespace = True
    has_content = True
    option_spec = {
        "class": directives.class_option,
        "name": directives.unchanged,
    }
    def run(self):
        args = self.arguments
        summary = args[0] if 0 < len(args) else "Details"
        result = nodes.container(
                "",
                classes=self.options.get("classes", []),
                type="mydetails"
        )
        textnodes, messages = self.state.inline_text(
                summary,
                self.lineno
        )
        result += nodes.paragraph(
                summary,
                "",
                *textnodes
        )
        result += messages
        self.state.nested_parse(
                self.content,
                self.content_offset,
                result
        )
        self.add_name(result)
        return [result]

class MyDetailsTransform(SphinxPostTransform):
    default_priority = 200
    def run(self):
        matcher = NodeMatcher(nodes.container, type="mydetails")
        for node in self.document.traverse(matcher):
            newnode = details(**node.attributes)
            newnode += summary("", "", *node[0])
            newnode.extend(node[1:])
            node.replace_self(newnode)

def setup(app):
    app.add_node(details, html=(visit_details, depart_details))
    app.add_node(summary, html=(visit_summary, depart_summary))
    app.add_directive("mydetails", MyDetails)
    app.add_post_transform(MyDetailsTransform)
    return {
        "version": "0.1",
        "parallel_read_safe": True,
        "parallel_write_safe": True,
    }

