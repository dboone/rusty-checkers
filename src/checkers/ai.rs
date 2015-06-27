// This enum describes an offset direction and magnitude.
enum TileOffset {
	Positive(usize),
	Negative(usize)
}

// offsets a value based on the given offset direction and magnitude 
fn offset_value
(start_value : usize, value_offset : &TileOffset)
-> usize {
	match *value_offset {
	TileOffset::Negative(magnitude) => start_value - magnitude,
	TileOffset::Positive(magnitude) => start_value + magnitude,
	}
}

// Offsets a tile based on the given offset direction
// and magnitude in the row and column dimensions.
//
// Returns a 2 element tuple, where the first element
// is the offset row, and the second element is the
// offset column.
fn offset_tile
(start_row : usize,
		start_col : usize,
		col_offset : &TileOffset,
		row_offset : &TileOffset)
-> (usize, usize) {
	(offset_value(start_row, row_offset),
			offset_value(start_col, col_offset))
}

// checks if a value is in the given range using the given offset
//TODO maybe a range object can be used here as a param instead
// of the start and max values
fn is_offset_value_in_range
(start_value : usize,
		max_value : usize,
		value_offset : &TileOffset)
-> bool {
	match *value_offset {
	TileOffset::Negative(magnitude) => start_value >= magnitude,
	TileOffset::Positive(magnitude) => start_value <= max_value - magnitude
	}
}

// checks if a tile on the board can be reached when
// moving from one position on the board to another
fn is_tile_offset_on_bounds
(board : &super::Board,
		start_row : usize,
		start_col : usize,
		row_offset : &TileOffset,
		col_offset : &TileOffset)
-> bool {
	let max_row_index = board.number_rows() - 1;
	let max_col_index = board.number_columns() - 1;
	
	is_offset_value_in_range(start_row, max_row_index, row_offset)
	&& is_offset_value_in_range(start_col, max_col_index, col_offset)
}



//TODO could use some parameterized tests here
//TODO may be able to remove these tests onece the public API for this module is in place

#[test]
#[cfg(test)]
fn offset_value_negative_offset_1() {
	let offset = TileOffset::Negative(2);
	let result = offset_value(5, &offset);
	assert_eq!(3, result);
}

#[test]
#[cfg(test)]
fn offset_value_negative_offset_2() {
	let offset = TileOffset::Negative(1);
	let result = offset_value(3, &offset);
	assert_eq!(2, result);
}

#[test]
#[cfg(test)]
fn offset_value_positive_offset_1() {
	let offset = TileOffset::Positive(1);
	let result = offset_value(0, &offset);
	assert_eq!(1, result);
}

#[test]
#[cfg(test)]
fn offset_value_positive_offset_2() {
	let offset = TileOffset::Positive(2);
	let result = offset_value(5, &offset);
	assert_eq!(7, result);
}

#[test]
#[cfg(test)]
fn is_offset_value_in_range_positive_zero_offset() {
	let offset = TileOffset::Positive(0);
	assert!(is_offset_value_in_range(0, 7, &offset));
}

#[test]
#[cfg(test)]
fn is_offset_value_in_range_negative_zero_offset() {
	let offset = TileOffset::Negative(0);
	assert!(is_offset_value_in_range(0, 7, &offset));
}

#[test]
#[cfg(test)]
fn is_offset_value_in_range_valid_positive_offset() {
	let offset = TileOffset::Positive(2);
	assert!(is_offset_value_in_range(5, 7, &offset));
}

#[test]
#[cfg(test)]
fn is_offset_value_in_range_invalid_positive_offset() {
	let offset = TileOffset::Positive(2);
	assert!(!is_offset_value_in_range(6, 7, &offset));
}

#[test]
#[cfg(test)]
fn is_offset_value_in_range_valid_negative_offset() {
	let offset = TileOffset::Negative(2);
	assert!(is_offset_value_in_range(2, 7, &offset));
}

#[test]
#[cfg(test)]
fn is_offset_value_in_range_invalid_negative_offset() {
	let offset = TileOffset::Negative(2);
	assert!(!is_offset_value_in_range(1, 7, &offset));
}