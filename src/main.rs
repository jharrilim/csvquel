mod cli;
mod eval;
mod lexer;
mod parse;
mod source;
mod syntax;

use clap::Parser;
use lexer::Lexer;
use source::Source;
use syntax::TokenKind;

use crate::eval::Evaluator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();
    let tokens: Vec<_> = Lexer::new(&*args.query).collect();
    let mut source = Source::new(&tokens);
    // I can't imagine someone querying for anything other than a select,
    // so we can just keep the query evaluation simple for now

    if source.peek_kind() != Some(TokenKind::Select) {
        return Err("Query must start with select".into());
    }
    source.next_token();

    let cols = parse::parse_columns(&mut source)?;
    // FROM is already consumed at this point
    if source.peek_kind() != Some(TokenKind::Identifier) {
        // Even though we're not using it, we'll check it for SQL consistency
        return Err("Expected table name".into());
    }
    source.next_token(); // skip table name

    let where_clause = if source.peek_kind() == Some(TokenKind::Where) {
        source.next_token(); // skip where
        Some(parse::parse_predicates(&mut source)?)
    } else {
        None
    };

    if let Some(f) = args.file {
        let r = csv::Reader::from_path(f)?;

        let res = Evaluator::new(r).evaluate(cols, where_clause);
        println!("{:?}", res);
    }

    Ok(())
}
