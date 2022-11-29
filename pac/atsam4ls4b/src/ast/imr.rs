#[doc = "Register `IMR` reader"]
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
#[doc = "Field `OVF` reader - Overflow"]
pub type OVF_R = crate::BitReader<OVFSELECT_A>;
#[doc = "Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFSELECT_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<OVFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: OVFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl OVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVFSELECT_A {
        match self.bits {
            false => OVFSELECT_A::_0,
            true => OVFSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVFSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVFSELECT_A::_1
    }
}
#[doc = "Field `ALARM0` reader - Alarm 0"]
pub type ALARM0_R = crate::BitReader<ALARM0SELECT_A>;
#[doc = "Alarm 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM0SELECT_A {
    #[doc = "0: Interupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<ALARM0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM0SELECT_A {
        match self.bits {
            false => ALARM0SELECT_A::_0,
            true => ALARM0SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALARM0SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALARM0SELECT_A::_1
    }
}
#[doc = "Field `ALARM1` reader - Alarm 1"]
pub type ALARM1_R = crate::BitReader<ALARM1SELECT_A>;
#[doc = "Alarm 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM1SELECT_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<ALARM1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ALARM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM1SELECT_A {
        match self.bits {
            false => ALARM1SELECT_A::_0,
            true => ALARM1SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALARM1SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALARM1SELECT_A::_1
    }
}
#[doc = "Field `PER0` reader - Periodic 0"]
pub type PER0_R = crate::BitReader<PER0SELECT_A>;
#[doc = "Periodic 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER0SELECT_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<PER0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PER0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PER0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER0SELECT_A {
        match self.bits {
            false => PER0SELECT_A::_0,
            true => PER0SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PER0SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PER0SELECT_A::_1
    }
}
#[doc = "Field `PER1` reader - Periodic 1"]
pub type PER1_R = crate::BitReader<PER1SELECT_A>;
#[doc = "Periodic 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER1SELECT_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<PER1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PER1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PER1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER1SELECT_A {
        match self.bits {
            false => PER1SELECT_A::_0,
            true => PER1SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PER1SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PER1SELECT_A::_1
    }
}
#[doc = "Field `READY` reader - AST Ready"]
pub type READY_R = crate::BitReader<READYSELECT_A>;
#[doc = "AST Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READYSELECT_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<READYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: READYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READYSELECT_A {
        match self.bits {
            false => READYSELECT_A::_0,
            true => READYSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == READYSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == READYSELECT_A::_1
    }
}
#[doc = "Field `CLKRDY` reader - Clock Ready"]
pub type CLKRDY_R = crate::BitReader<CLKRDYSELECT_A>;
#[doc = "Clock Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKRDYSELECT_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<CLKRDYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CLKRDYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKRDYSELECT_A {
        match self.bits {
            false => CLKRDYSELECT_A::_0,
            true => CLKRDYSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKRDYSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKRDYSELECT_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Periodic 0"]
    #[inline(always)]
    pub fn per0(&self) -> PER0_R {
        PER0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Periodic 1"]
    #[inline(always)]
    pub fn per1(&self) -> PER1_R {
        PER1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 25 - AST Ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - Clock Ready"]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
