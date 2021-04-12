#[doc = "Reader of register EMR"]
pub type R = crate::R<u32, super::EMR>;
#[doc = "Writer for register EMR"]
pub type W = crate::W<u32, super::EMR>;
#[doc = "Register EMR `reset()`'s with value 0"]
impl crate::ResetValue for super::EMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Comparison Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMPMODE_A {
    #[doc = "0: Generates an event when the converted data is lower than the low threshold of the window."]
    LOW = 0,
    #[doc = "1: Generates an event when the converted data is higher than the high threshold of the window."]
    HIGH = 1,
    #[doc = "2: Generates an event when the converted data is in the comparison window."]
    IN = 2,
    #[doc = "3: Generates an event when the converted data is out of the comparison window."]
    OUT = 3,
}
impl From<CMPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CMPMODE`"]
pub type CMPMODE_R = crate::R<u8, CMPMODE_A>;
impl CMPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPMODE_A {
        match self.bits {
            0 => CMPMODE_A::LOW,
            1 => CMPMODE_A::HIGH,
            2 => CMPMODE_A::IN,
            3 => CMPMODE_A::OUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CMPMODE_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CMPMODE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == CMPMODE_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == CMPMODE_A::OUT
    }
}
#[doc = "Write proxy for field `CMPMODE`"]
pub struct CMPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMPMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Generates an event when the converted data is lower than the low threshold of the window."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CMPMODE_A::LOW)
    }
    #[doc = "Generates an event when the converted data is higher than the high threshold of the window."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CMPMODE_A::HIGH)
    }
    #[doc = "Generates an event when the converted data is in the comparison window."]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(CMPMODE_A::IN)
    }
    #[doc = "Generates an event when the converted data is out of the comparison window."]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(CMPMODE_A::OUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `CMPSEL`"]
pub type CMPSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPSEL`"]
pub struct CMPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `CMPALL`"]
pub type CMPALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMPALL`"]
pub struct CMPALL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `CMPFILTER`"]
pub type CMPFILTER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CMPFILTER`"]
pub struct CMPFILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> CMPFILTER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Over Sampling Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSR_A {
    #[doc = "0: no averaging. ADC sample rate is maximum."]
    NO_AVERAGE = 0,
    #[doc = "1: 1-bit enhanced resolution by interpolation. ADC sample rate divided by 4."]
    OSR4 = 1,
    #[doc = "2: 2-bit enhanced resolution by interpolation. ADC sample rate divided by 16."]
    OSR16 = 2,
}
impl From<OSR_A> for u8 {
    #[inline(always)]
    fn from(variant: OSR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSR`"]
pub type OSR_R = crate::R<u8, OSR_A>;
impl OSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OSR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OSR_A::NO_AVERAGE),
            1 => Val(OSR_A::OSR4),
            2 => Val(OSR_A::OSR16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NO_AVERAGE`"]
    #[inline(always)]
    pub fn is_no_average(&self) -> bool {
        *self == OSR_A::NO_AVERAGE
    }
    #[doc = "Checks if the value of the field is `OSR4`"]
    #[inline(always)]
    pub fn is_osr4(&self) -> bool {
        *self == OSR_A::OSR4
    }
    #[doc = "Checks if the value of the field is `OSR16`"]
    #[inline(always)]
    pub fn is_osr16(&self) -> bool {
        *self == OSR_A::OSR16
    }
}
#[doc = "Write proxy for field `OSR`"]
pub struct OSR_W<'a> {
    w: &'a mut W,
}
impl<'a> OSR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "no averaging. ADC sample rate is maximum."]
    #[inline(always)]
    pub fn no_average(self) -> &'a mut W {
        self.variant(OSR_A::NO_AVERAGE)
    }
    #[doc = "1-bit enhanced resolution by interpolation. ADC sample rate divided by 4."]
    #[inline(always)]
    pub fn osr4(self) -> &'a mut W {
        self.variant(OSR_A::OSR4)
    }
    #[doc = "2-bit enhanced resolution by interpolation. ADC sample rate divided by 16."]
    #[inline(always)]
    pub fn osr16(self) -> &'a mut W {
        self.variant(OSR_A::OSR16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Averaging on Single Trigger Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ASTE_A {
    #[doc = "0: The average requests several trigger events."]
    MULTI_TRIG_AVERAGE = 0,
    #[doc = "1: The average requests only one trigger event."]
    SINGLE_TRIG_AVERAGE = 1,
}
impl From<ASTE_A> for bool {
    #[inline(always)]
    fn from(variant: ASTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ASTE`"]
pub type ASTE_R = crate::R<bool, ASTE_A>;
impl ASTE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASTE_A {
        match self.bits {
            false => ASTE_A::MULTI_TRIG_AVERAGE,
            true => ASTE_A::SINGLE_TRIG_AVERAGE,
        }
    }
    #[doc = "Checks if the value of the field is `MULTI_TRIG_AVERAGE`"]
    #[inline(always)]
    pub fn is_multi_trig_average(&self) -> bool {
        *self == ASTE_A::MULTI_TRIG_AVERAGE
    }
    #[doc = "Checks if the value of the field is `SINGLE_TRIG_AVERAGE`"]
    #[inline(always)]
    pub fn is_single_trig_average(&self) -> bool {
        *self == ASTE_A::SINGLE_TRIG_AVERAGE
    }
}
#[doc = "Write proxy for field `ASTE`"]
pub struct ASTE_W<'a> {
    w: &'a mut W,
}
impl<'a> ASTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASTE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The average requests several trigger events."]
    #[inline(always)]
    pub fn multi_trig_average(self) -> &'a mut W {
        self.variant(ASTE_A::MULTI_TRIG_AVERAGE)
    }
    #[doc = "The average requests only one trigger event."]
    #[inline(always)]
    pub fn single_trig_average(self) -> &'a mut W {
        self.variant(ASTE_A::SINGLE_TRIG_AVERAGE)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `TAG`"]
pub type TAG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAG`"]
pub struct TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&self) -> CMPMODE_R {
        CMPMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CMPSEL_R {
        CMPSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&self) -> CMPALL_R {
        CMPALL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&self) -> CMPFILTER_R {
        CMPFILTER_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Over Sampling Rate"]
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Averaging on Single Trigger Event"]
    #[inline(always)]
    pub fn aste(&self) -> ASTE_R {
        ASTE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - TAG of the ADC_LDCR register"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparison Mode"]
    #[inline(always)]
    pub fn cmpmode(&mut self) -> CMPMODE_W {
        CMPMODE_W { w: self }
    }
    #[doc = "Bits 4:7 - Comparison Selected Channel"]
    #[inline(always)]
    pub fn cmpsel(&mut self) -> CMPSEL_W {
        CMPSEL_W { w: self }
    }
    #[doc = "Bit 9 - Compare All Channels"]
    #[inline(always)]
    pub fn cmpall(&mut self) -> CMPALL_W {
        CMPALL_W { w: self }
    }
    #[doc = "Bits 12:13 - Compare Event Filtering"]
    #[inline(always)]
    pub fn cmpfilter(&mut self) -> CMPFILTER_W {
        CMPFILTER_W { w: self }
    }
    #[doc = "Bits 16:17 - Over Sampling Rate"]
    #[inline(always)]
    pub fn osr(&mut self) -> OSR_W {
        OSR_W { w: self }
    }
    #[doc = "Bit 20 - Averaging on Single Trigger Event"]
    #[inline(always)]
    pub fn aste(&mut self) -> ASTE_W {
        ASTE_W { w: self }
    }
    #[doc = "Bit 24 - TAG of the ADC_LDCR register"]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W {
        TAG_W { w: self }
    }
}
