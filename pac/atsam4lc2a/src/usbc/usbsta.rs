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
pub struct VBUSRQ_R(crate::FieldReader<bool, bool>);
impl VBUSRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBUSRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUSRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Speed Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPEED_A {
    #[doc = "0: `0`"]
    FULL = 0,
    #[doc = "1: `1`"]
    HIGH = 1,
    #[doc = "2: `10`"]
    LOW = 2,
}
impl From<SPEED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPEED_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPEED` reader - Speed Status"]
pub struct SPEED_R(crate::FieldReader<u8, SPEED_A>);
impl SPEED_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPEED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPEED_A> {
        match self.bits {
            0 => Some(SPEED_A::FULL),
            1 => Some(SPEED_A::HIGH),
            2 => Some(SPEED_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        **self == SPEED_A::FULL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == SPEED_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == SPEED_A::LOW
    }
}
impl core::ops::Deref for SPEED_R {
    type Target = crate::FieldReader<u8, SPEED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKUSABLE` reader - USB Clock Usable"]
pub struct CLKUSABLE_R(crate::FieldReader<bool, bool>);
impl CLKUSABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKUSABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKUSABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSPEND` reader - Suspend module state"]
pub struct SUSPEND_R(crate::FieldReader<bool, bool>);
impl SUSPEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 9 - VBus Request"]
    #[inline(always)]
    pub fn vbusrq(&self) -> VBUSRQ_R {
        VBUSRQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Speed Status"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 14 - USB Clock Usable"]
    #[inline(always)]
    pub fn clkusable(&self) -> CLKUSABLE_R {
        CLKUSABLE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Suspend module state"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 16) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
