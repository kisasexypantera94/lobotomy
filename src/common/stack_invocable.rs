use std::mem;
use std::ptr;

pub struct StackInvocable<const SIZE: usize> {
    storage: [u8; SIZE],
    invoke_fn: fn(*mut u8),
    drop_fn: fn(*mut u8),
}

impl<const SIZE: usize> StackInvocable<SIZE> {
    #[inline(always)]
    pub fn new<F: FnMut() + Send + 'static>(func: F) -> Self {
        let mut storage = [0; SIZE];
        more_asserts::assert_le!(mem::size_of::<F>(), mem::size_of_val(&storage)); // TODO: fail at compile time

        let invoke_fn: fn(*mut u8) = |data| {
            let func_ptr: *mut F = data as *mut F;
            let mut func: F = unsafe { ptr::read(func_ptr) };
            func();
            mem::forget(func); // Prevent dropping the moved value
        };

        let drop_fn: fn(*mut u8) = |data| {
            let func_ptr: *mut F = data as *mut F;
            let _: F = unsafe { ptr::read(func_ptr) };
        };

        let func_ptr = storage.as_mut_ptr() as *mut F;
        unsafe {
            ptr::write(func_ptr, func);
        }

        StackInvocable {
            storage,
            invoke_fn,
            drop_fn,
        }
    }

    #[inline(always)]
    pub fn invoke(&mut self) {
        (self.invoke_fn)(self.storage.as_ptr() as *mut u8);
    }
}

impl<const SIZE: usize> Drop for StackInvocable<SIZE> {
    #[inline(always)]
    fn drop(&mut self) {
        (self.drop_fn)(self.storage.as_ptr() as *mut u8);
    }
}
