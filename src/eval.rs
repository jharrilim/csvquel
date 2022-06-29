use std::fs::File;

use csv::Reader;

use crate::syntax::{Comp, Logic, Predicate, WhereClause};

#[derive(Debug, PartialEq)]
pub struct Evaluation {
    pub headers: Vec<String>,
    pub data: Vec<Vec<String>>,
}

pub struct Evaluator {
    reader: Reader<File>,
}

impl Evaluator {
    pub fn new(reader: Reader<File>) -> Self {
        Self { reader }
    }
    pub fn evaluate(&mut self, _cols: Vec<String>, where_clause: Option<WhereClause>) -> Evaluation {
        let headers = self
            .reader
            .headers()
            .expect("Failed to read headers")
            .iter()
            .map(|h| h.to_string())
            .collect();
        let mut data = Vec::<Vec<String>>::new();

        for row in self.reader.records() {
            let row = row.unwrap();

            let row_cols: Vec<_> = row.iter().map(|r| r.to_string()).collect();

            match where_clause {
                Some(ref where_clause) => {
                    if Evaluator::evaluate_where_clause(&headers, &row_cols, &where_clause) {
                        data.push(row_cols);
                    }
                }
                None => {
                    data.push(row_cols);
                }
            }

        }

        Evaluation { headers, data }
    }

    fn evaluate_where_clause(headers: &Vec<String>, row_cols: &Vec<String>, where_clause: &WhereClause) -> bool {
        let mut row_cols = row_cols.iter();
        let mut preds = where_clause.0.iter();
        let mut logic = where_clause.1.iter();
        let mut result = Evaluator::evaluate_predicate(headers, &mut row_cols, &mut preds);
        while let Some(logic) = logic.next() {
            result = match logic {
                Logic::And => result && Evaluator::evaluate_predicate(headers, &mut row_cols, &mut preds),
                Logic::Or => result || Evaluator::evaluate_predicate(headers, &mut row_cols, &mut preds),
            };
        }
        result
    }
    fn evaluate_predicate(
        headers: &Vec<String>,
        row_cols: &mut std::slice::Iter<String>,
        preds: &mut std::slice::Iter<Predicate>,
    ) -> bool {
        let pred = preds.next().unwrap();
        let val = headers
            .iter()
            .position(|h| h == &pred.col)
            .and_then(|i| row_cols.clone().nth(i))
            .unwrap();
        let pred_val = &pred.val;
        println!("{} {:?} {}", val, pred.comp, pred_val);
        match pred.comp {
            Comp::Equals => pred_val == val,
            Comp::Like => val.contains(pred_val),
            Comp::Ilike => val.to_lowercase().contains(pred_val.to_lowercase().as_str()),
        }
    }
}
