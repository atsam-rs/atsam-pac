#[doc = "Register `CMPMUPD6` writer"]
pub struct W(crate::W<CMPMUPD6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPMUPD6_SPEC>;
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
impl From<crate::W<CMPMUPD6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPMUPD6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CENUPD` writer - Comparison x Enable Update"]
pub type CENUPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPMUPD6_SPEC, bool, O>;
#[doc = "Field `CTRUPD` writer - Comparison x Trigger Update"]
pub type CTRUPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPMUPD6_SPEC, u8, u8, 4, O>;
#[doc = "Field `CPRUPD` writer - Comparison x Period Update"]
pub type CPRUPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPMUPD6_SPEC, u8, u8, 4, O>;
#[doc = "Field `CUPRUPD` writer - Comparison x Update Period Update"]
pub type CUPRUPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPMUPD6_SPEC, u8, u8, 4, O>;
impl W {
    #[doc = "Bit 0 - Comparison x Enable Update"]
    #[inline(always)]
    #[must_use]
    pub fn cenupd(&mut self) -> CENUPD_W<0> {
        CENUPD_W::new(self)
    }
    #[doc = "Bits 4:7 - Comparison x Trigger Update"]
    #[inline(always)]
    #[must_use]
    pub fn ctrupd(&mut self) -> CTRUPD_W<4> {
        CTRUPD_W::new(self)
    }
    #[doc = "Bits 8:11 - Comparison x Period Update"]
    #[inline(always)]
    #[must_use]
    pub fn cprupd(&mut self) -> CPRUPD_W<8> {
        CPRUPD_W::new(self)
    }
    #[doc = "Bits 16:19 - Comparison x Update Period Update"]
    #[inline(always)]
    #[must_use]
    pub fn cuprupd(&mut self) -> CUPRUPD_W<16> {
        CUPRUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparison 6 Mode Update Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpmupd6](index.html) module"]
pub struct CMPMUPD6_SPEC;
impl crate::RegisterSpec for CMPMUPD6_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmpmupd6::W](W) writer structure"]
impl crate::Writable for CMPMUPD6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
