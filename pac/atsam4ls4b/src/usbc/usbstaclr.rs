#[doc = "Register `USBSTACLR` writer"]
pub struct W(crate::W<USBSTACLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBSTACLR_SPEC>;
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
impl From<crate::W<USBSTACLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBSTACLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMACERIC` writer - RAMACERI Clear"]
pub type RAMACERIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTACLR_SPEC, bool, O>;
#[doc = "Field `VBUSRQC` writer - VBUSRQ Clear"]
pub type VBUSRQC_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTACLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 8 - RAMACERI Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ramaceric(&mut self) -> RAMACERIC_W<8> {
        RAMACERIC_W::new(self)
    }
    #[doc = "Bit 9 - VBUSRQ Clear"]
    #[inline(always)]
    #[must_use]
    pub fn vbusrqc(&mut self) -> VBUSRQC_W<9> {
        VBUSRQC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbstaclr](index.html) module"]
pub struct USBSTACLR_SPEC;
impl crate::RegisterSpec for USBSTACLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [usbstaclr::W](W) writer structure"]
impl crate::Writable for USBSTACLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBSTACLR to value 0"]
impl crate::Resettable for USBSTACLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
