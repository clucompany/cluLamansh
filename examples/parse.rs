
extern crate cluLamansh;

use cluLamansh::new_custom_lamansh;
use cluLamansh::lamansh::sized::U8;
use cluLamansh::lamansh::sized::U64;

pub fn main() {
	let lamansh = new_custom_lamansh::<U8, U64>(
		&[
		1u8, /*count len_header*/  
		
		0u8,0u8,0u8,0u8,0u8,0u8,0u8,3u8, /* count len value */
		0u8,0u8,0u8,0u8,0u8,0u8,0u8,2u8, /**/  
		
		
		1u8,1u8,1u8,    /* value */
		23u8,55u8
		]
	).unwrap();
	
	let mut iter = lamansh.iter();
	
	assert_eq!(iter.next(), Some( Result::Ok( &[1u8, 1u8, 1u8][..] ) ));
	assert_eq!(iter.next(), Some( Result::Ok( &[23u8, 55u8][..] ) ));
	assert_eq!(iter.next(), None);
	
}
