// use std::rc::{ Rc };
// use std::hash::Hash;
// use crate::types::dataframe::row::Row;

// pub trait SelectRow {
//     type RowItem: Eq + Hash + Clone + Copy;
//     type Item: Eq + Hash + Clone + Copy;
//     fn select_row(&self) -> Rc<Row<Cell<Self::RowItem, Self::Item>>>;
// }

// // #[derive(Debug)]
// pub struct Cell<K, T> where 
//     K: Eq + Hash + Clone + Copy,
//     T: Eq + Hash + Clone + Copy {
    
//         row: Rc<Row<Cell<K, T>>>,
//         pub value: T
// }

// impl<K, T> Cell<K, T> where 
//     K: Eq + Hash + Clone + Copy,
//     T: Eq + Hash + Clone + Copy {
//     pub fn new(row: Rc<Row<Cell<K, T>>>, value: T) -> Self {
//         Self {
//             row,
//             value
//         }
//     }
// }

// // impl<T> Cell<T> {
// //     pub fn new(row: Rc<Row<Cell<u16>>>, value: T) -> Self {
// //         // row.add_cell(&Self);
// //         Self {
// //             // row,
// //             value
// //         }
// //     }

// //     // pub fn add_to_row(&self) {
// //     //     self.row.add_cell(Rc::new(self));
// //     // }
// // }

// impl<K, T> SelectRow for Cell<K, T> where 
//     K: Eq + Hash + Clone + Copy,
//     T: Eq + Hash + Clone + Copy {
//     type RowItem = K;
//     type Item = T;
//     fn select_row(&self) -> Rc<Row<Cell<K, T>>> {
//         self.row.clone()
//     }
// }