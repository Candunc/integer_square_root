extern crate integer_square_root;
use integer_square_root::SquareRoot;

#[test]
fn tests() {
	// The accuracy of the sqrt is tested in lib.rs, so this simply ensures the
	// exports work as intended.

	assert_eq!(16usize.sqrt(),4);
	assert_eq!(16u64.sqrt(),4);
	assert_eq!(16u32.sqrt(),4);
	assert_eq!(16u16.sqrt(),4);
	assert_eq!(16u8.sqrt(),4);

	assert_eq!((-16isize).sqrt(),isize::min_value());
	assert_eq!((-16i64).sqrt(),i64::min_value());
	assert_eq!((-16i32).sqrt(),i32::min_value());
	assert_eq!((-16i16).sqrt(),i16::min_value());
	assert_eq!((-16i8).sqrt(),i8::min_value());

	assert_eq!(16isize.sqrt(),4);
	assert_eq!(16i64.sqrt(),4);
	assert_eq!(16i32.sqrt(),4);
	assert_eq!(16i16.sqrt(),4);
	assert_eq!(16i8.sqrt(),4);
}