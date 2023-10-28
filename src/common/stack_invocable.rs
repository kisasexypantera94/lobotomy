use std::mem;
use std::ptr;

pub struct StackInvocable<const SIZE: usize> {
    storage: [u8; SIZE],
    invoke_fn: fn(*mut u8),
    drop_fn: fn(*mut u8),
}

impl<const SIZE: usize> StackInvocable<SIZE> {
    pub fn new<F: FnMut()>(func: F) -> Self {
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

    pub fn invoke(&self) {
        (self.invoke_fn)(self.storage.as_ptr() as *mut u8);
    }
}

impl<const SIZE: usize> Drop for StackInvocable<SIZE> {
    fn drop(&mut self) {
        (self.drop_fn)(self.storage.as_ptr() as *mut u8);
    }
}

#[test]
fn test_stack_invocable() {
    let mut val = 0;

    {
        struct Foo<'a> {
            x: &'a mut i64,
        }

        impl<'a> Drop for Foo<'a> {
            fn drop(&mut self) {
                *self.x += 1;
            }
        }

        let foo = Foo { x: &mut val };
        let invocable = StackInvocable::<2048>::new(move || {
            *foo.x += 1;
        });

        invocable.invoke();
    }

    assert_eq!(val, 2);
}
