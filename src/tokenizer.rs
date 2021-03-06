use ::regex::{Regex, CaptureMatches};

const KEYWORDS: [&str; 2] = ["let", "lambda"];
const EXPRESSIONS: [(&str, &str); 10] = [
    ("identifier", r"[a-zA-Z_][a-zA-Z_0-9]*"),
    ("number", r"[0-9]+"),
    ("string", r#""(?:[^"\\]*(?:\.[^"\\]*)*)""#),
    ("assignment", r"="),
    ("operator", r"[+\-*/%]"),
    ("whitespace", r"[ \t]+"),
    ("lparen", r"\("),
    ("rparen", r"\)"),
    ("newline", r"\n"),
    ("unexpected", r".")
];

#[derive(Debug)]
pub struct Token<'t> {
    line: usize,
    column: usize,

    kind: &'static str,
    value: &'t str,
}

pub struct Tokens<'t> {
    captures: CaptureMatches<'static, 't>,
    line_num: usize,
    line_start: usize,
}

impl<'t> Tokens<'t> {
    pub fn tokenize(code: &'t str) -> Self {
        lazy_static! {
            static ref RE: Regex = {
                let regexp = EXPRESSIONS.iter()
                                        .map(|&(name, expr)| format!("(?P<{}>{})", name, expr))
                                        .collect::<Vec<_>>()
                                        .join("|");

                Regex::new(&regexp).expect("Wrongful Tokenizer RegExp.")
            };
        }

        Self {
            captures: RE.captures_iter(code),
            line_num: 1,
            line_start: 0,
        }
    }
}

impl<'t> Iterator for Tokens<'t> {
    type Item = Token<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.captures.next() {
            Some(capture) => {
                for &(kind, _) in EXPRESSIONS.iter() {
                    if let Some(value) = capture.name(kind) {
                        let column = value.start() - self.line_start;

                        match kind {
                            "unexpected" => {
                                // XXX: Panic-ing might not be a good idea here. Can we somehow signal
                                // that something's wrong?
                                panic!("{:?} was unexpected at line {} and column {}.",
                                       value.as_str(), self.line_num, column);
                            },

                            "whitespace" => {
                                // We want to skip over whitespace, so we just return the next
                                // item.
                                return self.next();
                            },

                            "newline" => {
                                self.line_start = value.end();
                                self.line_num += 1;
                                return self.next();
                            }

                            "identifier" if KEYWORDS.contains(&value.as_str()) => {
                                return Some(Token {line:   self.line_num,
                                                   column: column,
                                                   kind:   "keyword",
                                                   value:  value.as_str()});
                            },

                            _ => {
                                return Some(Token {line:   self.line_num,
                                                   column: column,
                                                   kind:   kind,
                                                   value:  value.as_str()});
                            }
                        }
                    }
                }

                panic!("This should never happen.");
            },

            None => None,
        }
    }
}
