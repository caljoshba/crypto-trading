use std::rc::{ Rc, Weak };
use std::cell::RefCell;
// use std::hash::Hash;
use crate::types::dataframe::cell::{
    SelectRow,
    // Cell
};



// // #[derive(Debug)]
pub struct Row<T: SelectRow> {
    index: u16,
    cells: RefCell<Vec<Weak<T>>>
}

impl<T> Row<T>
where T: SelectRow {
    pub fn new(index: u16) -> Self {
        Self {
            index,
            cells: RefCell::new(vec![])
        }
    }

    pub fn add_cell(&mut self, cell: &Rc<T>) where T: SelectRow {
        self.cells.borrow_mut().push(Rc::downgrade(cell));
    }
}