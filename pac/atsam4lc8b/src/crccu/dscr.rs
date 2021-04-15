#[doc = "Reader of register DSCR"]
pub type R = crate::R<u32, super::DSCR>;
#[doc = "Writer for register DSCR"]
pub type W = crate::W<u32, super::DSCR>;
#[doc = "Register DSCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DSCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DSCR`"]
pub type DSCR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DSCR`"]
pub struct DSCR_W<'a> {
    w: &'a mut W,
}
impl<'a> DSCR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x007f_ffff << 9)) | (((value as u32) & 0x007f_ffff) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bits 9:31 - Description Base Address"]
    #[inline(always)]
    pub fn dscr(&self) -> DSCR_R {
        DSCR_R::new(((self.bits >> 9) & 0x007f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 9:31 - Description Base Address"]
    #[inline(always)]
    pub fn dscr(&mut self) -> DSCR_W {
        DSCR_W { w: self }
    }
}
