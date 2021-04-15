#[doc = "Reader of register PMCON"]
pub type R = crate::R<u32, super::PMCON>;
#[doc = "Writer for register PMCON"]
pub type W = crate::W<u32, super::PMCON>;
#[doc = "Register PMCON `reset()`'s with value 0"]
impl crate::ResetValue for super::PMCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PS`"]
pub type PS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PS`"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PSCREQ`"]
pub type PSCREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PSCREQ`"]
pub struct PSCREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PSCREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `PSCM`"]
pub type PSCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PSCM`"]
pub struct PSCM_W<'a> {
    w: &'a mut W,
}
impl<'a> PSCM_W<'a> {
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
#[doc = "Reader of field `BKUP`"]
pub type BKUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKUP`"]
pub struct BKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> BKUP_W<'a> {
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
#[doc = "Reader of field `RET`"]
pub type RET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RET`"]
pub struct RET_W<'a> {
    w: &'a mut W,
}
impl<'a> RET_W<'a> {
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
#[doc = "Reader of field `SLEEP`"]
pub type SLEEP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLEEP`"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `CK32S`"]
pub type CK32S_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CK32S`"]
pub struct CK32S_W<'a> {
    w: &'a mut W,
}
impl<'a> CK32S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `FASTWKUP`"]
pub type FASTWKUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FASTWKUP`"]
pub struct FASTWKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTWKUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Power Scaling Configuration Value"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Power Scaling Change Request"]
    #[inline(always)]
    pub fn pscreq(&self) -> PSCREQ_R {
        PSCREQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Power Scaling Change Mode"]
    #[inline(always)]
    pub fn pscm(&self) -> PSCM_R {
        PSCM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BACKUP Mode"]
    #[inline(always)]
    pub fn bkup(&self) -> BKUP_R {
        BKUP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RETENTION Mode"]
    #[inline(always)]
    pub fn ret(&self) -> RET_R {
        RET_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - SLEEP mode Configuration"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - 32Khz-1Khz Clock Source Selection"]
    #[inline(always)]
    pub fn ck32s(&self) -> CK32S_R {
        CK32S_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Fast Wakeup"]
    #[inline(always)]
    pub fn fastwkup(&self) -> FASTWKUP_R {
        FASTWKUP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power Scaling Configuration Value"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 2 - Power Scaling Change Request"]
    #[inline(always)]
    pub fn pscreq(&mut self) -> PSCREQ_W {
        PSCREQ_W { w: self }
    }
    #[doc = "Bit 3 - Power Scaling Change Mode"]
    #[inline(always)]
    pub fn pscm(&mut self) -> PSCM_W {
        PSCM_W { w: self }
    }
    #[doc = "Bit 8 - BACKUP Mode"]
    #[inline(always)]
    pub fn bkup(&mut self) -> BKUP_W {
        BKUP_W { w: self }
    }
    #[doc = "Bit 9 - RETENTION Mode"]
    #[inline(always)]
    pub fn ret(&mut self) -> RET_W {
        RET_W { w: self }
    }
    #[doc = "Bits 12:13 - SLEEP mode Configuration"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 16 - 32Khz-1Khz Clock Source Selection"]
    #[inline(always)]
    pub fn ck32s(&mut self) -> CK32S_W {
        CK32S_W { w: self }
    }
    #[doc = "Bit 24 - Fast Wakeup"]
    #[inline(always)]
    pub fn fastwkup(&mut self) -> FASTWKUP_W {
        FASTWKUP_W { w: self }
    }
}
