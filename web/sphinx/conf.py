import os
import sys

paths = (".", "./ext", )
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

html_theme = "mysphinxtheme"
html_theme_path = ["."]
html_theme_options = {
        "description": "N-body pendulum simulator",
        "github_repo": f"https://github.com/NaokiHori/{project}",
}

html_static_path = ["_static"]
