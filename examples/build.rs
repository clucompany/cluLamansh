
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
		&b"TEST45"[..], 
		&b"TEST2"[..], 
		&b"TEST3"[..],
		&b""[..],
		&b"1"[..],
	].to_lamansh::<CountElements, ValueLenElements>().unwrap();
	
	println!("LamanshArray {:?}", array);
	let lamash = new_custom_lamansh::<CountElements, ValueLenElements>(array).unwrap();

	let mut iter = lamash.iter();


	assert_eq!(iter.next(), Some( Result::Ok( &b"TEST"[..] ) ));
	assert_eq!(iter.next(), Some( Result::Ok( &b"TEST45"[..] ) ));
	assert_eq!(iter.next(), Some( Result::Ok( &b"TEST2"[..] ) ));
	assert_eq!(iter.next(), Some( Result::Ok( &b"TEST3"[..] ) ));
	assert_eq!(iter.next(), Some( Result::Ok( &b""[..] ) ));
	assert_eq!(iter.next(), Some( Result::Ok( &b"1"[..] ) ));

	assert_eq!(iter.next(), None);
}
