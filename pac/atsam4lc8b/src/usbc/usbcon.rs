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
pub type FRZCLK_R = crate::BitReader<bool>;
#[doc = "Field `FRZCLK` writer - Freeze USB Clock"]
pub type FRZCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCON_SPEC, bool, O>;
#[doc = "Field `USBE` reader - USBC Enable"]
pub type USBE_R = crate::BitReader<bool>;
#[doc = "Field `USBE` writer - USBC Enable"]
pub type USBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCON_SPEC, bool, O>;
#[doc = "Field `UIMOD` reader - USBC Mode"]
pub type UIMOD_R = crate::BitReader<bool>;
#[doc = "Field `UIMOD` writer - USBC Mode"]
pub type UIMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBCON_SPEC, bool, O>;
impl R {
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    pub fn frzclk(&self) -> FRZCLK_R {
        FRZCLK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - USBC Enable"]
    #[inline(always)]
    pub fn usbe(&self) -> USBE_R {
        USBE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - USBC Mode"]
    #[inline(always)]
    pub fn uimod(&self) -> UIMOD_R {
        UIMOD_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Freeze USB Clock"]
    #[inline(always)]
    #[must_use]
    pub fn frzclk(&mut self) -> FRZCLK_W<14> {
        FRZCLK_W::new(self)
    }
    #[doc = "Bit 15 - USBC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbe(&mut self) -> USBE_W<15> {
        USBE_W::new(self)
    }
    #[doc = "Bit 24 - USBC Mode"]
    #[inline(always)]
    #[must_use]
    pub fn uimod(&mut self) -> UIMOD_W<24> {
        UIMOD_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCON to value 0x0100_4000"]
impl crate::Resettable for USBCON_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_4000;
}
