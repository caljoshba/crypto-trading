use std::collections::HashMap;
use crate::types::dataframe::cell::Cell;

pub struct Column<T> {
    values_map: HashMap<T, Vec<Cell<T>>>,
    values: Vec<T>,

}