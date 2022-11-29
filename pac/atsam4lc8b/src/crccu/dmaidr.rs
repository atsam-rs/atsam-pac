#[doc = "Register `DMAIDR` writer"]
pub struct W(crate::W<DMAIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAIDR_SPEC>;
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
impl From<crate::W<DMAIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAIDR` writer - DMA Interrupt Disable"]
pub type DMAIDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMAIDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - DMA Interrupt Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaidr(&mut self) -> DMAIDR_W<0> {
        DMAIDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmaidr](index.html) module"]
pub struct DMAIDR_SPEC;
impl crate::RegisterSpec for DMAIDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dmaidr::W](W) writer structure"]
impl crate::Writable for DMAIDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMAIDR to value 0"]
impl crate::Resettable for DMAIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
