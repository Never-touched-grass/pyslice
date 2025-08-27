# PySlice: Python Lexer Generator
Releases
[![GitHub release](https://img.shields.io/github/v/release/Never-touched-grass/pyslice)](https://github.com/Never-touched-grass/pyslice/releases)
[![License](https://img.shields.io/github/license/Never-touched-grass/pyslice)](LICENSE)

**PySlice** is a fast and lightweight **lexer generator for Python**, written in Rust. It reads `.lex` files and generates a ready-to-use Python lexer.

---

## ✨ Features
- ✅ Simple `.lex` syntax
- ✅ Generates pure Python code
- ✅ Cross-platform (Windows & Linux)
- ✅ Ignore tokens using `IGNORE` keyword
- ✅ Run --test to test your lexer on an input string
- ✅ Run --verbose on --test to get verbose output
- ✅ Run `pyslice <lexer.lex> <--verbose> for the REPL.
---

## 📦 Installation

Download the latest release from the [**Releases page**](https://github.com/Never-touched-grass/pyslice/releases):

- **Windows**: `pyslice.exe`
- **Linux**: `pyslice`
- **For all Linux users, make sure to `chmod +x pyslice` and run `./pyslice` to use it**

(Optional) Add it to your `PATH` for easy use.

---

## 📝 Example `.lex` File

```lex
; Comments start with a semicolon
; Token definitions: NAME    REGEX    [OPTIONS]

NUMBER       [0-9]+
IDENT        [a-zA-Z_][a-zA-Z0-9_]*
PLUS         \+
MINUS        \-
MULT         \*
DIV          /
LPAREN       \(
RPAREN       \)
WHITESPACE   [ \t\n]+    IGNORE
COMMENT      ;.*         IGNORE
```
