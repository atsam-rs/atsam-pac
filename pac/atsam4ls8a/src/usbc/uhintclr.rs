#[doc = "Register `UHINTCLR` writer"]
pub struct W(crate::W<UHINTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHINTCLR_SPEC>;
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
impl From<crate::W<UHINTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UHINTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCONNIC` writer - DCONNI Clear"]
pub struct DCONNIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONNIC_W<'a> {
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
#[doc = "Field `DDISCIC` writer - DDISCI Clear"]
pub struct DDISCIC_W<'a> {
    w: &'a mut W,
}
impl<'a> DDISCIC_W<'a> {
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
#[doc = "Field `RSTIC` writer - RSTI Clear"]
pub struct RSTIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIC_W<'a> {
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
#[doc = "Field `RSMEDIC` writer - RSMEDI Clear"]
pub struct RSMEDIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSMEDIC_W<'a> {
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
#[doc = "Field `RXRSMIC` writer - RXRSMI Clear"]
pub struct RXRSMIC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRSMIC_W<'a> {
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
#[doc = "Field `HSOFIC` writer - HSOFI Clear"]
pub struct HSOFIC_W<'a> {
    w: &'a mut W,
}
impl<'a> HSOFIC_W<'a> {
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
#[doc = "Field `HWUPIC` writer - HWUPI Clear"]
pub struct HWUPIC_W<'a> {
    w: &'a mut W,
}
impl<'a> HWUPIC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - DCONNI Clear"]
    #[inline(always)]
    pub fn dconnic(&mut self) -> DCONNIC_W {
        DCONNIC_W { w: self }
    }
    #[doc = "Bit 1 - DDISCI Clear"]
    #[inline(always)]
    pub fn ddiscic(&mut self) -> DDISCIC_W {
        DDISCIC_W { w: self }
    }
    #[doc = "Bit 2 - RSTI Clear"]
    #[inline(always)]
    pub fn rstic(&mut self) -> RSTIC_W {
        RSTIC_W { w: self }
    }
    #[doc = "Bit 3 - RSMEDI Clear"]
    #[inline(always)]
    pub fn rsmedic(&mut self) -> RSMEDIC_W {
        RSMEDIC_W { w: self }
    }
    #[doc = "Bit 4 - RXRSMI Clear"]
    #[inline(always)]
    pub fn rxrsmic(&mut self) -> RXRSMIC_W {
        RXRSMIC_W { w: self }
    }
    #[doc = "Bit 5 - HSOFI Clear"]
    #[inline(always)]
    pub fn hsofic(&mut self) -> HSOFIC_W {
        HSOFIC_W { w: self }
    }
    #[doc = "Bit 6 - HWUPI Clear"]
    #[inline(always)]
    pub fn hwupic(&mut self) -> HWUPIC_W {
        HWUPIC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Global Interrrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhintclr](index.html) module"]
pub struct UHINTCLR_SPEC;
impl crate::RegisterSpec for UHINTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uhintclr::W](W) writer structure"]
impl crate::Writable for UHINTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UHINTCLR to value 0"]
impl crate::Resettable for UHINTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
