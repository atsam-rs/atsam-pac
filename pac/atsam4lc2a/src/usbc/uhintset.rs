#[doc = "Register `UHINTSET` writer"]
pub struct W(crate::W<UHINTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHINTSET_SPEC>;
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
impl core::convert::From<crate::W<UHINTSET_SPEC>> for W {
    fn from(writer: crate::W<UHINTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCONNIS` writer - DCONNI Set"]
pub struct DCONNIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONNIS_W<'a> {
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
#[doc = "Field `DDISCIS` writer - DDISCI Set"]
pub struct DDISCIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DDISCIS_W<'a> {
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
#[doc = "Field `RSTIS` writer - RSTI Set"]
pub struct RSTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIS_W<'a> {
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
#[doc = "Field `RSMEDIS` writer - RSMEDI Set"]
pub struct RSMEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RSMEDIS_W<'a> {
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
#[doc = "Field `RXRSMIS` writer - RXRSMI Set"]
pub struct RXRSMIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRSMIS_W<'a> {
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
#[doc = "Field `HSOFIS` writer - HSOFI Set"]
pub struct HSOFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HSOFIS_W<'a> {
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
#[doc = "Field `HWUPIS` writer - HWUPI Set"]
pub struct HWUPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HWUPIS_W<'a> {
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
    #[doc = "Bit 0 - DCONNI Set"]
    #[inline(always)]
    pub fn dconnis(&mut self) -> DCONNIS_W {
        DCONNIS_W { w: self }
    }
    #[doc = "Bit 1 - DDISCI Set"]
    #[inline(always)]
    pub fn ddiscis(&mut self) -> DDISCIS_W {
        DDISCIS_W { w: self }
    }
    #[doc = "Bit 2 - RSTI Set"]
    #[inline(always)]
    pub fn rstis(&mut self) -> RSTIS_W {
        RSTIS_W { w: self }
    }
    #[doc = "Bit 3 - RSMEDI Set"]
    #[inline(always)]
    pub fn rsmedis(&mut self) -> RSMEDIS_W {
        RSMEDIS_W { w: self }
    }
    #[doc = "Bit 4 - RXRSMI Set"]
    #[inline(always)]
    pub fn rxrsmis(&mut self) -> RXRSMIS_W {
        RXRSMIS_W { w: self }
    }
    #[doc = "Bit 5 - HSOFI Set"]
    #[inline(always)]
    pub fn hsofis(&mut self) -> HSOFIS_W {
        HSOFIS_W { w: self }
    }
    #[doc = "Bit 6 - HWUPI Set"]
    #[inline(always)]
    pub fn hwupis(&mut self) -> HWUPIS_W {
        HWUPIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Global Interrupt Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhintset](index.html) module"]
pub struct UHINTSET_SPEC;
impl crate::RegisterSpec for UHINTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uhintset::W](W) writer structure"]
impl crate::Writable for UHINTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UHINTSET to value 0"]
impl crate::Resettable for UHINTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
