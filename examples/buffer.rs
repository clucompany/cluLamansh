
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
