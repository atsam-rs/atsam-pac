#[doc = "Register `UPSTA5SET` writer"]
pub struct W(crate::W<UPSTA5SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPSTA5SET_SPEC>;
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
impl From<crate::W<UPSTA5SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPSTA5SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXINIS` writer - RXINI Set"]
pub struct RXINIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXINIS_W<'a> {
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
#[doc = "Field `TXOUTIS` writer - TXOUTI Set"]
pub struct TXOUTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOUTIS_W<'a> {
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
#[doc = "Field `TXSTPIS` writer - TXSTPI Set"]
pub struct TXSTPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTPIS_W<'a> {
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
#[doc = "Field `PERRIS` writer - PERRI Set"]
pub struct PERRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> PERRIS_W<'a> {
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
#[doc = "Field `NAKEDIS` writer - NAKEDI Set"]
pub struct NAKEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKEDIS_W<'a> {
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
#[doc = "Field `ERRORFIS` writer - ERRORFI Set"]
pub struct ERRORFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORFIS_W<'a> {
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
#[doc = "Field `RXSTALLDIS` writer - RXSTALLDI Set"]
pub struct RXSTALLDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTALLDIS_W<'a> {
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
#[doc = "Field `RAMACERIS` writer - RAMACERI Set"]
pub struct RAMACERIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACERIS_W<'a> {
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
impl W {
    #[doc = "Bit 0 - RXINI Set"]
    #[inline(always)]
    pub fn rxinis(&mut self) -> RXINIS_W {
        RXINIS_W { w: self }
    }
    #[doc = "Bit 1 - TXOUTI Set"]
    #[inline(always)]
    pub fn txoutis(&mut self) -> TXOUTIS_W {
        TXOUTIS_W { w: self }
    }
    #[doc = "Bit 2 - TXSTPI Set"]
    #[inline(always)]
    pub fn txstpis(&mut self) -> TXSTPIS_W {
        TXSTPIS_W { w: self }
    }
    #[doc = "Bit 3 - PERRI Set"]
    #[inline(always)]
    pub fn perris(&mut self) -> PERRIS_W {
        PERRIS_W { w: self }
    }
    #[doc = "Bit 4 - NAKEDI Set"]
    #[inline(always)]
    pub fn nakedis(&mut self) -> NAKEDIS_W {
        NAKEDIS_W { w: self }
    }
    #[doc = "Bit 5 - ERRORFI Set"]
    #[inline(always)]
    pub fn errorfis(&mut self) -> ERRORFIS_W {
        ERRORFIS_W { w: self }
    }
    #[doc = "Bit 6 - RXSTALLDI Set"]
    #[inline(always)]
    pub fn rxstalldis(&mut self) -> RXSTALLDIS_W {
        RXSTALLDIS_W { w: self }
    }
    #[doc = "Bit 10 - RAMACERI Set"]
    #[inline(always)]
    pub fn ramaceris(&mut self) -> RAMACERIS_W {
        RAMACERIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta5set](index.html) module"]
pub struct UPSTA5SET_SPEC;
impl crate::RegisterSpec for UPSTA5SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [upsta5set::W](W) writer structure"]
impl crate::Writable for UPSTA5SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UPSTA5SET to value 0"]
impl crate::Resettable for UPSTA5SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
