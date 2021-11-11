use num_cpus;
use rayon_core::scope;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{BuildHasherDefault, Hash};
use std::sync::Mutex;
use twox_hash::XxHash64;

#[allow(clippy::drop_copy)]
/// Examples
/// ```
/// use elements_frequency::interface::frequency_finder;
///
/// let myList = [1, 1, -6, 2, 6, 2, 7, 1];
///
/// let frequency_map = frequency_finder(&myList);
///
/// println!("{:?}", frequency_map);
/// ```
pub fn frequency_finder<T>(list: &[T]) -> HashMap<T, u64, BuildHasherDefault<XxHash64>>
where
	T: Copy + Hash + Eq + Sync + Send + Debug,
{
	// Number of threads available on this system
	let logical_cores = num_cpus::get();
	// Define How many elements each thread will handle
	let range: usize = list.len() / logical_cores;
	// If number of elements are not divisible by number of threads,
	// -> after this index all elements will be processed in a separate thread
	let split_index: usize = logical_cores * range;
	// Number of elements to be processed in a separate thread
	let remainder: usize = list.len() % logical_cores;

	// A map that hashes every unique elements to it's frequency
	let map_mtx: Mutex<HashMap<T, u64, BuildHasherDefault<XxHash64>>> =
		Mutex::new(Default::default());
	// Can't pass `map_mtx` into closure as the value will move, so pass a ref instead
	let map_ref = &map_mtx;

	scope(|s| {
		for idx in 1..=logical_cores {
			s.spawn(move |_| {
				let mut map_guard = map_ref.lock().unwrap();

				for item in list.iter().take(idx * range).skip((idx - 1) * range) {
					match map_guard.get_mut(item) {
						// If the item(key) is found, increment it's count(value) by 1
						Some(val) => *val += 1,
						// Otherwise insert the item and set the count to 1, and drop return value
						None => drop(map_guard.insert(*item, 1)),
					}
				}
			});
		}

		// One separate thread for rest of the elements
		s.spawn(move |_| {
			let mut map_guard = map_ref.lock().unwrap();

			for item in list.iter().skip(split_index).take(remainder) {
				match map_guard.get_mut(item) {
					Some(val) => *val += 1,

					None => drop(map_guard.insert(*item, 1)),
				}
			}
		})
	});

	Mutex::into_inner(map_mtx).unwrap()
}
