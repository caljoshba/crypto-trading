// use crate::types::kraken_ws::{
//     response::{
//         PairResult,
//         Bid
//     },
//     Pair
// };
// use std::collections::LinkedList;

// #[derive(Debug)]
// pub struct Trades {
//     list: LinkedList<Bid>,
//     pair: Pair
// }

// impl Trades {
//     pub fn new(pair: Pair) -> Self {
//         Self {
//             list: LinkedList::new(),
//             pair
//         }
//     }

//     pub fn append(&mut self, value: Option<PairResult>) {
//         if let Some(pair) = value {
//             self.list.push_back(pair.b);
//         }
//     }
// }