use std::rc::{ Rc };
use crate::types::dataframe::row::Row;

pub trait SelectRow {
    type Output;
    fn select_row(&self) -> Self::Output;
}

#[derive(Debug)]
pub struct Cell<T> {
    row: Rc<Row<Self>>,
    value: T
}

impl<T> Cell<T> {
    pub fn new(row: Rc<Row<Self>>, value: T) -> Self {
        Self {
            row,
            value
        }
    }
}

impl<T> SelectRow for Cell<T> {
    type Output = Rc<Row<Cell<T>>>;
    fn select_row(&self) -> Self::Output {
        self.row.clone()
    }
}