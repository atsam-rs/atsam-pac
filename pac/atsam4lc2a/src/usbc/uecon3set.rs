#[doc = "Register `UECON3SET` writer"]
pub struct W(crate::W<UECON3SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UECON3SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<UECON3SET_SPEC>> for W {
    fn from(writer: crate::W<UECON3SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINES` writer - TXINE Set"]
pub struct TXINES_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINES_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `RXOUTES` writer - RXOUTE Set"]
pub struct RXOUTES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOUTES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RXSTPES` writer - RXSTPE Set"]
pub struct RXSTPES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTPES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `NAKOUTES` writer - NAKOUTE Set"]
pub struct NAKOUTES_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKOUTES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `NAKINES` writer - NAKINE Set"]
pub struct NAKINES_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKINES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `STALLEDES` writer - STALLEDE Set"]
pub struct STALLEDES_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEDES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `NREPLYS` writer - NREPLY Set"]
pub struct NREPLYS_W<'a> {
    w: &'a mut W,
}
impl<'a> NREPLYS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RAMACERES` writer - RAMACERE Set"]
pub struct RAMACERES_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACERES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `NBUSYBKES` writer - NBUSYBKE Set"]
pub struct NBUSYBKES_W<'a> {
    w: &'a mut W,
}
impl<'a> NBUSYBKES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `KILLBKS` writer - KILLBK Set"]
pub struct KILLBKS_W<'a> {
    w: &'a mut W,
}
impl<'a> KILLBKS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `NYETDISS` writer - NYETDIS Set"]
pub struct NYETDISS_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETDISS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `RSTDTS` writer - RSTDT Set"]
pub struct RSTDTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTDTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `STALLRQS` writer - STALLRQ Set"]
pub struct STALLRQS_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLRQS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `BUSY0S` writer - BUSY0 Set"]
pub struct BUSY0S_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY0S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `BUSY1S` writer - BUSY1 Set"]
pub struct BUSY1S_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY1S_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - TXINE Set"]
    #[inline(always)]
    pub fn txines(&mut self) -> TXINES_W {
        TXINES_W { w: self }
    }
    #[doc = "Bit 1 - RXOUTE Set"]
    #[inline(always)]
    pub fn rxoutes(&mut self) -> RXOUTES_W {
        RXOUTES_W { w: self }
    }
    #[doc = "Bit 2 - RXSTPE Set"]
    #[inline(always)]
    pub fn rxstpes(&mut self) -> RXSTPES_W {
        RXSTPES_W { w: self }
    }
    #[doc = "Bit 3 - NAKOUTE Set"]
    #[inline(always)]
    pub fn nakoutes(&mut self) -> NAKOUTES_W {
        NAKOUTES_W { w: self }
    }
    #[doc = "Bit 4 - NAKINE Set"]
    #[inline(always)]
    pub fn nakines(&mut self) -> NAKINES_W {
        NAKINES_W { w: self }
    }
    #[doc = "Bit 6 - STALLEDE Set"]
    #[inline(always)]
    pub fn stalledes(&mut self) -> STALLEDES_W {
        STALLEDES_W { w: self }
    }
    #[doc = "Bit 8 - NREPLY Set"]
    #[inline(always)]
    pub fn nreplys(&mut self) -> NREPLYS_W {
        NREPLYS_W { w: self }
    }
    #[doc = "Bit 11 - RAMACERE Set"]
    #[inline(always)]
    pub fn ramaceres(&mut self) -> RAMACERES_W {
        RAMACERES_W { w: self }
    }
    #[doc = "Bit 12 - NBUSYBKE Set"]
    #[inline(always)]
    pub fn nbusybkes(&mut self) -> NBUSYBKES_W {
        NBUSYBKES_W { w: self }
    }
    #[doc = "Bit 13 - KILLBK Set"]
    #[inline(always)]
    pub fn killbks(&mut self) -> KILLBKS_W {
        KILLBKS_W { w: self }
    }
    #[doc = "Bit 17 - NYETDIS Set"]
    #[inline(always)]
    pub fn nyetdiss(&mut self) -> NYETDISS_W {
        NYETDISS_W { w: self }
    }
    #[doc = "Bit 18 - RSTDT Set"]
    #[inline(always)]
    pub fn rstdts(&mut self) -> RSTDTS_W {
        RSTDTS_W { w: self }
    }
    #[doc = "Bit 19 - STALLRQ Set"]
    #[inline(always)]
    pub fn stallrqs(&mut self) -> STALLRQS_W {
        STALLRQS_W { w: self }
    }
    #[doc = "Bit 24 - BUSY0 Set"]
    #[inline(always)]
    pub fn busy0s(&mut self) -> BUSY0S_W {
        BUSY0S_W { w: self }
    }
    #[doc = "Bit 25 - BUSY1 Set"]
    #[inline(always)]
    pub fn busy1s(&mut self) -> BUSY1S_W {
        BUSY1S_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Control Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon3set](index.html) module"]
pub struct UECON3SET_SPEC;
impl crate::RegisterSpec for UECON3SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uecon3set::W](W) writer structure"]
impl crate::Writable for UECON3SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UECON3SET to value 0"]
impl crate::Resettable for UECON3SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
