#[doc = "Register `PMC_USB` reader"]
pub struct R(crate::R<PMC_USB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_USB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_USB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_USB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_USB` writer"]
pub struct W(crate::W<PMC_USB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_USB_SPEC>;
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
impl From<crate::W<PMC_USB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_USB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBDIV` reader - Divider for USB Clock"]
pub struct USBDIV_R(crate::FieldReader<u8, u8>);
impl USBDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBDIV` writer - Divider for USB Clock"]
pub struct USBDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> USBDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:11 - Divider for USB Clock"]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Divider for USB Clock"]
    #[inline(always)]
    pub fn usbdiv(&mut self) -> USBDIV_W {
        USBDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_usb](index.html) module"]
pub struct PMC_USB_SPEC;
impl crate::RegisterSpec for PMC_USB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_usb::R](R) reader structure"]
impl crate::Readable for PMC_USB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_usb::W](W) writer structure"]
impl crate::Writable for PMC_USB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_USB to value 0"]
impl crate::Resettable for PMC_USB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
