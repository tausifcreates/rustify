//! # sorted_rotated
//!
//! Suppose you have a sorted, then rotated list of numbers,
//! It will find the number you searching in `O(logN)` time.
//! 
//! The crate will not fail if the list is not rotated, or it
//! has duplicate items. In this case it will travel the list
//! twice, causing some performance drop. But it will surely 
//! fail if the list is not sorted (in ascending order).
//!
//! The method is simple: perform a binary search to find the pivot,
//! then minimize the range for binary search to find your desired
//! number. If there's no pivot it will simply do binary search on 
//! the list again.
//!
//! In both cases, it returns an `Option` type either with the index
//! of the found number or `None`. So you have to explicitly set up
//! `match` arms to extract the value.
//!
//! # Quick Start
//! ```rust
//! fn main() {
//!     let list: Vec<i32> = vec![5, 6, 1, 2, 3, 4];
//!
//!     // Or an array can be used as well:
//!     let list: [i32; 6] = vec![5, 6, 1, 2, 3, 4];
//!
//!     let desired: i32 = 2;
//!     
//!     let mut element = Elements::new(list, desired);
//!     
//!     match element.find_pivot().find_desired().result() {
//!         Some(idx) => println!("index: {}", idx),
//!         None => println!("Not found"),
//!     }
//! }
//! ```

pub struct Elements<'list> {
    list: &'list [i32],
    len: usize,
    pivot: Option<usize>,
    desired: i32,
    result: Option<usize>,
}

impl<'list> Elements<'list> {
    /// Returns a new instance of `Elements` struct.
    pub fn new(list: &'list [i32], desired: i32) -> Self {
        Elements {
            list,
            len: list.len(),
            pivot: None,
            desired,
            result: None,
        }
    }

    /// Performs a binary search to find the pivot point.
    pub fn find_pivot(&mut self) -> &mut Self {
        let list: &[i32] = self.list;

        let len: usize = self.len;

        // As the algorithm is `O(logN)`, so we can decide how many turns
        // it will take, by taking the log of N base 2.
        let turns: usize = (len as f32).log(2.0).abs() as usize + 1;

        let mut left: usize = 0;

        let mut right: usize = len - 1;

        for _ in 0..turns {
            let mid: usize = (left + right) / 2;

            if list[mid] > list[mid + 1] {
                self.pivot = Some(mid);
                break;
            }

            // The idea is list[0] is always greater than list[n-1].
            if list[mid] > list[left] {
                left = mid;
            } else if list[mid] < list[left] {
                right = mid;
            }
        }

        self
    }

    /// Again performs a binary search, but this time within a reduced
    /// range.
    pub fn find_desired(&mut self) -> &Self {
        let list: &[i32] = self.list;

        let len: usize = self.len;

        let desired: i32 = self.desired;

        let pivot: Option<usize> = self.pivot;

        let mut left: usize = 0;

        let mut right: usize = len - 1;

        if list[left] == desired {
            self.result = Some(left);
            return self;
        } else if list[right] == desired {
            self.result = Some(right);
            return self;
        }

        let mut distance: Option<usize> = None;

        match pivot {
            Some(pivot_idx) => {
                if list[pivot_idx] == desired {
                    self.result = Some(pivot_idx);
                    return self;
                }

                if desired > list[left] {
                    right = pivot_idx;
                    distance = Some(pivot_idx - left);
                } else if desired < list[left] {
                    left = pivot_idx;
                    distance = Some(right - pivot_idx);
                }
            }

            None => {
                distance = Some(right - left);
            }
        };

        let mut turns: usize = 0;

        match distance {
            Some(0) => (),

            _ => {
                turns = (distance.unwrap() as f64).log(2.0).abs() as usize + 1;
            }
        }

        for _ in 0..turns {
            let mid: usize = (left + right) / 2;

            if list[mid] == desired {
                self.result = Some(mid);
                break;
            }

            if list[mid] < desired {
                left = mid;
            } else if list[mid] > desired {
                right = mid;
            }
        }

        self
    }

    /// Returns an `Option` type containing either the index or `None`.
    pub fn result(&self) -> Option<usize> {
        self.result
    }
}
