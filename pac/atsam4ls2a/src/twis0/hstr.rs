#[doc = "Reader of register HSTR"]
pub type R = crate::R<u32, super::HSTR>;
#[doc = "Writer for register HSTR"]
pub type W = crate::W<u32, super::HSTR>;
#[doc = "Register HSTR `reset()`'s with value 0"]
impl crate::ResetValue for super::HSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HDDAT`"]
pub type HDDAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HDDAT`"]
pub struct HDDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> HDDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Data Hold Cycles"]
    #[inline(always)]
    pub fn hddat(&self) -> HDDAT_R {
        HDDAT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Data Hold Cycles"]
    #[inline(always)]
    pub fn hddat(&mut self) -> HDDAT_W {
        HDDAT_W { w: self }
    }
}
