pub trait SquareRoot<T> {
	fn sqrt(&self) -> T;
}

// For systems < 16 bits or > 64 bits. No support for 128 bit architectures https://github.com/rust-lang/rfcs/issues/1748
#[cfg(not(any(target_pointer_width="16",target_pointer_width="32",target_pointer_width="64")))]
impl SquareRoot<usize> for usize {
	fn sqrt(&self) -> usize {
		panic!("The int_sqrt library does not support the use of usize on your platform. Valid pointer widths: 16, 32, 64");
	}
}

#[cfg(not(any(target_pointer_width="16",target_pointer_width="32",target_pointer_width="64")))]
impl SquareRoot<isize> for isize {
	fn sqrt(&self) -> isize {
		panic!("The int_sqrt library does not support the use of isize your platform. Valid pointer widths: 16, 32, 64");
	}
}

// 64 bit platforms
#[cfg(target_pointer_width="64")]
impl SquareRoot<usize> for usize {
	fn sqrt(&self) -> usize {
		isqrt_64(&(*self as u64)) as usize
	}
}

#[cfg(target_pointer_width="64")]
impl SquareRoot<isize> for isize {
	fn sqrt(&self) -> isize {
		if !is_valid_signed(&(*self as isize)) {
			return isize::min_value();
		}
		isqrt_64(&(*self as u64)) as isize
	}
}

// 32 bit platforms
#[cfg(target_pointer_width="32")]
impl SquareRoot<usize> for usize {
	fn sqrt(&self) -> usize {
		isqrt_32(&(*self as u32)) as usize
	}
}

#[cfg(target_pointer_width="32")]
impl SquareRoot<isize> for isize {
	fn sqrt(&self) -> isize {
		if !is_valid_signed(&(*self as isize)) {
			return isize::min_value();
		}
		isqrt_32(&(*self as u32)) as isize
	}
}

// 16 bit platforms
#[cfg(target_pointer_width="16")]
impl SquareRoot<usize> for usize {
	fn sqrt(&self) -> usize {
		isqrt_16(&(*self as u16)) as usize
	}
}

#[cfg(target_pointer_width="16")]
impl SquareRoot<isize> for isize {
	fn sqrt(&self) -> isize {
		if !is_valid_signed(&(*self as isize)) {
			return isize::min_value();
		}
		isqrt_16(&(*self as u16)) as isize
	}
}

// All other types
impl SquareRoot<u64> for u64 {
	fn sqrt(&self) -> u64 {
		isqrt_64(self)
	}
}


impl SquareRoot<i64> for i64 {
	fn sqrt(&self) -> i64 {
		if !is_valid_signed(&(*self as isize)) {
			return i64::min_value();
		}
		isqrt_64(&(*self as u64)) as i64
	}
}

impl SquareRoot<u32> for u32 {
	fn sqrt(&self) -> u32 {
		isqrt_32(self)
	}
}

impl SquareRoot<i32> for i32 {
	fn sqrt(&self) -> i32 {
		if !is_valid_signed(&(*self as isize)) {
			return i32::min_value();
		}
		isqrt_32(&(*self as u32)) as i32
	}
}

impl SquareRoot<u16> for u16 {
	fn sqrt(&self) -> u16 {
		isqrt_16(self)
	}
}

impl SquareRoot<i16> for i16 {
	fn sqrt(&self) -> i16 {
		if !is_valid_signed(&(*self as isize)) {
			return i16::min_value();
		}
		isqrt_16(&(*self as u16)) as i16
	}
}

impl SquareRoot<u8> for u8 {
	fn sqrt(&self) -> u8 {
		isqrt_8(self)
	}
}

impl SquareRoot<i8> for i8 {
	fn sqrt(&self) -> i8 {
		if !is_valid_signed(&(*self as isize)) {
			return i8::min_value();
		}
		isqrt_8(&(*self as u8)) as i8
	}
}

fn is_valid_signed(n: &isize) -> bool {
	n >= &0isize
}

// Support for i128 once it is stable: https://github.com/rust-lang/rust/issues/35118

// Lots of code reuse with different types. Could implement into a generic in the future. (See https://github.com/rust-lang/rust/issues/44580)
// Computes the square root of a number without casting to a floating point. Infinitely slower than the casting operation
// Implemented using the "fastest" integer square root algorithm on this page: http://www.codecodex.com/wiki/Calculate_an_integer_square_root#C
fn isqrt_64(n: &u64) -> u64 {
	let mut root = 0u64;
	let mut remainder = n.clone();
	let mut place = 2u64.pow(62);

	while place > remainder {
		place = place >> 2;
	}
	while place != 0 {
		if remainder >= root + place {
			remainder -= root + place;
			root += place << 1;
		}
		root = root >> 1;
		place = place >> 2;
	}
	root
}

fn isqrt_32(n: &u32) -> u32 {
	let mut root = 0u32;
	let mut remainder = n.clone();
	let mut place = 2u32.pow(30);

	while place > remainder {
		place = place >> 2;
	}
	while place != 0 {
		if remainder >= root + place {
			remainder -= root + place;
			root += place << 1;
		}
		root = root >> 1;
		place = place >> 2;
	}
	root
}

fn isqrt_16(n: &u16) -> u16 {
	let mut root = 0u16;
	let mut remainder = n.clone();
	let mut place = 2u16.pow(14);

	while place > remainder {
		place = place >> 2;
	}
	while place != 0 {
		if remainder >= root + place {
			remainder -= root + place;
			root += place << 1;
		}
		root = root >> 1;
		place = place >> 2;
	}
	root
}

fn isqrt_8(n: &u8) -> u8 {
	let mut root = 0u8;
	let mut remainder = n.clone();
	let mut place = 2u8.pow(6);

	while place > remainder {
		place = place >> 2;
	}
	while place != 0 {
		if remainder >= root + place {
			remainder -= root + place;
			root += place << 1;
		}
		root = root >> 1;
		place = place >> 2;
	}
	root
}

#[cfg(test)]
mod tests {
	/* 
		There are no tests for isize or usize, as they are simply pointers to 
		u{16,32,64} on the platform. Plus, on non 16, 32, or 64 bit architectures it
		causes a panic. We have _some_ support for these non-standard platforms, 
		all you have to do is cast to an explicit size like i32 and not depend on
		{usize,isize}.sqrt().
	*/

	#[test]
	fn perfect_squares_u64() {
		use isqrt_64;
		let b: u64 = u32::max_value() as u64;
		let a: u64 = b - u16::max_value() as u64;
		for i in a..b {
			assert_eq!(isqrt_64(&(i*i)),i);
		}

		use SquareRoot;
		assert_eq!(16u64.sqrt(),4);
	}
	
	#[test]
	fn perfect_squares_u32() {
		use isqrt_32;
		for i in 0..(u16::max_value() as u32) {
			assert_eq!(isqrt_32(&(i*i)),i);
		}

		use SquareRoot;
		assert_eq!(16u32.sqrt(),4);
	}

	#[test]
	fn perfect_squares_u16() {
		use isqrt_16;
		for i in 0..(u8::max_value() as u16) {
			assert_eq!(isqrt_16(&(i*i)),i);
		}

		use SquareRoot;
		assert_eq!(16u16.sqrt(),4);
	}

	#[test]
	fn perfect_squares_u8() {
		use isqrt_8;
		for i in 0..16 {
			assert_eq!(isqrt_8(&(i*i)),i);
		}

		use SquareRoot;
		assert_eq!(16u8.sqrt(),4);
	}


	#[test]
	fn valid_signed_integer() {
		for i in -256isize..-1isize {
			use is_valid_signed;
			assert_eq!(is_valid_signed(&i),false);
		}

		for i in 0..256isize {
			use is_valid_signed;
			assert_eq!(is_valid_signed(&i),true);
		}

		use SquareRoot;
		assert_eq!(16i64.sqrt(),4);
		assert_eq!(16i32.sqrt(),4);
		assert_eq!(16i16.sqrt(),4);
		assert_eq!(16i8.sqrt(),4);
	}
}
