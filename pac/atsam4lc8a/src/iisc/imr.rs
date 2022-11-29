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
#[doc = "Field `RXRDY` reader - Receive Ready Interrupt Mask"]
pub type RXRDY_R = crate::BitReader<RXRDYSELECT_A>;
#[doc = "Receive Ready Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXRDYSELECT_A {
    #[doc = "0: The corresponding interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: The corresponding interrupt is enabled"]
    ENABLED = 1,
}
impl From<RXRDYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RXRDYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRDYSELECT_A {
        match self.bits {
            false => RXRDYSELECT_A::DISABLED,
            true => RXRDYSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXRDYSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXRDYSELECT_A::ENABLED
    }
}
#[doc = "Field `RXOR` reader - Receive Overrun Interrupt Mask"]
pub type RXOR_R = crate::BitReader<RXORSELECT_A>;
#[doc = "Receive Overrun Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXORSELECT_A {
    #[doc = "0: The corresponding interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: The corresponding interrupt is enabled"]
    ENABLED = 1,
}
impl From<RXORSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RXORSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXORSELECT_A {
        match self.bits {
            false => RXORSELECT_A::DISABLED,
            true => RXORSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXORSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXORSELECT_A::ENABLED
    }
}
#[doc = "Field `TXRDY` reader - Transmit Ready Interrupt Mask"]
pub type TXRDY_R = crate::BitReader<TXRDYSELECT_A>;
#[doc = "Transmit Ready Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRDYSELECT_A {
    #[doc = "0: The corresponding interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: The corresponding interrupt is enabled"]
    ENABLED = 1,
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
            false => TXRDYSELECT_A::DISABLED,
            true => TXRDYSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXRDYSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXRDYSELECT_A::ENABLED
    }
}
#[doc = "Field `TXUR` reader - Transmit Underrun Interrupt Mask"]
pub type TXUR_R = crate::BitReader<TXURSELECT_A>;
#[doc = "Transmit Underrun Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXURSELECT_A {
    #[doc = "0: The corresponding interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: The corresponding interrupt is enabled"]
    ENABLED = 1,
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
            false => TXURSELECT_A::DISABLED,
            true => TXURSELECT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXURSELECT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXURSELECT_A::ENABLED
    }
}
impl R {
    #[doc = "Bit 1 - Receive Ready Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn rxor(&self) -> RXOR_R {
        RXOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun Interrupt Mask"]
    #[inline(always)]
    pub fn txur(&self) -> TXUR_R {
        TXUR_R::new(((self.bits >> 6) & 1) != 0)
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
