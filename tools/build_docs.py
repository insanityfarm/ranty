#!/usr/bin/env python3

from __future__ import annotations

import html
import os
import re
import shutil
from pathlib import Path


OUT_DIR = Path("docs")

TREE = [
    ("Introduction", "intro.html", []),
    ("Getting Started", "getting-started.html", []),
    (
        "Language",
        "language.html",
        [
            ("Text", "language/text.html", []),
            ("Blocks", "language/blocks.html", []),
            ("Functions", "language/functions.html", []),
            ("Data Types", "language/data-types.html", []),
            ("Accessors", "language/accessors.html", []),
            (
                "Keywords",
                "language/keywords.html",
                [
                    ("@require", "language/keywords/require.html", []),
                    ("@return", "language/keywords/return.html", []),
                    ("@continue", "language/keywords/continue.html", []),
                    ("@break", "language/keywords/break.html", []),
                    ("@weight", "language/keywords/weight.html", []),
                    ("@text", "language/keywords/text.html", []),
                ],
            ),
            ("Operators", "language/operators.html", []),
            ("Conditional Expressions", "language/conditional-expressions.html", []),
            ("Output Modifiers", "language/output-modifiers.html", []),
        ],
    ),
    (
        "Runtime Features",
        "runtime.html",
        [
            ("Attributes", "runtime/attributes.html", []),
            ("Formatters", "runtime/formatters.html", []),
            ("Modules", "modules.html", []),
            ("CLI And REPL", "cli.html", []),
        ],
    ),
    (
        "Standard Library",
        "stdlib.html",
        [
            ("General", "stdlib/general.html", []),
            ("Attributes And Control Flow", "stdlib/control-flow.html", []),
            ("Collections", "stdlib/collections.html", []),
            ("Generators", "stdlib/generators.html", []),
            ("Formatting", "stdlib/formatting.html", []),
            ("Strings", "stdlib/strings.html", []),
            ("Boolean", "stdlib/boolean.html", []),
            ("Comparison", "stdlib/comparison.html", []),
            ("Math", "stdlib/math.html", []),
            ("Conversion", "stdlib/conversion.html", []),
            ("Verification", "stdlib/verification.html", []),
            ("Assertion", "stdlib/assertion.html", []),
            ("Constants", "stdlib/constants.html", []),
        ],
    ),
    ("Diagnostics", "compiler-messages.html", []),
    ("Glossary", "glossary.html", []),
]

STDLIB_CATEGORIES = {
    "stdlib/general.html": [
        "alt",
        "call",
        "cat",
        "either",
        "len",
        "type",
        "seed",
        "tap",
        "print",
        "range",
        "require",
        "irange",
        "fork",
        "unfork",
        "try",
        "ds-request",
        "ds-query-sources",
        "proto",
        "set-proto",
    ],
    "stdlib/control-flow.html": [
        "if",
        "elseif",
        "else",
        "mksel",
        "rep",
        "sel",
        "sep",
        "mut",
        "sel-skip",
        "sel-freeze",
        "sel-frozen",
        "reset-attrs",
        "step",
        "step-index",
        "step-count",
    ],
    "stdlib/collections.html": [
        "assoc",
        "augment",
        "augment-self",
        "augment-thru",
        "chunks",
        "clear",
        "fill-self",
        "fill-thru",
        "has",
        "index-of",
        "insert",
        "keys",
        "last-index-of",
        "list",
        "nlist",
        "remove",
        "rev",
        "sift-self",
        "sift-thru",
        "sift",
        "squish-self",
        "squish-thru",
        "squish",
        "take",
        "translate",
        "values",
        "filter",
        "join",
        "map",
        "sort-self",
        "sort-thru",
        "sort",
        "shuffle-self",
        "shuffle-thru",
        "shuffle",
        "sum",
        "tuple",
        "push",
        "pop",
        "oxford-join",
        "zip",
    ],
    "stdlib/generators.html": [
        "alpha",
        "dig",
        "digh",
        "dignz",
        "maybe",
        "pick",
        "pickn",
        "pick-sparse",
        "rand",
        "randf",
        "rand-list",
        "randf-list",
        "rand-list-sum",
    ],
    "stdlib/formatting.html": [
        "ws-fmt",
        "num-fmt",
        "num-fmt-system",
        "num-fmt-alt",
        "num-fmt-padding",
        "num-fmt-precision",
        "num-fmt-upper",
        "num-fmt-endian",
        "num-fmt-sign",
        "num-fmt-infinity",
        "num-fmt-group-sep",
        "num-fmt-decimal-sep",
    ],
    "stdlib/strings.html": [
        "char",
        "lower",
        "upper",
        "seg",
        "split",
        "lines",
        "indent",
        "string-replace",
        "trim",
        "ord",
        "ord-all",
    ],
    "stdlib/boolean.html": ["and", "not", "or", "xor"],
    "stdlib/comparison.html": ["eq", "neq", "gt", "lt", "ge", "le"],
    "stdlib/math.html": [
        "abs",
        "add",
        "sub",
        "mul",
        "div",
        "mul-add",
        "mod",
        "neg",
        "pow",
        "recip",
        "clamp",
        "min",
        "max",
        "floor",
        "ceil",
        "frac",
        "asin",
        "sin",
        "acos",
        "cos",
        "atan",
        "atan2",
        "tan",
        "sqrt",
    ],
    "stdlib/conversion.html": ["to-int", "to-float", "to-string", "to-bool", "to-list", "to-tuple"],
    "stdlib/verification.html": [
        "is-string",
        "is-int",
        "is-float",
        "is-number",
        "is-bool",
        "is-nothing",
        "is-nan",
        "is-odd",
        "is-even",
        "is-factor",
        "is-between",
        "is-some",
        "is",
    ],
    "stdlib/assertion.html": ["assert", "assert-not", "assert-eq", "assert-neq"],
}

CONSTANTS = [
    ("RANT_VERSION", "The shipped language version string."),
    ("BUILD_VERSION", "The crate build version string."),
    ("EPSILON", "The smallest positive `float` greater than zero."),
    ("MIN_FLOAT", "The lowest finite `float` value."),
    ("MAX_FLOAT", "The highest finite `float` value."),
    ("MIN_INT", "The minimum `int` value."),
    ("MAX_INT", "The maximum `int` value."),
    ("INFINITY", "Positive infinity."),
    ("NEG_INFINITY", "Negative infinity."),
    ("NAN", "A NaN `float` value."),
]

MANUAL_SIGNATURES = {
    "alt": "[%alt: a; ...rest]",
    "call": "[%call: func; args?]",
    "cat": "[%cat: ...values]",
    "either": "[%either: cond; a; b]",
    "len": "[%len: value]",
    "type": "[%type: value]",
    "seed": "[%seed]",
    "tap": "[%tap: ...]",
    "print": "[%print: ...values]",
    "range": "[%range: a; b?; step?]",
    "require": "[%require: path]",
    "irange": "[%irange: a; b?; step?]",
    "fork": "[%fork: seed?]",
    "unfork": "[%unfork]",
    "try": "[%try: context; handler?]",
    "ds-request": "[%ds-request: id; ...args]",
    "ds-query-sources": "[%ds-query-sources]",
    "proto": "[%proto: map]",
    "set-proto": "[%set-proto: map; proto?]",
    "if": "[%if: cond]",
    "elseif": "[%elseif: cond]",
    "else": "[%else]",
    "mksel": "[%mksel: mode]",
    "rep": "[%rep: reps]",
    "sel": "[%sel: selector?]",
    "sep": "[%sep: separator]",
    "mut": "[%mut: mutator?]",
    "sel-skip": "[%sel-skip: selector; n?]",
    "sel-freeze": "[%sel-freeze: selector; frozen?]",
    "sel-frozen": "[%sel-frozen: selector]",
    "reset-attrs": "[%reset-attrs]",
    "step": "[%step]",
    "step-index": "[%step-index]",
    "step-count": "[%step-count]",
}

MANUAL_SUMMARIES = {
    "alt": "Prints the first argument that is not `nothing`.",
    "call": "Calls a function with a list of argument values.",
    "cat": "Prints each argument into the current scope.",
    "either": "Prints `a` when the condition is true, otherwise `b`.",
    "len": "Returns the length of a string, list, map, or range-like value.",
    "type": "Prints the runtime type name of a value.",
    "seed": "Prints the currently active RNG seed.",
    "tap": "Consumes arguments and produces no output.",
    "print": "Prints values into the caller's output scope.",
    "range": "Builds a half-open integer range.",
    "require": "Imports a module through the active module resolver.",
    "irange": "Builds an inclusive integer range.",
    "fork": "Pushes a derived RNG onto the RNG stack.",
    "unfork": "Pops the most recent derived RNG.",
    "try": "Runs a function with an optional runtime-error handler.",
    "ds-request": "Calls a registered data source by ID.",
    "ds-query-sources": "Returns the list of registered data-source IDs.",
    "proto": "Returns a map's prototype.",
    "set-proto": "Sets or clears a map's prototype.",
    "mksel": "Creates a selector value for use with `[sel]`.",
    "rep": "Configures repetition count or repetition mode for the current block.",
    "sel": "Sets the active selector for the current block, or returns it when called without arguments.",
    "sep": "Sets the separator for repeated block iterations.",
    "mut": "Assigns a mutator function for block elements.",
    "sel-skip": "Advances a selector without printing the selected value.",
    "sel-freeze": "Freezes or unfreezes a selector in place.",
    "sel-frozen": "Returns whether a selector is frozen.",
    "reset-attrs": "Clears the active block-attribute state.",
    "step": "Returns the current step number as a one-based integer.",
    "step-index": "Returns the current step index as a zero-based integer.",
    "step-count": "Returns the number of steps in the active block.",
    "assoc": "Builds a map from alternating keys and values.",
    "augment": "Returns a map containing the merged keys of two maps.",
    "augment-self": "Mutates a map by merging another map into it.",
    "augment-thru": "Merges a map and then prints the mutated handle.",
    "chunks": "Splits a list into evenly sized sublists.",
    "clear": "Removes all items from a mutable collection.",
    "fill-self": "Mutates a list by filling it with a value.",
    "fill-thru": "Fills a list and then prints the same handle.",
    "has": "Returns whether a collection contains a value or key.",
    "index-of": "Returns the first matching list index or `nothing`.",
    "insert": "Inserts into a list by index or a map by key.",
    "keys": "Returns a list of map keys.",
    "last-index-of": "Returns the last matching list index or `nothing`.",
    "list": "Builds a list from its arguments.",
    "nlist": "Builds a list containing repeated copies of a value.",
    "remove": "Removes from a list by index or a map by key.",
    "rev": "Reverses an ordered value.",
    "sift-self": "Mutates a list by removing `nothing` values.",
    "sift-thru": "Sifts a list and then prints the same handle.",
    "sift": "Returns a sifted copy of a list.",
    "squish-self": "Mutates a list by removing duplicate values.",
    "squish-thru": "Squishes a list and then prints the same handle.",
    "squish": "Returns a de-duplicated copy of a list.",
    "take": "Removes and returns a list item or map entry.",
    "translate": "Maps values through a translation map.",
    "values": "Returns a list of map values.",
    "filter": "Filters a collection through a predicate.",
    "join": "Joins a collection into text with a separator.",
    "map": "Transforms a collection through a callback.",
    "sort-self": "Sorts a list in place.",
    "sort-thru": "Sorts a list in place and prints the same handle.",
    "sort": "Returns a sorted copy of a list.",
    "shuffle-self": "Shuffles a list in place.",
    "shuffle-thru": "Shuffles a list and prints the same handle.",
    "shuffle": "Returns a shuffled copy of a list.",
    "sum": "Adds the contents of an ordered collection.",
    "tuple": "Builds a tuple from its arguments.",
    "push": "Appends a value to a list.",
    "pop": "Removes and returns the last list item.",
    "oxford-join": "Joins text with a final Oxford-comma conjunction.",
    "zip": "Zips collections together, optionally through a callback.",
    "alpha": "Generates random ASCII letters.",
    "dig": "Generates random decimal digits.",
    "digh": "Generates random hexadecimal digits.",
    "dignz": "Generates random nonzero decimal digits.",
    "maybe": "Prints a value with a probability threshold.",
    "pick": "Chooses one random value from its arguments.",
    "pickn": "Chooses multiple random values.",
    "pick-sparse": "Chooses values independently across the argument list.",
    "rand": "Generates a random integer.",
    "randf": "Generates a random float.",
    "rand-list": "Generates a list of random integers.",
    "randf-list": "Generates a list of random floats.",
    "rand-list-sum": "Generates random integers whose sum matches a target total.",
    "ws-fmt": "Gets or sets whitespace-normalization mode.",
    "num-fmt": "Gets or sets number-format options as a map.",
    "num-fmt-system": "Gets or sets the numeral system.",
    "num-fmt-alt": "Gets or sets alternate numeral formatting.",
    "num-fmt-padding": "Gets or sets minimum number padding.",
    "num-fmt-precision": "Gets or sets float precision.",
    "num-fmt-upper": "Gets or sets uppercase numeral output.",
    "num-fmt-endian": "Gets or sets endianness for supported numeral systems.",
    "num-fmt-sign": "Gets or sets sign-display behavior.",
    "num-fmt-infinity": "Gets or sets infinity-display behavior.",
    "num-fmt-group-sep": "Gets or sets the digit-group separator.",
    "num-fmt-decimal-sep": "Gets or sets the decimal separator.",
    "char": "Creates a string from a Unicode scalar value.",
    "lower": "Converts text to lowercase.",
    "upper": "Converts text to uppercase.",
    "seg": "Splits a string into grapheme clusters.",
    "split": "Splits a string on a separator.",
    "lines": "Splits text into lines.",
    "indent": "Prefixes each line with indentation text.",
    "string-replace": "Replaces matching substrings.",
    "trim": "Trims leading and trailing whitespace.",
    "ord": "Returns the code point of a character.",
    "ord-all": "Returns the code points for every character in a string.",
    "and": "Performs boolean conjunction.",
    "not": "Performs boolean negation.",
    "or": "Performs boolean disjunction.",
    "xor": "Performs boolean exclusive-or.",
    "eq": "Returns whether two values are equal.",
    "neq": "Returns whether two values are not equal.",
    "gt": "Returns whether the left value is greater than the right value.",
    "lt": "Returns whether the left value is less than the right value.",
    "ge": "Returns whether the left value is greater than or equal to the right value.",
    "le": "Returns whether the left value is less than or equal to the right value.",
    "abs": "Returns the absolute value.",
    "add": "Adds two values.",
    "sub": "Subtracts the right value from the left value.",
    "mul": "Multiplies two values.",
    "div": "Divides the left value by the right value.",
    "mul-add": "Performs a multiply-add operation.",
    "mod": "Computes the modulo remainder.",
    "neg": "Negates a numeric value.",
    "pow": "Raises a value to a power.",
    "recip": "Returns the reciprocal of a numeric value.",
    "clamp": "Clamps a value between two bounds.",
    "min": "Returns the smaller of two values.",
    "max": "Returns the larger of two values.",
    "floor": "Rounds a float downward.",
    "ceil": "Rounds a float upward.",
    "frac": "Returns the fractional part of a float.",
    "asin": "Returns the inverse sine.",
    "sin": "Returns the sine.",
    "acos": "Returns the inverse cosine.",
    "cos": "Returns the cosine.",
    "atan": "Returns the inverse tangent.",
    "atan2": "Returns the two-argument inverse tangent.",
    "tan": "Returns the tangent.",
    "sqrt": "Returns the square root.",
    "to-int": "Converts a value to `int`.",
    "to-float": "Converts a value to `float`.",
    "to-string": "Converts a value to `string`.",
    "to-bool": "Converts a value to `bool` using Rant truthiness rules.",
    "to-list": "Converts a value to `list`.",
    "to-tuple": "Converts a value to `tuple`.",
    "is-string": "Returns whether a value is a string.",
    "is-int": "Returns whether a value is an int.",
    "is-float": "Returns whether a value is a float.",
    "is-number": "Returns whether a value is numeric.",
    "is-bool": "Returns whether a value is a bool.",
    "is-nothing": "Returns whether a value is `nothing`.",
    "is-nan": "Returns whether a value is NaN.",
    "is-odd": "Returns whether an integer is odd.",
    "is-even": "Returns whether an integer is even.",
    "is-factor": "Returns whether one integer divides another evenly.",
    "is-between": "Returns whether a value falls between two bounds.",
    "is-some": "Returns whether a value is not `nothing`.",
    "is": "Returns whether a value has the named runtime type.",
    "assert": "Raises an assertion error when the condition is false.",
    "assert-not": "Raises an assertion error when the condition is true.",
    "assert-eq": "Raises an assertion error when two values differ.",
    "assert-neq": "Raises an assertion error when two values are equal.",
}

COMPILER_WARNING_VARIANTS = {
    "TemporalAssignPipeRedefinesVariable",
    "UnusedVariable",
    "UnusedParameter",
    "UnusedFunction",
    "EmptyFunctionBody",
    "NestedFunctionDefMarkedConstant",
}

RUNTIME_ERROR_ROWS = [
    ("STACK_OVERFLOW_ERROR", "Execution exceeded the maximum call or value stack size."),
    ("STACK_UNDERFLOW_ERROR", "The runtime attempted to pop an empty stack."),
    ("INVALID_ACCESS_ERROR", "A variable lookup or write was invalid in the current scope."),
    ("INVALID_OP_ERROR", "An operation was not valid for the current runtime state."),
    ("INTERNAL_ERROR", "The runtime encountered an internal engine failure."),
    ("ARG_MISMATCH_ERROR", "A function received the wrong number of arguments."),
    ("ARG_ERROR", "A function argument had an invalid value or type."),
    ("INVOKE_ERROR", "A non-function value was called as if it were a function."),
    ("ASSERT_ERROR", "An assertion failed."),
    ("TYPE_ERROR", "A value had an unexpected runtime type."),
    ("VALUE_ERROR", "A value conversion or construction failed."),
    ("INDEX_ERROR", "An index operation failed."),
    ("KEY_ERROR", "A keyed map lookup or write failed."),
    ("SLICE_ERROR", "A slice operation failed."),
    ("SELECTOR_ERROR", "A selector could not produce the next result."),
    ("MODULE_ERROR", "Module resolution, compilation, or initialization failed."),
    ("USER_ERROR", "A program raised an error explicitly through `[error]`."),
    ("CONTROL_FLOW_ERROR", "A control-flow keyword was used outside a valid target."),
    ("DATA_SOURCE_ERROR", "A data source call failed."),
]

STYLES = """
:root {
  --bg: #f4efdf;
  --panel: #f9f5ea;
  --text: #2a241a;
  --muted: #6a5d48;
  --line: #d6ccb7;
  --accent: #914a1b;
  --accent-soft: #b96b39;
  --code-bg: #ede4d1;
}

* { box-sizing: border-box; }
html, body { margin: 0; padding: 0; background: var(--bg); color: var(--text); font: 16px/1.6 Georgia, "Times New Roman", serif; }
a { color: var(--accent); text-decoration: none; }
a:hover { text-decoration: underline; }
code, pre { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
code { background: var(--code-bg); padding: 0.1rem 0.35rem; border-radius: 4px; }
pre { background: var(--code-bg); padding: 1rem; overflow-x: auto; border: 1px solid var(--line); border-radius: 8px; }
pre code { background: transparent; padding: 0; }
table { width: 100%; border-collapse: collapse; margin: 1rem 0 1.5rem; }
th, td { text-align: left; vertical-align: top; border-bottom: 1px solid var(--line); padding: 0.65rem 0.75rem; }
th { font-size: 0.92rem; text-transform: uppercase; letter-spacing: 0.03em; color: var(--muted); }
blockquote { margin: 1rem 0; padding: 0.75rem 1rem; border-left: 4px solid var(--accent-soft); background: var(--panel); }
ul, ol { padding-left: 1.35rem; }

.layout { display: grid; grid-template-columns: 18rem minmax(0, 1fr); min-height: 100vh; }
.sidebar { background: var(--panel); border-right: 1px solid var(--line); padding: 1.2rem 1rem 2rem; position: sticky; top: 0; height: 100vh; overflow-y: auto; }
.brand { font-size: 1.2rem; font-weight: 700; margin-bottom: 1rem; }
.brand a { color: var(--text); }
.sidebar ol { list-style: none; margin: 0; padding-left: 0; }
.sidebar li { margin: 0.2rem 0; }
.sidebar li ol { padding-left: 1rem; margin-top: 0.25rem; }
.sidebar a { display: block; padding: 0.15rem 0; color: var(--muted); }
.sidebar .active > a { color: var(--accent); font-weight: 700; }
.content { padding: 2rem 3rem 3rem; max-width: 60rem; }
.topbar { display: flex; justify-content: space-between; align-items: center; margin-bottom: 2rem; color: var(--muted); font-size: 0.95rem; }
h1, h2, h3 { line-height: 1.2; margin-top: 1.6rem; }
h1 { font-size: 2.2rem; margin-top: 0; }
h2 { font-size: 1.45rem; border-top: 1px solid var(--line); padding-top: 1rem; }
h3 { font-size: 1.12rem; }
.header { color: inherit; }

@media (max-width: 960px) {
  .layout { grid-template-columns: 1fr; }
  .sidebar { position: static; height: auto; border-right: 0; border-bottom: 1px solid var(--line); }
  .content { padding: 1.5rem 1rem 2rem; }
}
"""


def relative_to(current_path: str, target_path: str) -> str:
    current_dir = Path(current_path).parent
    return Path(os.path.relpath(target_path, current_dir if str(current_dir) else ".")).as_posix()


def page_paths(tree):
    for _, path, children in tree:
        yield path
        yield from page_paths(children)


def render_sidebar(tree, current_path: str) -> str:
    def render_nodes(nodes):
        parts = ["<ol>"]
        for title, path, children in nodes:
            active = " active" if path == current_path else ""
            parts.append(f'<li class="{active.strip()}"><a href="{relative_to(current_path, path)}">{html.escape(title)}</a>')
            if children:
                parts.append(render_nodes(children))
            parts.append("</li>")
        parts.append("</ol>")
        return "".join(parts)

    return render_nodes(tree)


def render_page(path: str, title: str, content: str) -> str:
    return f"""<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8">
    <title>{html.escape(title)} - Rant Reference</title>
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="description" content="Stable Rant 4.0 language, runtime, CLI, and standard-library reference.">
    <link rel="stylesheet" href="{relative_to(path, 'styles.css')}">
  </head>
  <body>
    <div class="layout">
      <nav class="sidebar" aria-label="Table of contents">
        <div class="brand"><a href="{relative_to(path, 'intro.html')}">Rant Reference</a></div>
        {render_sidebar(TREE, path)}
      </nav>
      <main class="content">
        <div class="topbar">
          <div>Stable 4.0 Reference</div>
          <div><a href="https://github.com/rant-lang/rant">Repository</a></div>
        </div>
        {content}
      </main>
    </div>
  </body>
</html>
"""


def write_page(path: str, title: str, content: str) -> None:
    dest = OUT_DIR / path
    dest.parent.mkdir(parents=True, exist_ok=True)
    dest.write_text(render_page(path, title, content), encoding="utf-8")


def paragraph(text: str) -> str:
    return f"<p>{text}</p>"


def heading(level: int, text: str) -> str:
    ident = text.lower().replace(" ", "-").replace("@", "").replace("/", "-")
    return f'<h{level} id="{ident}"><a class="header" href="#{ident}">{html.escape(text)}</a></h{level}>'


def table(headers: list[str], rows: list[list[str]]) -> str:
    head = "".join(f"<th>{html.escape(h)}</th>" for h in headers)
    body_rows = []
    for row in rows:
        body_rows.append("<tr>" + "".join(f"<td>{cell}</td>" for cell in row) + "</tr>")
    return "<table><thead><tr>" + head + "</tr></thead><tbody>" + "".join(body_rows) + "</tbody></table>"


def doc_example(code: str, expected: str) -> str:
    return f'<pre data-rant-example="true" data-expect="{html.escape(expected)}"><code class="language-rant">{html.escape(code)}</code></pre>'


def render_intro():
    return (
        "Introduction",
        "".join(
            [
                heading(1, "Introduction"),
                paragraph("Rant 4.0 is a procedural templating language for generating text and structured values. This reference covers the stable language, runtime behavior, module system, CLI, and standard library."),
                paragraph("The in-repo documentation was rebuilt using the archived reference site as the starting map and then rewritten to match the shipped 4.0 implementation."),
                heading(2, "Contents"),
                "<ul>"
                '<li><a href="getting-started.html"><strong>Getting Started</strong></a> for installation, the first program, and the CLI quickstart.</li>'
                '<li><a href="language.html"><strong>Language</strong></a> for syntax, text behavior, functions, accessors, and control flow.</li>'
                '<li><a href="runtime.html"><strong>Runtime Features</strong></a> for attributes, formatters, modules, determinism, and the CLI.</li>'
                '<li><a href="stdlib.html"><strong>Standard Library</strong></a> for the exported built-ins grouped by category.</li>'
                '<li><a href="compiler-messages.html"><strong>Diagnostics</strong></a> for compiler messages and runtime error categories.</li>'
                "</ul>",
            ]
        ),
    )


def render_getting_started():
    return (
        "Getting Started",
        "".join(
            [
                heading(1, "Getting Started"),
                paragraph("Rant can be used through the CLI or embedded as a Rust library."),
                heading(2, "Install The CLI"),
                '<pre><code class="language-sh">cargo install rant --version 4.0.0 --features cli</code></pre>',
                paragraph("From a checkout, you can also run the CLI with `cargo run --features cli -- --help`."),
                heading(2, "Your First Program"),
                doc_example(
                    "[$greet:name] {\n  Hello, <name>!\n}\n\n[greet:world]",
                    "Hello, world!",
                ),
                heading(2, "CLI Quickstart"),
                '<pre><code class="language-sh"># Start the REPL\nrant\n\n# Run inline code\nrant --eval \'[rep:3][sep:\\s]{beep}\'\n\n# Run a file\nrant examples/helloworld.rant</code></pre>',
                heading(2, "Embed In Rust"),
                '<pre><code class="language-rust">use rant::Rant;\n\nlet mut rant = Rant::new();\nlet program = rant.compile_quiet("Hello, world!")?;\nlet output = rant.run(&program)?;</code></pre>',
            ]
        ),
    )


def render_language_overview():
    return (
        "Language",
        "".join(
            [
                heading(1, "Language"),
                paragraph("Rant is text-first: fragments, expressions, and nested scopes all print into an output buffer. Blocks, functions, accessors, and keywords build on that output model."),
                "<ul>"
                '<li><a href="language/text.html">Text</a>: fragments, whitespace normalization, hinting, and sinking.</li>'
                '<li><a href="language/blocks.html">Blocks</a>: selection, repetition, weights, and protected blocks.</li>'
                '<li><a href="language/functions.html">Functions</a>: definitions, parameters, calls, lambdas, and piping.</li>'
                '<li><a href="language/data-types.html">Data Types</a>: the runtime value model and truthiness.</li>'
                '<li><a href="language/accessors.html">Accessors</a>: getters, setters, fallback values, and descoping.</li>'
                '<li><a href="language/keywords.html">Keywords</a>: built-in control-flow and module keywords.</li>'
                "</ul>",
            ]
        ),
    )


def render_text_page():
    return (
        "Text",
        "".join(
            [
                heading(1, "Text"),
                paragraph("Plain source text becomes output fragments. By default, same-line whitespace between text units is normalized to a single ASCII space."),
                doc_example("One  two   three", "One two three"),
                heading(2, "Line Breaks"),
                paragraph("A line break between adjacent text fragments does not insert a space by itself."),
                doc_example("Water\nmelon", "Watermelon"),
                heading(2, "Hinting"),
                paragraph("Use a backtick before an expression when it should participate in surrounding text spacing."),
                doc_example('<$name = "world">Hello, `<name>!', "Hello, world!"),
                heading(2, "Sinking"),
                paragraph("Use `~` to remove pending adjacent whitespace around the next text-producing unit."),
                doc_example("{\\:} ~{\\(}", ":("),
            ]
        ),
    )


def render_blocks_page():
    return (
        "Blocks",
        "".join(
            [
                heading(1, "Blocks"),
                paragraph("A block is a set of one or more elements enclosed in braces. Each time the block runs, one element is selected unless block attributes change the behavior."),
                heading(2, "Elements"),
                paragraph("Separate elements with `|`. A single-element block is often used as a nested scope or formatter boundary."),
                heading(2, "Repetition"),
                paragraph("Use `[rep]` to repeat a block, `[sep]` to configure a separator, and `[sel]` to control selection order."),
                heading(2, "Weights"),
                paragraph("Use `@weight` at the start of a block element to assign a weight expression to that element."),
                heading(2, "Protected Blocks"),
                paragraph("Prefix a block with `@` to create a protected block when you need to prevent surrounding syntax from consuming it as another construct."),
            ]
        ),
    )


def render_functions_page():
    return (
        "Functions",
        "".join(
            [
                heading(1, "Functions"),
                paragraph("Define named functions with `[$name: params] { ... }` and call them with `[name: args]`. Arguments are separated with semicolons."),
                doc_example('[$square:x] {[mul:<x>;<x>]}\n[square:3]', "9"),
                heading(2, "Parameters"),
                paragraph("A required parameter is written as `x`. Optional parameters use `x?`. Variadic parameters use `args*` for zero or more values or `args+` for one or more values."),
                heading(2, "Lambdas"),
                paragraph("Anonymous functions use `[?: params] { ... }` and capture surrounding variables by reference when needed."),
                heading(2, "Piping"),
                paragraph("Use `|>` inside a function-call argument list to pipe a result into the next function call."),
                '<pre><code class="language-rant">[split: "the quick brown fox"; \\s |> filter: []; [?:word] { [len: <word> |> le: 3] } |> join: \\s]</code></pre>',
            ]
        ),
    )


def render_data_types_page():
    rows = [
        ["<code>string</code>", "Text data.", "Truthy when non-empty."],
        ["<code>int</code>", "64-bit signed integer.", "Truthy when nonzero."],
        ["<code>float</code>", "64-bit floating-point value.", "Truthy when nonzero and not NaN."],
        ["<code>bool</code>", "Boolean value.", "Uses its own value."],
        ["<code>list</code>", "Ordered mutable collection.", "Truthy when non-empty."],
        ["<code>tuple</code>", "Ordered immutable collection.", "Always truthy."],
        ["<code>map</code>", "Keyed mutable collection.", "Truthy when non-empty."],
        ["<code>range</code>", "Lazy integer range.", "Truthy when non-empty."],
        ["<code>function</code>", "Callable value.", "Always truthy."],
        ["<code>selector</code>", "Selection strategy object.", "Always truthy."],
        ["<code>nothing</code>", "Unit-like empty value.", "Always falsy."],
    ]
    return (
        "Data Types",
        "".join(
            [
                heading(1, "Data Types"),
                paragraph("Rant programs operate on a small set of runtime value types. Collections and functions are first-class values, so a script can generate structured output as well as text."),
                table(["Type", "Description", "Truthiness"], rows),
            ]
        ),
    )


def render_accessors_page():
    return (
        "Accessors",
        "".join(
            [
                heading(1, "Accessors"),
                paragraph("Accessors read and write variables, collection members, and nested values."),
                heading(2, "Getters"),
                paragraph("Use `<name>` to read a variable, `<list/0>` to index a list, and `<map/key>` to access a keyed map value."),
                heading(2, "Setters"),
                paragraph("Setter forms live inside angle brackets as well and support assignment, definition, and compound assignment."),
                heading(2, "Fallback Values"),
                paragraph("Use `?` inside an accessor when a missing lookup should fall back to a default value instead of raising an error."),
                '<pre><code class="language-rant">&lt;f/-2 ? 0&gt;</code></pre>',
                heading(2, "Globals And Descoping"),
                paragraph("Accessors can target globals explicitly or walk outward through parent scopes when a local binding is shadowed."),
                heading(2, "Anonymous Accessors"),
                paragraph("Anonymous accessors let an inline value become the root of a lookup path."),
            ]
        ),
    )


def render_keywords_overview():
    return (
        "Keywords",
        "".join(
            [
                heading(1, "Keywords"),
                paragraph("Rant keywords are prefixed with `@` and provide built-in control-flow and module operations."),
                "<ul>"
                '<li><a href="keywords/require.html"><code>@require</code></a>: import a module.</li>'
                '<li><a href="keywords/return.html"><code>@return</code></a>: return from the nearest function.</li>'
                '<li><a href="keywords/continue.html"><code>@continue</code></a>: continue the nearest repeater iteration.</li>'
                '<li><a href="keywords/break.html"><code>@break</code></a>: stop the nearest repeater.</li>'
                '<li><a href="keywords/weight.html"><code>@weight</code></a>: assign a block-element weight.</li>'
                '<li><a href="keywords/text.html"><code>@text</code></a>: mark a definition as text-producing for auto-hinting.</li>'
                "</ul>",
            ]
        ),
    )


def render_require_keyword():
    return (
        "@require",
        "".join(
            [
                heading(1, "@require"),
                paragraph("`@require` imports a module into the current scope. The default binding name is the requested file name without the `.rant` extension."),
                '<pre><code class="language-rant">@require "math/seq"\n[seq/fib: 8]</code></pre>',
                paragraph("You can provide an alias with `@require alias: \"path\"`. See the Modules page for resolution order, caching, cycle detection, and failure behavior."),
            ]
        ),
    )


def render_return_keyword():
    return (
        "@return",
        "".join(
            [
                heading(1, "@return"),
                paragraph("`@return` exits the nearest reachable function."),
                paragraph("`@return value` uses the supplied value as the function result. Plain `@return` uses the current function output instead."),
                paragraph("`@return` does not target repeaters or outer functions past the nearest function boundary."),
            ]
        ),
    )


def render_continue_keyword():
    return (
        "@continue",
        "".join(
            [
                heading(1, "@continue"),
                paragraph("`@continue` skips the remainder of the nearest active repeater iteration and starts the next iteration."),
                paragraph("With an expression, `@continue value` passes that value to the repeater. Without an expression, the current element output is used."),
                paragraph("It can cross nested blocks that belong to the same repeater, but it does not cross function boundaries. Using it where no repeater is reachable raises a control-flow runtime error."),
            ]
        ),
    )


def render_break_keyword():
    return (
        "@break",
        "".join(
            [
                heading(1, "@break"),
                paragraph("`@break` exits the nearest active repeater immediately."),
                paragraph("With an expression, `@break value` becomes the repeater result. Without an expression, the current element output becomes the repeater result."),
                paragraph("It can exit through nested blocks owned by the repeater, but it does not cross function boundaries. Using it where no repeater is reachable raises a control-flow runtime error."),
            ]
        ),
    )


def render_weight_keyword():
    return (
        "@weight",
        "".join(
            [
                heading(1, "@weight"),
                paragraph("`@weight` appears at the start of a block element and assigns that element's selection weight."),
                paragraph("Weight expressions can evaluate to any value. Numeric values are used directly; other values are converted through truthiness."),
            ]
        ),
    )


def render_text_keyword():
    return (
        "@text",
        "".join(
            [
                heading(1, "@text"),
                paragraph("`@text` marks a function or definition as text-producing so surrounding whitespace is handled with auto-hinting behavior."),
                paragraph("Use it when a value should behave like text in surrounding output without requiring explicit backtick hints at every call site."),
            ]
        ),
    )


def render_operators_page():
    rows = [
        ["Arithmetic", "<code>+</code> <code>-</code> <code>*</code> <code>/</code> <code>%</code> <code>**</code>"],
        ["Comparison", "<code>==</code> <code>!=</code> <code>&lt;</code> <code>&lt;=</code> <code>&gt;</code> <code>&gt;=</code>"],
        ["Logic", "<code>!</code> <code>&amp;</code> <code>|</code> <code>^</code>"],
        ["Assignment", "Setter forms and compound assignments on accessors."],
    ]
    return (
        "Operators",
        "".join(
            [
                heading(1, "Operators"),
                paragraph("Rant supports arithmetic, comparison, and logic operators with expression precedence."),
                table(["Category", "Operators"], rows),
            ]
        ),
    )


def render_conditionals_page():
    rows = [
        ["<code>bool</code>", "Unchanged."],
        ["<code>int</code>", "Truthy when nonzero."],
        ["<code>float</code>", "Truthy when nonzero and not NaN. Infinity is truthy."],
        ["<code>string</code>, <code>list</code>, <code>map</code>, <code>range</code>", "Truthy when non-empty."],
        ["<code>tuple</code>, <code>function</code>, <code>selector</code>", "Always truthy."],
        ["<code>nothing</code>", "Always falsy."],
    ]
    return (
        "Conditional Expressions",
        "".join(
            [
                heading(1, "Conditional Expressions"),
                paragraph("Conditional expressions are built from `@if`, `@elseif`, and `@else`. Conditions are evaluated from left to right until the first truthy branch succeeds."),
                doc_example("@if 0: {\n  zero\n} @else: {\n  nonzero\n}", "nonzero"),
                heading(2, "Truthiness"),
                table(["Type", "Truthiness"], rows),
                paragraph("Once a branch succeeds, later conditions are not evaluated and their bodies are not run."),
            ]
        ),
    )


def render_output_modifiers_page():
    return (
        "Output Modifiers",
        "".join(
            [
                heading(1, "Output Modifiers"),
                paragraph("Output modifiers are block elements that transform the caller's current output before the element's own result is written back."),
                heading(2, "The @edit Operator"),
                paragraph("`@edit` consumes the caller's current output, optionally binds it to a local name, and replaces it with the modifier body."),
                doc_example('"example" { @edit x: `<x> `<x> }', "example example"),
                doc_example('"example" { @edit: "overwritten" }', "overwritten"),
                heading(2, "Placement Rules"),
                paragraph("`@edit` must appear at the start of a block element."),
            ]
        ),
    )


def render_runtime_overview():
    return (
        "Runtime Features",
        "".join(
            [
                heading(1, "Runtime Features"),
                paragraph("Rant's runtime tracks output buffers, block attributes, number and whitespace formatting, an RNG stack, module caching, and diagnostics."),
                paragraph("A `Rant` context owns the global scope, the module cache, registered data sources, and the current RNG seed."),
            ]
        ),
    )


def render_runtime_attributes():
    rows = [
        ["<code>[rep]</code>", "Configures repetition count or repetition mode."],
        ["<code>[sep]</code>", "Sets the separator between repeated iterations."],
        ["<code>[sel]</code>", "Sets or returns the active selector."],
        ["<code>[mut]</code>", "Sets a mutator function for block elements."],
        ["<code>[if]</code>, <code>[elseif]</code>, <code>[else]</code>", "Configure conditional block behavior."],
        ["<code>[step]</code>, <code>[step-index]</code>, <code>[step-count]</code>", "Expose the current block position."],
        ["<code>[reset-attrs]</code>", "Clears the active attribute state."],
    ]
    return (
        "Attributes",
        "".join(
            [
                heading(1, "Attributes"),
                paragraph("Attributes configure how the next block executes. In stable 4.0 they are exposed through standard-library functions."),
                table(["Attribute", "Purpose"], rows),
            ]
        ),
    )


def render_runtime_formatters():
    rows = [
        ["<code>[ws-fmt]</code>", "Gets or sets whitespace-normalization mode for the current scope."],
        ["<code>[num-fmt]</code>", "Gets or sets number-format options as a map."],
        ["<code>[num-fmt-*</code> family]", "Update individual number-format properties such as system, precision, sign style, and separators."],
    ]
    return (
        "Formatters",
        "".join(
            [
                heading(1, "Formatters"),
                paragraph("Stable 4.0 exposes whitespace and number formatting through the current output scope."),
                table(["Formatter", "Purpose"], rows),
                paragraph("Whitespace normalization defaults to a single ASCII space on the same line. Line breaks between adjacent text fragments are ignored unless explicit text output inserts them."),
            ]
        ),
    )


def render_modules_page():
    return (
        "Modules",
        "".join(
            [
                heading(1, "Modules"),
                paragraph("Rant modules are ordinary `.rant` files whose top-level result becomes the imported module value. In typical use a module returns a map of functions and values."),
                heading(2, "Search Order"),
                "<ol>"
                "<li>The importing program's directory when the caller came from a file-backed program.</li>"
                "<li>The resolver's local modules path, or the host working directory when no local path is configured.</li>"
                "<li>The global modules path from `RANT_MODULES_PATH` when global modules are enabled.</li>"
                "</ol>",
                heading(2, "File Names"),
                paragraph("The default resolver appends the `.rant` extension automatically and normalizes relative paths before loading."),
                heading(2, "Caching"),
                paragraph("Modules are cached per `Rant` context. Requiring the same module again returns the cached value instead of recompiling or re-running the module body."),
                heading(2, "Failures"),
                "<ul>"
                "<li>Missing modules raise `MODULE_ERROR`.</li>"
                "<li>Compile failures in module source raise `MODULE_ERROR` with compiler diagnostics.</li>"
                "<li>Runtime failures during module initialization propagate as runtime errors during import.</li>"
                "<li>Cyclic imports are rejected with a deterministic `MODULE_ERROR`.</li>"
                "</ul>",
            ]
        ),
    )


def render_cli_page():
    rows = [
        ["<code>-e</code>, <code>--eval</code>", "Runs an inline program string."],
        ["<code>-s</code>, <code>--seed</code>", "Sets the initial RNG seed as 1 to 16 hexadecimal digits, with an optional `0x` prefix."],
        ["<code>-b</code>, <code>--bench-mode</code>", "Prints compile and execution timing."],
        ["<code>-W</code>, <code>--no-warnings</code>", "Suppresses compiler warnings."],
        ["<code>-D</code>, <code>--no-debug</code>", "Disables debug symbol emission during compilation."],
    ]
    exit_rows = [
        ["<code>0</code>", "Success."],
        ["<code>64</code>", "Invalid CLI usage, such as an invalid seed."],
        ["<code>65</code>", "Compilation failed."],
        ["<code>66</code>", "Input file not found."],
        ["<code>70</code>", "Runtime execution failed."],
    ]
    return (
        "CLI And REPL",
        "".join(
            [
                heading(1, "CLI And REPL"),
                paragraph("The `rant` CLI runs inline code, files, or piped stdin. When no source is provided and stdin is a TTY, it starts the interactive REPL."),
                heading(2, "Execution Order"),
                "<ol><li><code>--eval PROGRAM</code></li><li><code>FILE</code></li><li>Piped stdin</li><li>REPL</li></ol>",
                heading(2, "Flags"),
                table(["Flag", "Description"], rows),
                heading(2, "Exit Codes"),
                table(["Code", "Meaning"], exit_rows),
                heading(2, "REPL Behavior"),
                paragraph("The REPL keeps top-level definitions between lines and suppresses unused-variable and unused-function warnings that would otherwise be noisy in an interactive session."),
            ]
        ),
    )


def render_stdlib_overview():
    return (
        "Standard Library",
        "".join(
            [
                heading(1, "Standard Library"),
                paragraph("The stable Rant 4.0 standard library is loaded by default and grouped into the categories listed in the sidebar."),
                paragraph("Function names shown in the reference are the exported public names, including hyphenated forms such as `pick-sparse` and `num-fmt-group-sep`."),
            ]
        ),
    )


def render_function_table(title: str, intro: str, names: list[str]):
    rows = [
        [
            f"<code>{html.escape(name)}</code>",
            f"<code>{html.escape(MANUAL_SIGNATURES.get(name, ''))}</code>",
            html.escape(MANUAL_SUMMARIES.get(name, f"Reference entry for `{name}`.")),
        ]
        for name in names
    ]
    return (
        title,
        "".join([heading(1, title), paragraph(intro), table(["Name", "Call Form", "Summary"], rows)]),
    )


def parse_compiler_messages():
    source = Path("src/compiler/message.rs").read_text(encoding="utf-8")
    code_map = {}
    message_map = {}
    for variant, code in re.findall(r"Self::(\w+)(?:\([^)]*\))?\s*=>\s*rcode!\((\d+)\)", source):
        code_map[variant] = f"R{code}"
    for variant, message in re.findall(r'Self::(\w+)(?:\([^)]*\))?\s*=>\s*rmsg!\("([^"]+)"', source):
        message_map[variant] = message
    rows = []
    for variant, code in sorted(code_map.items(), key=lambda item: item[1]):
        severity = "warning" if variant in COMPILER_WARNING_VARIANTS else "error"
        rows.append([f"<code>{code}</code>", severity, f"<code>{html.escape(message_map.get(variant, variant))}</code>"])
    return rows


def render_diagnostics():
    runtime_rows = [[f"<code>{code}</code>", html.escape(summary)] for code, summary in RUNTIME_ERROR_ROWS]
    return (
        "Diagnostics",
        "".join(
            [
                heading(1, "Diagnostics"),
                paragraph("The compiler emits stable message codes, and runtime failures report a stable error category in square brackets."),
                heading(2, "Compiler Messages"),
                table(["Code", "Severity", "Message Template"], parse_compiler_messages()),
                heading(2, "Runtime Error Categories"),
                table(["Category", "Summary"], runtime_rows),
            ]
        ),
    )


def render_glossary():
    rows = [
        ["Block", "A brace-delimited selection unit made of one or more elements."],
        ["Element", "A single selectable item inside a block."],
        ["Hinting", "Treating an expression as text for surrounding whitespace behavior."],
        ["Sinking", "Removing pending adjacent whitespace around the next text unit."],
        ["Repeater", "A block running under repetition, usually through `[rep]`."],
        ["Selector", "An object that chooses the next block element."],
        ["Scope", "A local output and variable environment."],
    ]
    return (
        "Glossary",
        "".join([heading(1, "Glossary"), table(["Term", "Meaning"], rows)]),
    )


def render_constants():
    rows = [[f"<code>{name}</code>", html.escape(summary)] for name, summary in CONSTANTS]
    return (
        "Constants",
        "".join([heading(1, "Constants"), paragraph("These global constants are loaded with the standard library."), table(["Name", "Summary"], rows)]),
    )


def render_page_content(path: str):
    if path == "intro.html":
        return render_intro()
    if path == "getting-started.html":
        return render_getting_started()
    if path == "language.html":
        return render_language_overview()
    if path == "language/text.html":
        return render_text_page()
    if path == "language/blocks.html":
        return render_blocks_page()
    if path == "language/functions.html":
        return render_functions_page()
    if path == "language/data-types.html":
        return render_data_types_page()
    if path == "language/accessors.html":
        return render_accessors_page()
    if path == "language/keywords.html":
        return render_keywords_overview()
    if path == "language/keywords/require.html":
        return render_require_keyword()
    if path == "language/keywords/return.html":
        return render_return_keyword()
    if path == "language/keywords/continue.html":
        return render_continue_keyword()
    if path == "language/keywords/break.html":
        return render_break_keyword()
    if path == "language/keywords/weight.html":
        return render_weight_keyword()
    if path == "language/keywords/text.html":
        return render_text_keyword()
    if path == "language/operators.html":
        return render_operators_page()
    if path == "language/conditional-expressions.html":
        return render_conditionals_page()
    if path == "language/output-modifiers.html":
        return render_output_modifiers_page()
    if path == "runtime.html":
        return render_runtime_overview()
    if path == "runtime/attributes.html":
        return render_runtime_attributes()
    if path == "runtime/formatters.html":
        return render_runtime_formatters()
    if path == "modules.html":
        return render_modules_page()
    if path == "cli.html":
        return render_cli_page()
    if path == "stdlib.html":
        return render_stdlib_overview()
    if path in STDLIB_CATEGORIES:
        title = Path(path).stem.replace("-", " ").title()
        if path == "stdlib/control-flow.html":
            title = "Attributes And Control Flow"
        intros = {
            "stdlib/general.html": "General-purpose functions, module helpers, and runtime utilities.",
            "stdlib/control-flow.html": "Functions that configure block behavior or expose active block state.",
            "stdlib/collections.html": "Functions for lists, tuples, maps, and collection transforms.",
            "stdlib/generators.html": "Random generation helpers built on the active RNG.",
            "stdlib/formatting.html": "Whitespace and number-format configuration helpers.",
            "stdlib/strings.html": "String construction, segmentation, casing, and character helpers.",
            "stdlib/boolean.html": "Boolean operations.",
            "stdlib/comparison.html": "Comparison predicates.",
            "stdlib/math.html": "Arithmetic and numeric utilities.",
            "stdlib/conversion.html": "Type-conversion helpers.",
            "stdlib/verification.html": "Predicates for type and value checks.",
            "stdlib/assertion.html": "Assertion helpers that raise runtime errors on failure.",
        }
        return render_function_table(title, intros[path], STDLIB_CATEGORIES[path])
    if path == "stdlib/constants.html":
        return render_constants()
    if path == "compiler-messages.html":
        return render_diagnostics()
    if path == "glossary.html":
        return render_glossary()
    raise KeyError(path)


def build_docs():
    if OUT_DIR.exists():
        shutil.rmtree(OUT_DIR)
    OUT_DIR.mkdir(parents=True)
    (OUT_DIR / "styles.css").write_text(STYLES, encoding="utf-8")
    for path in page_paths(TREE):
        title, content = render_page_content(path)
        write_page(path, title, content)
    shutil.copy2(OUT_DIR / "intro.html", OUT_DIR / "index.html")


if __name__ == "__main__":
    build_docs()
