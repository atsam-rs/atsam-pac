#[doc = "Register `CPRDUPD1` writer"]
pub struct W(crate::W<CPRDUPD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPRDUPD1_SPEC>;
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
impl From<crate::W<CPRDUPD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPRDUPD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPRDUPD` writer - Channel Period Update"]
pub type CPRDUPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPRDUPD1_SPEC, u32, u32, 24, O>;
impl W {
    #[doc = "Bits 0:23 - Channel Period Update"]
    #[inline(always)]
    #[must_use]
    pub fn cprdupd(&mut self) -> CPRDUPD_W<0> {
        CPRDUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Period Update Register (ch_num = 1)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprdupd1](index.html) module"]
pub struct CPRDUPD1_SPEC;
impl crate::RegisterSpec for CPRDUPD1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cprdupd1::W](W) writer structure"]
impl crate::Writable for CPRDUPD1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
