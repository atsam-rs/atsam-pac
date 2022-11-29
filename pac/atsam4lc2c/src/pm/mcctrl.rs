#[doc = "Register `MCCTRL` reader"]
pub struct R(crate::R<MCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCCTRL` writer"]
pub struct W(crate::W<MCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCCTRL_SPEC>;
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
impl From<crate::W<MCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCSEL` reader - Main Clock Select"]
pub type MCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MCSEL` writer - Main Clock Select"]
pub type MCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCCTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Main Clock Select"]
    #[inline(always)]
    pub fn mcsel(&self) -> MCSEL_R {
        MCSEL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Main Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn mcsel(&mut self) -> MCSEL_W<0> {
        MCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcctrl](index.html) module"]
pub struct MCCTRL_SPEC;
impl crate::RegisterSpec for MCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcctrl::R](R) reader structure"]
impl crate::Readable for MCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcctrl::W](W) writer structure"]
impl crate::Writable for MCCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCCTRL to value 0"]
impl crate::Resettable for MCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
