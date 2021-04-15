#[doc = "Reader of register CMDR"]
pub type R = crate::R<u32, super::CMDR>;
#[doc = "Writer for register CMDR"]
pub type W = crate::W<u32, super::CMDR>;
#[doc = "Register CMDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CMDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `READ`"]
pub type READ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `READ`"]
pub struct READ_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_W<'a> {
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
#[doc = "Reader of field `SADR`"]
pub type SADR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SADR`"]
pub struct SADR_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 1)) | (((value as u32) & 0x03ff) << 1);
        self.w
    }
}
#[doc = "Reader of field `TENBIT`"]
pub type TENBIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TENBIT`"]
pub struct TENBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> TENBIT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `REPSAME`"]
pub type REPSAME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REPSAME`"]
pub struct REPSAME_W<'a> {
    w: &'a mut W,
}
impl<'a> REPSAME_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `START`"]
pub type START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `START`"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `STOP`"]
pub type STOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STOP`"]
pub struct STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VALID`"]
pub struct VALID_W<'a> {
    w: &'a mut W,
}
impl<'a> VALID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `NBYTES`"]
pub type NBYTES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NBYTES`"]
pub struct NBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `PECEN`"]
pub type PECEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PECEN`"]
pub struct PECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PECEN_W<'a> {
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
#[doc = "Reader of field `ACKLAST`"]
pub type ACKLAST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACKLAST`"]
pub struct ACKLAST_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKLAST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `HS`"]
pub type HS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HS`"]
pub struct HS_W<'a> {
    w: &'a mut W,
}
impl<'a> HS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `HSMCODE`"]
pub type HSMCODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSMCODE`"]
pub struct HSMCODE_W<'a> {
    w: &'a mut W,
}
impl<'a> HSMCODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Transfer Direction"]
    #[inline(always)]
    pub fn read(&self) -> READ_R {
        READ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:10 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&self) -> SADR_R {
        SADR_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bit 11 - Ten Bit Addressing Mode"]
    #[inline(always)]
    pub fn tenbit(&self) -> TENBIT_R {
        TENBIT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transfer is to same address as previous address"]
    #[inline(always)]
    pub fn repsame(&self) -> REPSAME_R {
        REPSAME_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Send START condition"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Send STOP condition"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CMDR Valid"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Number of data bytes in transfer"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Packet Error Checking Enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - ACK Last Master RX Byte"]
    #[inline(always)]
    pub fn acklast(&self) -> ACKLAST_R {
        ACKLAST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - HS-mode"]
    #[inline(always)]
    pub fn hs(&self) -> HS_R {
        HS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - HS-mode Master Code"]
    #[inline(always)]
    pub fn hsmcode(&self) -> HSMCODE_R {
        HSMCODE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Direction"]
    #[inline(always)]
    pub fn read(&mut self) -> READ_W {
        READ_W { w: self }
    }
    #[doc = "Bits 1:10 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&mut self) -> SADR_W {
        SADR_W { w: self }
    }
    #[doc = "Bit 11 - Ten Bit Addressing Mode"]
    #[inline(always)]
    pub fn tenbit(&mut self) -> TENBIT_W {
        TENBIT_W { w: self }
    }
    #[doc = "Bit 12 - Transfer is to same address as previous address"]
    #[inline(always)]
    pub fn repsame(&mut self) -> REPSAME_W {
        REPSAME_W { w: self }
    }
    #[doc = "Bit 13 - Send START condition"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Bit 14 - Send STOP condition"]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W {
        STOP_W { w: self }
    }
    #[doc = "Bit 15 - CMDR Valid"]
    #[inline(always)]
    pub fn valid(&mut self) -> VALID_W {
        VALID_W { w: self }
    }
    #[doc = "Bits 16:23 - Number of data bytes in transfer"]
    #[inline(always)]
    pub fn nbytes(&mut self) -> NBYTES_W {
        NBYTES_W { w: self }
    }
    #[doc = "Bit 24 - Packet Error Checking Enable"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W {
        PECEN_W { w: self }
    }
    #[doc = "Bit 25 - ACK Last Master RX Byte"]
    #[inline(always)]
    pub fn acklast(&mut self) -> ACKLAST_W {
        ACKLAST_W { w: self }
    }
    #[doc = "Bit 26 - HS-mode"]
    #[inline(always)]
    pub fn hs(&mut self) -> HS_W {
        HS_W { w: self }
    }
    #[doc = "Bits 28:30 - HS-mode Master Code"]
    #[inline(always)]
    pub fn hsmcode(&mut self) -> HSMCODE_W {
        HSMCODE_W { w: self }
    }
}
