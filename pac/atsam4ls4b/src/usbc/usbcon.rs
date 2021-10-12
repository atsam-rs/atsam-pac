#[doc = "Register `USBCON` reader"]
pub struct R(crate::R<USBCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCON` writer"]
pub struct W(crate::W<USBCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCON_SPEC>;
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
impl From<crate::W<USBCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRZCLK` reader - Freeze USB Clock"]
pub struct FRZCLK_R(crate::FieldReader<bool, bool>);
impl FRZCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRZCLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRZCLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRZCLK` writer - Freeze USB Clock"]
pub struct FRZCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> FRZCLK_W<'a> {
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
#[doc = "Field `USBE` reader - USBC Enable"]
pub struct USBE_R(crate::FieldReader<bool, bool>);
impl USBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBE` writer - USBC Enable"]
pub struct USBE_W<'a> {
    w: &'a mut W,
}
impl<'a> USBE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `UIMOD` reader - USBC Mode"]
pub struct UIMOD_R(crate::FieldReader<bool, bool>);
impl UIMOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        UIMOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UIMOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UIMOD` writer - USBC Mode"]
pub struct UIMOD_W<'a> {
    w: &'a mut W,
}
impl<'a> UIMOD_W<'a> {
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
impl R {
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&self) -> FRZCLK_R {
        FRZCLK_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - USBC Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> USBE_R {
        USBE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - USBC Mode"]
    #[inline(always)]
    pub fn uimod(&self) -> UIMOD_R {
        UIMOD_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&mut self) -> FRZCLK_W {
        FRZCLK_W { w: self }
    }
    #[doc = "Bit 15 - USBC Enable"]
    #[inline(always)]
    pub fn usbe(&mut self) -> USBE_W {
        USBE_W { w: self }
    }
    #[doc = "Bit 24 - USBC Mode"]
    #[inline(always)]
    pub fn uimod(&mut self) -> UIMOD_W {
        UIMOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbcon](index.html) module"]
pub struct USBCON_SPEC;
impl crate::RegisterSpec for USBCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbcon::R](R) reader structure"]
impl crate::Readable for USBCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbcon::W](W) writer structure"]
impl crate::Writable for USBCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBCON to value 0x0100_4000"]
impl crate::Resettable for USBCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_4000
    }
}
