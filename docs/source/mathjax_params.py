mathjax_path = "https://cdn.jsdelivr.net/npm/mathjax@2/MathJax.js?config=TeX-AMS-MML_HTMLorMML"

macros = dict()

# derivatives
macros["tder"] = ["{\\frac{d         #1}{d         #2}}", 2]
macros["pder"] = ["{\\frac{\\partial #1}{\\partial #2}}", 2]
macros["dder"] = ["{\\frac{\\delta   #1}{\\delta   #2}}", 2]
# discrete differentiation and average in time
macros["dif"] = ["{\\delta{#1}}", 1]
macros["ave"] = ["{\\overline{#1}}", 1]
# indices
macros["ia"] = "{i}"
macros["ib"] = "{j}"
macros["ic"] = "{k}"
# generalised coordinates
macros["pos"] = "{\\theta}"
macros["vel"] = "{\\omega}"
# trigonometric matrices
macros["cmat"] = ["{\\cos \\left( \\pos_{#1} - \\pos_{#2} \\right)}", 2]
macros["smat"] = ["{\\sin \\left( \\pos_{#1} - \\pos_{#2} \\right)}", 2]

# other relations which appear for several times
macros["posvec"] = (
        "\\vec{e}_x \\sum_{\\ib = 0}^{\\ia} l \\cos \\pos_{\\ib}"
        "+"
        "\\vec{e}_y \\sum_{\\ib = 0}^{\\ia} l \\sin \\pos_{\\ib}"
        )
macros["kene"] = (
        "\\frac{1}{2} m l^2"
        "\\sum_{\\ib = 0}^{N - 1}"
        "\\sum_{\\ic = 0}^{N - 1}"
        "\\left\\{ N - \\max \\left( \\ib, \\ic \\right) \\right\\}"
        "\\vel_{\\ib}"
        "\\vel_{\\ic}"
        "\\cos \\left( \\pos_{\\ib} - \\pos_{\\ic} \\right)"
        )
macros["pene"] = (
        "-"
        "m g l"
        "\\sum_{\\ib = 0}^{N-1}"
        "\\left( N - \\ib \\right)"
        "\\sin{\\pos_{\\ib}}"
        )
macros["lag"] = (
        "m l^2"
        "\\sum_{\\ib = 0}^{N - 1}"
        "\\left\\{ N - \\max \\left( \\ia, \\ib \\right) \\right\\}"
        "\\tder{\\vel_{\\ib}}{t}"
        "\\cos \\left( \\pos_{\\ia} - \\pos_{\\ib} \\right)"
        "+"
        "m l^2"
        "\\sum_{\\ib = 0}^{N - 1}"
        "\\left\\{ N - \\max \\left( \\ia, \\ib \\right) \\right\\}"
        "\\vel_{\\ib}"
        "\\vel_{\\ib}"
        "\\sin \\left( \\pos_{\\ia} - \\pos_{\\ib} \\right)"
        "-"
        "m g l"
        "\\left( N - \\ia \\right)"
        "\\cos \\pos_{\\ia}"
        )

macros["dlag"] = (
        "\\sum_{\\ib = 0}^{N - 1}"
        "\\left\\{"
        "  N"
        "  -"
        "  \\max \\left( \\ia, \\ib \\right)"
        "\\right\\}"
        "\\dder{\\vel_{\\ib}}{t}"
        "\\ave{\\cmat{\\ia}{\\ib}}"
        "\\left("
        "  \\frac{1}{2}"
        "  +"
        "  \\frac{1}{2}"
        "  \\frac{"
        "     \\ave{"
        "        \\vel_{\\ia}"
        "        \\cmat{\\ia}{\\ib}"
        "     }"
        "  }{"
        "     \\ave{\\vel_{\\ia}}"
        "     \\,"
        "     \\ave{\\cmat{\\ia}{\\ib}}"
        "  }"
        "\\right)"
        "+"
        "\\sum_{\\ib = 0}^{N - 1}"
        "\\left\\{"
        "  N"
        "  -"
        "  \\max \\left( \\ia, \\ib \\right)"
        "\\right\\}"
        "\\ave{\\vel_{\\ib}}"
        "\\,"
        "\\ave{\\vel_{\\ib}}"
        "\\text{sinc} \\left("
        "  \\frac{"
        "     \\dif{\\pos_{\\ia}}"
        "  }{2}"
        "  -"
        "  \\frac{"
        "     \\dif{\\pos_{\\ib}}"
        "  }{2}"
        "\\right)"
        "\\sin \\left( \\ave{\\pos_{\\ia}} - \\ave{\\pos_{\\ib}} \\right)"
        "-"
        "\\left( N - \\ia \\right)"
        "\\text{sinc} \\frac{\\dif{\\pos_{\\ia}}}{2}"
        "\\cos \\ave{\\pos_{\\ia}}"
        "="
        "0_{\\ia}"
        )

mathjax3_config = {"TeX": {"Macros": macros}}
