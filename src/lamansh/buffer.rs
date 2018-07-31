
/*! 
Eliminating the re-creation of buffers when creating a binary protocol. 
 */

use std::ops::DerefMut;
use std::ops::Deref;

///Eliminating the re-creation of buffers when creating a binary protocol.
#[derive(Debug)]
pub struct LamanshBuffer(Vec<u8>);

impl LamanshBuffer {
     #[inline]
     pub fn new() -> LamanshBuffer {
          LamanshBuffer(Vec::new())
     }
     #[inline]
     pub fn with_capacity(len: usize) -> LamanshBuffer {
          LamanshBuffer(Vec::with_capacity(len))
     }

     #[inline]
     pub fn array(array: Vec<u8>) -> LamanshBuffer {
          LamanshBuffer(array)
     }

     #[inline]
     pub fn set_len(&mut self, new_len: usize) {
          let capacity = self.0.capacity();
          if capacity > new_len {
               unsafe {
                    self.0.set_len(new_len);
               }
               
          }else

          if capacity < new_len {
               self.0.reserve(new_len - capacity);
               unsafe {
                    self.0.set_len(new_len);
               }
          }
     }
}

impl Deref for LamanshBuffer {
     type Target = Vec<u8>;
     
     #[inline]
     fn deref(&self) -> &Self::Target {
          &self.0
     }
}

impl DerefMut for LamanshBuffer {
     #[inline]
     fn deref_mut(&mut self) -> &mut Self::Target {
          &mut self.0
     }
}


impl Into< Vec<u8> > for LamanshBuffer {
     #[inline]
     fn into(self) -> Vec<u8> {
          self.0
     }
}