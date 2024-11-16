import os
import sys

paths = (".", "./ext")
for path in paths:
    sys.path.append(os.path.abspath(path))

extensions = [
        "mydetails",
]

project = "Pendulum"
author = "Naoki Hori"
copyright = f"2023, {author}"

from mathjax_params import mathjax_path
from mathjax_params import mathjax3_config

html_theme = "alabaster"
html_theme_options = {
        "description": "N-body pendulum simulator",
        "github_banner": "false",
        "github_button": "true",
        "github_count": "true",
        "github_repo": project,
        "github_type": "star",
        "github_user": "NaokiHori",
        "gray_1": "#bbb",
        "gray_2": "#111",
        "gray_3": "#555",
        "pink_1": "#033",
        "pink_2": "#055",
        "pink_3": "#2ad3d3",
        "base_bg": "#000",
        "base_text": "#fff",
        "hr_border": "#4e4b49",
        "body_text": "#c1bcb6",
        "footer_text": "#777",
        "link": "#ffb494",
        "link_hover": "#92beff",
        "sidebar_text": "#aaa",
        "sidebar_link_underscore": "#666",
        "sidebar_search_button": "#333",
        "sidebar_list": "#fff",
        "anchor": "#222",
        "anchor_hover_bg": "#151515",
        "table_border": "#777",
        "admonition_border": "#333",
        "note_border": "#333",
        "seealso_border": "#333",
        "tip_border": "#333",
        "hint_border": "#333",
        "important_border": "#333",
        "highlight_bg": "#050c17",
        "xref_border": "#000",
        "xref_bg": "#040404",
        "admonition_xref_border": "#050505",
        "footnote_bg": "#020202",
        "narrow_sidebar_bg": "#ccc",
        "narrow_sidebar_fg": "#000",
        "viewcode_target_bg": "#002",
        "code_bg": "#130f0c",
        "code_text": "#ddd",
        "code_hover": "#111",
        "code_highlight": "#003",
}

html_static_path = ["_static"]
