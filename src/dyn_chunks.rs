

#[derive(Debug, Clone)]
pub struct DynExactChunks<'a, T:'a>{
    array: &'a [T],

    start_n: usize,
}


impl<'a, T> DynExactChunks<'a, T> {
    #[inline]
    pub fn array(slice: &'a [T]) -> Self {
        DynExactChunks { 
            array: slice,

            start_n: 0
        }
    }
    #[inline]
    pub fn next_usize(&mut self, usize: usize) -> Option<&'a [T]> {
        let new_n = self.start_n + usize;
        if let Some(array) = self.array.get(self.start_n .. new_n) {
            self.start_n = new_n;
            return Some( array );
        }
        None
    }

    #[inline]
    pub fn remainder(self) -> Option<&'a [T]> {
        self.array.get(self.start_n .. )
    }
}

impl<'a, T: 'a> From<&'a [T]> for DynExactChunks<'a, T> {
    #[inline]
    fn from(array: &'a [T]) -> DynExactChunks<'a, T> {
        DynExactChunks::array(array)
    }
}


#[derive(Debug)]
pub struct MutDynExactChunks<'a, T:'a>(&'a mut [T]);


impl<'a, T> MutDynExactChunks<'a, T> {
    #[inline]
    pub fn array(slice: &'a mut [T]) -> Self {
        MutDynExactChunks(slice)
    }
    #[inline]
    pub fn next_usize(&mut self, usize: usize) -> Option<&mut [T]> {
        if self.0.len() < usize {
            None
        } else {
            let tmp = ::std::mem::replace(&mut self.0, unsafe{ ::std::mem::uninitialized() });
            let (fst, snd) = tmp.split_at_mut(usize);
            self.0 = snd;
            Some(fst)
        }
    }

    #[inline]
    pub fn remainder(self) -> &'a mut [T] {
        self.0
    }

    /*#[inline]
    pub fn remainder_no_into(&mut self) -> Option<&mut [T]> {
        if self.0.len() == 0 {
            None
        }else {
            let tmp = ::std::mem::replace(&mut self.0, &mut []);
            
            Some( tmp )
        }
    }*/
}

impl<'a, T: 'a> From<&'a mut [T]> for MutDynExactChunks<'a, T> {
    #[inline]
    fn from(array: &'a mut [T]) -> MutDynExactChunks<'a, T> {
        MutDynExactChunks::array(array)
    }
}
