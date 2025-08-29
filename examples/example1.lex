; Comments look like this
; Structure is:
; [TOKEN_NAME] [REGEX] [OPTIONS]
; Example lexer:
NUM [0-9]+
VAR var
; Make sure VAR goes before IDENT because VARs value matches IDENTs
IDENT [a-zA-Z_][a-zA-Z0-9_]*
EQ \=
WHITESPACE [ \t\n\r] IGNORE
; Use the `IGNORE` keyword for tokens that need to be ignored
COMMENT //.* IGNORE
PLUS \+
MINUS \-
SLASH /
STAR \*
; The above code corresponds t0 example1.py
