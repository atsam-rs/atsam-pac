#[doc = "Register `UECON0CLR` writer"]
pub struct W(crate::W<UECON0CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UECON0CLR_SPEC>;
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
impl From<crate::W<UECON0CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UECON0CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINEC` writer - TXINE Clear"]
pub struct TXINEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINEC_W<'a> {
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
#[doc = "Field `RXOUTEC` writer - RXOUTE Clear"]
pub struct RXOUTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOUTEC_W<'a> {
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
#[doc = "Field `RXSTPEC` writer - RXSTPE Clear"]
pub struct RXSTPEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTPEC_W<'a> {
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
#[doc = "Field `NAKOUTEC` writer - NAKOUTE Clear"]
pub struct NAKOUTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKOUTEC_W<'a> {
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
#[doc = "Field `NAKINEC` writer - NAKINE Clear"]
pub struct NAKINEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKINEC_W<'a> {
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
#[doc = "Field `STALLEDEC` writer - STALLEDE Clear"]
pub struct STALLEDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEDEC_W<'a> {
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
#[doc = "Field `NREPLYC` writer - NREPLY Clear"]
pub struct NREPLYC_W<'a> {
    w: &'a mut W,
}
impl<'a> NREPLYC_W<'a> {
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
#[doc = "Field `RAMACEREC` writer - RAMACERE Clear"]
pub struct RAMACEREC_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACEREC_W<'a> {
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
#[doc = "Field `NBUSYBKEC` writer - NBUSYBKE Clear"]
pub struct NBUSYBKEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NBUSYBKEC_W<'a> {
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
#[doc = "Field `FIFOCONC` writer - FIFOCON Clear"]
pub struct FIFOCONC_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFOCONC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `NYETDISC` writer - NYETDIS Clear"]
pub struct NYETDISC_W<'a> {
    w: &'a mut W,
}
impl<'a> NYETDISC_W<'a> {
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
#[doc = "Field `STALLRQC` writer - STALLRQ Clear"]
pub struct STALLRQC_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLRQC_W<'a> {
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
#[doc = "Field `BUSY0C` writer - BUSY0 Clear"]
pub struct BUSY0C_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY0C_W<'a> {
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
#[doc = "Field `BUSY1C` writer - BUSY1 Clear"]
pub struct BUSY1C_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY1C_W<'a> {
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
    #[doc = "Bit 0 - TXINE Clear"]
    #[inline(always)]
    pub fn txinec(&mut self) -> TXINEC_W {
        TXINEC_W { w: self }
    }
    #[doc = "Bit 1 - RXOUTE Clear"]
    #[inline(always)]
    pub fn rxoutec(&mut self) -> RXOUTEC_W {
        RXOUTEC_W { w: self }
    }
    #[doc = "Bit 2 - RXSTPE Clear"]
    #[inline(always)]
    pub fn rxstpec(&mut self) -> RXSTPEC_W {
        RXSTPEC_W { w: self }
    }
    #[doc = "Bit 3 - NAKOUTE Clear"]
    #[inline(always)]
    pub fn nakoutec(&mut self) -> NAKOUTEC_W {
        NAKOUTEC_W { w: self }
    }
    #[doc = "Bit 4 - NAKINE Clear"]
    #[inline(always)]
    pub fn nakinec(&mut self) -> NAKINEC_W {
        NAKINEC_W { w: self }
    }
    #[doc = "Bit 6 - STALLEDE Clear"]
    #[inline(always)]
    pub fn stalledec(&mut self) -> STALLEDEC_W {
        STALLEDEC_W { w: self }
    }
    #[doc = "Bit 8 - NREPLY Clear"]
    #[inline(always)]
    pub fn nreplyc(&mut self) -> NREPLYC_W {
        NREPLYC_W { w: self }
    }
    #[doc = "Bit 11 - RAMACERE Clear"]
    #[inline(always)]
    pub fn ramacerec(&mut self) -> RAMACEREC_W {
        RAMACEREC_W { w: self }
    }
    #[doc = "Bit 12 - NBUSYBKE Clear"]
    #[inline(always)]
    pub fn nbusybkec(&mut self) -> NBUSYBKEC_W {
        NBUSYBKEC_W { w: self }
    }
    #[doc = "Bit 14 - FIFOCON Clear"]
    #[inline(always)]
    pub fn fifoconc(&mut self) -> FIFOCONC_W {
        FIFOCONC_W { w: self }
    }
    #[doc = "Bit 17 - NYETDIS Clear"]
    #[inline(always)]
    pub fn nyetdisc(&mut self) -> NYETDISC_W {
        NYETDISC_W { w: self }
    }
    #[doc = "Bit 19 - STALLRQ Clear"]
    #[inline(always)]
    pub fn stallrqc(&mut self) -> STALLRQC_W {
        STALLRQC_W { w: self }
    }
    #[doc = "Bit 24 - BUSY0 Clear"]
    #[inline(always)]
    pub fn busy0c(&mut self) -> BUSY0C_W {
        BUSY0C_W { w: self }
    }
    #[doc = "Bit 25 - BUSY1 Clear"]
    #[inline(always)]
    pub fn busy1c(&mut self) -> BUSY1C_W {
        BUSY1C_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Control Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon0clr](index.html) module"]
pub struct UECON0CLR_SPEC;
impl crate::RegisterSpec for UECON0CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uecon0clr::W](W) writer structure"]
impl crate::Writable for UECON0CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UECON0CLR to value 0"]
impl crate::Resettable for UECON0CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
