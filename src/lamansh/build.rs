

/*! Bringing data sets to the protocol */

use lamansh::buffer::LamanshBuffer;
use lamansh::sized::LamanshSized;

///Generalized methods of obtaining a protocol
pub trait ToLamansh {
	///Safe method of creating binary data.
	fn to_lamansh<NC: LamanshSized + 'static, N: LamanshSized + 'static>(self) -> Result<Vec<u8>, ToLamanshErr>;

	///Safe method of creating binary data.
	fn new_lamansh<NC: LamanshSized + 'static, N: LamanshSized + 'static>(&self) -> Result<Vec<u8>, ToLamanshErr>;

	//Unsafe method of creating binary data. Do not check the size of the input data.
	fn new_unsafe_lamansh<NC: LamanshSized + 'static, N: LamanshSized + 'static>(&self) -> Vec<u8>;

	//Unsafe method of creating binary data. Do not check the size of the input data.
	fn to_unsafe_lamansh<NC: LamanshSized + 'static, N: LamanshSized + 'static>(self) -> Vec<u8>;

	///Safe method of creating binary data. Using an external buffer.
	fn update_buffer<NC: LamanshSized + 'static, N: LamanshSized + 'static>(&self, vec_result: &mut LamanshBuffer) -> Result<(), ToLamanshErr>;

	//Unsafe method of creating binary data. Do not check the size of the input data. Using an external buffer.
	fn unsafe_update_buffer<NC: LamanshSized + 'static, N: LamanshSized + 'static>(&self, vec_result: &mut LamanshBuffer);
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ToLamanshErr {
	///Error occurs when empty data is received
	EmptyArray,

	/*ErrWriteCountHeaders,
	ErrWriteHeader,
	ErrWriteValue,

	///Insufficient header recording size
	ErrSmallNSized,

	///Insufficient value recording size
	ErrSmallVSized,*/

	CountOverflow,
	ValueOverflow
}


macro_rules! to_lamansh_build {
	($t:ty) => {
		impl<'a> ToLamansh for $t {

			///Safe method of creating binary data. Using an external buffer.
			fn update_buffer<NC: LamanshSized + 'static, N: LamanshSized + 'static>(&self, vec_result: &mut LamanshBuffer) -> Result<(), ToLamanshErr> {
				let len_elements = self.len();
				if len_elements == 0 {
					return Err( ToLamanshErr::EmptyArray );
				}
				if len_elements > NC::max_value() {
					return Err( ToLamanshErr::CountOverflow );
				}

				
				let n_c_sized = NC::byted();
				let n_sized = N::byted();
				
				{
					let mut len_n = 0;
					for array in self.iter() {
						let len = array.len();
						if len > N::max_value() {
							return Err( ToLamanshErr::ValueOverflow );
						}
							
						len_n += len;
					}
					

					vec_result.set_len(n_c_sized + (n_sized * len_elements) + len_n);
				}

				
				//let vec_result = vec_result.as_mut_slice(); //exclude possible redistribution
				{
					let array = unsafe{ vec_result.get_unchecked_mut( .. n_c_sized  ) };//WHY UNSAFE? SIZED FIX!
					NC::write_usize(len_elements-1, array);
				}

				let mut n = n_c_sized;
				let mut n_vh = n + ( n_sized * len_elements );
				let mut new_n: usize;
				let mut new_n_vh: usize;
				{

					for array in self.into_iter() {
						let array_len = array.len();

						{
							new_n = n + n_sized;
							let mut write_array = unsafe{ vec_result.get_unchecked_mut(n .. new_n) }; //WHY UNSAFE? SIZED FIX!
							n = new_n;
							N::write_usize(array_len, write_array);
						}

						{
							new_n_vh = n_vh + array_len;
							let mut write_array = unsafe{ vec_result.get_unchecked_mut(n_vh .. new_n_vh) }; //WHY UNSAFE? SIZED FIX!
							n_vh = new_n_vh;

							unsafe {
								::std::ptr::copy_nonoverlapping(
									array.as_ptr(), write_array.as_mut_ptr(), array_len);
							}
							//
							//test tests::err_to_empty_test ... ignored
							//test tests::build_u8_u8_lamansh      ... bench:           0 ns/iter (+/- 0)
							//test tests::new_lamansh_big_data     ... bench:         135 ns/iter (+/- 7)
							//test tests::new_run_lamansh_big_data ... bench:         271 ns/iter (+/- 13)
							//test tests::run_build_u8_u8_lamansh  ... bench:           4 ns/iter (+/- 0)
							//test tests::run_lamansh_big_data     ... bench:         142 ns/iter (+/- 9)
							//test tests::run_u8_u8_lamansh        ... bench:          13 ns/iter (+/- 0)


							//write_array.copy_from_slice(array);
							//
							//WHY? UNSAFE? SIZED ARRAY FIX!
							//test tests::err_to_empty_test ... ignored
							//test tests::build_u8_u8_lamansh      ... bench:           0 ns/iter (+/- 0)
							//test tests::new_lamansh_big_data     ... bench:         188 ns/iter (+/- 8)
							//test tests::new_run_lamansh_big_data ... bench:         346 ns/iter (+/- 28)
							//test tests::run_build_u8_u8_lamansh  ... bench:           5 ns/iter (+/- 0)
							//test tests::run_lamansh_big_data     ... bench:         161 ns/iter (+/- 9)
							//test tests::run_u8_u8_lamansh        ... bench:          16 ns/iter (+/- 1)
							//
						}
					}
				}

				Ok( () )
			}


			fn unsafe_update_buffer<NC: LamanshSized + 'static, N: LamanshSized + 'static>(&self, vec_result: &mut LamanshBuffer) {
				let len_elements = self.len();
				if len_elements == 0 {
					vec_result.clear();
				}
				/*if len_elements > NC::max_value() {
					return Err( ToLamanshErr::ErrSmallNSized );
				}*/ //UNSAFE LAMANSH

				
				let n_c_sized = NC::byted();
				let n_sized = N::byted();
				
				{
					let mut len_n = 0;
					for array in self.iter() {
						let len = array.len();
						/*if len > N::max_value() {
							return Err( ToLamanshErr::ErrSmallVSized );
						}*/ //UNSAFE LAMANSH
							
						len_n += len;
					}
					
					vec_result.set_len( n_c_sized + (n_sized * len_elements) + len_n );
				}

				
				//let vec_result = vec_result.as_mut_slice(); //exclude possible redistribution
				{
					let array = unsafe{ vec_result.get_unchecked_mut( .. n_c_sized  ) };//WHY UNSAFE? SIZED FIX!
					NC::write_usize(len_elements-1, array);
				}

				let mut n = n_c_sized;
				let mut n_vh = n + ( n_sized * len_elements );
				let mut new_n: usize;
				let mut new_n_vh: usize;
				{

					for array in self.into_iter() {
						let array_len = array.len();

						{
							new_n = n + n_sized;
							let mut write_array = unsafe{ vec_result.get_unchecked_mut(n .. new_n) }; //WHY UNSAFE? SIZED FIX!
							n = new_n;
							N::write_usize(array_len, write_array);
						}

						{
							new_n_vh = n_vh + array_len;
							let mut write_array = unsafe{ vec_result.get_unchecked_mut(n_vh .. new_n_vh) }; //WHY UNSAFE? SIZED FIX!
							n_vh = new_n_vh;

							unsafe {
								::std::ptr::copy_nonoverlapping(
									array.as_ptr(), write_array.as_mut_ptr(), array_len);
							}
							//
							//test tests::err_to_empty_test ... ignored
							//test tests::build_u8_u8_lamansh      ... bench:           0 ns/iter (+/- 0)
							//test tests::new_lamansh_big_data     ... bench:         135 ns/iter (+/- 7)
							//test tests::new_run_lamansh_big_data ... bench:         271 ns/iter (+/- 13)
							//test tests::run_build_u8_u8_lamansh  ... bench:           4 ns/iter (+/- 0)
							//test tests::run_lamansh_big_data     ... bench:         142 ns/iter (+/- 9)
							//test tests::run_u8_u8_lamansh        ... bench:          13 ns/iter (+/- 0)


							//write_array.copy_from_slice(array);
							//
							//WHY? UNSAFE? SIZED ARRAY FIX!
							//test tests::err_to_empty_test ... ignored
							//test tests::build_u8_u8_lamansh      ... bench:           0 ns/iter (+/- 0)
							//test tests::new_lamansh_big_data     ... bench:         188 ns/iter (+/- 8)
							//test tests::new_run_lamansh_big_data ... bench:         346 ns/iter (+/- 28)
							//test tests::run_build_u8_u8_lamansh  ... bench:           5 ns/iter (+/- 0)
							//test tests::run_lamansh_big_data     ... bench:         161 ns/iter (+/- 9)
							//test tests::run_u8_u8_lamansh        ... bench:          16 ns/iter (+/- 1)
							//
						}
					}
				}
				
			}


			fn to_lamansh<NC: LamanshSized + 'static, N: LamanshSized + 'static>(self) -> Result<Vec<u8>, ToLamanshErr> {
				let len_elements = self.len();
				if len_elements == 0 {
					return Err( ToLamanshErr::EmptyArray );
				}
				if len_elements > NC::max_value() {
					return Err( ToLamanshErr::CountOverflow );
				}

				
				let n_c_sized = NC::byted();
				let n_sized = N::byted();
				
				let mut vec_result = {
					let mut len_n = 0;
					for array in self.iter() {
						let len = array.len();
						if len > N::max_value() {
							return Err( ToLamanshErr::ValueOverflow );
						}
							
						len_n += len;
					}
					

					vec![unsafe{ ::std::mem::uninitialized() }; n_c_sized + (n_sized * len_elements) + len_n]	
				};

				
				//let vec_result = vec_result.as_mut_slice(); //exclude possible redistribution
				{
					let array = unsafe{ vec_result.get_unchecked_mut( .. n_c_sized  ) };//WHY UNSAFE? SIZED FIX!
					NC::write_usize(len_elements-1, array);
				}

				let mut n = n_c_sized;
				let mut n_vh = n + ( n_sized * len_elements );
				let mut new_n: usize;
				let mut new_n_vh: usize;
				{

					for array in self.into_iter() {
						let array_len = array.len();

						{
							new_n = n + n_sized;
							let mut write_array = unsafe{ vec_result.get_unchecked_mut(n .. new_n) }; //WHY UNSAFE? SIZED FIX!
							n = new_n;
							N::write_usize(array_len, write_array);
						}

						{
							new_n_vh = n_vh + array_len;
							let mut write_array = unsafe{ vec_result.get_unchecked_mut(n_vh .. new_n_vh) }; //WHY UNSAFE? SIZED FIX!
							n_vh = new_n_vh;

							unsafe {
								::std::ptr::copy_nonoverlapping(
									array.as_ptr(), write_array.as_mut_ptr(), array_len);
							}
							//
							//test tests::err_to_empty_test ... ignored
							//test tests::build_u8_u8_lamansh      ... bench:           0 ns/iter (+/- 0)
							//test tests::new_lamansh_big_data     ... bench:         135 ns/iter (+/- 7)
							//test tests::new_run_lamansh_big_data ... bench:         271 ns/iter (+/- 13)
							//test tests::run_build_u8_u8_lamansh  ... bench:           4 ns/iter (+/- 0)
							//test tests::run_lamansh_big_data     ... bench:         142 ns/iter (+/- 9)
							//test tests::run_u8_u8_lamansh        ... bench:          13 ns/iter (+/- 0)


							//write_array.copy_from_slice(array);
							//
							//WHY? UNSAFE? SIZED ARRAY FIX!
							//test tests::err_to_empty_test ... ignored
							//test tests::build_u8_u8_lamansh      ... bench:           0 ns/iter (+/- 0)
							//test tests::new_lamansh_big_data     ... bench:         188 ns/iter (+/- 8)
							//test tests::new_run_lamansh_big_data ... bench:         346 ns/iter (+/- 28)
							//test tests::run_build_u8_u8_lamansh  ... bench:           5 ns/iter (+/- 0)
							//test tests::run_lamansh_big_data     ... bench:         161 ns/iter (+/- 9)
							//test tests::run_u8_u8_lamansh        ... bench:          16 ns/iter (+/- 1)
							//
						}
					}
				}
				
				
				Ok( vec_result )
			}
			
			fn new_lamansh<NC: LamanshSized + 'static, N: LamanshSized + 'static>(&self) -> Result<Vec<u8>, ToLamanshErr> {
				let len_elements = self.len();
				if len_elements == 0 {
					return Err( ToLamanshErr::EmptyArray );
				}
				if len_elements > NC::max_value() {
					return Err( ToLamanshErr::CountOverflow );
				}

				
				let n_c_sized = NC::byted();
				let n_sized = N::byted();
				
				let mut vec_result = {
					let mut len_n = 0;
					for array in self.iter() {
						let len = array.len();
						if len > N::max_value() {
							return Err( ToLamanshErr::ValueOverflow );
						}
							
						len_n += len;
					}
					

					vec![unsafe{ ::std::mem::uninitialized() }; n_c_sized + (n_sized * len_elements) + len_n]	
				};

				
				//let vec_result = vec_result.as_mut_slice(); //exclude possible redistribution
				{
					let array = unsafe{ vec_result.get_unchecked_mut( .. n_c_sized  ) };//WHY UNSAFE? SIZED FIX!
					NC::write_usize(len_elements-1, array);
				}

				let mut n = n_c_sized;
				let mut n_vh = n + ( n_sized * len_elements );
				let mut new_n: usize;
				let mut new_n_vh: usize;
				{

					for array in self.into_iter() {
						let array_len = array.len();

						{
							new_n = n + n_sized;
							let mut write_array = unsafe{ vec_result.get_unchecked_mut(n .. new_n) }; //WHY UNSAFE? SIZED FIX!
							n = new_n;
							N::write_usize(array_len, write_array);
						}

						{
							new_n_vh = n_vh + array_len;
							let mut write_array = unsafe{ vec_result.get_unchecked_mut(n_vh .. new_n_vh) }; //WHY UNSAFE? SIZED FIX!
							n_vh = new_n_vh;

							unsafe {
								::std::ptr::copy_nonoverlapping(
									array.as_ptr(), write_array.as_mut_ptr(), array_len);
							}
							//
							//test tests::err_to_empty_test ... ignored
							//test tests::build_u8_u8_lamansh      ... bench:           0 ns/iter (+/- 0)
							//test tests::new_lamansh_big_data     ... bench:         135 ns/iter (+/- 7)
							//test tests::new_run_lamansh_big_data ... bench:         271 ns/iter (+/- 13)
							//test tests::run_build_u8_u8_lamansh  ... bench:           4 ns/iter (+/- 0)
							//test tests::run_lamansh_big_data     ... bench:         142 ns/iter (+/- 9)
							//test tests::run_u8_u8_lamansh        ... bench:          13 ns/iter (+/- 0)


							//write_array.copy_from_slice(array);
							//
							//WHY? UNSAFE? SIZED ARRAY FIX!
							//test tests::err_to_empty_test ... ignored
							//test tests::build_u8_u8_lamansh      ... bench:           0 ns/iter (+/- 0)
							//test tests::new_lamansh_big_data     ... bench:         188 ns/iter (+/- 8)
							//test tests::new_run_lamansh_big_data ... bench:         346 ns/iter (+/- 28)
							//test tests::run_build_u8_u8_lamansh  ... bench:           5 ns/iter (+/- 0)
							//test tests::run_lamansh_big_data     ... bench:         161 ns/iter (+/- 9)
							//test tests::run_u8_u8_lamansh        ... bench:          16 ns/iter (+/- 1)
							//
						}
					}
				}
				
				
				Ok( vec_result )
			}

			fn new_unsafe_lamansh<NC: LamanshSized + 'static, N: LamanshSized + 'static>(&self) -> Vec<u8> {
				let len_elements = self.len();
				if len_elements == 0 {
					return vec![]; //UNSAFE LAMANSH
				}
				/*if len_elements > NC::max_value() {
					return Err( ToLamanshErr::ErrSmallNSized );
				}*/ //UNSAFE LAMANSH

				
				let n_c_sized = NC::byted();
				let n_sized = N::byted();
				
				let mut vec_result = {
					let mut len_n = 0;
					for array in self.iter() {
						let len = array.len();
						/*if len > N::max_value() {
							return Err( ToLamanshErr::ErrSmallVSized );
						}*/ //UNSAFE LAMANSH
							
						len_n += len;
					}
					

					vec![unsafe{ ::std::mem::uninitialized() }; n_c_sized + (n_sized * len_elements) + len_n]	
				};

				
				//let vec_result = vec_result.as_mut_slice(); //exclude possible redistribution
				{
					let array = unsafe{ vec_result.get_unchecked_mut( .. n_c_sized  ) };//WHY UNSAFE? SIZED FIX!
					NC::write_usize(len_elements-1, array);
				}

				let mut n = n_c_sized;
				let mut n_vh = n + ( n_sized * len_elements );
				let mut new_n: usize;
				let mut new_n_vh: usize;
				{

					for array in self.into_iter() {
						let array_len = array.len();

						{
							new_n = n + n_sized;
							let mut write_array = unsafe{ vec_result.get_unchecked_mut(n .. new_n) }; //WHY UNSAFE? SIZED FIX!
							n = new_n;
							N::write_usize(array_len, write_array);
						}

						{
							new_n_vh = n_vh + array_len;
							let mut write_array = unsafe{ vec_result.get_unchecked_mut(n_vh .. new_n_vh) }; //WHY UNSAFE? SIZED FIX!
							n_vh = new_n_vh;

							unsafe {
								::std::ptr::copy_nonoverlapping(
									array.as_ptr(), write_array.as_mut_ptr(), array_len);
							}
							//
							//test tests::err_to_empty_test ... ignored
							//test tests::build_u8_u8_lamansh      ... bench:           0 ns/iter (+/- 0)
							//test tests::new_lamansh_big_data     ... bench:         135 ns/iter (+/- 7)
							//test tests::new_run_lamansh_big_data ... bench:         271 ns/iter (+/- 13)
							//test tests::run_build_u8_u8_lamansh  ... bench:           4 ns/iter (+/- 0)
							//test tests::run_lamansh_big_data     ... bench:         142 ns/iter (+/- 9)
							//test tests::run_u8_u8_lamansh        ... bench:          13 ns/iter (+/- 0)


							//write_array.copy_from_slice(array);
							//
							//WHY? UNSAFE? SIZED ARRAY FIX!
							//test tests::err_to_empty_test ... ignored
							//test tests::build_u8_u8_lamansh      ... bench:           0 ns/iter (+/- 0)
							//test tests::new_lamansh_big_data     ... bench:         188 ns/iter (+/- 8)
							//test tests::new_run_lamansh_big_data ... bench:         346 ns/iter (+/- 28)
							//test tests::run_build_u8_u8_lamansh  ... bench:           5 ns/iter (+/- 0)
							//test tests::run_lamansh_big_data     ... bench:         161 ns/iter (+/- 9)
							//test tests::run_u8_u8_lamansh        ... bench:          16 ns/iter (+/- 1)
							//
						}
					}
				}
				
				
				vec_result
			}
			fn to_unsafe_lamansh<NC: LamanshSized + 'static, N: LamanshSized + 'static>(self) -> Vec<u8> {
				let len_elements = self.len();
				if len_elements == 0 {
					return vec![]; //UNSAFE LAMANSH
				}
				/*if len_elements > NC::max_value() {
					return Err( ToLamanshErr::ErrSmallNSized );
				}*/ //UNSAFE LAMANSH

				
				let n_c_sized = NC::byted();
				let n_sized = N::byted();
				
				let mut vec_result = {
					let mut len_n = 0;
					for array in self.iter() {
						let len = array.len();
						/*if len > N::max_value() {
							return Err( ToLamanshErr::ErrSmallVSized );
						}*/ //UNSAFE LAMANSH
							
						len_n += len;
					}
					

					vec![unsafe{ ::std::mem::uninitialized() }; n_c_sized + (n_sized * len_elements) + len_n]	
				};

				
				//let vec_result = vec_result.as_mut_slice(); //exclude possible redistribution
				{
					let array = unsafe{ vec_result.get_unchecked_mut( .. n_c_sized  ) };//WHY UNSAFE? SIZED FIX!
					NC::write_usize(len_elements-1, array);
				}

				let mut n = n_c_sized;
				let mut n_vh = n + ( n_sized * len_elements );
				let mut new_n: usize;
				let mut new_n_vh: usize;
				{

					for array in self.into_iter() {
						let array_len = array.len();

						{
							new_n = n + n_sized;
							let mut write_array = unsafe{ vec_result.get_unchecked_mut(n .. new_n) }; //WHY UNSAFE? SIZED FIX!
							n = new_n;
							N::write_usize(array_len, write_array);
						}

						{
							new_n_vh = n_vh + array_len;
							let mut write_array = unsafe{ vec_result.get_unchecked_mut(n_vh .. new_n_vh) }; //WHY UNSAFE? SIZED FIX!
							n_vh = new_n_vh;

							unsafe {
								::std::ptr::copy_nonoverlapping(
									array.as_ptr(), write_array.as_mut_ptr(), array_len);
							}
							//
							//test tests::err_to_empty_test ... ignored
							//test tests::build_u8_u8_lamansh      ... bench:           0 ns/iter (+/- 0)
							//test tests::new_lamansh_big_data     ... bench:         135 ns/iter (+/- 7)
							//test tests::new_run_lamansh_big_data ... bench:         271 ns/iter (+/- 13)
							//test tests::run_build_u8_u8_lamansh  ... bench:           4 ns/iter (+/- 0)
							//test tests::run_lamansh_big_data     ... bench:         142 ns/iter (+/- 9)
							//test tests::run_u8_u8_lamansh        ... bench:          13 ns/iter (+/- 0)


							//write_array.copy_from_slice(array);
							//
							//WHY? UNSAFE? SIZED ARRAY FIX!
							//test tests::err_to_empty_test ... ignored
							//test tests::build_u8_u8_lamansh      ... bench:           0 ns/iter (+/- 0)
							//test tests::new_lamansh_big_data     ... bench:         188 ns/iter (+/- 8)
							//test tests::new_run_lamansh_big_data ... bench:         346 ns/iter (+/- 28)
							//test tests::run_build_u8_u8_lamansh  ... bench:           5 ns/iter (+/- 0)
							//test tests::run_lamansh_big_data     ... bench:         161 ns/iter (+/- 9)
							//test tests::run_u8_u8_lamansh        ... bench:          16 ns/iter (+/- 1)
							//
						}
					}
				}
				
				
				vec_result
			}
		}
	};
}


to_lamansh_build!(Vec<Vec<u8>>);
to_lamansh_build!(Vec<&'a [u8]>);

to_lamansh_build!(&'a [&'a [u8]]);
to_lamansh_build!(&'a [Vec<u8>]);


macro_rules! build_array_primitive {
	(a, $($N:expr)+) => {
		$(
			to_lamansh_build!([&'a [u8]; $N]);
		)+
	};
	(a_n, $($N:expr)+) => {
		$(
			to_lamansh_build!(&'a [[u8; $N]]);
		)+
	};
}


build_array_primitive! {a,
	0  1  2  3  4  5  6  7  8  9
	10 11 12 13 14 15 16 17 18 19
	20 21 22 23 24 25 26 27 28 29
	30 /*31 32 33 34 35 36 37 38 39
	40 41 42 43 44 45 46 47 48 49 50
	51 52 53 54 55 56 57 58 59 60 61
	62 63 64 65 66 67 68 69 70 71 72
	73 74 75 76 77 78 79 80 81 82 83
	84 85 86 87 88 89 90 91 92 93 94
	95 96 97 98 99 
	100 101 102 103 104 105 106 107 108 109
	110 111 112 113 114 115 116 117 118 119
	120 121 122 123 124 125 126 127 128 129
	130 131 132 133 134 135 136 137 138 139
	140 141 142 143 144 145 146 147 148 149
	150 151 152 153 154 155 156 157 158 159
	160 161 162 163 164 165 166 167 168 169
	170 171 172 173 174 175 176 177 178 179
	180 181 182 183 184 185 186 187 188 189
	190 191 192 193 194 195 196 197 198 199
	200 201 202 203 204 205 206 207 208 209
	210 211 212 213 214 215 216 217 218 219
	220 221 222 223 224 225 226 227 228 229
	230 231 232 233 234 235 236 237 238 239
	240 241 242 243 244 245 246 247 248 249
	250 251 252 253 254 255*/
}


build_array_primitive! {a_n,
	0  1  2  3  4  5  6  7  8  9
	10 11 12 13 14 15 16 17 18 19
	20 21 22 23 24 25 26 27 28 29
	30 /*31 32 33 34 35 36 37 38 39
	40 41 42 43 44 45 46 47 48 49 50
	51 52 53 54 55 56 57 58 59 60 61
	62 63 64 65 66 67 68 69 70 71 72
	73 74 75 76 77 78 79 80 81 82 83
	84 85 86 87 88 89 90 91 92 93 94
	95 96 97 98 99 
	100 101 102 103 104 105 106 107 108 109
	110 111 112 113 114 115 116 117 118 119
	120 121 122 123 124 125 126 127 128 129
	130 131 132 133 134 135 136 137 138 139
	140 141 142 143 144 145 146 147 148 149
	150 151 152 153 154 155 156 157 158 159
	160 161 162 163 164 165 166 167 168 169
	170 171 172 173 174 175 176 177 178 179
	180 181 182 183 184 185 186 187 188 189
	190 191 192 193 194 195 196 197 198 199
	200 201 202 203 204 205 206 207 208 209
	210 211 212 213 214 215 216 217 218 219
	220 221 222 223 224 225 226 227 228 229
	230 231 232 233 234 235 236 237 238 239
	240 241 242 243 244 245 246 247 248 249
	250 251 252 253 254 255*/
}

