#[doc = "Reader of register SR%s"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Counter Overflow Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COVFS_A {
    #[doc = "0: No counter overflow has occurred since the last read of the Status Register."]
    _0 = 0,
    #[doc = "1: A counter overflow has occurred since the last read of the Status Register."]
    _1 = 1,
}
impl From<COVFS_A> for bool {
    #[inline(always)]
    fn from(variant: COVFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COVFS`"]
pub type COVFS_R = crate::R<bool, COVFS_A>;
impl COVFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COVFS_A {
        match self.bits {
            false => COVFS_A::_0,
            true => COVFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COVFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COVFS_A::_1
    }
}
#[doc = "Load Overrun Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOVRS_A {
    #[doc = "0: Load overrun has not occurred since the last read of the Status Register or WAVE:1."]
    _0 = 0,
    #[doc = "1: RA or RB have been loaded at least twice without any read of the corresponding register since the last read of the StatusRegister, if WAVE:0."]
    _1 = 1,
}
impl From<LOVRS_A> for bool {
    #[inline(always)]
    fn from(variant: LOVRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOVRS`"]
pub type LOVRS_R = crate::R<bool, LOVRS_A>;
impl LOVRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOVRS_A {
        match self.bits {
            false => LOVRS_A::_0,
            true => LOVRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOVRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOVRS_A::_1
    }
}
#[doc = "RA Compare Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPAS_A {
    #[doc = "0: RA Compare has not occurred since the last read of the Status Register or WAVE:0."]
    _0 = 0,
    #[doc = "1: RA Compare has occurred since the last read of the Status Register, if WAVE:1."]
    _1 = 1,
}
impl From<CPAS_A> for bool {
    #[inline(always)]
    fn from(variant: CPAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPAS`"]
pub type CPAS_R = crate::R<bool, CPAS_A>;
impl CPAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPAS_A {
        match self.bits {
            false => CPAS_A::_0,
            true => CPAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPAS_A::_1
    }
}
#[doc = "RB Compare Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPBS_A {
    #[doc = "0: RB Compare has not occurred since the last read of the Status Register or WAVE:0."]
    _0 = 0,
    #[doc = "1: RB Compare has occurred since the last read of the Status Register, if WAVE:1."]
    _1 = 1,
}
impl From<CPBS_A> for bool {
    #[inline(always)]
    fn from(variant: CPBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPBS`"]
pub type CPBS_R = crate::R<bool, CPBS_A>;
impl CPBS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPBS_A {
        match self.bits {
            false => CPBS_A::_0,
            true => CPBS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPBS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPBS_A::_1
    }
}
#[doc = "RC Compare Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPCS_A {
    #[doc = "0: RC Compare has not occurred since the last read of the Status Register."]
    _0 = 0,
    #[doc = "1: RC Compare has occurred since the last read of the Status Register."]
    _1 = 1,
}
impl From<CPCS_A> for bool {
    #[inline(always)]
    fn from(variant: CPCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPCS`"]
pub type CPCS_R = crate::R<bool, CPCS_A>;
impl CPCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPCS_A {
        match self.bits {
            false => CPCS_A::_0,
            true => CPCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPCS_A::_1
    }
}
#[doc = "RA Loading Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRAS_A {
    #[doc = "0: RA Load has not occurred since the last read of the Status Register or WAVE:1."]
    _0 = 0,
    #[doc = "1: RA Load has occurred since the last read of the Status Register, if WAVE:0."]
    _1 = 1,
}
impl From<LDRAS_A> for bool {
    #[inline(always)]
    fn from(variant: LDRAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LDRAS`"]
pub type LDRAS_R = crate::R<bool, LDRAS_A>;
impl LDRAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRAS_A {
        match self.bits {
            false => LDRAS_A::_0,
            true => LDRAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDRAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDRAS_A::_1
    }
}
#[doc = "RB Loading Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRBS_A {
    #[doc = "0: RB Load has not occurred since the last read of the Status Register or WAVE:1."]
    _0 = 0,
    #[doc = "1: RB Load has occurred since the last read of the Status Register, if WAVE:0."]
    _1 = 1,
}
impl From<LDRBS_A> for bool {
    #[inline(always)]
    fn from(variant: LDRBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LDRBS`"]
pub type LDRBS_R = crate::R<bool, LDRBS_A>;
impl LDRBS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRBS_A {
        match self.bits {
            false => LDRBS_A::_0,
            true => LDRBS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDRBS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDRBS_A::_1
    }
}
#[doc = "External Trigger Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETRGS_A {
    #[doc = "0: External trigger has not occurred since the last read of the Status Register."]
    _0 = 0,
    #[doc = "1: External trigger has occurred since the last read of the Status Register."]
    _1 = 1,
}
impl From<ETRGS_A> for bool {
    #[inline(always)]
    fn from(variant: ETRGS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETRGS`"]
pub type ETRGS_R = crate::R<bool, ETRGS_A>;
impl ETRGS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETRGS_A {
        match self.bits {
            false => ETRGS_A::_0,
            true => ETRGS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ETRGS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ETRGS_A::_1
    }
}
#[doc = "Clock Enabling Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSTA_A {
    #[doc = "0: Clock is disabled."]
    _0 = 0,
    #[doc = "1: Clock is enabled."]
    _1 = 1,
}
impl From<CLKSTA_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSTA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKSTA`"]
pub type CLKSTA_R = crate::R<bool, CLKSTA_A>;
impl CLKSTA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSTA_A {
        match self.bits {
            false => CLKSTA_A::_0,
            true => CLKSTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKSTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKSTA_A::_1
    }
}
#[doc = "TIOA Mirror\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTIOA_A {
    #[doc = "0: TIOA is low. If WAVE:0, this means that TIOA pin is low. If WAVE:1, this means that TIOA is driven low."]
    _0 = 0,
    #[doc = "1: TIOA is high. If WAVE:0, this means that TIOA pin is high. If WAVE:1, this means that TIOA is driven high."]
    _1 = 1,
}
impl From<MTIOA_A> for bool {
    #[inline(always)]
    fn from(variant: MTIOA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MTIOA`"]
pub type MTIOA_R = crate::R<bool, MTIOA_A>;
impl MTIOA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTIOA_A {
        match self.bits {
            false => MTIOA_A::_0,
            true => MTIOA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTIOA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTIOA_A::_1
    }
}
#[doc = "TIOB Mirror\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTIOB_A {
    #[doc = "0: TIOB is low. If WAVE:0, this means that TIOB pin is low. If WAVE:1, this means that TIOB is driven low."]
    _0 = 0,
    #[doc = "1: TIOB is high. If WAVE:0, this means that TIOB pin is high. If WAVE:1, this means that TIOB is driven high."]
    _1 = 1,
}
impl From<MTIOB_A> for bool {
    #[inline(always)]
    fn from(variant: MTIOB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MTIOB`"]
pub type MTIOB_R = crate::R<bool, MTIOB_A>;
impl MTIOB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTIOB_A {
        match self.bits {
            false => MTIOB_A::_0,
            true => MTIOB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTIOB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTIOB_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Counter Overflow Status"]
    #[inline(always)]
    pub fn covfs(&self) -> COVFS_R {
        COVFS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Load Overrun Status"]
    #[inline(always)]
    pub fn lovrs(&self) -> LOVRS_R {
        LOVRS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RA Compare Status"]
    #[inline(always)]
    pub fn cpas(&self) -> CPAS_R {
        CPAS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RB Compare Status"]
    #[inline(always)]
    pub fn cpbs(&self) -> CPBS_R {
        CPBS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RC Compare Status"]
    #[inline(always)]
    pub fn cpcs(&self) -> CPCS_R {
        CPCS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RA Loading Status"]
    #[inline(always)]
    pub fn ldras(&self) -> LDRAS_R {
        LDRAS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RB Loading Status"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LDRBS_R {
        LDRBS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Trigger Status"]
    #[inline(always)]
    pub fn etrgs(&self) -> ETRGS_R {
        ETRGS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Clock Enabling Status"]
    #[inline(always)]
    pub fn clksta(&self) -> CLKSTA_R {
        CLKSTA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIOA Mirror"]
    #[inline(always)]
    pub fn mtioa(&self) -> MTIOA_R {
        MTIOA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIOB Mirror"]
    #[inline(always)]
    pub fn mtiob(&self) -> MTIOB_R {
        MTIOB_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
