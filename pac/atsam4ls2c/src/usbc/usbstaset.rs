#[doc = "Register `USBSTASET` writer"]
pub struct W(crate::W<USBSTASET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBSTASET_SPEC>;
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
impl From<crate::W<USBSTASET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBSTASET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMACERIS` writer - RAMACERI Set"]
pub type RAMACERIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTASET_SPEC, bool, O>;
#[doc = "Field `VBUSRQS` writer - VBUSRQ Set"]
pub type VBUSRQS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTASET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 8 - RAMACERI Set"]
    #[inline(always)]
    #[must_use]
    pub fn ramaceris(&mut self) -> RAMACERIS_W<8> {
        RAMACERIS_W::new(self)
    }
    #[doc = "Bit 9 - VBUSRQ Set"]
    #[inline(always)]
    #[must_use]
    pub fn vbusrqs(&mut self) -> VBUSRQS_W<9> {
        VBUSRQS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Status Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbstaset](index.html) module"]
pub struct USBSTASET_SPEC;
impl crate::RegisterSpec for USBSTASET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbstaset::W](W) writer structure"]
impl crate::Writable for USBSTASET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBSTASET to value 0"]
impl crate::Resettable for USBSTASET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
