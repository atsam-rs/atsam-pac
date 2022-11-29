#[doc = "Register `CMPV5` reader"]
pub struct R(crate::R<CMPV5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPV5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPV5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPV5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPV5` writer"]
pub struct W(crate::W<CMPV5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPV5_SPEC>;
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
impl From<crate::W<CMPV5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPV5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CV` reader - Comparison x Value"]
pub type CV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CV` writer - Comparison x Value"]
pub type CV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMPV5_SPEC, u32, u32, 24, O>;
#[doc = "Field `CVM` reader - Comparison x Value Mode"]
pub type CVM_R = crate::BitReader<bool>;
#[doc = "Field `CVM` writer - Comparison x Value Mode"]
pub type CVM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMPV5_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&self) -> CVM_R {
        CVM_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    #[must_use]
    pub fn cv(&mut self) -> CV_W<0> {
        CV_W::new(self)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cvm(&mut self) -> CVM_W<24> {
        CVM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparison 5 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpv5](index.html) module"]
pub struct CMPV5_SPEC;
impl crate::RegisterSpec for CMPV5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpv5::R](R) reader structure"]
impl crate::Readable for CMPV5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpv5::W](W) writer structure"]
impl crate::Writable for CMPV5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPV5 to value 0"]
impl crate::Resettable for CMPV5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
