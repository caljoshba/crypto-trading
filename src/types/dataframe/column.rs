// use core::hash::Hash;
// use std::collections::HashMap;
// use std::rc::{ Rc };
// use crate::types::dataframe::cell::Cell;

// pub struct Column<'c, T: Hash + Eq> {
//     values_map: HashMap<&'c T, Vec<Rc<Cell<T>>>>,
//     values: Vec<&'c T>,
// }

// impl<'c, T> Column<'c, T> where
//     T: Hash + Eq {
//     pub fn new() -> Self {
//         Self {
//             values_map: HashMap::new(),
//             values: vec![]
//         }
//     }

//     pub fn add_cell(&mut self, cell: Rc<Cell<T>>) {
//         let entry = self.values_map.get_mut(&cell.value);
//         if let Some(value) = entry {
//             value.push(Rc::clone(&cell));
//         } else {
//             self.values_map.insert(&cell.value, vec![Rc::clone(&cell)]);
//         }
//     }
// }