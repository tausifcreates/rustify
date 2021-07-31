//! # elements_frequency
//!
//! Finds frequency table of the unique elements present in the list.
//! In the table the elements come in "First come first serve" manner,
//! namingly the order they appear in the list.
//!
//! This lbrary can work with any types that implement `Clone`.
//! So it is expected to work with Strings, slices, integers etc.
//!
//! # Quick Start
//! ```
//! use elements_frequency::interface::{Row, Elements};
//!
//! let list = vec!["hi", "who", "me", "who", "me"];
//! 
//! // Or you can use an array instead:
//! let list = ["hi", "who", "me", "who", "me"];
//! 
//! let mut elements = Elements::new(&list);
//!
//! let table = elements.hash_couple().update_order().result();
//!    
//! println!("{:?}", table);
//!
//! //
//! // [
//! //    Row { element: "hi", frequency: 1 }, 
//! //    Row { element: "who", frequency: 2 }, 
//! //    Row { element: "me", frequency: 2 },
//! // ]   
//! //
//! ```
pub mod interface;
