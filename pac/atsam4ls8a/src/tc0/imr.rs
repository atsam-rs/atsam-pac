#[doc = "Register `IMR%s` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COVFS` reader - Counter Overflow"]
pub type COVFS_R = crate::BitReader<COVFSSELECT_A>;
#[doc = "Counter Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COVFSSELECT_A {
    #[doc = "0: The Counter Overflow Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The Counter Overflow Interrupt is enabled."]
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
#[doc = "Field `LOVRS` reader - Load Overrun"]
pub type LOVRS_R = crate::BitReader<LOVRSSELECT_A>;
#[doc = "Load Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOVRSSELECT_A {
    #[doc = "0: The Load Overrun Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The Load Overrun Interrupt is enabled."]
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
#[doc = "Field `CPAS` reader - RA Compare"]
pub type CPAS_R = crate::BitReader<CPASSELECT_A>;
#[doc = "RA Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPASSELECT_A {
    #[doc = "0: The RA Compare Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The RA Compare Interrupt is enabled."]
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
#[doc = "Field `CPBS` reader - RB Compare"]
pub type CPBS_R = crate::BitReader<CPBSSELECT_A>;
#[doc = "RB Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPBSSELECT_A {
    #[doc = "0: The RB Compare Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The RB Compare Interrupt is enabled."]
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
#[doc = "Field `CPCS` reader - RC Compare"]
pub type CPCS_R = crate::BitReader<CPCSSELECT_A>;
#[doc = "RC Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPCSSELECT_A {
    #[doc = "0: The RC Compare Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The RC Compare Interrupt is enabled."]
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
#[doc = "Field `LDRAS` reader - RA Loading"]
pub type LDRAS_R = crate::BitReader<LDRASSELECT_A>;
#[doc = "RA Loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDRASSELECT_A {
    #[doc = "0: The Load RA Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The Load RA Interrupt is enabled."]
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
#[doc = "Field `LDRBS` reader - RB Loading"]
pub type LDRBS_R = crate::BitReader<LDRBSSELECT_A>;
#[doc = "RB Loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDRBSSELECT_A {
    #[doc = "0: The Load RB Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The Load RB Interrupt is enabled."]
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
#[doc = "Field `ETRGS` reader - External Trigger"]
pub type ETRGS_R = crate::BitReader<ETRGSSELECT_A>;
#[doc = "External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETRGSSELECT_A {
    #[doc = "0: The External Trigger Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The External Trigger Interrupt is enabled."]
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
impl R {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    pub fn covfs(&self) -> COVFS_R {
        COVFS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    pub fn lovrs(&self) -> LOVRS_R {
        LOVRS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    pub fn cpas(&self) -> CPAS_R {
        CPAS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    pub fn cpbs(&self) -> CPBS_R {
        CPBS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    pub fn cpcs(&self) -> CPCS_R {
        CPCS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    pub fn ldras(&self) -> LDRAS_R {
        LDRAS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LDRBS_R {
        LDRBS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn etrgs(&self) -> ETRGS_R {
        ETRGS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register Channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR%s to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
