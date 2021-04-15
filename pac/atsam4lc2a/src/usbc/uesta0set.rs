#[doc = "Register `UESTA0SET` writer"]
pub struct W(crate::W<UESTA0SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UESTA0SET_SPEC>;
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
impl core::convert::From<crate::W<UESTA0SET_SPEC>> for W {
    fn from(writer: crate::W<UESTA0SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINIS` writer - TXINI Set"]
pub struct TXINIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINIS_W<'a> {
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
#[doc = "Field `RXOUTIS` writer - RXOUTI Set"]
pub struct RXOUTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOUTIS_W<'a> {
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
#[doc = "Field `RXSTPIS` writer - RXSTPI Set"]
pub struct RXSTPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTPIS_W<'a> {
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
#[doc = "Field `NAKOUTIS` writer - NAKOUTI Set"]
pub struct NAKOUTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKOUTIS_W<'a> {
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
#[doc = "Field `NAKINIS` writer - NAKINI Set"]
pub struct NAKINIS_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKINIS_W<'a> {
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
#[doc = "Field `STALLEDIS` writer - STALLEDI Set"]
pub struct STALLEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `NBUSYBKS` writer - NBUSYBK Set"]
pub struct NBUSYBKS_W<'a> {
    w: &'a mut W,
}
impl<'a> NBUSYBKS_W<'a> {
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
impl W {
    #[doc = "Bit 0 - TXINI Set"]
    #[inline(always)]
    pub fn txinis(&mut self) -> TXINIS_W {
        TXINIS_W { w: self }
    }
    #[doc = "Bit 1 - RXOUTI Set"]
    #[inline(always)]
    pub fn rxoutis(&mut self) -> RXOUTIS_W {
        RXOUTIS_W { w: self }
    }
    #[doc = "Bit 2 - RXSTPI Set"]
    #[inline(always)]
    pub fn rxstpis(&mut self) -> RXSTPIS_W {
        RXSTPIS_W { w: self }
    }
    #[doc = "Bit 3 - NAKOUTI Set"]
    #[inline(always)]
    pub fn nakoutis(&mut self) -> NAKOUTIS_W {
        NAKOUTIS_W { w: self }
    }
    #[doc = "Bit 4 - NAKINI Set"]
    #[inline(always)]
    pub fn nakinis(&mut self) -> NAKINIS_W {
        NAKINIS_W { w: self }
    }
    #[doc = "Bit 6 - STALLEDI Set"]
    #[inline(always)]
    pub fn stalledis(&mut self) -> STALLEDIS_W {
        STALLEDIS_W { w: self }
    }
    #[doc = "Bit 11 - RAMACERI Set"]
    #[inline(always)]
    pub fn ramaceris(&mut self) -> RAMACERIS_W {
        RAMACERIS_W { w: self }
    }
    #[doc = "Bit 12 - NBUSYBK Set"]
    #[inline(always)]
    pub fn nbusybks(&mut self) -> NBUSYBKS_W {
        NBUSYBKS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta0set](index.html) module"]
pub struct UESTA0SET_SPEC;
impl crate::RegisterSpec for UESTA0SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uesta0set::W](W) writer structure"]
impl crate::Writable for UESTA0SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UESTA0SET to value 0"]
impl crate::Resettable for UESTA0SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
