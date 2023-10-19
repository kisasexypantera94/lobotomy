#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MDFlags2Set(pub u64);
impl MDFlags2Set {
    #[inline]
    pub fn new(value: u64) -> Self {
        MDFlags2Set(value)
    }

    #[inline]
    pub fn clear(&mut self) -> &mut Self {
        self.0 = 0;
        self
    }

    #[inline]
    pub fn get_zero(&self) -> bool {
        0 != self.0 & (1 << 0)
    }

    #[inline]
    pub fn set_zero(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 0)
        } else {
            self.0 & !(1 << 0)
        };
        self
    }
}
impl core::fmt::Debug for MDFlags2Set {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "MDFlags2Set[zero(0)={}]",
            self.get_zero(),)
    }
}
