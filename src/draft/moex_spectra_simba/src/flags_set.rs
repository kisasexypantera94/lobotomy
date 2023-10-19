#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FlagsSet(pub u64);
impl FlagsSet {
    #[inline]
    pub fn new(value: u64) -> Self {
        FlagsSet(value)
    }

    #[inline]
    pub fn clear(&mut self) -> &mut Self {
        self.0 = 0;
        self
    }

    #[inline]
    pub fn get_evening_or_morning_session(&self) -> bool {
        0 != self.0 & (1 << 0)
    }

    #[inline]
    pub fn set_evening_or_morning_session(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 0)
        } else {
            self.0 & !(1 << 0)
        };
        self
    }

    #[inline]
    pub fn get_anonymous_trading(&self) -> bool {
        0 != self.0 & (1 << 4)
    }

    #[inline]
    pub fn set_anonymous_trading(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 4)
        } else {
            self.0 & !(1 << 4)
        };
        self
    }

    #[inline]
    pub fn get_private_trading(&self) -> bool {
        0 != self.0 & (1 << 5)
    }

    #[inline]
    pub fn set_private_trading(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 5)
        } else {
            self.0 & !(1 << 5)
        };
        self
    }

    #[inline]
    pub fn get_day_session(&self) -> bool {
        0 != self.0 & (1 << 6)
    }

    #[inline]
    pub fn set_day_session(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 6)
        } else {
            self.0 & !(1 << 6)
        };
        self
    }

    #[inline]
    pub fn get_multi_leg(&self) -> bool {
        0 != self.0 & (1 << 8)
    }

    #[inline]
    pub fn set_multi_leg(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 8)
        } else {
            self.0 & !(1 << 8)
        };
        self
    }

    #[inline]
    pub fn get_collateral(&self) -> bool {
        0 != self.0 & (1 << 18)
    }

    #[inline]
    pub fn set_collateral(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 18)
        } else {
            self.0 & !(1 << 18)
        };
        self
    }

    #[inline]
    pub fn get_intraday_exercise(&self) -> bool {
        0 != self.0 & (1 << 19)
    }

    #[inline]
    pub fn set_intraday_exercise(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 19)
        } else {
            self.0 & !(1 << 19)
        };
        self
    }
}
impl core::fmt::Debug for FlagsSet {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "FlagsSet[evening_or_morning_session(0)={},anonymous_trading(4)={},private_trading(5)={},day_session(6)={},multi_leg(8)={},collateral(18)={},intraday_exercise(19)={}]",
            self.get_evening_or_morning_session(),self.get_anonymous_trading(),self.get_private_trading(),self.get_day_session(),self.get_multi_leg(),self.get_collateral(),self.get_intraday_exercise(),)
    }
}
