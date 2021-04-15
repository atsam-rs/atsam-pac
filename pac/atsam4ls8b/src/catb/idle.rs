#[doc = "Reader of register IDLE"]
pub type R = crate::R<u32, super::IDLE>;
#[doc = "Writer for register IDLE"]
pub type W = crate::W<u32, super::IDLE>;
#[doc = "Register IDLE `reset()`'s with value 0"]
impl crate::ResetValue for super::IDLE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FIDLE`"]
pub type FIDLE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FIDLE`"]
pub struct FIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIDLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `RIDLE`"]
pub type RIDLE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RIDLE`"]
pub struct RIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RIDLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 12)) | (((value as u32) & 0xffff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Fractional Sensor Idle"]
    #[inline(always)]
    pub fn fidle(&self) -> FIDLE_R {
        FIDLE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:27 - Integer Sensor Idle"]
    #[inline(always)]
    pub fn ridle(&self) -> RIDLE_R {
        RIDLE_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Fractional Sensor Idle"]
    #[inline(always)]
    pub fn fidle(&mut self) -> FIDLE_W {
        FIDLE_W { w: self }
    }
    #[doc = "Bits 12:27 - Integer Sensor Idle"]
    #[inline(always)]
    pub fn ridle(&mut self) -> RIDLE_W {
        RIDLE_W { w: self }
    }
}
