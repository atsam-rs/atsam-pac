#[doc = "Register `SR` reader"]
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
#[doc = "Field `BUSY` reader - ABDACB Busy"]
pub type BUSY_R = crate::BitReader<bool>;
#[doc = "Field `TXRDY` reader - Transmit Ready"]
pub type TXRDY_R = crate::BitReader<TXRDYSELECT_A>;
#[doc = "Transmit Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRDYSELECT_A {
    #[doc = "0: No Audio DAC TX Ready has occured since the last time ISR was read or since reset"]
    _0 = 0,
    #[doc = "1: At least one Audio DAC TX Ready has occured since the last time ISR was read or since reset"]
    _1 = 1,
}
impl From<TXRDYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXRDYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRDYSELECT_A {
        match self.bits {
            false => TXRDYSELECT_A::_0,
            true => TXRDYSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXRDYSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXRDYSELECT_A::_1
    }
}
#[doc = "Field `TXUR` reader - Transmit Underrun"]
pub type TXUR_R = crate::BitReader<TXURSELECT_A>;
#[doc = "Transmit Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXURSELECT_A {
    #[doc = "0: No Audio DAC Underrun has occured since the last time ISR was read or since reset"]
    _0 = 0,
    #[doc = "1: At least one Audio DAC Underrun has occured since the last time ISR was read or since reset"]
    _1 = 1,
}
impl From<TXURSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXURSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXUR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXURSELECT_A {
        match self.bits {
            false => TXURSELECT_A::_0,
            true => TXURSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXURSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXURSELECT_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - ABDACB Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Underrun"]
    #[inline(always)]
    pub fn txur(&self) -> TXUR_R {
        TXUR_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
