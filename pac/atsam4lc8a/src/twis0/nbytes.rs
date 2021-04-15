#[doc = "Reader of register NBYTES"]
pub type R = crate::R<u32, super::NBYTES>;
#[doc = "Writer for register NBYTES"]
pub type W = crate::W<u32, super::NBYTES>;
#[doc = "Register NBYTES `reset()`'s with value 0"]
impl crate::ResetValue for super::NBYTES {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NBYTES`"]
pub type NBYTES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBYTES`"]
pub struct NBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of Bytes to Transfer"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Number of Bytes to Transfer"]
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W {
        NBYTES_W { w: self }
    }
}
