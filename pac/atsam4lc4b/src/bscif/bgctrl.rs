#[doc = "Reader of register BGCTRL"]
pub type R = crate::R<u32, super::BGCTRL>;
#[doc = "Writer for register BGCTRL"]
pub type W = crate::W<u32, super::BGCTRL>;
#[doc = "Register BGCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::BGCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ADC Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCISEL_A {
    #[doc = "0: `0`"]
    DIS = 0,
    #[doc = "1: `1`"]
    VTEMP = 1,
    #[doc = "2: `10`"]
    VREF = 2,
}
impl From<ADCISEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCISEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADCISEL`"]
pub type ADCISEL_R = crate::R<u8, ADCISEL_A>;
impl ADCISEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADCISEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADCISEL_A::DIS),
            1 => Val(ADCISEL_A::VTEMP),
            2 => Val(ADCISEL_A::VREF),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ADCISEL_A::DIS
    }
    #[doc = "Checks if the value of the field is `VTEMP`"]
    #[inline(always)]
    pub fn is_vtemp(&self) -> bool {
        *self == ADCISEL_A::VTEMP
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == ADCISEL_A::VREF
    }
}
#[doc = "Write proxy for field `ADCISEL`"]
pub struct ADCISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCISEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCISEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADCISEL_A::DIS)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn vtemp(self) -> &'a mut W {
        self.variant(ADCISEL_A::VTEMP)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(ADCISEL_A::VREF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `TSEN`"]
pub type TSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSEN`"]
pub struct TSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TSEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC Input Selection"]
    #[inline(always)]
    pub fn adcisel(&self) -> ADCISEL_R {
        ADCISEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 8 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC Input Selection"]
    #[inline(always)]
    pub fn adcisel(&mut self) -> ADCISEL_W {
        ADCISEL_W { w: self }
    }
    #[doc = "Bit 8 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsen(&mut self) -> TSEN_W {
        TSEN_W { w: self }
    }
}
