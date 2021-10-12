#[doc = "Register `UESTA4CLR` writer"]
pub struct W(crate::W<UESTA4CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UESTA4CLR_SPEC>;
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
impl From<crate::W<UESTA4CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UESTA4CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXINIC` writer - TXINI Clear"]
pub struct TXINIC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXINIC_W<'a> {
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
#[doc = "Field `RXOUTIC` writer - RXOUTI Clear"]
pub struct RXOUTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOUTIC_W<'a> {
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
#[doc = "Field `RXSTPIC` writer - RXSTPI Clear"]
pub struct RXSTPIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTPIC_W<'a> {
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
#[doc = "Field `NAKOUTIC` writer - NAKOUTI Clear"]
pub struct NAKOUTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKOUTIC_W<'a> {
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
#[doc = "Field `NAKINIC` writer - NAKINI Clear"]
pub struct NAKINIC_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKINIC_W<'a> {
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
#[doc = "Field `STALLEDIC` writer - STALLEDI Clear"]
pub struct STALLEDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> STALLEDIC_W<'a> {
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
#[doc = "Field `RAMACERIC` writer - RAMACERI Clear"]
pub struct RAMACERIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RAMACERIC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - TXINI Clear"]
    #[inline(always)]
    pub fn txinic(&mut self) -> TXINIC_W {
        TXINIC_W { w: self }
    }
    #[doc = "Bit 1 - RXOUTI Clear"]
    #[inline(always)]
    pub fn rxoutic(&mut self) -> RXOUTIC_W {
        RXOUTIC_W { w: self }
    }
    #[doc = "Bit 2 - RXSTPI Clear"]
    #[inline(always)]
    pub fn rxstpic(&mut self) -> RXSTPIC_W {
        RXSTPIC_W { w: self }
    }
    #[doc = "Bit 3 - NAKOUTI Clear"]
    #[inline(always)]
    pub fn nakoutic(&mut self) -> NAKOUTIC_W {
        NAKOUTIC_W { w: self }
    }
    #[doc = "Bit 4 - NAKINI Clear"]
    #[inline(always)]
    pub fn nakinic(&mut self) -> NAKINIC_W {
        NAKINIC_W { w: self }
    }
    #[doc = "Bit 6 - STALLEDI Clear"]
    #[inline(always)]
    pub fn stalledic(&mut self) -> STALLEDIC_W {
        STALLEDIC_W { w: self }
    }
    #[doc = "Bit 11 - RAMACERI Clear"]
    #[inline(always)]
    pub fn ramaceric(&mut self) -> RAMACERIC_W {
        RAMACERIC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta4clr](index.html) module"]
pub struct UESTA4CLR_SPEC;
impl crate::RegisterSpec for UESTA4CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uesta4clr::W](W) writer structure"]
impl crate::Writable for UESTA4CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UESTA4CLR to value 0"]
impl crate::Resettable for UESTA4CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
