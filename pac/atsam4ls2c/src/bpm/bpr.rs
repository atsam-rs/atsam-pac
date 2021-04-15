#[doc = "Reader of register BPR"]
pub type R = crate::R<u32, super::BPR>;
#[doc = "Writer for register BPR"]
pub type W = crate::W<u32, super::BPR>;
#[doc = "Register BPR `reset()`'s with value 0"]
impl crate::ResetValue for super::BPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RUNPSPB`"]
pub type RUNPSPB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RUNPSPB`"]
pub struct RUNPSPB_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNPSPB_W<'a> {
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
#[doc = "Reader of field `PSMPSPB`"]
pub type PSMPSPB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PSMPSPB`"]
pub struct PSMPSPB_W<'a> {
    w: &'a mut W,
}
impl<'a> PSMPSPB_W<'a> {
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
#[doc = "Reader of field `SEQSTN`"]
pub type SEQSTN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SEQSTN`"]
pub struct SEQSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> SEQSTN_W<'a> {
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
#[doc = "Reader of field `PSBTD`"]
pub type PSBTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PSBTD`"]
pub struct PSBTD_W<'a> {
    w: &'a mut W,
}
impl<'a> PSBTD_W<'a> {
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
#[doc = "Reader of field `PSHFD`"]
pub type PSHFD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PSHFD`"]
pub struct PSHFD_W<'a> {
    w: &'a mut W,
}
impl<'a> PSHFD_W<'a> {
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
#[doc = "Reader of field `DLYRSTD`"]
pub type DLYRSTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLYRSTD`"]
pub struct DLYRSTD_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYRSTD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `BIASSEN`"]
pub type BIASSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BIASSEN`"]
pub struct BIASSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASSEN_W<'a> {
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
#[doc = "Reader of field `LATSEN`"]
pub type LATSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LATSEN`"]
pub struct LATSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LATSEN_W<'a> {
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
#[doc = "Reader of field `BOD18CONT`"]
pub type BOD18CONT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOD18CONT`"]
pub struct BOD18CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD18CONT_W<'a> {
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
#[doc = "Reader of field `POBS`"]
pub type POBS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POBS`"]
pub struct POBS_W<'a> {
    w: &'a mut W,
}
impl<'a> POBS_W<'a> {
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
#[doc = "Reader of field `FFFW`"]
pub type FFFW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFFW`"]
pub struct FFFW_W<'a> {
    w: &'a mut W,
}
impl<'a> FFFW_W<'a> {
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
#[doc = "Reader of field `FBRDYEN`"]
pub type FBRDYEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FBRDYEN`"]
pub struct FBRDYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FBRDYEN_W<'a> {
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
#[doc = "Reader of field `FVREFSEN`"]
pub type FVREFSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FVREFSEN`"]
pub struct FVREFSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FVREFSEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Run Mode Power Scaling Preset Bypass"]
    #[inline(always)]
    pub fn runpspb(&self) -> RUNPSPB_R {
        RUNPSPB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power Save Mode Power Scaling Preset Bypass"]
    #[inline(always)]
    pub fn psmpspb(&self) -> PSMPSPB_R {
        PSMPSPB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sequencial Startup from ULP (Active Low)"]
    #[inline(always)]
    pub fn seqstn(&self) -> SEQSTN_R {
        SEQSTN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Power Scaling Bias Timing Disable"]
    #[inline(always)]
    pub fn psbtd(&self) -> PSBTD_R {
        PSBTD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Power Scaling Halt Flash Until VREGOK Disable"]
    #[inline(always)]
    pub fn pshfd(&self) -> PSHFD_R {
        PSHFD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Delaying Reset Disable"]
    #[inline(always)]
    pub fn dlyrstd(&self) -> DLYRSTD_R {
        DLYRSTD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bias Switch Enable"]
    #[inline(always)]
    pub fn biassen(&self) -> BIASSEN_R {
        BIASSEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Latdel Switch Enable"]
    #[inline(always)]
    pub fn latsen(&self) -> LATSEN_R {
        LATSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BOD18 in continuous mode not disabled in WAIT/RET/BACKUP modes"]
    #[inline(always)]
    pub fn bod18cont(&self) -> BOD18CONT_R {
        BOD18CONT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pico Uart Observability"]
    #[inline(always)]
    pub fn pobs(&self) -> POBS_R {
        POBS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Force Flash Fast Wakeup"]
    #[inline(always)]
    pub fn fffw(&self) -> FFFW_R {
        FFFW_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Flash Bias Ready Enable"]
    #[inline(always)]
    pub fn fbrdyen(&self) -> FBRDYEN_R {
        FBRDYEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Flash Vref Switch Enable"]
    #[inline(always)]
    pub fn fvrefsen(&self) -> FVREFSEN_R {
        FVREFSEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Run Mode Power Scaling Preset Bypass"]
    #[inline(always)]
    pub fn runpspb(&mut self) -> RUNPSPB_W {
        RUNPSPB_W { w: self }
    }
    #[doc = "Bit 1 - Power Save Mode Power Scaling Preset Bypass"]
    #[inline(always)]
    pub fn psmpspb(&mut self) -> PSMPSPB_W {
        PSMPSPB_W { w: self }
    }
    #[doc = "Bit 2 - Sequencial Startup from ULP (Active Low)"]
    #[inline(always)]
    pub fn seqstn(&mut self) -> SEQSTN_W {
        SEQSTN_W { w: self }
    }
    #[doc = "Bit 3 - Power Scaling Bias Timing Disable"]
    #[inline(always)]
    pub fn psbtd(&mut self) -> PSBTD_W {
        PSBTD_W { w: self }
    }
    #[doc = "Bit 4 - Power Scaling Halt Flash Until VREGOK Disable"]
    #[inline(always)]
    pub fn pshfd(&mut self) -> PSHFD_W {
        PSHFD_W { w: self }
    }
    #[doc = "Bit 5 - Delaying Reset Disable"]
    #[inline(always)]
    pub fn dlyrstd(&mut self) -> DLYRSTD_W {
        DLYRSTD_W { w: self }
    }
    #[doc = "Bit 6 - Bias Switch Enable"]
    #[inline(always)]
    pub fn biassen(&mut self) -> BIASSEN_W {
        BIASSEN_W { w: self }
    }
    #[doc = "Bit 7 - Latdel Switch Enable"]
    #[inline(always)]
    pub fn latsen(&mut self) -> LATSEN_W {
        LATSEN_W { w: self }
    }
    #[doc = "Bit 8 - BOD18 in continuous mode not disabled in WAIT/RET/BACKUP modes"]
    #[inline(always)]
    pub fn bod18cont(&mut self) -> BOD18CONT_W {
        BOD18CONT_W { w: self }
    }
    #[doc = "Bit 9 - Pico Uart Observability"]
    #[inline(always)]
    pub fn pobs(&mut self) -> POBS_W {
        POBS_W { w: self }
    }
    #[doc = "Bit 10 - Force Flash Fast Wakeup"]
    #[inline(always)]
    pub fn fffw(&mut self) -> FFFW_W {
        FFFW_W { w: self }
    }
    #[doc = "Bit 11 - Flash Bias Ready Enable"]
    #[inline(always)]
    pub fn fbrdyen(&mut self) -> FBRDYEN_W {
        FBRDYEN_W { w: self }
    }
    #[doc = "Bit 12 - Flash Vref Switch Enable"]
    #[inline(always)]
    pub fn fvrefsen(&mut self) -> FVREFSEN_W {
        FVREFSEN_W { w: self }
    }
}
