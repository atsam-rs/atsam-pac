#[doc = "Reader of register WTH"]
pub type R = crate::R<u32, super::WTH>;
#[doc = "Writer for register WTH"]
pub type W = crate::W<u32, super::WTH>;
#[doc = "Register WTH `reset()`'s with value 0"]
impl crate::ResetValue for super::WTH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LT`"]
pub type LT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `LT`"]
pub struct LT_W<'a> {
    w: &'a mut W,
}
impl<'a> LT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Reader of field `HT`"]
pub type HT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HT`"]
pub struct HT_W<'a> {
    w: &'a mut W,
}
impl<'a> HT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Low threshold"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Low threshold"]
    #[inline(always)]
    pub fn lt(&mut self) -> LT_W {
        LT_W { w: self }
    }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W {
        HT_W { w: self }
    }
}
