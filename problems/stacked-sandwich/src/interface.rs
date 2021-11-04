use std::cmp::Ordering;

#[derive(Debug)]
pub struct OccurancePattern {
	pub row: usize,
	pub leftbound_col: usize,
	pub rightbound_col: usize,
}


/// ```
/// use stacked_sandwich::interface::equal_is_bigger;
///
/// let sandwich = [1, 2, 4, 5,  3, 4, 4, 5,  5, 5, 8, 9];
/// let rows = 3;
/// let cols = 4;
/// let search_element = 5;
/// let result = equal_is_bigger(&sandwich, cols, rows, search_element);
/// ```
pub fn equal_is_bigger(
	sandwich: &[i32],
	cols: usize,
	rows: usize,
	search_element: i32,
) -> Vec<OccurancePattern> {
	let mut at_idx = cols - 1;
	let mut occurance_pattern: Vec<OccurancePattern> = Vec::new();

	while at_idx < rows * cols {
		match search_element.cmp(&sandwich[at_idx]) {
			// go one idx back in that row
			Ordering::Less => at_idx -= 1,

			// jump one column
			Ordering::Greater => at_idx += cols,

			// Store this idx and jump one column
			Ordering::Equal => {
				let idx_row: usize = at_idx / cols;
				let idx_col: usize = at_idx - idx_row * cols;

				let idx_pos = OccurancePattern {
					row: idx_row,
					leftbound_col: idx_col,
					rightbound_col: idx_col,
				};

				occurance_pattern.push(idx_pos);

				at_idx += cols;
			}
		}
	}

	march_left(sandwich, cols, occurance_pattern)
}

pub fn march_left(
	sandwich: &[i32],
	cols: usize,
	mut occurance_pattern: Vec<OccurancePattern>,
) -> Vec<OccurancePattern> {
	// A tracker to find leftmost bound for the searching
	// Initialize it to 1st reported occurance position
	let mut left_marching_idx: usize =
		occurance_pattern[0].row * cols + occurance_pattern[0].rightbound_col;

	for op_idx in occurance_pattern.iter_mut() {
		// Starting idx for each row
		let row_start_idx: usize = op_idx.row * cols;

		let prev_leftbound_row: usize = left_marching_idx / cols;
		let prev_leftbound_col: usize = left_marching_idx - prev_leftbound_row * cols;

		if op_idx.rightbound_col < prev_leftbound_col {
			// Set march point on this rows rightbound column
			left_marching_idx = row_start_idx + op_idx.rightbound_col;
		} else {
			// Align to previous rows leftbound column & set march point
			left_marching_idx = row_start_idx + prev_leftbound_col;
		}

		// Marching backwards until you meet the starting point of this row
		// When the previous element is smaller, quit
		while row_start_idx < left_marching_idx
			&& sandwich[left_marching_idx] == sandwich[left_marching_idx - 1]
		{
			// Otherwise, retreat one idx in this row
			left_marching_idx -= 1;
		}

		// When marching completes, calculate leftbound position for this row
		let leftbound_row: usize = left_marching_idx / cols;
		let leftbound_col: usize = left_marching_idx - leftbound_row * cols;

		// Update Ocuurance Pattern
		// Set leftbound column as the start of range
		op_idx.leftbound_col = leftbound_col;
	}

	occurance_pattern
}
