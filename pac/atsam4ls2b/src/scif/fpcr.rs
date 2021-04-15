#[doc = "Reader of register FPCR"]
pub type R = crate::R<u32, super::FPCR>;
#[doc = "Writer for register FPCR"]
pub type W = crate::W<u32, super::FPCR>;
#[doc = "Register FPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::FPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FPEN`"]
pub type FPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FPEN`"]
pub struct FPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPEN_W<'a> {
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
#[doc = "Reader of field `CKSEL`"]
pub type CKSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CKSEL`"]
pub struct CKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - High Resolution Prescaler Enable"]
    #[inline(always)]
    pub fn fpen(&self) -> FPEN_R {
        FPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Clock Input Selection"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - High Resolution Prescaler Enable"]
    #[inline(always)]
    pub fn fpen(&mut self) -> FPEN_W {
        FPEN_W { w: self }
    }
    #[doc = "Bits 1:3 - Clock Input Selection"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W {
        CKSEL_W { w: self }
    }
}
