#[doc = "Register `USBSTA` reader"]
pub struct R(crate::R<USBSTA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBSTA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBSTA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBSTA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VBUSRQ` reader - VBus Request"]
pub type VBUSRQ_R = crate::BitReader<bool>;
#[doc = "Field `SPEED` reader - Speed Status"]
pub type SPEED_R = crate::FieldReader<u8, SPEEDSELECT_A>;
#[doc = "Speed Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPEEDSELECT_A {
    #[doc = "0: `0`"]
    FULL = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
    #[doc = "2: `10`"]
    LOW = 2,
}
impl From<SPEEDSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEEDSELECT_A) -> Self {
        variant as _
    }
}
impl SPEED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPEEDSELECT_A> {
        match self.bits {
            0 => Some(SPEEDSELECT_A::FULL),
            1 => Some(SPEEDSELECT_A::HIGH),
            2 => Some(SPEEDSELECT_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == SPEEDSELECT_A::FULL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPEEDSELECT_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPEEDSELECT_A::LOW
    }
}
#[doc = "Field `CLKUSABLE` reader - USB Clock Usable"]
pub type CLKUSABLE_R = crate::BitReader<bool>;
#[doc = "Field `SUSPEND` reader - Suspend module state"]
pub type SUSPEND_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 9 - VBus Request"]
    #[inline(always)]
    pub fn vbusrq(&self) -> VBUSRQ_R {
        VBUSRQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Speed Status"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - USB Clock Usable"]
    #[inline(always)]
    pub fn clkusable(&self) -> CLKUSABLE_R {
        CLKUSABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Suspend module state"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "General Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbsta](index.html) module"]
pub struct USBSTA_SPEC;
impl crate::RegisterSpec for USBSTA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbsta::R](R) reader structure"]
impl crate::Readable for USBSTA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USBSTA to value 0x0001_0000"]
impl crate::Resettable for USBSTA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
