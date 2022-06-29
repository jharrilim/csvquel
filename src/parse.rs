use crate::{
    source::Source,
    syntax::{Logic, Predicate, TokenKind, WhereClause},
};

pub fn parse_columns(source: &mut Source) -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let mut cols = Vec::<String>::new();

  while let Some(t) = source.next_token() {
      match t.kind {
          TokenKind::Identifier => {
              cols.push(t.text.to_string());
          }
          TokenKind::Comma => continue,
          TokenKind::From => break,
          _ => {
              return Err(format!("Unexpected token {}", t.text).into());
          }
      }
  }
  Ok(cols)
}

pub fn parse_predicates(source: &mut Source) -> Result<WhereClause, Box<dyn std::error::Error>> {
    let mut preds = Vec::<Predicate>::new();
    let mut logic = Vec::<Logic>::new();

    while !source.is_end() {
        match source.peek_kind().unwrap() {
            TokenKind::Identifier => {
                let p = parse_predicate(source)?;
                preds.push(p);
            }
            TokenKind::And | TokenKind::Or => {
                logic.push(source.next_token().unwrap().kind.into());
            }
            TokenKind::Semicolon | TokenKind::OrderBy => break,
            _ => {
                return Err(format!("Unexpected token {}", source.peek_token().unwrap().text).into());
            }
        }
    }
    Ok(WhereClause(preds, logic))
}

fn parse_predicate(source: &mut Source) -> Result<Predicate, Box<dyn std::error::Error>> {
    let col = source.next_token().unwrap().text.to_string();
    let t = source.next_token();
    if t.is_none() {
        return Err("Unexpected end of query".into());
    }
    let t = t.unwrap();

    let comp = match t.kind {
        TokenKind::Equals | TokenKind::Like | TokenKind::Ilike => t.kind,
        _ => {
            return Err(format!("Unexpected token {}", t.text).into());
        }
    };

    let t = source.next_token();
    if t.is_none() {
        return Err("Unexpected end of query".into());
    }
    let t = t.unwrap();

    let val = match t.kind {
        TokenKind::Identifier | TokenKind::Number => t.text.to_string(),
        _ => {
            return Err(format!("Unexpected token {}", t.text).into());
        }
    };
    Ok(Predicate {
        col,
        comp: comp.into(),
        val,
    })
}
