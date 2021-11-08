#[cfg(test)]
mod tests {
	use between_us::interface::find_distance;

	fn setup(list: &[u32], test_against: Option<usize>) {
		let result = find_distance(list);
		assert_eq!(test_against, result);
	}

	#[test]
	fn test1() {
		let list = [5, 3, 7, 1, 6, 8, 4];
		let test_against: Option<usize> = Some(5);
		setup(&list, test_against);
	}

	#[test]
	fn test2() {
		let list = [34, 8, 10, 3, 2, 80, 30, 33, 1];
		let test_against: Option<usize> = Some(6);
		setup(&list, test_against);
	}

	#[test]
	fn test3() {
		let list = [9, 2, 3, 4, 5, 6, 7, 8, 18, 0];
		let test_against: Option<usize> = Some(8);
		setup(&list, test_against);
	}

	#[test]
	fn test4() {
		let list = [1, 2, 3, 4, 5, 6];
		let test_against: Option<usize> = Some(5);
		setup(&list, test_against);
	}

	#[test]
	fn test5() {
		let list = [6, 5, 4, 3, 2, 1];
		let test_against: Option<usize> = None;
		setup(&list, test_against);
	}

	#[test]
	fn test6() {
		let list = [20, 22, 23, 25, 27, 21, 17, 16, 13, 10, 7, 5, 6, 7, 8];
		let test_against: Option<usize> = Some(5);
		setup(&list, test_against);
	}

	#[test]
	fn test7() {
		let list = [7, 5, 4, 2, 1, 3, 6, 9, 11, 15, 17, 19, 18, 16, 14];
		let test_against: Option<usize> = Some(14);
		setup(&list, test_against);
	}

	#[test]
	fn test8() {
		let list = [2,1];
		let test_against: Option<usize> = None;
		setup(&list, test_against);
	}

	#[test]
	fn test9() {
		let list = [2,2];
		let test_against: Option<usize> = None;
		setup(&list, test_against);
	}

	#[test]
	fn test10() {
		let list = [2,3];
		let test_against: Option<usize> = Some(1);
		setup(&list, test_against);
	}
}
