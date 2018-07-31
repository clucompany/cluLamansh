#![feature(try_from)]
#![feature(exact_chunks)]
#![feature(test)]

/*!
A binary record of the values stored in the array using the Lamansh protocol.


# Protocol Lamansh
```rust
extern crate cluLamansh;

use cluLamansh::lamansh::sized::U8;
use cluLamansh::lamansh::sized::U64;
use cluLamansh::new_custom_lamansh;
let lamansh = new_custom_lamansh::<U8, U64>( // PROTOCOL SIZED U8 - 255 elements, U64 - 64 sized value len
	&[/* 2 bin value, [1, 1, 1,] and [23, 55] */

		1u8,					/* count len_header, 8 bit */  
		
		0u8,0u8,0u8,0u8,0u8,0u8,0u8,3u8,	/* count len value, 64 bit */
		0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8,  
		
		
		1u8,1u8,1u8,				/* value, max 64 bit value */
		23u8,55u8
	
	]
).unwrap();
```

# Use
```rust
extern crate cluLamansh;

use cluLamansh::new_custom_lamansh;
use cluLamansh::lamansh::build::ToLamansh;
use cluLamansh::lamansh::sized::U64;
use cluLamansh::lamansh::sized::U8;

type CountElements = U8;
type ValueLenElements = U64;

pub fn main() {
	let array = &[
		&b"TEST"[..], 
		&b""[..],
		&b"my_test"[..],
	].to_lamansh::<CountElements, ValueLenElements>().unwrap();
	
	let lamash = new_custom_lamansh::<CountElements, ValueLenElements>(array).unwrap();

	let mut iter = lamash.iter();


	assert_eq!(iter.next(), Some( Result::Ok( &b"TEST"[..] ) ));
	assert_eq!(iter.next(), Some( Result::Ok( &b""[..] ) ));
	assert_eq!(iter.next(), Some( Result::Ok( &b"my_test"[..] ) ));

	assert_eq!(iter.next(), None);
}
```

# Use Buffer
Eliminates buffer redistribution.
```
extern crate cluLamansh;

use cluLamansh::new_custom_lamansh;
use cluLamansh::lamansh::build::ToLamansh;
use cluLamansh::lamansh::sized::U64;
use cluLamansh::lamansh::sized::U8;
use cluLamansh::lamansh::buffer::LamanshBuffer;

type CountElements = U8;
type ValueLenElements = U64;

pub fn main() {
     let mut buffer = LamanshBuffer::new();

     for a in 99 .. 120 {
          let string_a = a.to_string();

          &[
               &b"TEST"[..], 
               &b"TEST45"[..], 
               &b"TEST2"[..], 
               &b"TEST3"[..],
               &b""[..],
               string_a.as_bytes(),
          ].update_buffer::<CountElements, ValueLenElements>(&mut buffer).unwrap();
          
          let lamash = new_custom_lamansh::<CountElements, ValueLenElements>(&mut buffer).unwrap();

          let mut iter = lamash.iter();


          assert_eq!(iter.next(), Some( Result::Ok( &b"TEST"[..] ) ));
          assert_eq!(iter.next(), Some( Result::Ok( &b"TEST45"[..] ) ));
          assert_eq!(iter.next(), Some( Result::Ok( &b"TEST2"[..] ) ));
          assert_eq!(iter.next(), Some( Result::Ok( &b"TEST3"[..] ) ));
          assert_eq!(iter.next(), Some( Result::Ok( &b""[..] ) ));
          assert_eq!(iter.next(), Some( Result::Ok( string_a.as_bytes() ) ));

          assert_eq!(iter.next(), None);
     }
	
}
```

*/

pub mod lamansh;

use lamansh::sized::LamanshNameCountSized;
use lamansh::sized::LamanshValueSized;
use lamansh::cluLamansh;
use lamansh::cluLamanshErr;

use lamansh::sized::U8;
use lamansh::sized::LamanshSized;

extern crate test;

///Create a handler from the received data. Sized name = default, Sized value = default
#[inline]
pub fn new_lamansh<'a>(array: &'a [u8]) -> Result<cluLamansh<'a, LamanshNameCountSized, LamanshValueSized>, cluLamanshErr> {
	new_custom_lamansh(array)
}

///Create a handler from the received data. Sized name = custom, Sized value = custom
#[inline]
pub fn new_custom_lamansh<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static>(array: &'a [u8]) -> Result<cluLamansh<'a, NC, N>, cluLamanshErr> {
	cluLamansh::new(
		array,
	)
}

///Create a handler from the received data. Sized name = U8 (255 len), Sized value = U8 (255 len)
#[inline]
pub fn new_lamansh_mini<'a>(array: &'a [u8]) -> Result< cluLamansh<'a, U8, U8> , cluLamanshErr> {
	new_custom_lamansh(array)
}


#[cfg(test)]
mod tests {
	extern crate byteorder;
	use self::byteorder::BigEndian;
	use self::byteorder::ByteOrder;

	use lamansh::sized::U8;
	use lamansh::sized::U16;
	use new_custom_lamansh;
	use lamansh::sized::U64;
	use lamansh::build::ToLamansh;
	use test::Bencher;
	use lamansh::build::ToLamanshErr;
	use lamansh::array::LamanshArray;
	use lamansh::buffer::LamanshBuffer;
	use lamansh::array::LamanshVecArray;
	use lamansh::array::LamanshSliceArray;

	#[bench]
	fn build_u8_u8_lamansh(b: &mut Bencher) {
		b.iter(|| {
			let _lamansh = new_custom_lamansh::<U8, U64>(
				&[
				1u8, /**/  
				
				0u8,0u8,0u8,0u8,0u8,0u8,0u8,3u8, 
				0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8, /**/  
				
				
				1u8,1u8,1u8,    
				23u8,55u8
				]
			).unwrap();
			
		});
	}

	#[bench]
	fn run_u8_u8_lamansh(b: &mut Bencher) {
		let lamansh = new_custom_lamansh::<U8, U64>(
			&[
			1u8, /**/  
			
			0u8,0u8,0u8,0u8,0u8,0u8,0u8,3u8, 
			0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8, /**/  
			
			
			1u8,1u8,1u8,    
			23u8,55u8
			]
		).unwrap();
		b.iter(|| {
			/*for array in Lamansh.iter() {
				
			}*/
			let mut iter = lamansh.iter();
			
			assert_eq!(iter.next(), Some( Result::Ok( &[1u8, 1u8, 1u8][..] ) ));
			assert_eq!(iter.next(), Some( Result::Ok( &[23u8, 55u8][..] ) ));
			assert_eq!(iter.next(), None);
		});
	}
	
	#[bench]
	fn run_build_u8_u8_lamansh(b: &mut Bencher) {
		
		b.iter(|| {	
			let lamansh = new_custom_lamansh::<U8, U64>(
				&[
				1u8, /**/  
				
				0u8,0u8,0u8,0u8,0u8,0u8,0u8,3u8, 
				0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8, /**/  
				
				
				1u8,1u8,1u8,    
				23u8,55u8
				]
			).unwrap();
			
			let mut iter = lamansh.iter();
			
			assert_eq!(iter.next(), Some( Result::Ok( &[1u8, 1u8, 1u8][..] ) ));
			assert_eq!(iter.next(), Some( Result::Ok( &[23u8, 55u8][..] ) ));
			assert_eq!(iter.next(), None);
		});
	}
	
	#[bench]
	fn new_lamansh_big_data(b: &mut Bencher) {
		b.iter(|| {	
			let _array = &[
				&b"test"[..], 
				&b"fds"[..], 
				&b"astaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaX"[..]
			].to_lamansh::<U8, U64>().unwrap();
		});
	}
	#[bench]
	fn new_run_lamansh_big_data(b: &mut Bencher) {
		b.iter(|| {	
			let array = &[
				&b"test"[..], &b"fds"[..], &b"astaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaX"[..]
			].to_lamansh::<U8, U64>().unwrap();
		
			let lamansh = new_custom_lamansh::<U8, U64>(array).unwrap();
			
			let mut iter = lamansh.iter();
			assert_eq!(iter.next(), Some( Result::Ok( &b"test"[..] ) ));
			assert_eq!(iter.next(), Some( Result::Ok( &b"fds"[..] ) ));
			assert_eq!(iter.next(), Some( Result::Ok( &b"astaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaX"[..] ) ));
			assert_eq!(iter.next(), None);
		});
	}

	#[bench]
	fn new_run_lamansh_big_data_buffer(b: &mut Bencher) {
		let mut buffer = LamanshBuffer::new();
		b.iter(|| {	
			let array = &[
				&b"test"[..], &b"fds"[..], &b"astaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaX"[..]
			][..];
			array.update_buffer::<U8, U64>(&mut buffer).unwrap();
		
			let lamansh = new_custom_lamansh::<U8, U64>(&buffer).unwrap();
			
			let mut iter = lamansh.iter();
			assert_eq!(iter.next(), Some( Result::Ok( &b"test"[..] ) ));
			assert_eq!(iter.next(), Some( Result::Ok( &b"fds"[..] ) ));
			assert_eq!(iter.next(), Some( Result::Ok( &b"astaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaX"[..] ) ));
			assert_eq!(iter.next(), None);
		});
	}

	#[bench]
	fn run_lamansh_big_data(b: &mut Bencher) {
		let array = &[&b"test"[..], &b"fds"[..], &b"astaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaX"[..]].to_lamansh::<U8, U64>().unwrap();

		b.iter(|| {	
		
			let lamansh = new_custom_lamansh::<U8, U64>(array).unwrap();
			
			let mut iter = lamansh.iter();
			assert_eq!(iter.next(), Some( Result::Ok( &b"test"[..] ) ));
			assert_eq!(iter.next(), Some( Result::Ok( &b"fds"[..] ) ));
			assert_eq!(iter.next(), Some( Result::Ok( &b"astaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaX"[..] ) ));
			assert_eq!(iter.next(), None);
		});
	}

	#[test]
	fn err_to_empty_test() {
		let array: &[&[u8]] = &[];

		assert_eq!(array.to_lamansh::<U8, U8>(), Err( ToLamanshErr::EmptyArray ));
	}

	#[test]
	fn count_overflow() {
		{//u8 - 255
			let vec = vec![[0u8; 0]; (u8::max_value() as usize) +1 ];

			let result = vec.to_lamansh::<U8, U64>();
			assert_eq!(result, Err( ToLamanshErr::CountOverflow ));
		}
		/*{//u16
			let vec = vec![[0u8; 0]; (u16::max_value() as usize) +1 ];

			let result = vec.to_lamansh::<U16, U64>();
			assert_eq!(result, Err( ToLamanshErr::ErrSmallNSized ));
		}
		{//u32
			let vec = vec![[0u8; 0]; (u32::max_value() as usize) +1 ];

			let result = vec.to_lamansh::<U32, U64>();
			assert_eq!(result, Err( ToLamanshErr::ErrSmallNSized ));
		}
			//Why it is painted over? slow execution of tests
		*/
	}

	#[test]
	fn value_overflow() {
		{//u8 - 255
			let vec = vec![
				vec![0u8; (u8::max_value() as usize) +1]
			
				; 5 
			];

			let result = vec.to_lamansh::<U16, U8>();
			assert_eq!(result, Err( ToLamanshErr::ValueOverflow ));
		}
	}

	#[test]
	fn array_value_overflow() {
		let mut array = LamanshVecArray::<U8, U8>::new();

		assert_eq!(
			array.push(vec![0u8; (u8::max_value() as usize) +1]), 

			Err( ToLamanshErr::ValueOverflow )
		);
		assert_eq!(
			array.push(vec![0u8; u8::max_value() as usize]), 

			Ok( () )
		);
	}
	#[test]
	fn array_count_overflow() {
		let mut array = LamanshVecArray::<U8, U8>::array (
			
			vec![vec![0u8]; u8::max_value() as usize]
		).unwrap();

		assert_eq!(
			array.push(vec![0u8; u8::max_value() as usize]), 

			Err( ToLamanshErr::CountOverflow )
		);
		let _e = array.pop();
		assert_eq!(
			array.push(vec![0u8; u8::max_value() as usize]), 

			Ok( () )
		);
	}

	#[test]

	fn test_array() {
		let mut array = LamanshSliceArray::<U8, U8>::new();
		
		assert_eq!(	array.push(&b"12"[..]),		Ok(()) );
		assert_eq!(	array.push(&b"12232"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );

		assert_eq!(	
			array.to_lamansh(),

			[
				&b"12"[..],
				&b"12232"[..],
				&b"5463"[..]
			].to_lamansh::<U8, U8>().unwrap()
		);
	}

	#[test]

	fn test_clear_array_and_big_endigian() {
		//WHY TEST?
		//test whether the data of the input array is deleted!
		let mut array = [255, 0, 2, 125];

		BigEndian::write_u32(&mut array, 256);

		assert_eq!( array, [0, 0, 1, 0] );
	}


	#[bench]
	fn bench_build_usebuffer(b: &mut Bencher) {
		let mut array = LamanshSliceArray::<U8, U8>::new();
		
		assert_eq!(	array.push(&b"12"[..]),		Ok(()) );
		assert_eq!(	array.push(&b"12232"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );

		let mut buffer = LamanshBuffer::new();
		
		b.iter(|| {	
			array.update_buffer(&mut buffer);
		});
	}

	#[bench]
	fn bench_build_nousebuffer(b: &mut Bencher) {
		let mut array = LamanshSliceArray::<U8, U8>::new();
		
		assert_eq!(	array.push(&b"12"[..]),		Ok(()) );
		assert_eq!(	array.push(&b"12232"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		assert_eq!(	array.push(&b"5463"[..]),	Ok(()) );
		
		b.iter(|| {	
			array.to_lamansh();
		});
	}

}
