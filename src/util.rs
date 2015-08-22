extern crate std;

/// Computes the absolute value of the difference 
/// between two number. Supports unsigned types.
/// 
/// # Examples
///
/// ```
/// let diff = absolute_diff(2u8, 5u8);
/// assert_eq!(3, diff);
/// 
/// let inv_diff = absolute_diff(5u8, 2u8);
/// assert_eq!(3, inv_diff);
/// ```
pub fn absolute_diff
<T : std::ops::Sub<Output=T> + std::cmp::Ord>
(a : T, b : T)
-> T {
	if a > b {
		a - b
	} else {
		b - a
	}
}

#[cfg(test)]
mod test {
	use super::*;

	fn test_absolute_diff(a : usize, b : usize, expected : usize) {
		let result = absolute_diff(a, b);
		assert_eq!(expected, result);
	}

	ptest!(test_absolute_diff [
		test_absolute_diff_0_0(0, 0, 0),
		test_absolute_diff_0_1(0, 1, 1),
		test_absolute_diff_1_0(1, 0, 1),
		test_absolute_diff_1_1(1, 1, 0),
		test_absolute_diff_9_3(9, 3, 6),
		test_absolute_diff_3_12(3, 12, 9)		
	]);
}