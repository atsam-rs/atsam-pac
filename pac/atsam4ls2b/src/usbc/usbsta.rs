#[doc = "Reader of register USBSTA"]
pub type R = crate::R<u32, super::USBSTA>;
#[doc = "Reader of field `VBUSRQ`"]
pub type VBUSRQ_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<u8, SPEED_A>;
impl SPEED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPEED_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPEED_A::FULL),
            1 => Val(SPEED_A::HIGH),
            2 => Val(SPEED_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == SPEED_A::FULL
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == SPEED_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == SPEED_A::LOW
    }
}
#[doc = "Reader of field `CLKUSABLE`"]
pub type CLKUSABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SUSPEND`"]
pub type SUSPEND_R = crate::R<bool, bool>;
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
