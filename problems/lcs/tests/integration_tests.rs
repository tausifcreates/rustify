#[cfg(test)]
mod tests {
	use lcs::interface::compute_lcs;

	fn setup(slice_a: &[char], slice_b: &[char]) {
		compute_lcs(slice_a, slice_b)
	}

	#[test]
	fn test1() {
		let list_a = ['c', 'a', 'a', 'b', 'c', 'b', 'a', 'b', 'c', 'c'];
		let list_b = ['b', 'c', 'a', 'c', 'c', 'b', 'b', 'a', 'b', 'a'];
		setup(&list_a, &list_b);
	}
}
