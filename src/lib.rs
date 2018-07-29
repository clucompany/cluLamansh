#![feature(try_from)]
#![feature(exact_chunks)]
#![feature(test)]

/*!
A binary record of the values stored in the array using the Lamansh protocol.


# Protocol Lamansh
```rust
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
	use lamansh::sized::U8;
	use new_custom_lamansh;
	use lamansh::sized::U64;
	use lamansh::build::ToLamansh;
	use test::Bencher;
	use lamansh::build::ToLamanshErr;

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
			let array = &[&b"test"[..], &b"fds"[..], &b"astaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaastaaaaaaaaaaaaX"[..]].to_lamansh::<U8, U64>().unwrap();
		
			let lamansh = new_custom_lamansh::<U8, U64>(array).unwrap();
			
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

		assert_eq!(array.to_lamansh::<U8, U8>(), Err( ToLamanshErr::ErrEmptyArray ));
	}
}
