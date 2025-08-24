# PySlice: Python Lexer Generator
PySlice is a Python lexer generator that reads `.lex` files and outputs a lexer
## Syntax
The syntax of a .lex file goes like this:
```
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
; The token above is ignored, due to the 'IGNORE' keyword.
COMMENT      ;.*         IGNORE
```
## Usage
Say you saved the file above as `mylexer.lex`. You would run the following command:
`pyslice mylexer.lex lexer.py`
That command parses mylexer.lex and outputs lexer.py.
