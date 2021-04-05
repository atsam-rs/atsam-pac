#[doc = "Reader of register TSSS"]
pub type R = crate::R<u32, super::TSSS>;
#[doc = "Writer for register TSSS"]
pub type W = crate::W<u32, super::TSSS>;
#[doc = "Register TSSS `reset()`'s with value 0"]
impl crate::ResetValue for super::TSSS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VTS`"]
pub type VTS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `VTS`"]
pub struct VTS_W<'a> {
    w: &'a mut W,
}
impl<'a> VTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value of Timer Seconds Register Capture"]
    #[inline(always)]
    pub fn vts(&self) -> VTS_R {
        VTS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value of Timer Seconds Register Capture"]
    #[inline(always)]
    pub fn vts(&mut self) -> VTS_W {
        VTS_W { w: self }
    }
}
