import os
import sys

project = 'Pendulum'
copyright = '2023, Naoki Hori'
author = 'Naoki Hori'

sys.path.append(os.path.abspath("./ext"))
extensions = [
        "details",
        "sphinx.ext.mathjax",
]

html_theme = 'alabaster'
html_theme_options = {
    "fixed_sidebar": "true",
    "github_user": "NaokiHori",
    "github_repo": "Pendulum",
    "github_type": "true",
}

