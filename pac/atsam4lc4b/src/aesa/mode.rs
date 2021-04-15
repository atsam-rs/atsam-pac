#[doc = "Reader of register MODE"]
pub type R = crate::R<u32, super::MODE>;
#[doc = "Writer for register MODE"]
pub type W = crate::W<u32, super::MODE>;
#[doc = "Register MODE `reset()`'s with value 0x000f_0000"]
impl crate::ResetValue for super::MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x000f_0000
    }
}
#[doc = "Reader of field `ENCRYPT`"]
pub type ENCRYPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENCRYPT`"]
pub struct ENCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENCRYPT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `KEYSIZE`"]
pub type KEYSIZE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `KEYSIZE`"]
pub struct KEYSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `DMA`"]
pub type DMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA`"]
pub struct DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `OPMODE`"]
pub type OPMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OPMODE`"]
pub struct OPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> OPMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Reader of field `CFBS`"]
pub type CFBS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CFBS`"]
pub struct CFBS_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `CTYPE`"]
pub type CTYPE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CTYPE`"]
pub struct CTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTYPE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Encryption"]
    #[inline(always)]
    pub fn encrypt(&self) -> ENCRYPT_R {
        ENCRYPT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Key Size"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - DMA Mode"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Confidentiality Mode of Operation"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Cipher Feedback Data Segment Size"]
    #[inline(always)]
    pub fn cfbs(&self) -> CFBS_R {
        CFBS_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - Countermeasure Type"]
    #[inline(always)]
    pub fn ctype(&self) -> CTYPE_R {
        CTYPE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Encryption"]
    #[inline(always)]
    pub fn encrypt(&mut self) -> ENCRYPT_W {
        ENCRYPT_W { w: self }
    }
    #[doc = "Bits 1:2 - Key Size"]
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W {
        KEYSIZE_W { w: self }
    }
    #[doc = "Bit 3 - DMA Mode"]
    #[inline(always)]
    pub fn dma(&mut self) -> DMA_W {
        DMA_W { w: self }
    }
    #[doc = "Bits 4:6 - Confidentiality Mode of Operation"]
    #[inline(always)]
    pub fn opmode(&mut self) -> OPMODE_W {
        OPMODE_W { w: self }
    }
    #[doc = "Bits 8:10 - Cipher Feedback Data Segment Size"]
    #[inline(always)]
    pub fn cfbs(&mut self) -> CFBS_W {
        CFBS_W { w: self }
    }
    #[doc = "Bits 16:19 - Countermeasure Type"]
    #[inline(always)]
    pub fn ctype(&mut self) -> CTYPE_W {
        CTYPE_W { w: self }
    }
}
