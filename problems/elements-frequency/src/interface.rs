use crossbeam_utils::thread;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::{Mutex, MutexGuard};

/// Examples
/// ```
/// use elements_frequency::interface::frequency_finder;
///
/// fn main () {
///     let myList = [1, 1, -6, 2, 6, 2, 7, 1];
///
///     let myThreads = 6;
///
///     let frequency_map = frequency_finder(&myList, myThreads);
///
///     println!("{:?}", frequency_map);
/// }
///
/// ```
pub fn frequency_finder<T>(list: &[T], nthreads: usize) -> HashMap<T, u32>
    where
        T: Copy + Hash + Eq + Sync + Send + Debug,
{
    let len: usize = list.len();

    let range: usize = len / nthreads;

    let split_index: usize = nthreads * range;

    let residue: usize = len % nthreads;

    // A map that hashes every unique elements to it's frequency
    let map_mtx: Mutex<HashMap<T, u32>> = Mutex::new(HashMap::new());
    let map_ref = &map_mtx;

    thread::scope(|s| {
        for i in 1..=nthreads {
            s.spawn(move |_| {
                let mut map_guard: MutexGuard<HashMap<T, u32>> = map_ref.lock().unwrap();

                for k in (i - 1) * range..i * range {
                    match map_guard.get_mut(&list[k]) {
                        Some(val) => {
                            *val += 1;
                        }

                        None => {
                            map_guard.insert(list[k], 1);
                        }
                    }
                }
            });
        }
    }).unwrap();

    let mut map = Mutex::into_inner(map_mtx).unwrap();

    // In case, there are residual numbers.
    for k in split_index..split_index + residue {
        match map.get_mut(&list[k]) {
            Some(val) => {
                *val += 1;
            }

            None => {
                map.insert(list[k], 1);
            }
        }
    }

    map
}
