#[doc = "Reader of register CFDCTRL"]
pub type R = crate::R<u32, super::CFDCTRL>;
#[doc = "Writer for register CFDCTRL"]
pub type W = crate::W<u32, super::CFDCTRL>;
#[doc = "Register CFDCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CFDCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CFDEN`"]
pub type CFDEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CFDEN`"]
pub struct CFDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CFDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `SFV`"]
pub type SFV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SFV`"]
pub struct SFV_W<'a> {
    w: &'a mut W,
}
impl<'a> SFV_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Failure Detection Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 31 - Store Final Value"]
    #[inline(always)]
    pub fn sfv(&self) -> SFV_R {
        SFV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detection Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CFDEN_W {
        CFDEN_W { w: self }
    }
    #[doc = "Bit 31 - Store Final Value"]
    #[inline(always)]
    pub fn sfv(&mut self) -> SFV_W {
        SFV_W { w: self }
    }
}
