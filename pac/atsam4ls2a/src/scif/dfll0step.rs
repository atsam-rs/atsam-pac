#[doc = "Reader of register DFLL0STEP"]
pub type R = crate::R<u32, super::DFLL0STEP>;
#[doc = "Writer for register DFLL0STEP"]
pub type W = crate::W<u32, super::DFLL0STEP>;
#[doc = "Register DFLL0STEP `reset()`'s with value 0"]
impl crate::ResetValue for super::DFLL0STEP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSTEP`"]
pub type FSTEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FSTEP`"]
pub struct FSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `CSTEP`"]
pub type CSTEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CSTEP`"]
pub struct CSTEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Fine Maximum Step"]
    #[inline(always)]
    pub fn fstep(&self) -> FSTEP_R {
        FSTEP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - Coarse Maximum Step"]
    #[inline(always)]
    pub fn cstep(&self) -> CSTEP_R {
        CSTEP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fine Maximum Step"]
    #[inline(always)]
    pub fn fstep(&mut self) -> FSTEP_W {
        FSTEP_W { w: self }
    }
    #[doc = "Bits 16:20 - Coarse Maximum Step"]
    #[inline(always)]
    pub fn cstep(&mut self) -> CSTEP_W {
        CSTEP_W { w: self }
    }
}
