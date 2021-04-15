#[doc = "Reader of register VCR1"]
pub type R = crate::R<u32, super::VCR1>;
#[doc = "Writer for register VCR1"]
pub type W = crate::W<u32, super::VCR1>;
#[doc = "Register VCR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::VCR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VOLUME`"]
pub type VOLUME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `VOLUME`"]
pub struct VOLUME_W<'a> {
    w: &'a mut W,
}
impl<'a> VOLUME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
#[doc = "Reader of field `MUTE`"]
pub type MUTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MUTE`"]
pub struct MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Volume Control"]
    #[inline(always)]
    pub fn volume(&self) -> VOLUME_R {
        VOLUME_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Mute"]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Volume Control"]
    #[inline(always)]
    pub fn volume(&mut self) -> VOLUME_W {
        VOLUME_W { w: self }
    }
    #[doc = "Bit 31 - Mute"]
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W {
        MUTE_W { w: self }
    }
}
