# cluLamansh

[![Build Status](https://travis-ci.org/clucompany/cluLamansh.svg?branch=master)](https://travis-ci.org/clucompany/cluLamansh)
[![Apache licensed](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/cluLamansh)](https://crates.io/crates/cluLamansh)
[![Documentation](https://docs.rs/cluLamansh/badge.svg)](https://docs.rs/cluLamansh)

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

# Use Buffer
Eliminates buffer redistribution.

```rust
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

# License

Copyright 2018 #UlinProject Денис Котляров

Licensed under the Apache License, Version 2.0
