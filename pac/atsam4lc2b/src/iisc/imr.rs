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
#[doc = "Receive Ready Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRDY_A {
    #[doc = "0: The corresponding interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: The corresponding interrupt is enabled"]
    ENABLED = 1,
}
impl From<RXRDY_A> for bool {
    #[inline(always)]
    fn from(variant: RXRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXRDY` reader - Receive Ready Interrupt Mask"]
pub struct RXRDY_R(crate::FieldReader<bool, RXRDY_A>);
impl RXRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRDY_A {
        match self.bits {
            false => RXRDY_A::DISABLED,
            true => RXRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXRDY_A::ENABLED
    }
}
impl core::ops::Deref for RXRDY_R {
    type Target = crate::FieldReader<bool, RXRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Receive Overrun Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOR_A {
    #[doc = "0: The corresponding interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: The corresponding interrupt is enabled"]
    ENABLED = 1,
}
impl From<RXOR_A> for bool {
    #[inline(always)]
    fn from(variant: RXOR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXOR` reader - Receive Overrun Interrupt Mask"]
pub struct RXOR_R(crate::FieldReader<bool, RXOR_A>);
impl RXOR_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOR_A {
        match self.bits {
            false => RXOR_A::DISABLED,
            true => RXOR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RXOR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RXOR_A::ENABLED
    }
}
impl core::ops::Deref for RXOR_R {
    type Target = crate::FieldReader<bool, RXOR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Ready Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDY_A {
    #[doc = "0: The corresponding interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: The corresponding interrupt is enabled"]
    ENABLED = 1,
}
impl From<TXRDY_A> for bool {
    #[inline(always)]
    fn from(variant: TXRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRDY` reader - Transmit Ready Interrupt Mask"]
pub struct TXRDY_R(crate::FieldReader<bool, TXRDY_A>);
impl TXRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRDY_A {
        match self.bits {
            false => TXRDY_A::DISABLED,
            true => TXRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXRDY_A::ENABLED
    }
}
impl core::ops::Deref for TXRDY_R {
    type Target = crate::FieldReader<bool, TXRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Transmit Underrun Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXUR_A {
    #[doc = "0: The corresponding interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: The corresponding interrupt is enabled"]
    ENABLED = 1,
}
impl From<TXUR_A> for bool {
    #[inline(always)]
    fn from(variant: TXUR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXUR` reader - Transmit Underrun Interrupt Mask"]
pub struct TXUR_R(crate::FieldReader<bool, TXUR_A>);
impl TXUR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXUR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXUR_A {
        match self.bits {
            false => TXUR_A::DISABLED,
            true => TXUR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TXUR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TXUR_A::ENABLED
    }
}
impl core::ops::Deref for TXUR_R {
    type Target = crate::FieldReader<bool, TXUR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Receive Ready Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn rxor(&self) -> RXOR_R {
        RXOR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Transmit Underrun Interrupt Mask"]
    #[inline(always)]
    pub fn txur(&self) -> TXUR_R {
        TXUR_R::new(((self.bits >> 6) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
