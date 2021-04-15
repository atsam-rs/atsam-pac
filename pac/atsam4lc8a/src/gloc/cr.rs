#[doc = "Reader of register CR%s"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR%s"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AEN`"]
pub type AEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AEN`"]
pub struct AEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `FILTEN`"]
pub type FILTEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FILTEN`"]
pub struct FILTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTEN_W<'a> {
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
    #[doc = "Bits 0:3 - Input mask"]
    #[inline(always)]
    pub fn aen(&self) -> AEN_R {
        AEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Filter enable"]
    #[inline(always)]
    pub fn filten(&self) -> FILTEN_R {
        FILTEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input mask"]
    #[inline(always)]
    pub fn aen(&mut self) -> AEN_W {
        AEN_W { w: self }
    }
    #[doc = "Bit 31 - Filter enable"]
    #[inline(always)]
    pub fn filten(&mut self) -> FILTEN_W {
        FILTEN_W { w: self }
    }
}
