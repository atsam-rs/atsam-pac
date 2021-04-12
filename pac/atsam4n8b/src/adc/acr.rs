#[doc = "Reader of register ACR"]
pub type R = crate::R<u32, super::ACR>;
#[doc = "Writer for register ACR"]
pub type W = crate::W<u32, super::ACR>;
#[doc = "Register ACR `reset()`'s with value 0x0008_0000"]
impl crate::ResetValue for super::ACR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0008_0000
    }
}
#[doc = "Internal Reference Voltage Change Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IRVCE_A {
    #[doc = "0: The internal reference voltage is stuck at the default value (see the product electrical charac-teristics for further details)."]
    STUCK_AT_DEFAULT = 0,
    #[doc = "1: The internal reference voltage is defined by field IRVS."]
    SELECTION = 1,
}
impl From<IRVCE_A> for bool {
    #[inline(always)]
    fn from(variant: IRVCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IRVCE`"]
pub type IRVCE_R = crate::R<bool, IRVCE_A>;
impl IRVCE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRVCE_A {
        match self.bits {
            false => IRVCE_A::STUCK_AT_DEFAULT,
            true => IRVCE_A::SELECTION,
        }
    }
    #[doc = "Checks if the value of the field is `STUCK_AT_DEFAULT`"]
    #[inline(always)]
    pub fn is_stuck_at_default(&self) -> bool {
        *self == IRVCE_A::STUCK_AT_DEFAULT
    }
    #[doc = "Checks if the value of the field is `SELECTION`"]
    #[inline(always)]
    pub fn is_selection(&self) -> bool {
        *self == IRVCE_A::SELECTION
    }
}
#[doc = "Write proxy for field `IRVCE`"]
pub struct IRVCE_W<'a> {
    w: &'a mut W,
}
impl<'a> IRVCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRVCE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The internal reference voltage is stuck at the default value (see the product electrical charac-teristics for further details)."]
    #[inline(always)]
    pub fn stuck_at_default(self) -> &'a mut W {
        self.variant(IRVCE_A::STUCK_AT_DEFAULT)
    }
    #[doc = "The internal reference voltage is defined by field IRVS."]
    #[inline(always)]
    pub fn selection(self) -> &'a mut W {
        self.variant(IRVCE_A::SELECTION)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `IRVS`"]
pub type IRVS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRVS`"]
pub struct IRVS_W<'a> {
    w: &'a mut W,
}
impl<'a> IRVS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 3)) | (((value as u32) & 0x0f) << 3);
        self.w
    }
}
#[doc = "Reader of field `FORCEREF`"]
pub type FORCEREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCEREF`"]
pub struct FORCEREF_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `ONREF`"]
pub type ONREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONREF`"]
pub struct ONREF_W<'a> {
    w: &'a mut W,
}
impl<'a> ONREF_W<'a> {
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
impl R {
    #[doc = "Bit 2 - Internal Reference Voltage Change Enable"]
    #[inline(always)]
    pub fn irvce(&self) -> IRVCE_R {
        IRVCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:6 - Internal Reference Voltage Selection"]
    #[inline(always)]
    pub fn irvs(&self) -> IRVS_R {
        IRVS_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - Force Internal Reference Voltage"]
    #[inline(always)]
    pub fn forceref(&self) -> FORCEREF_R {
        FORCEREF_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Internal Voltage Reference ON"]
    #[inline(always)]
    pub fn onref(&self) -> ONREF_R {
        ONREF_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Internal Reference Voltage Change Enable"]
    #[inline(always)]
    pub fn irvce(&mut self) -> IRVCE_W {
        IRVCE_W { w: self }
    }
    #[doc = "Bits 3:6 - Internal Reference Voltage Selection"]
    #[inline(always)]
    pub fn irvs(&mut self) -> IRVS_W {
        IRVS_W { w: self }
    }
    #[doc = "Bit 19 - Force Internal Reference Voltage"]
    #[inline(always)]
    pub fn forceref(&mut self) -> FORCEREF_W {
        FORCEREF_W { w: self }
    }
    #[doc = "Bit 20 - Internal Voltage Reference ON"]
    #[inline(always)]
    pub fn onref(&mut self) -> ONREF_W {
        ONREF_W { w: self }
    }
}
