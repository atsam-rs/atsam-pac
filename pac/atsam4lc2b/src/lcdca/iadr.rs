#[doc = "Writer for register IADR"]
pub type W = crate::W<u32, super::IADR>;
#[doc = "Register IADR `reset()`'s with value 0"]
impl crate::ResetValue for super::IADR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DATA`"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Write proxy for field `DMASK`"]
pub struct DMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> DMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `OFF`"]
pub struct OFF_W<'a> {
    w: &'a mut W,
}
impl<'a> OFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - Segments Value"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Bits 8:15 - Data Mask"]
    #[inline(always)]
    pub fn dmask(&mut self) -> DMASK_W {
        DMASK_W { w: self }
    }
    #[doc = "Bits 16:20 - Byte Offset"]
    #[inline(always)]
    pub fn off(&mut self) -> OFF_W {
        OFF_W { w: self }
    }
}
