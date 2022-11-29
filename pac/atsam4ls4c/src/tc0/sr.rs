#[doc = "Register `SR%s` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COVFS` reader - Counter Overflow Status"]
pub type COVFS_R = crate::BitReader<COVFSSELECT_A>;
#[doc = "Counter Overflow Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COVFSSELECT_A {
    #[doc = "0: No counter overflow has occurred since the last read of the Status Register."]
    _0 = 0,
    #[doc = "1: A counter overflow has occurred since the last read of the Status Register."]
    _1 = 1,
}
impl From<COVFSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: COVFSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl COVFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COVFSSELECT_A {
        match self.bits {
            false => COVFSSELECT_A::_0,
            true => COVFSSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COVFSSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COVFSSELECT_A::_1
    }
}
#[doc = "Field `LOVRS` reader - Load Overrun Status"]
pub type LOVRS_R = crate::BitReader<LOVRSSELECT_A>;
#[doc = "Load Overrun Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOVRSSELECT_A {
    #[doc = "0: Load overrun has not occurred since the last read of the Status Register or WAVE:1."]
    _0 = 0,
    #[doc = "1: RA or RB have been loaded at least twice without any read of the corresponding register since the last read of the StatusRegister, if WAVE:0."]
    _1 = 1,
}
impl From<LOVRSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LOVRSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LOVRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOVRSSELECT_A {
        match self.bits {
            false => LOVRSSELECT_A::_0,
            true => LOVRSSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOVRSSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOVRSSELECT_A::_1
    }
}
#[doc = "Field `CPAS` reader - RA Compare Status"]
pub type CPAS_R = crate::BitReader<CPASSELECT_A>;
#[doc = "RA Compare Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPASSELECT_A {
    #[doc = "0: RA Compare has not occurred since the last read of the Status Register or WAVE:0."]
    _0 = 0,
    #[doc = "1: RA Compare has occurred since the last read of the Status Register, if WAVE:1."]
    _1 = 1,
}
impl From<CPASSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPASSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPASSELECT_A {
        match self.bits {
            false => CPASSELECT_A::_0,
            true => CPASSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPASSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPASSELECT_A::_1
    }
}
#[doc = "Field `CPBS` reader - RB Compare Status"]
pub type CPBS_R = crate::BitReader<CPBSSELECT_A>;
#[doc = "RB Compare Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPBSSELECT_A {
    #[doc = "0: RB Compare has not occurred since the last read of the Status Register or WAVE:0."]
    _0 = 0,
    #[doc = "1: RB Compare has occurred since the last read of the Status Register, if WAVE:1."]
    _1 = 1,
}
impl From<CPBSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPBSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPBSSELECT_A {
        match self.bits {
            false => CPBSSELECT_A::_0,
            true => CPBSSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPBSSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPBSSELECT_A::_1
    }
}
#[doc = "Field `CPCS` reader - RC Compare Status"]
pub type CPCS_R = crate::BitReader<CPCSSELECT_A>;
#[doc = "RC Compare Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPCSSELECT_A {
    #[doc = "0: RC Compare has not occurred since the last read of the Status Register."]
    _0 = 0,
    #[doc = "1: RC Compare has occurred since the last read of the Status Register."]
    _1 = 1,
}
impl From<CPCSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPCSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPCSSELECT_A {
        match self.bits {
            false => CPCSSELECT_A::_0,
            true => CPCSSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPCSSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPCSSELECT_A::_1
    }
}
#[doc = "Field `LDRAS` reader - RA Loading Status"]
pub type LDRAS_R = crate::BitReader<LDRASSELECT_A>;
#[doc = "RA Loading Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDRASSELECT_A {
    #[doc = "0: RA Load has not occurred since the last read of the Status Register or WAVE:1."]
    _0 = 0,
    #[doc = "1: RA Load has occurred since the last read of the Status Register, if WAVE:0."]
    _1 = 1,
}
impl From<LDRASSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LDRASSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LDRAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRASSELECT_A {
        match self.bits {
            false => LDRASSELECT_A::_0,
            true => LDRASSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDRASSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDRASSELECT_A::_1
    }
}
#[doc = "Field `LDRBS` reader - RB Loading Status"]
pub type LDRBS_R = crate::BitReader<LDRBSSELECT_A>;
#[doc = "RB Loading Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDRBSSELECT_A {
    #[doc = "0: RB Load has not occurred since the last read of the Status Register or WAVE:1."]
    _0 = 0,
    #[doc = "1: RB Load has occurred since the last read of the Status Register, if WAVE:0."]
    _1 = 1,
}
impl From<LDRBSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LDRBSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LDRBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRBSSELECT_A {
        match self.bits {
            false => LDRBSSELECT_A::_0,
            true => LDRBSSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDRBSSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDRBSSELECT_A::_1
    }
}
#[doc = "Field `ETRGS` reader - External Trigger Status"]
pub type ETRGS_R = crate::BitReader<ETRGSSELECT_A>;
#[doc = "External Trigger Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETRGSSELECT_A {
    #[doc = "0: External trigger has not occurred since the last read of the Status Register."]
    _0 = 0,
    #[doc = "1: External trigger has occurred since the last read of the Status Register."]
    _1 = 1,
}
impl From<ETRGSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ETRGSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ETRGS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETRGSSELECT_A {
        match self.bits {
            false => ETRGSSELECT_A::_0,
            true => ETRGSSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ETRGSSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ETRGSSELECT_A::_1
    }
}
#[doc = "Field `CLKSTA` reader - Clock Enabling Status"]
pub type CLKSTA_R = crate::BitReader<CLKSTASELECT_A>;
#[doc = "Clock Enabling Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKSTASELECT_A {
    #[doc = "0: Clock is disabled."]
    _0 = 0,
    #[doc = "1: Clock is enabled."]
    _1 = 1,
}
impl From<CLKSTASELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSTASELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKSTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSTASELECT_A {
        match self.bits {
            false => CLKSTASELECT_A::_0,
            true => CLKSTASELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKSTASELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKSTASELECT_A::_1
    }
}
#[doc = "Field `MTIOA` reader - TIOA Mirror"]
pub type MTIOA_R = crate::BitReader<MTIOASELECT_A>;
#[doc = "TIOA Mirror\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTIOASELECT_A {
    #[doc = "0: TIOA is low. If WAVE:0, this means that TIOA pin is low. If WAVE:1, this means that TIOA is driven low."]
    _0 = 0,
    #[doc = "1: TIOA is high. If WAVE:0, this means that TIOA pin is high. If WAVE:1, this means that TIOA is driven high."]
    _1 = 1,
}
impl From<MTIOASELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MTIOASELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MTIOA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTIOASELECT_A {
        match self.bits {
            false => MTIOASELECT_A::_0,
            true => MTIOASELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTIOASELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTIOASELECT_A::_1
    }
}
#[doc = "Field `MTIOB` reader - TIOB Mirror"]
pub type MTIOB_R = crate::BitReader<MTIOBSELECT_A>;
#[doc = "TIOB Mirror\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MTIOBSELECT_A {
    #[doc = "0: TIOB is low. If WAVE:0, this means that TIOB pin is low. If WAVE:1, this means that TIOB is driven low."]
    _0 = 0,
    #[doc = "1: TIOB is high. If WAVE:0, this means that TIOB pin is high. If WAVE:1, this means that TIOB is driven high."]
    _1 = 1,
}
impl From<MTIOBSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MTIOBSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MTIOB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTIOBSELECT_A {
        match self.bits {
            false => MTIOBSELECT_A::_0,
            true => MTIOBSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MTIOBSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MTIOBSELECT_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Counter Overflow Status"]
    #[inline(always)]
    pub fn covfs(&self) -> COVFS_R {
        COVFS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Overrun Status"]
    #[inline(always)]
    pub fn lovrs(&self) -> LOVRS_R {
        LOVRS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RA Compare Status"]
    #[inline(always)]
    pub fn cpas(&self) -> CPAS_R {
        CPAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RB Compare Status"]
    #[inline(always)]
    pub fn cpbs(&self) -> CPBS_R {
        CPBS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RC Compare Status"]
    #[inline(always)]
    pub fn cpcs(&self) -> CPCS_R {
        CPCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RA Loading Status"]
    #[inline(always)]
    pub fn ldras(&self) -> LDRAS_R {
        LDRAS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RB Loading Status"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LDRBS_R {
        LDRBS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Trigger Status"]
    #[inline(always)]
    pub fn etrgs(&self) -> ETRGS_R {
        ETRGS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Clock Enabling Status"]
    #[inline(always)]
    pub fn clksta(&self) -> CLKSTA_R {
        CLKSTA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIOA Mirror"]
    #[inline(always)]
    pub fn mtioa(&self) -> MTIOA_R {
        MTIOA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIOB Mirror"]
    #[inline(always)]
    pub fn mtiob(&self) -> MTIOB_R {
        MTIOB_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[doc = "Status Register Channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR%s to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
