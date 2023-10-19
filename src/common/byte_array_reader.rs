pub struct ByteArrayReader<'a> {
    data_ref: &'a [u8],
    data_cur: usize,
}

impl<'a> ByteArrayReader<'a> {
    pub fn new(data: &'a [u8]) -> ByteArrayReader<'a> {
        ByteArrayReader {
            data_ref: data,
            data_cur: 0,
        }
    }

    pub fn read_as<T: Copy>(&mut self) -> T {
        let mut val = std::mem::MaybeUninit::uninit();
        unsafe {
            let val_ptr = val.as_mut_ptr() as *mut u8;
            let data_ptr = self.data_ref[self.data_cur..].as_ptr();
            let len = std::mem::size_of::<T>();
            std::ptr::copy_nonoverlapping(data_ptr, val_ptr, len);
        }

        self.skip(std::mem::size_of::<T>());
        unsafe { val.assume_init() }
    }

    pub fn skip(&mut self, n: usize) {
        self.data_cur += n;
        assert!(self.data_cur <= self.data_ref.len());
    }

    pub fn has_more(&self) -> bool {
        self.data_cur < self.data_ref.len()
    }

    pub fn as_slice(self) -> &'a [u8] {
        &self.data_ref[self.data_cur..]
    }
}
