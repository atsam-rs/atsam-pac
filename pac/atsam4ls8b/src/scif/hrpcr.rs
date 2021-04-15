#[doc = "Reader of register HRPCR"]
pub type R = crate::R<u32, super::HRPCR>;
#[doc = "Writer for register HRPCR"]
pub type W = crate::W<u32, super::HRPCR>;
#[doc = "Register HRPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::HRPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HRPEN`"]
pub type HRPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HRPEN`"]
pub struct HRPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HRPEN_W<'a> {
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
#[doc = "Reader of field `HRCOUNT`"]
pub type HRCOUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `HRCOUNT`"]
pub struct HRCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HRCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - High Resolution Prescaler Enable"]
    #[inline(always)]
    pub fn hrpen(&self) -> HRPEN_R {
        HRPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Clock Input Selection"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 8:31 - High Resolution Counter"]
    #[inline(always)]
    pub fn hrcount(&self) -> HRCOUNT_R {
        HRCOUNT_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - High Resolution Prescaler Enable"]
    #[inline(always)]
    pub fn hrpen(&mut self) -> HRPEN_W {
        HRPEN_W { w: self }
    }
    #[doc = "Bits 1:3 - Clock Input Selection"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W {
        CKSEL_W { w: self }
    }
    #[doc = "Bits 8:31 - High Resolution Counter"]
    #[inline(always)]
    pub fn hrcount(&mut self) -> HRCOUNT_W {
        HRCOUNT_W { w: self }
    }
}
