#[cfg(test)]
mod tests {
	use lisa::interface::find_lis;

	fn setup(slice: &[i32], test_against: &[i32]) {
		let lis = find_lis(slice);
		for (idx, item) in lis.iter().enumerate() {
			assert_eq!(*item, test_against[idx]);
		}
	}

	#[test]
	fn test1() {
		let slice = [0, 8, 4, 12, 2, 10, 6, 14, 1, 9, 5, 13, 3, 11, 7, 15];
		let test_against = [0, 2, 6, 9, 11, 15];
		setup(&slice, &test_against);
	}

	#[test]
	fn test2() {
		let slice = [10, 22, 9, 33, 21, 50, 41, 60, 80];
		let test_against = [10, 22, 33, 41, 60, 80];
		setup(&slice, &test_against);
	}

	#[test]
	fn test3() {
		let slice = [3, 10, 2, 1, 20];
		let test_against = [3, 10, 20];
		setup(&slice, &test_against);
	}

	#[test]
	fn test4() {
		let slice = [3, 2];
		let test_against =[2];
		setup(&slice, &test_against);
	}

	#[test]
	fn test5() {
		let slice = [50, 3, 10, 7, 40, 80];
		let test_against = [3, 7, 40, 80];
		setup(&slice, &test_against);
	}
}
