

/*! Arrays with validity checks during a push. It is not recommended to use as a regular array. */

use std::borrow::Cow;
use lamansh::buffer::LamanshBuffer;
use std::ops::Deref;
use std::convert::TryFrom;
use lamansh::build::ToLamanshErr;
use std::marker::PhantomData;
use ::lamansh::sized::LamanshSized;



macro_rules! build_array {
     (slice, $name: ident, $typee: ty ) => {
          #[derive(Debug)]
          pub struct $name<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> {
               array:         Vec<$typee>,

               _phantom:      PhantomData<NC>,
               _phantom_n:    PhantomData<N>,
          }
          impl<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> LamanshArray<'a, $typee> for $name<'a, NC, N> {
               fn update_buffer(&self, vec_result: &mut LamanshBuffer) {
                    let len_elements = self.len();
                    if len_elements == 0 {
                         vec_result.clear();
                         return;
                    }

                    let n_c_sized = NC::byted();
                    let n_sized = N::byted();
                              
                    {
                         let mut len_n = 0;
                         for array in self.iter() {
                              let len = array.len();
                                             
                              len_n += len;
                         }
                                   
                         vec_result.set_len( n_c_sized + (n_sized * len_elements) + len_n );
                    };
                    {
                         let array: &mut [u8] = unsafe{ vec_result.get_unchecked_mut( .. n_c_sized  ) };//WHY UNSAFE? SIZED FIX!
                         NC::write_usize(len_elements-1, array);
                    }
                    let mut n = n_c_sized;
                    let mut n_vh = n + ( n_sized * len_elements );
                    let mut new_n: usize;
                    let mut new_n_vh: usize;
                    {

                         for array in self.iter() {
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
                              }
                         }
                    }
               }

               fn to_lamansh(&self) -> Vec<u8> {
                    let len_elements = self.len();
                    if len_elements == 0 {
                         return vec![];
                    }

                    let n_c_sized = NC::byted();
                    let n_sized = N::byted();
                              
                    let mut vec_result = {
                         let mut len_n = 0;
                         for array in self.iter() {
                              let len = array.len();
                                             
                              len_n += len;
                         }
                                   

                         vec![unsafe{ ::std::mem::uninitialized() }; n_c_sized + (n_sized * len_elements) + len_n]	
                    };
                    {
                         let array = unsafe{ vec_result.get_unchecked_mut( .. n_c_sized  ) };//WHY UNSAFE? SIZED FIX!
                         NC::write_usize(len_elements-1, array);
                    }
                    let mut n = n_c_sized;
                    let mut n_vh = n + ( n_sized * len_elements );
                    let mut new_n: usize;
                    let mut new_n_vh: usize;
                    {

                         for array in self.iter() {
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
                              }
                         }
                    }
                    vec_result
               }
               #[inline]
               fn clear(&mut self) {
                    self.array.clear();
               }
               #[inline]
               fn remove(&mut self, index: usize) -> $typee {
                    self.array.remove(index)
               }
               #[inline]
               fn append(&mut self, array: &mut Vec<$typee>) -> Result< (), ToLamanshErr > {
                    if self.array.len()+array.len() > NC::max_value() {
                         return Err( ToLamanshErr::CountOverflow );
                    }
                    for array in array.iter() {
                         if array.len() > N::max_value() {
                              return Err( ToLamanshErr::ValueOverflow );
                         }
                    }

                    self.array.append(array);

                    Ok( () )
               }

               #[inline]
               fn push<I: Into<$typee>>(&mut self, array: I) -> Result< (), ToLamanshErr > {
                    self.push_array(array.into())
               }
               #[inline]
               fn push_array(&mut self, array: $typee) -> Result< (), ToLamanshErr > {
                    if self.array.len()+1 > NC::max_value() {
                         return Err( ToLamanshErr::CountOverflow );
                    }
                    if array.len() > N::max_value() {
                         return Err( ToLamanshErr::ValueOverflow );
                    }
                    self.array.push(array);

                    Ok( () )
               }
               #[inline]
               unsafe fn as_mut_vec(&mut self) -> &mut Vec<$typee> {
                    &mut self.array
               }
               #[inline]
               fn pop(&mut self) -> Option<$typee> {
                    self.array.pop()
               }
          }

          impl<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> $name<'a, NC, N> {
               #[inline]
               pub fn new() -> Self {
                    Self::empty()
               }
               #[inline]
               pub fn empty() -> Self {
                    Self {
                         array:         Vec::new(),

                         _phantom:      PhantomData,
                         _phantom_n:    PhantomData,
                    }
               }
               #[inline]
               pub fn with_capacity(capacity: usize) -> Self {
                    Self {
                         array:         Vec::with_capacity(capacity),

                         _phantom:      PhantomData,
                         _phantom_n:    PhantomData,
                    }
               }

               pub fn array(array: Vec<$typee>) -> Result< Self, ToLamanshErr > {
                    let len = array.len();
                    if len > 0 {
                         if len > NC::max_value() {
                              return Err( ToLamanshErr::CountOverflow );
                         }
                         let sized = N::max_value();
                         for a in array.iter() {
                              if a.len() > sized {
                                   return Err( ToLamanshErr::ValueOverflow );
                              }
                         }
                    }

                    Ok(
                         Self {
                              array:         array,

                              _phantom:      PhantomData,
                              _phantom_n:    PhantomData,
                         }
                    )
               }

               #[inline]
               pub unsafe fn array_unchecked(array: Vec<$typee>) -> Self {
                    Self {
                         array:         array,

                         _phantom:      PhantomData,
                         _phantom_n:    PhantomData,
                    }
               }               

               
          }


          impl<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> TryFrom<Vec<$typee>> for $name<'a, NC, N> {
               type Error = ToLamanshErr;

               #[inline]
               fn try_from(array: Vec<$typee>) -> Result< Self, Self::Error > {
                    $name::array(array)
               }
          }

          impl<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> Into< Vec<$typee> > for $name<'a, NC, N> {
               #[inline]
               fn into(self) -> Vec<$typee> {
                    self.array
               }
          }

          impl<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> AsRef< Vec<$typee> > for $name<'a, NC, N> {
               #[inline]
               fn as_ref(&self) -> &Vec<$typee> {
                    &self.array
               }
          }
          impl<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> AsRef< $name<'a, NC, N> > for $name<'a, NC, N> {
               #[inline]
               fn as_ref(&self) -> &$name<'a, NC, N> {
                    self
               }
          }

          impl<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> Deref for $name<'a, NC, N> {
               type Target = Vec<$typee>;
               #[inline]
               fn deref(&self) -> &Vec<$typee> {
                    &self.array
               }
          }
     };

     (boxed, $name: ident, $typee: ty ) => {

          #[derive(Debug)]
          pub struct $name<NC: LamanshSized + 'static, N: LamanshSized + 'static> {
               array:         Vec<$typee>,

               _phantom:      PhantomData<NC>,
               _phantom_n:    PhantomData<N>,
          }

          

          impl<NC: LamanshSized + 'static, N: LamanshSized + 'static> $name<NC, N> {
               #[inline]
               pub fn new() -> Self {
                    Self::empty()
               }
               #[inline]
               pub fn empty() -> Self {
                    Self {
                         array:         Vec::new(),

                         _phantom:      PhantomData,
                         _phantom_n:    PhantomData,
                    }
               }
               #[inline]
               pub fn with_capacity(capacity: usize) -> Self {
                    Self {
                         array:         Vec::with_capacity(capacity),

                         _phantom:      PhantomData,
                         _phantom_n:    PhantomData,
                    }
               }

               pub fn array(array: Vec<$typee>) -> Result< Self, ToLamanshErr > {
                    let len = array.len();
                    if len > 0 {
                         if len > NC::max_value() {
                              return Err( ToLamanshErr::CountOverflow );
                         }
                         let sized = N::max_value();
                         for a in array.iter() {
                              if a.len() > sized {
                                   return Err( ToLamanshErr::ValueOverflow );
                              }
                         }
                    }

                    Ok(
                         Self {
                              array:         array,

                              _phantom:      PhantomData,
                              _phantom_n:    PhantomData,
                         }
                    )
               }

               #[inline]
               pub unsafe fn array_unchecked(array: Vec<$typee>) -> Self {
                    Self {
                         array:         array,

                         _phantom:      PhantomData,
                         _phantom_n:    PhantomData,
                    }
               }
          }

          impl<'a, NC: LamanshSized + 'static, N: LamanshSized + 'static> LamanshArray<'a, $typee> for $name<NC, N> {
               fn update_buffer(&self, vec_result: &mut LamanshBuffer) {
                    let len_elements = self.len();
                    if len_elements == 0 {
                         vec_result.clear();
                         return;
                    }

                    let n_c_sized = NC::byted();
                    let n_sized = N::byted();
                              
                    {
                         let mut len_n = 0;
                         for array in self.iter() {
                              let len = array.len();
                                             
                              len_n += len;
                         }
                                   
                         vec_result.set_len( n_c_sized + (n_sized * len_elements) + len_n );
                    };
                    {
                         let array: &mut [u8] = unsafe{ vec_result.get_unchecked_mut( .. n_c_sized  ) };//WHY UNSAFE? SIZED FIX!
                         NC::write_usize(len_elements-1, array);
                    }
                    let mut n = n_c_sized;
                    let mut n_vh = n + ( n_sized * len_elements );
                    let mut new_n: usize;
                    let mut new_n_vh: usize;
                    {

                         for array in self.iter() {
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
                              }
                         }
                    }
               }

               fn to_lamansh(&self) -> Vec<u8> {
                    let len_elements = self.len();
                    if len_elements == 0 {
                         return vec![];
                    }

                    let n_c_sized = NC::byted();
                    let n_sized = N::byted();
                              
                    let mut vec_result = {
                         let mut len_n = 0;
                         for array in self.iter() {
                              let len = array.len();
                                             
                              len_n += len;
                         }
                                   

                         vec![unsafe{ ::std::mem::uninitialized() }; n_c_sized + (n_sized * len_elements) + len_n]	
                    };
                    {
                         let array = unsafe{ vec_result.get_unchecked_mut( .. n_c_sized  ) };//WHY UNSAFE? SIZED FIX!
                         NC::write_usize(len_elements-1, array);
                    }
                    let mut n = n_c_sized;
                    let mut n_vh = n + ( n_sized * len_elements );
                    let mut new_n: usize;
                    let mut new_n_vh: usize;
                    {

                         for array in self.iter() {
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
                              }
                         }
                    }
                    vec_result
               }
               #[inline]
               fn clear(&mut self) {
                    self.array.clear();
               }
               #[inline]
               fn remove(&mut self, index: usize) -> $typee {
                    self.array.remove(index)
               }
               #[inline]
               fn append(&mut self, array: &mut Vec<$typee>) -> Result< (), ToLamanshErr > {
                    if self.array.len()+array.len() > NC::max_value() {
                         return Err( ToLamanshErr::CountOverflow );
                    }
                    for array in array.iter() {
                         if array.len() > N::max_value() {
                              return Err( ToLamanshErr::ValueOverflow );
                         }
                    }

                    self.array.append(array);

                    Ok( () )
               }

               #[inline]
               fn push<I: Into<$typee>>(&mut self, array: I) -> Result< (), ToLamanshErr > {
                    self.push_array(array.into())
               }
               #[inline]
               fn push_array(&mut self, array: $typee) -> Result< (), ToLamanshErr > {
                    if self.array.len()+1 > NC::max_value() {
                         return Err( ToLamanshErr::CountOverflow );
                    }
                    if array.len() > N::max_value() {
                         return Err( ToLamanshErr::ValueOverflow );
                    }
                    self.array.push(array);

                    Ok( () )
               }
               #[inline]
               unsafe fn as_mut_vec(&mut self) -> &mut Vec<$typee> {
                    &mut self.array
               }
               #[inline]
               fn pop(&mut self) -> Option<$typee> {
                    self.array.pop()
               }
          }


          impl<NC: LamanshSized + 'static, N: LamanshSized + 'static> TryFrom<Vec<$typee>> for $name<NC, N> {
               type Error = ToLamanshErr;

               #[inline]
               fn try_from(array: Vec<$typee>) -> Result< Self, Self::Error > {
                    $name::array(array)
               }
          }

          impl<NC: LamanshSized + 'static, N: LamanshSized + 'static> Into< Vec<$typee> > for $name<NC, N> {
               #[inline]
               fn into(self) -> Vec<$typee> {
                    self.array
               }
          }

          impl<NC: LamanshSized + 'static, N: LamanshSized + 'static> AsRef< Vec<$typee> > for $name<NC, N> {
               #[inline]
               fn as_ref(&self) -> &Vec<$typee> {
                    &self.array
               }
          }
          impl<NC: LamanshSized + 'static, N: LamanshSized + 'static> AsRef< $name<NC, N> > for $name<NC, N> {
               #[inline]
               fn as_ref(&self) -> &$name<NC, N> {
                    self
               }
          }

          impl<NC: LamanshSized + 'static, N: LamanshSized + 'static> Deref for $name<NC, N> {
               type Target = Vec<$typee>;
               #[inline]
               fn deref(&self) -> &Vec<$typee> {
                    &self.array
               }
          }
     };
}


///Array generalization
pub trait LamanshArray<'a, Element: 'a>: Deref<Target = Vec<Element>> + AsRef<Self> + AsRef<Vec<Element>> + Into<Vec<Element>> { //+ AsRef<Vec<Self::Element>>
     //type Element;

     fn update_buffer(&self, vec_result: &mut LamanshBuffer);
     fn to_lamansh(&self) -> Vec<u8>;

     fn clear(&mut self);
     fn remove(&mut self, index: usize) -> Element;
     fn pop(&mut self) -> Option<Element>;

     //fn append_lamansh(&mut self, lamansh: &mut LamanshArray<Element>) -> Result< (), ToLamanshErr >;
     fn append(&mut self, array: &mut Vec<Element>) -> Result< (), ToLamanshErr >;
     fn push_array(&mut self, array: Element) -> Result< (), ToLamanshErr >;
     fn push<I: Into<Element>>(&mut self, array: I) -> Result< (), ToLamanshErr >;

     unsafe fn as_mut_vec(&mut self) -> &mut Vec<Element>;
}




build_array!(boxed, LamanshVecArray, Vec<u8> );
build_array!(slice, LamanshCowArray, Cow<'a, Vec<u8>> );
build_array!(slice, LamanshSliceArray, &'a [u8] );