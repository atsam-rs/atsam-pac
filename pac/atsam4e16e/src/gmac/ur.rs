#[doc = "Reader of register UR"]
pub type R = crate::R<u32, super::UR>;
#[doc = "Writer for register UR"]
pub type W = crate::W<u32, super::UR>;
#[doc = "Register UR `reset()`'s with value 0"]
impl crate::ResetValue for super::UR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMIIMII`"]
pub type RMIIMII_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMIIMII`"]
pub struct RMIIMII_W<'a> {
    w: &'a mut W,
}
impl<'a> RMIIMII_W<'a> {
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
#[doc = "Reader of field `HDFC`"]
pub type HDFC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDFC`"]
pub struct HDFC_W<'a> {
    w: &'a mut W,
}
impl<'a> HDFC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `BPDG`"]
pub type BPDG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BPDG`"]
pub struct BPDG_W<'a> {
    w: &'a mut W,
}
impl<'a> BPDG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmiimii(&self) -> RMIIMII_R {
        RMIIMII_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 6 - Half Duplex Flow Control"]
    #[inline(always)]
    pub fn hdfc(&self) -> HDFC_R {
        HDFC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BPDG Bypass Deglitchers"]
    #[inline(always)]
    pub fn bpdg(&self) -> BPDG_R {
        BPDG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rmiimii(&mut self) -> RMIIMII_W {
        RMIIMII_W { w: self }
    }
    #[doc = "Bit 6 - Half Duplex Flow Control"]
    #[inline(always)]
    pub fn hdfc(&mut self) -> HDFC_W {
        HDFC_W { w: self }
    }
    #[doc = "Bit 7 - BPDG Bypass Deglitchers"]
    #[inline(always)]
    pub fn bpdg(&mut self) -> BPDG_W {
        BPDG_W { w: self }
    }
}
