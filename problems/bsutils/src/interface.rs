pub fn fnl<T>(slice: &[T], given: T) -> (Option<usize>, Option<usize>)
where
    T: Copy + PartialOrd,
{
    let (first_occur_idx, last_occur_idx): (Option<usize>, Option<usize>);
    first_occur_idx = find_first_idx(slice, given);

    match first_occur_idx {
        Some(idx) => {
            last_occur_idx = Some(find_last_idx(&slice[idx..], given).unwrap() + idx);
        }
        None => last_occur_idx = None,
    }

    (first_occur_idx, last_occur_idx)
}

pub fn find_first_idx<F>(slice: &[F], given: F) -> Option<usize>
where
    F: PartialOrd,
{
    let slice_len: usize = slice.len();

    let mut left_idx: usize = 0;
    let mut right_idx: usize = slice_len - 1;
    let mut mid_idx: usize = (left_idx + right_idx) / 2;

    let mut first_occur_idx: Option<usize> = None;
    let turns: usize = (slice_len as f64).log2().floor() as usize;

    for _ in 1..=turns {
        if slice[mid_idx] >= given {
            right_idx = mid_idx;
            mid_idx = (left_idx + right_idx) / 2;
        } else {
            if slice[mid_idx + 1] == given {
                first_occur_idx = Some(mid_idx + 1);
            } else {
                left_idx = mid_idx;
                mid_idx = (left_idx + right_idx) / 2;
            }
        }
    }

    if left_idx == right_idx {
        if slice[left_idx] == given {
            first_occur_idx = Some(left_idx);
        }
    } else if right_idx == left_idx + 1 {
        if slice[left_idx] == given {
            first_occur_idx = Some(left_idx);
        } else if slice[right_idx] == given {
            first_occur_idx = Some(right_idx)
        }
    }

    first_occur_idx
}

pub fn find_last_idx<L>(slice: &[L], given: L) -> Option<usize>
where
    L: PartialOrd,
{
    let slice_len: usize = slice.len();

    let mut left_idx: usize = 0;
    let mut right_idx: usize = slice_len - 1;
    let mut mid_idx: usize = (left_idx + right_idx) / 2;

    let mut last_occur_idx: Option<usize> = None;

    let turns: usize = (slice_len as f64).log2().floor() as usize;

    for _ in 1..=turns {
        if slice[mid_idx] > given {
            if slice[mid_idx - 1] == given {
                last_occur_idx = Some(mid_idx - 1);
            } else {
                right_idx = mid_idx;
                mid_idx = (left_idx + right_idx) / 2;
            }
        } else {
            left_idx = mid_idx;
            mid_idx = (left_idx + right_idx) / 2;
        }
    }

    if left_idx == right_idx {
        last_occur_idx = Some(left_idx);
    } else if right_idx == left_idx + 1 {
        if slice[right_idx] == given {
            last_occur_idx = Some(right_idx);
        } else {
            last_occur_idx = Some(left_idx);
        }
    }

    last_occur_idx
}
