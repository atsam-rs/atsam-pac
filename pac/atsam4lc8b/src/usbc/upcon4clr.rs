#[doc = "Register `UPCON4CLR` writer"]
pub struct W(crate::W<UPCON4CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPCON4CLR_SPEC>;
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
impl From<crate::W<UPCON4CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPCON4CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINEC` writer - RXINE Clear"]
pub struct RXINEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINEC_W<'a> {
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
#[doc = "Field `TXOUTEC` writer - TXOUTE Clear"]
pub struct TXOUTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOUTEC_W<'a> {
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
#[doc = "Field `TXSTPEC` writer - TXSTPE Clear"]
pub struct TXSTPEC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTPEC_W<'a> {
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
#[doc = "Field `PERREC` writer - PERRE Clear"]
pub struct PERREC_W<'a> {
    w: &'a mut W,
}
impl<'a> PERREC_W<'a> {
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
#[doc = "Field `NAKEDEC` writer - NAKEDE Clear"]
pub struct NAKEDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKEDEC_W<'a> {
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
#[doc = "Field `ERRORFIEC` writer - ERRORFIE Clear"]
pub struct ERRORFIEC_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORFIEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RXSTALLDEC` writer - RXTALLDE Clear"]
pub struct RXSTALLDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALLDEC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
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
#[doc = "Field `PFREEZEC` writer - PFREEZE Clear"]
pub struct PFREEZEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PFREEZEC_W<'a> {
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
#[doc = "Field `INITBKC` writer - INITBK Clear"]
pub struct INITBKC_W<'a> {
    w: &'a mut W,
}
impl<'a> INITBKC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - RXINE Clear"]
    #[inline(always)]
    pub fn rxinec(&mut self) -> RXINEC_W {
        RXINEC_W { w: self }
    }
    #[doc = "Bit 1 - TXOUTE Clear"]
    #[inline(always)]
    pub fn txoutec(&mut self) -> TXOUTEC_W {
        TXOUTEC_W { w: self }
    }
    #[doc = "Bit 2 - TXSTPE Clear"]
    #[inline(always)]
    pub fn txstpec(&mut self) -> TXSTPEC_W {
        TXSTPEC_W { w: self }
    }
    #[doc = "Bit 3 - PERRE Clear"]
    #[inline(always)]
    pub fn perrec(&mut self) -> PERREC_W {
        PERREC_W { w: self }
    }
    #[doc = "Bit 4 - NAKEDE Clear"]
    #[inline(always)]
    pub fn nakedec(&mut self) -> NAKEDEC_W {
        NAKEDEC_W { w: self }
    }
    #[doc = "Bit 5 - ERRORFIE Clear"]
    #[inline(always)]
    pub fn errorfiec(&mut self) -> ERRORFIEC_W {
        ERRORFIEC_W { w: self }
    }
    #[doc = "Bit 6 - RXTALLDE Clear"]
    #[inline(always)]
    pub fn rxstalldec(&mut self) -> RXSTALLDEC_W {
        RXSTALLDEC_W { w: self }
    }
    #[doc = "Bit 10 - RAMACERE Clear"]
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
    #[doc = "Bit 17 - PFREEZE Clear"]
    #[inline(always)]
    pub fn pfreezec(&mut self) -> PFREEZEC_W {
        PFREEZEC_W { w: self }
    }
    #[doc = "Bit 19 - INITBK Clear"]
    #[inline(always)]
    pub fn initbkc(&mut self) -> INITBKC_W {
        INITBKC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Control Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon4clr](index.html) module"]
pub struct UPCON4CLR_SPEC;
impl crate::RegisterSpec for UPCON4CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [upcon4clr::W](W) writer structure"]
impl crate::Writable for UPCON4CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UPCON4CLR to value 0"]
impl crate::Resettable for UPCON4CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
