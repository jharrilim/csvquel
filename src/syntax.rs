use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    // keywords
    #[token("select", ignore(case))]
    Select,
    #[token("from", ignore(case))]
    From,
    #[token("where", ignore(case))]
    Where,
    #[token("and", ignore(case))]
    And,
    #[token("or", ignore(case))]
    Or,
    #[token("order by", ignore(case))]
    OrderBy,
    #[token("asc", ignore(case))]
    Asc,
    #[token("desc", ignore(case))]
    Desc,
    #[token(";")]
    Semicolon,
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token("*")]
    Star,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    // comparison
    #[token("=")]
    Equals,
    #[token("like", ignore(case))]
    Like,
    #[token("ilike", ignore(case))]
    Ilike,

    #[regex(r"[a-zA-Z_][a-zA-Z\d_]*")]
    Identifier,

    #[regex(r"[0-9]+")]
    Number,

    #[regex(r"[\s]+", logos::skip)]
    Whitespace,
    #[error]
    Error,
}

impl TokenKind {
    pub fn is_trivia(self) -> bool {
        matches!(self, Self::Whitespace)
    }
}

impl Into<Comp> for TokenKind {
    fn into(self) -> Comp {
        match self {
            TokenKind::Equals => Comp::Equals,
            TokenKind::Like => Comp::Like,
            TokenKind::Ilike => Comp::Ilike,
            _ => panic!("unexpected token"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Comp {
    Equals,
    Like,
    Ilike,
}

#[derive(Debug, PartialEq)]
pub enum Logic {
    And,
    Or,
}

impl Into<Logic> for TokenKind {
    fn into(self) -> Logic {
        match self {
            TokenKind::And => Logic::And,
            TokenKind::Or => Logic::Or,
            _ => panic!("unexpected token"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Predicate {
    pub col: String,
    pub comp: Comp,
    pub val: String,
}

#[derive(Debug, PartialEq)]
pub struct WhereClause(pub Vec<Predicate>, pub Vec<Logic>);
