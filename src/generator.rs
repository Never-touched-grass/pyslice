use crate::confparse::*;

pub fn generate(input: Vec<TokDef>) -> String {
    let mut py = String::new();
    py.push_str(
        r#"import re

class Token:
    def __init__(self, type_, lexeme):
        self.type = type_
        self.lexeme = lexeme

    def __repr__(self):
        return f"Token({self.type}, '{self.lexeme}')"

class Lexer:
    def __init__(self, inp, verbose=False):
        self.text = inp
        self.pos = 0
        self.verbose = verbose
        self.token_defs = [
"#,
    );

    for t in &input {
        let ignore = if t.ignore { "True" } else { "False" };
        py.push_str(&format!(
            "        ('{}', re.compile(r'{}'), {}),\n",
            t.name, t.pattern, ignore
        ));
    }

    py.push_str("    ]\n\n");
    py.push_str(
        r#"    def next_token(self):
        if self.pos >= len(self.text):
            return None
        text = self.text[self.pos:]
        for name, pattern, ignore in self.token_defs:
            match = pattern.match(text)
            if match:
                lexeme = match.group(0)
                self.pos += len(lexeme)
                if ignore:
                    return self.next_token()
                if self.verbose:
                    self.print_verbose(name, lexeme)
                return Token(name, lexeme)
        raise ValueError(f"Unexpected character at position {self.pos}: '{text[0]}')")
"#,
    );
    py.push_str(
        r#"    def tokenize(self):
        tokens = []
        while True:
            tok = self.next_token()
            if tok is None:
                break
            tokens.append(tok)
        return tokens
"#,
    );
    py.push_str(
        r#"    def print_verbose(self, name, lexeme):
        print(f"[VERBOSE] Matched {name}: '{lexeme}'")      
        "#
    );
    println!("[TOKENS]");
    py
}
