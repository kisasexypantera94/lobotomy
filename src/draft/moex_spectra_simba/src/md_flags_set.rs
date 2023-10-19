#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MDFlagsSet(pub u64);
impl MDFlagsSet {
    #[inline]
    pub fn new(value: u64) -> Self {
        MDFlagsSet(value)
    }

    #[inline]
    pub fn clear(&mut self) -> &mut Self {
        self.0 = 0;
        self
    }

    #[inline]
    pub fn get_day(&self) -> bool {
        0 != self.0 & (1 << 0)
    }

    #[inline]
    pub fn set_day(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 0)
        } else {
            self.0 & !(1 << 0)
        };
        self
    }

    #[inline]
    pub fn get_ioc(&self) -> bool {
        0 != self.0 & (1 << 1)
    }

    #[inline]
    pub fn set_ioc(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 1)
        } else {
            self.0 & !(1 << 1)
        };
        self
    }

    #[inline]
    pub fn get_non_quote(&self) -> bool {
        0 != self.0 & (1 << 2)
    }

    #[inline]
    pub fn set_non_quote(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 2)
        } else {
            self.0 & !(1 << 2)
        };
        self
    }

    #[inline]
    pub fn get_end_of_transaction(&self) -> bool {
        0 != self.0 & (1 << 12)
    }

    #[inline]
    pub fn set_end_of_transaction(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 12)
        } else {
            self.0 & !(1 << 12)
        };
        self
    }

    #[inline]
    pub fn get_due_to_cross_cancel(&self) -> bool {
        0 != self.0 & (1 << 13)
    }

    #[inline]
    pub fn set_due_to_cross_cancel(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 13)
        } else {
            self.0 & !(1 << 13)
        };
        self
    }

    #[inline]
    pub fn get_second_leg(&self) -> bool {
        0 != self.0 & (1 << 14)
    }

    #[inline]
    pub fn set_second_leg(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 14)
        } else {
            self.0 & !(1 << 14)
        };
        self
    }

    #[inline]
    pub fn get_fok(&self) -> bool {
        0 != self.0 & (1 << 19)
    }

    #[inline]
    pub fn set_fok(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 19)
        } else {
            self.0 & !(1 << 19)
        };
        self
    }

    #[inline]
    pub fn get_replace(&self) -> bool {
        0 != self.0 & (1 << 20)
    }

    #[inline]
    pub fn set_replace(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 20)
        } else {
            self.0 & !(1 << 20)
        };
        self
    }

    #[inline]
    pub fn get_cancel(&self) -> bool {
        0 != self.0 & (1 << 21)
    }

    #[inline]
    pub fn set_cancel(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 21)
        } else {
            self.0 & !(1 << 21)
        };
        self
    }

    #[inline]
    pub fn get_mass_cancel(&self) -> bool {
        0 != self.0 & (1 << 22)
    }

    #[inline]
    pub fn set_mass_cancel(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 22)
        } else {
            self.0 & !(1 << 22)
        };
        self
    }

    #[inline]
    pub fn get_negotiated(&self) -> bool {
        0 != self.0 & (1 << 26)
    }

    #[inline]
    pub fn set_negotiated(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 26)
        } else {
            self.0 & !(1 << 26)
        };
        self
    }

    #[inline]
    pub fn get_multi_leg(&self) -> bool {
        0 != self.0 & (1 << 27)
    }

    #[inline]
    pub fn set_multi_leg(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 27)
        } else {
            self.0 & !(1 << 27)
        };
        self
    }

    #[inline]
    pub fn get_cross_trade(&self) -> bool {
        0 != self.0 & (1 << 29)
    }

    #[inline]
    pub fn set_cross_trade(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 29)
        } else {
            self.0 & !(1 << 29)
        };
        self
    }

    #[inline]
    pub fn get_cod(&self) -> bool {
        0 != self.0 & (1 << 32)
    }

    #[inline]
    pub fn set_cod(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 32)
        } else {
            self.0 & !(1 << 32)
        };
        self
    }

    #[inline]
    pub fn get_active_side(&self) -> bool {
        0 != self.0 & (1 << 41)
    }

    #[inline]
    pub fn set_active_side(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 41)
        } else {
            self.0 & !(1 << 41)
        };
        self
    }

    #[inline]
    pub fn get_passive_side(&self) -> bool {
        0 != self.0 & (1 << 42)
    }

    #[inline]
    pub fn set_passive_side(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 42)
        } else {
            self.0 & !(1 << 42)
        };
        self
    }

    #[inline]
    pub fn get_synthetic(&self) -> bool {
        0 != self.0 & (1 << 45)
    }

    #[inline]
    pub fn set_synthetic(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 45)
        } else {
            self.0 & !(1 << 45)
        };
        self
    }

    #[inline]
    pub fn get_rfs(&self) -> bool {
        0 != self.0 & (1 << 46)
    }

    #[inline]
    pub fn set_rfs(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 46)
        } else {
            self.0 & !(1 << 46)
        };
        self
    }

    #[inline]
    pub fn get_synthetic_passive(&self) -> bool {
        0 != self.0 & (1 << 57)
    }

    #[inline]
    pub fn set_synthetic_passive(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 57)
        } else {
            self.0 & !(1 << 57)
        };
        self
    }

    #[inline]
    pub fn get_boc(&self) -> bool {
        0 != self.0 & (1 << 60)
    }

    #[inline]
    pub fn set_boc(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 60)
        } else {
            self.0 & !(1 << 60)
        };
        self
    }

    #[inline]
    pub fn get_during_discrete_auction(&self) -> bool {
        0 != self.0 & (1 << 62)
    }

    #[inline]
    pub fn set_during_discrete_auction(&mut self, value: bool) -> &mut Self {
        self.0 = if value {
            self.0 | (1 << 62)
        } else {
            self.0 & !(1 << 62)
        };
        self
    }
}
impl core::fmt::Debug for MDFlagsSet {
    #[inline]
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(fmt, "MDFlagsSet[day(0)={},ioc(1)={},non_quote(2)={},end_of_transaction(12)={},due_to_cross_cancel(13)={},second_leg(14)={},fok(19)={},replace(20)={},cancel(21)={},mass_cancel(22)={},negotiated(26)={},multi_leg(27)={},cross_trade(29)={},cod(32)={},active_side(41)={},passive_side(42)={},synthetic(45)={},rfs(46)={},synthetic_passive(57)={},boc(60)={},during_discrete_auction(62)={}]",
            self.get_day(),self.get_ioc(),self.get_non_quote(),self.get_end_of_transaction(),self.get_due_to_cross_cancel(),self.get_second_leg(),self.get_fok(),self.get_replace(),self.get_cancel(),self.get_mass_cancel(),self.get_negotiated(),self.get_multi_leg(),self.get_cross_trade(),self.get_cod(),self.get_active_side(),self.get_passive_side(),self.get_synthetic(),self.get_rfs(),self.get_synthetic_passive(),self.get_boc(),self.get_during_discrete_auction(),)
    }
}
