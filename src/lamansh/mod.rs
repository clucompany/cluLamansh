
/*!Methods of recording and reading the protocol */


pub mod iter;
pub mod build;
pub mod sized;

use lamansh::iter::cluLamanshIntoIter;
use std::convert::TryFrom;
use std::marker::PhantomData;
use self::sized::LamanshSized;
use lamansh::iter::cluLamanshIter;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub struct cluLamansh<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> {
	array: &'a [u8],
	
	value_head_array: &'a [u8],
	value_array: &'a [u8],
	
	_n_count_phantom: PhantomData<NC>,
	_n_phantom: PhantomData<N>,
}


impl<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> cluLamansh<'a, NC, N> {
	#[inline]
	pub fn new(array: &'a [u8]) -> Result< Self, cluLamanshErr > {
		Self::array(array)
	}
	
	pub fn array(array: &'a [u8]) -> Result< Self, cluLamanshErr > {
		
		let ( value_head_array, value_array ) = {			
			let byted_count = NC::byted();

			let count_header = {
				match array.get( .. byted_count) {
					Some(a) => NC::read_usize(a)+1,
					_ => return Err( cluLamanshErr::ErrGetSliceSizeHeadArray ),
				}
			};
			

			let size_value = N::byted();
			let n = byted_count + (size_value * count_header);

			let header_array = match array.get(byted_count .. n) {
				Some(a) => a,
				_ => return Err( cluLamanshErr::ErrGetSliceValueHeadArray ),
			};

			let value_array = match array.get(n .. ) {
				Some(a) => a,
				_ => return Err( cluLamanshErr::ErrGetSliceValueArray ),
			};
			
			( header_array, value_array )
		};
		
		
		
		Ok( 
			Self {
				array: array,
				
				value_head_array: value_head_array,
				value_array: value_array,
				
				_n_count_phantom: PhantomData,
				_n_phantom: PhantomData,
			}
		 )
	}
	
	#[inline]
	pub fn iter<'b>(&'a self) -> cluLamanshIter<'a, 'b, NC, N> {
		cluLamanshIter::new(self)
	}
	#[inline]
	pub fn into_iter(self) -> cluLamanshIntoIter<'a, NC, N> {
		cluLamanshIntoIter::new(self)
	}
}


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum cluLamanshErr {
	ErrEmptyLamansh,
	
	ErrGetSliceSizeHeadArray,
	ErrGetSliceValueHeadArray,
	ErrGetSliceValueArray,
	
	ErrNextSliceValue,
}



/*
impl<'a, N: LamanshSized + 'a, V: LamanshSized + 'a> Into<&'a [u8]> for cluLamansh<'a, N, V> {
	#[inline]
	fn into(self) -> &'a [u8] {
		self.array
	}
}*/

impl<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> TryFrom<&'a [u8]> for cluLamansh<'a, NC, N> {
	type Error = cluLamanshErr;
	#[inline]
	fn try_from(array: &'a [u8]) -> Result< cluLamansh<'a, NC, N>, Self::Error > {
		cluLamansh::array(array)
	}
}