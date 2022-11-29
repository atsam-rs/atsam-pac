#[doc = "Register `CUPD2` writer"]
pub struct W(crate::W<CUPD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CUPD2_SPEC>;
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
impl From<crate::W<CUPD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CUPD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CUPD` writer - "]
pub type CUPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CUPD2_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cupd(&mut self) -> CUPD_W<0> {
        CUPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Update Register (ch_num = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cupd2](index.html) module"]
pub struct CUPD2_SPEC;
impl crate::RegisterSpec for CUPD2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cupd2::W](W) writer structure"]
impl crate::Writable for CUPD2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
