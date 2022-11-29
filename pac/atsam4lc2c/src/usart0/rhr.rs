#[doc = "Register `RHR` reader"]
pub struct R(crate::R<RHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXCHR` reader - Received Character"]
pub type RXCHR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RXSYNH` reader - Received Sync"]
pub type RXSYNH_R = crate::BitReader<RXSYNHSELECT_A>;
#[doc = "Received Sync\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXSYNHSELECT_A {
    #[doc = "0: Last character received is a Data"]
    _0 = 0,
    #[doc = "1: Last character received is a Command"]
    _1 = 1,
}
impl From<RXSYNHSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RXSYNHSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXSYNH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXSYNHSELECT_A {
        match self.bits {
            false => RXSYNHSELECT_A::_0,
            true => RXSYNHSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXSYNHSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXSYNHSELECT_A::_1
    }
}
impl R {
    #[doc = "Bits 0:8 - Received Character"]
    #[inline(always)]
    pub fn rxchr(&self) -> RXCHR_R {
        RXCHR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - Received Sync"]
    #[inline(always)]
    pub fn rxsynh(&self) -> RXSYNH_R {
        RXSYNH_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Receiver Holding Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rhr](index.html) module"]
pub struct RHR_SPEC;
impl crate::RegisterSpec for RHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rhr::R](R) reader structure"]
impl crate::Readable for RHR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RHR to value 0"]
impl crate::Resettable for RHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
