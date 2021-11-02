#[cfg(test)]
mod tests {
	use stacked_sandwich::interface::equal_is_bigger;
	
	fn setup(
		sandwich: &[i32],
		cols: usize,
		rows: usize,
		search_element: i32,
		expect: &[(usize, usize, usize)],
	) {
		let result = equal_is_bigger(sandwich, cols, rows, search_element);
		println!("{:?}", result);
		assert_eq!(expect.len(), result.len());
		for i in 0..result.len() {
			assert_eq!(expect[i].0, result[i].row);
			assert_eq!(expect[i].1, result[i].col_start);
			assert_eq!(expect[i].2, result[i].col_end);
		}
	}

	#[test]
	fn all_pos() {
		let sandwich = [1, 7, 7, 7, 7, 2, 7, 7, 7, 8, 7, 7, 10, 11, 13];
		let rows = 3;
		let cols = 5;
		let search_element = 7;
		let expect = [(0, 1, 4), (1, 1, 3), (2, 0, 1)];
		setup(&sandwich, cols, rows, search_element, &expect);
	}

	#[test]
	fn test2() {
		let sandwich = [1, 2, 2, 5, 5, 3, 5, 5, 6, 9];
		let rows = 2;
		let cols = 5;
		let search_element = 5;
		let expect = [(0, 3, 4), (1, 1, 2)];
		setup(&sandwich, cols, rows, search_element, &expect);
	}

	#[test]
	fn test3() {
		let sandwich = [1, 2, 5, 5, 5, 7, 7, 7, 5, 8, 9, 9];
		let rows = 3;
		let cols = 4;
		let search_element = 5;
		let expect = [(0, 2, 3), (1, 0, 0), (2, 0, 0)];
		setup(&sandwich, cols, rows, search_element, &expect);
	}

	#[test]
	fn test4() {
		let sandwich = [1, 5, 5, 5, 5, 2, 5, 5, 5, 6, 5, 6, 7, 7, 7];
		let rows = 3;
		let cols = 5;
		let search_element = 5;
		let expect = [(0, 1, 4), (1, 1, 3), (2, 0, 0)];
		setup(&sandwich, cols, rows, search_element, &expect);
	}
}
