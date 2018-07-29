
/*! Data processing in the main structure */

use lamansh::cluLamanshErr;
use lamansh::cluLamansh;
use lamansh::sized::LamanshSized;


#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct cluLamanshIter<'a: 'b, 'b, NC: LamanshSized + 'static, N: LamanshSized + 'static> {
	lamansh: &'b cluLamansh<'a, NC, N>,
	
	//header_iter: ExactChunks<'b, u8>,

	header_n: usize,
	value_n: usize,

	//value_iter: DynExactChunks<'b, u8>,
}

impl<'a: 'b, 'b, NC: LamanshSized + 'static, N: LamanshSized + 'static> cluLamanshIter<'a, 'b, NC, N> {
	#[inline]
	pub fn new(lamansh: &'b cluLamansh<'a, NC, N>) -> Self {
		Self {
			lamansh: lamansh,
			
			header_n: 0,
			value_n: 0,

			//header_iter: lamansh.value_head_array.exact_chunks(N::byted()),
			//value_iter: DynExactChunks::from(lamansh.value_array),
			//index_value: 0,
		}
	}
}


impl<'a: 'b, 'b, NC: LamanshSized + 'static, N: LamanshSized + 'static> Iterator for cluLamanshIter<'a, 'b, NC, N> {
	type Item = Result<&'b [u8], cluLamanshErr>;
	
	#[inline]
	fn next(&mut self) -> Option< Self::Item > {
		/*let size_len = match self.header_iter.next() {
			Some(a) => N::read_usize(a),
			_ => return None,
		};*/

		let size_len = {
			let new_n = N::byted() + self.header_n;
			match self.lamansh.value_head_array.get(self.header_n .. new_n) {
				Some(a) => {
					self.header_n = new_n;

					N::read_usize(a)
				},
				_ => return None,
			}
		};

		let new_n = size_len + self.value_n;
		match self.lamansh.value_array.get(self.value_n .. new_n) {
			Some(a) => {
				self.value_n = new_n;
				
				Some( Ok( a ) )
			},
			_ => Some( Err( cluLamanshErr::ErrNextSliceValue ) )
		}
		
		
	}
}



#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct cluLamanshIntoIter<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> {
	lamansh: cluLamansh<'a, NC, N>,
	
	//header_iter: ExactChunks<'b, u8>,

	header_n: usize,
	value_n: usize,

	//value_iter: DynExactChunks<'b, u8>,
}

impl<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> cluLamanshIntoIter<'a, NC, N> {
	#[inline]
	pub fn new(lamansh: cluLamansh<'a, NC, N>) -> Self {
		Self {
			lamansh: lamansh,
			
			header_n: 0,
			value_n: 0,

			//header_iter: lamansh.value_head_array.exact_chunks(N::byted()),
			//value_iter: DynExactChunks::from(lamansh.value_array),
			//index_value: 0,
		}
	}
}


impl<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> Iterator for cluLamanshIntoIter<'a, NC, N> {
	type Item = Result<&'a [u8], cluLamanshErr>;
	
	#[inline]
	fn next(&mut self) -> Option< Self::Item > {
		/*let size_len = match self.header_iter.next() {
			Some(a) => N::read_usize(a),
			_ => return None,
		};*/

		let size_len = {
			let new_n = N::byted() + self.header_n;
			match self.lamansh.value_head_array.get(self.header_n .. new_n) {
				Some(a) => {
					self.header_n = new_n;

					N::read_usize(a)
				},
				_ => return None,
			}
		};

		let new_n = size_len + self.value_n;
		match self.lamansh.value_array.get(self.value_n .. new_n) {
			Some(a) => {
				self.value_n = new_n;
				
				Some( Ok( a ) )
			},
			_ => Some( Err( cluLamanshErr::ErrNextSliceValue ) )
		}
		
		
	}
}
