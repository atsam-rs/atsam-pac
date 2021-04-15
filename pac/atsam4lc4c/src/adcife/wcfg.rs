#[doc = "Reader of register WCFG"]
pub type R = crate::R<u32, super::WCFG>;
#[doc = "Writer for register WCFG"]
pub type W = crate::W<u32, super::WCFG>;
#[doc = "Register WCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::WCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WM`"]
pub type WM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `WM`"]
pub struct WM_W<'a> {
    w: &'a mut W,
}
impl<'a> WM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:14 - Window Monitor Mode"]
    #[inline(always)]
    pub fn wm(&self) -> WM_R {
        WM_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14 - Window Monitor Mode"]
    #[inline(always)]
    pub fn wm(&mut self) -> WM_W {
        WM_W { w: self }
    }
}
