#[doc = "Register `CPRD1` reader"]
pub struct R(crate::R<CPRD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPRD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPRD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPRD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPRD1` writer"]
pub struct W(crate::W<CPRD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPRD1_SPEC>;
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
impl From<crate::W<CPRD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPRD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPRD` reader - Channel Period"]
pub type CPRD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CPRD` writer - Channel Period"]
pub type CPRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPRD1_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    pub fn cprd(&self) -> CPRD_R {
        CPRD_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Period"]
    #[inline(always)]
    #[must_use]
    pub fn cprd(&mut self) -> CPRD_W<0> {
        CPRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Channel Period Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cprd1](index.html) module"]
pub struct CPRD1_SPEC;
impl crate::RegisterSpec for CPRD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cprd1::R](R) reader structure"]
impl crate::Readable for CPRD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cprd1::W](W) writer structure"]
impl crate::Writable for CPRD1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPRD1 to value 0"]
impl crate::Resettable for CPRD1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
