pub mod cell;
pub mod column;
pub mod row;


// https://doc.rust-lang.org/std/rc/index.html

// use std::rc::{ Rc, Weak };
// use std::cell::RefCell;

// fn main() {
//     let mut c1: Vec<Rc<Cell>> = Vec::new();
//     let mut c2: Vec<Rc<Cell>> = Vec::new();
//     let mut c3: Vec<Rc<Row>> = Vec::new();
    
//     for i in 1..10 {
//         let row = Rc::new(Row::new(i));
//         let cell1 = Rc::new(Cell::new(Rc::clone(&row), i));
//         let cell2 = Rc::new(Cell::new(Rc::clone(&row), i * 100));
        
//         {
//             let mut cells = row.cells.borrow_mut();
//             cells.push(Rc::downgrade((&cell1)));
//             cells.push(Rc::downgrade((&cell2)));
//         }
        
//         c1.push(cell1);
//         c2.push(cell2);
//         c3.push(row);
        
//     }
    
//     // for gadget_weak in gadget_owner.gadgets.borrow().iter() {

//     //     let gadget = gadget_weak.upgrade().unwrap();
//     //     println!("Gadget {} owned by {}", gadget.id, gadget.owner.name);
//     // }
    
//     println!("{:?}", &c1);
//     println!("{:?}", &c2);
// }
