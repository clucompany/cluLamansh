
/*! Data types for a name or value lamansh. */

extern crate byteorder;
use self::byteorder::BigEndian;
use self::byteorder::ByteOrder;

use std::fmt::Debug;
use std::hash::Hash;

pub type LamanshNameCountSized = LamanshU8;
pub type LamanshNameSized = LamanshU8;
pub type LamanshValueSized = LamanshU64;

///255 len
pub type U8 = LamanshU8;
///65535 len
pub type U16 = LamanshU16;
///4294967295 len
pub type U32 = LamanshU32;
///18446744073709551615 len
pub type U64 = LamanshU64;

//pub type U128 = LamanshU128;


///255 len
pub type LamanshN8 = LamanshU8;
///65535 len
pub type LamanshN16 = LamanshU16;

pub type LamanshN24 = LamanshU24;

///4294967295 len
pub type LamanshN32 = LamanshU32;
///18446744073709551615 len
pub type LamanshN64 = LamanshU64;



pub type LamanshV8 = LamanshU8;
pub type LamanshV16 = LamanshU16;
pub type LamanshV24 = LamanshU24;
pub type LamanshV32 = LamanshU32;
pub type LamanshV64 = LamanshU64;
//pub type LamanshV128 = LamanshU128;



#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LamanshU8 {}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LamanshU16 {}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LamanshU24 {}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LamanshU32 {}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LamanshU64 {}
/*#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum LamanshU128 {}*/

pub trait LamanshSized: Clone + Debug + Eq + Hash + Ord + PartialEq + PartialOrd {
	type WriteType;
	
	#[inline(always)]
	fn bits() -> usize;
	
	#[inline(always)]
	fn byted() -> usize;
	
	#[inline(always)]
	fn read<'a>(array: &'a [u8]) -> Self::WriteType;

	#[inline(always)]
	fn read_usize<'a>(array: &'a [u8]) -> usize;

	
	#[inline(always)]
	fn write<'a>(num: Self::WriteType, array: &'a mut [u8]);

	#[inline(always)]
	fn write_usize<'a>(num: usize, array: &'a mut [u8]);
	
	#[inline(always)]
	fn max_value() -> usize;
}

impl LamanshSized for LamanshU8 {
	type WriteType = u8;
	
	#[inline(always)]
	fn bits() -> usize {
		8
	}
	#[inline(always)]
	fn byted() -> usize {
		Self::bits()/8
	}
	
	
	#[inline(always)]
	fn read<'a>(array: &'a [u8]) -> Self::WriteType {
		array[0]
	}
	#[inline(always)]
	fn read_usize<'a>(array: &'a [u8]) -> usize {
		array[0] as usize
	}
	
	#[inline(always)]
	fn write<'a>(num: Self::WriteType, array: &'a mut [u8]) {
		//BigEndian::write_u8(array, num)
		array[0] = num;
	}
	#[inline(always)]
	fn write_usize<'a>(num: usize, array: &'a mut [u8]) {
		//BigEndian::write_u8(array, num)
		array[0] = num as u8;
	}
	
	#[inline(always)]
	fn max_value() -> usize {
		u8::max_value() as usize
	}
}

impl LamanshSized for LamanshU16 {
	type WriteType = u16;
	
	#[inline(always)]
	fn bits() -> usize {
		16
	}
	#[inline(always)]
	fn byted() -> usize {
		Self::bits()/8
	}
	
	#[inline(always)]
	fn read<'a>(array: &'a [u8]) -> Self::WriteType {
		BigEndian::read_u16(array)
	}
	#[inline(always)]
	fn read_usize<'a>(array: &'a [u8]) -> usize {
		BigEndian::read_u16(array) as usize
	}
	
	#[inline(always)]
	fn write<'a>(num: Self::WriteType, array: &'a mut [u8]) {
		BigEndian::write_u16(array, num)
	}
	#[inline(always)]
	fn write_usize<'a>(num: usize, array: &'a mut [u8]) {
		BigEndian::write_u16(array, num as Self::WriteType)
	}
	
	#[inline(always)]
	fn max_value() -> usize {
		u16::max_value() as usize
	}
}

impl LamanshSized for LamanshU24 {
	type WriteType = u32;
	
	#[inline(always)]
	fn bits() -> usize {
		24
	}
	#[inline(always)]
	fn byted() -> usize {
		Self::bits()/8
	}
	
	#[inline(always)]
	fn read<'a>(array: &'a [u8]) -> Self::WriteType {
		BigEndian::read_u24(array)
	}
	#[inline(always)]
	fn read_usize<'a>(array: &'a [u8]) -> usize {
		BigEndian::read_u24(array) as usize
	}
	
	#[inline(always)]
	fn write<'a>(num: Self::WriteType, array: &'a mut [u8]) {
		BigEndian::write_u24(array, num)
	}
	#[inline(always)]
	fn write_usize<'a>(num: usize, array: &'a mut [u8]) {
		BigEndian::write_u24(array, num as Self::WriteType)
	}
	
	#[inline(always)]
	fn max_value() -> usize {
		u32::max_value() as usize
	}
}

impl LamanshSized for LamanshU32 {
	type WriteType = u32;
	
	#[inline(always)]
	fn bits() -> usize {
		32
	}
	#[inline(always)]
	fn byted() -> usize {
		Self::bits()/8
	}
	
	#[inline(always)]
	fn read<'a>(array: &'a [u8]) -> Self::WriteType {
		BigEndian::read_u32(array)
	}
	#[inline(always)]
	fn read_usize<'a>(array: &'a [u8]) -> usize {
		BigEndian::read_u32(array) as usize
	}
	
	#[inline(always)]
	fn write<'a>(num: Self::WriteType, array: &'a mut [u8]) {
		BigEndian::write_u32(array, num)
	}
	#[inline(always)]
	fn write_usize<'a>(num: usize, array: &'a mut [u8]) {
		BigEndian::write_u32(array, num as Self::WriteType)
	}
	
	#[inline(always)]
	fn max_value() -> usize {
		u32::max_value() as usize
	}
}

impl LamanshSized for LamanshU64 {
	type WriteType = u64;
	
	#[inline(always)]
	fn bits() -> usize {
		64
	}
	
	#[inline(always)]
	fn byted() -> usize {
		Self::bits()/8
	}
	
	#[inline(always)]
	fn read<'a>(array: &'a [u8]) -> Self::WriteType {
		BigEndian::read_u64(array)
	}
	#[inline(always)]
	fn read_usize<'a>(array: &'a [u8]) -> usize {
		BigEndian::read_u64(array) as usize
	}
	
	#[inline(always)]
	fn write<'a>(num: Self::WriteType, array: &'a mut [u8]) {
		BigEndian::write_u64(array, num)
	}
	#[inline(always)]
	fn write_usize<'a>(num: usize, array: &'a mut [u8]) {
		BigEndian::write_u64(array, num as Self::WriteType)
	}
	
	#[inline(always)]
	fn max_value() -> usize {
		u64::max_value() as usize
	}
}

