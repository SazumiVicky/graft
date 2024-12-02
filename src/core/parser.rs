use std::{collections::{HashMap, VecDeque}, sync::Arc};
use parking_lot::RwLock;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid token sequence: {0}")]
    InvalidToken(String),
    #[error("unexpected end of input")]
    UnexpectedEOF,
    #[error("syntax error: {0}")]
    Syntax(String),
}

type Result<T> = std::result::Result<T, ParseError>;

#[derive(Debug, Clone)]
pub struct Tok {
    val: String,
    pos: usize,
    typ: TokType,
}

#[derive(Debug, Clone, PartialEq)]
enum TokType {
    Id,
    Op,
    Num,
    Sym,
}

pub struct Prs {
    toks: VecDeque<Tok>,
    ctx: Arc<RwLock<PrsCtx>>,
    idx: usize,
}

struct PrsCtx {
    syms: HashMap<String, f64>,
    depth: usize,
}

impl Prs {
    pub fn new(input: &str) -> Self {
        Self {
            toks: Self::lex(input),
            ctx: Arc::new(RwLock::new(PrsCtx {
                syms: HashMap::new(),
                depth: 0,
            })),
            idx: 0,
        }
    }

    fn lex(input: &str) -> VecDeque<Tok> {
        let mut toks = VecDeque::new();
        let mut pos = 0;
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                '0'..='9' => {
                    let mut num = String::new();
                    while let Some(&d) = chars.peek() {
                        if d.is_ascii_digit() || d == '.' {
                            num.push(d);
                            chars.next();
                            pos += 1;
                        } else {
                            break;
                        }
                    }
                    toks.push_back(Tok {
                        val: num,
                        pos,
                        typ: TokType::Num,
                    });
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut id = String::new();
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_alphanumeric() || c == '_' {
                            id.push(c);
                            chars.next();
                            pos += 1;
                        } else {
                            break;
                        }
                    }
                    toks.push_back(Tok {
                        val: id,
                        pos,
                        typ: TokType::Id,
                    });
                }
                '+' | '-' | '*' | '/' | '^' => {
                    toks.push_back(Tok {
                        val: c.to_string(),
                        pos,
                        typ: TokType::Op,
                    });
                    chars.next();
                    pos += 1;
                }
                _ => {
                    chars.next();
                    pos += 1;
                }
            }
        }
        toks
    }

    pub fn parse(&mut self) -> Result<f64> {
        self.expr()
    }

    fn expr(&mut self) -> Result<f64> {
        let mut lhs = self.term()?;

        while let Some(tok) = self.peek() {
            match tok.val.as_str() {
                "+" => {
                    self.next();
                    lhs += self.term()?;
                }
                "-" => {
                    self.next();
                    lhs -= self.term()?;
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn term(&mut self) -> Result<f64> {
        let mut lhs = self.factor()?;

        while let Some(tok) = self.peek() {
            match tok.val.as_str() {
                "*" => {
                    self.next();
                    lhs *= self.factor()?;
                }
                "/" => {
                    self.next();
                    let rhs = self.factor()?;
                    if rhs == 0.0 {
                        return Err(ParseError::Syntax("division by zero".into()));
                    }
                    lhs /= rhs;
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn factor(&mut self) -> Result<f64> {
        let tok = self.next().ok_or(ParseError::UnexpectedEOF)?;
        
        match tok.typ {
            TokType::Num => tok.val.parse::<f64>().map_err(|_| {
                ParseError::InvalidToken(format!("invalid number: {}", tok.val))
            }),
            TokType::Id => {
                let ctx = self.ctx.read();
                ctx.syms
                    .get(&tok.val)
                    .copied()
                    .ok_or_else(|| ParseError::Syntax(format!("undefined variable: {}", tok.val)))
            }
            _ => Err(ParseError::InvalidToken(format!(
                "unexpected token: {}",
                tok.val
            ))),
        }
    }

    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.idx)
    }

    fn next(&mut self) -> Option<Tok> {
        if self.idx < self.toks.len() {
            let tok = self.toks[self.idx].clone();
            self.idx += 1;
            Some(tok)
        } else {
            None
        }
    }
}