#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEN`"]
pub type SEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEN`"]
pub struct SEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEN_W<'a> {
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
#[doc = "Reader of field `SMEN`"]
pub type SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMEN`"]
pub struct SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SMATCH`"]
pub type SMATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMATCH`"]
pub struct SMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> SMATCH_W<'a> {
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
#[doc = "Reader of field `GCMATCH`"]
pub type GCMATCH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCMATCH`"]
pub struct GCMATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> GCMATCH_W<'a> {
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
#[doc = "Reader of field `STREN`"]
pub type STREN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STREN`"]
pub struct STREN_W<'a> {
    w: &'a mut W,
}
impl<'a> STREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `SWRST`"]
pub type SWRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWRST`"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
#[doc = "Reader of field `SMBALERT`"]
pub type SMBALERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMBALERT`"]
pub struct SMBALERT_W<'a> {
    w: &'a mut W,
}
impl<'a> SMBALERT_W<'a> {
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
#[doc = "Reader of field `SMDA`"]
pub type SMDA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMDA`"]
pub struct SMDA_W<'a> {
    w: &'a mut W,
}
impl<'a> SMDA_W<'a> {
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
#[doc = "Reader of field `SMHH`"]
pub type SMHH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMHH`"]
pub struct SMHH_W<'a> {
    w: &'a mut W,
}
impl<'a> SMHH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ACK`"]
pub type ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACK`"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
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
#[doc = "Reader of field `CUP`"]
pub type CUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CUP`"]
pub struct CUP_W<'a> {
    w: &'a mut W,
}
impl<'a> CUP_W<'a> {
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
#[doc = "Reader of field `SOAM`"]
pub type SOAM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOAM`"]
pub struct SOAM_W<'a> {
    w: &'a mut W,
}
impl<'a> SOAM_W<'a> {
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
#[doc = "Reader of field `SODR`"]
pub type SODR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SODR`"]
pub struct SODR_W<'a> {
    w: &'a mut W,
}
impl<'a> SODR_W<'a> {
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
#[doc = "Reader of field `ADR`"]
pub type ADR_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `ADR`"]
pub struct ADR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `BRIDGE`"]
pub type BRIDGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BRIDGE`"]
pub struct BRIDGE_W<'a> {
    w: &'a mut W,
}
impl<'a> BRIDGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Slave Enable"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SMBus Mode Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Slave Address Match"]
    #[inline(always)]
    pub fn smatch(&self) -> SMATCH_R {
        SMATCH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - General Call Address Match"]
    #[inline(always)]
    pub fn gcmatch(&self) -> GCMATCH_R {
        GCMATCH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Clock Stretch Enable"]
    #[inline(always)]
    pub fn stren(&self) -> STREN_R {
        STREN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SMBus Alert"]
    #[inline(always)]
    pub fn smbalert(&self) -> SMBALERT_R {
        SMBALERT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&self) -> SMDA_R {
        SMDA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&self) -> SMHH_R {
        SMHH_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Packet Error Checking Enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Slave Receiver Data Phase ACK Value"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - NBYTES Count Up"]
    #[inline(always)]
    pub fn cup(&self) -> CUP_R {
        CUP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Stretch Clock on Address Match"]
    #[inline(always)]
    pub fn soam(&self) -> SOAM_R {
        SOAM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Stretch Clock on Data Byte Reception"]
    #[inline(always)]
    pub fn sodr(&self) -> SODR_R {
        SODR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - Slave Address"]
    #[inline(always)]
    pub fn adr(&self) -> ADR_R {
        ADR_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Ten Bit Address Match"]
    #[inline(always)]
    pub fn tenbit(&self) -> TENBIT_R {
        TENBIT_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Bridge Control Enable"]
    #[inline(always)]
    pub fn bridge(&self) -> BRIDGE_R {
        BRIDGE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Slave Enable"]
    #[inline(always)]
    pub fn sen(&mut self) -> SEN_W {
        SEN_W { w: self }
    }
    #[doc = "Bit 1 - SMBus Mode Enable"]
    #[inline(always)]
    pub fn smen(&mut self) -> SMEN_W {
        SMEN_W { w: self }
    }
    #[doc = "Bit 2 - Slave Address Match"]
    #[inline(always)]
    pub fn smatch(&mut self) -> SMATCH_W {
        SMATCH_W { w: self }
    }
    #[doc = "Bit 3 - General Call Address Match"]
    #[inline(always)]
    pub fn gcmatch(&mut self) -> GCMATCH_W {
        GCMATCH_W { w: self }
    }
    #[doc = "Bit 4 - Clock Stretch Enable"]
    #[inline(always)]
    pub fn stren(&mut self) -> STREN_W {
        STREN_W { w: self }
    }
    #[doc = "Bit 7 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 8 - SMBus Alert"]
    #[inline(always)]
    pub fn smbalert(&mut self) -> SMBALERT_W {
        SMBALERT_W { w: self }
    }
    #[doc = "Bit 9 - SMBus Default Address"]
    #[inline(always)]
    pub fn smda(&mut self) -> SMDA_W {
        SMDA_W { w: self }
    }
    #[doc = "Bit 10 - SMBus Host Header"]
    #[inline(always)]
    pub fn smhh(&mut self) -> SMHH_W {
        SMHH_W { w: self }
    }
    #[doc = "Bit 11 - Packet Error Checking Enable"]
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W {
        PECEN_W { w: self }
    }
    #[doc = "Bit 12 - Slave Receiver Data Phase ACK Value"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
    #[doc = "Bit 13 - NBYTES Count Up"]
    #[inline(always)]
    pub fn cup(&mut self) -> CUP_W {
        CUP_W { w: self }
    }
    #[doc = "Bit 14 - Stretch Clock on Address Match"]
    #[inline(always)]
    pub fn soam(&mut self) -> SOAM_W {
        SOAM_W { w: self }
    }
    #[doc = "Bit 15 - Stretch Clock on Data Byte Reception"]
    #[inline(always)]
    pub fn sodr(&mut self) -> SODR_W {
        SODR_W { w: self }
    }
    #[doc = "Bits 16:25 - Slave Address"]
    #[inline(always)]
    pub fn adr(&mut self) -> ADR_W {
        ADR_W { w: self }
    }
    #[doc = "Bit 26 - Ten Bit Address Match"]
    #[inline(always)]
    pub fn tenbit(&mut self) -> TENBIT_W {
        TENBIT_W { w: self }
    }
    #[doc = "Bit 27 - Bridge Control Enable"]
    #[inline(always)]
    pub fn bridge(&mut self) -> BRIDGE_W {
        BRIDGE_W { w: self }
    }
}
