#[doc = "Reader of register TIMING"]
pub type R = crate::R<u32, super::TIMING>;
#[doc = "Writer for register TIMING"]
pub type W = crate::W<u32, super::TIMING>;
#[doc = "Register TIMING `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMING {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TLEVEL`"]
pub type TLEVEL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TLEVEL`"]
pub struct TLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TLEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `TIDLE`"]
pub type TIDLE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TIDLE`"]
pub struct TIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIDLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Relative Level Smoothing"]
    #[inline(always)]
    pub fn tlevel(&self) -> TLEVEL_R {
        TLEVEL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Idle Smoothening"]
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Relative Level Smoothing"]
    #[inline(always)]
    pub fn tlevel(&mut self) -> TLEVEL_W {
        TLEVEL_W { w: self }
    }
    #[doc = "Bits 16:27 - Idle Smoothening"]
    #[inline(always)]
    pub fn tidle(&mut self) -> TIDLE_W {
        TIDLE_W { w: self }
    }
}
