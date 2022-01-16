use crate::types::kraken_ws::PairResult;
use std::collections::LinkedList;

#[derive(Debug)]
pub struct Trades {
    list: LinkedList<PairResult>
}

impl Trades {
    pub fn new() -> Self {
        Self {
            list: LinkedList::new()
        }
    }

    pub fn append(&mut self, value: Option<PairResult>) {
        if let Some(pair) = value {
            self.list.push_back(pair);
        }
    }
}