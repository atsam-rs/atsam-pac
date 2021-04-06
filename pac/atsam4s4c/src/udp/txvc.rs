#[doc = "Reader of register TXVC"]
pub type R = crate::R<u32, super::TXVC>;
#[doc = "Writer for register TXVC"]
pub type W = crate::W<u32, super::TXVC>;
#[doc = "Register TXVC `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::TXVC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `TXVDIS`"]
pub type TXVDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXVDIS`"]
pub struct TXVDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXVDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `PUON`"]
pub type PUON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PUON`"]
pub struct PUON_W<'a> {
    w: &'a mut W,
}
impl<'a> PUON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Transceiver Disable"]
    #[inline(always)]
    pub fn txvdis(&self) -> TXVDIS_R {
        TXVDIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pull-up On"]
    #[inline(always)]
    pub fn puon(&self) -> PUON_R {
        PUON_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Transceiver Disable"]
    #[inline(always)]
    pub fn txvdis(&mut self) -> TXVDIS_W {
        TXVDIS_W { w: self }
    }
    #[doc = "Bit 9 - Pull-up On"]
    #[inline(always)]
    pub fn puon(&mut self) -> PUON_W {
        PUON_W { w: self }
    }
}
