#[doc = "Register `HSTR` reader"]
pub struct R(crate::R<HSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTR` writer"]
pub struct W(crate::W<HSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTR_SPEC>;
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
impl From<crate::W<HSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDDAT` reader - Data Hold Cycles"]
pub type HDDAT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HDDAT` writer - Data Hold Cycles"]
pub type HDDAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSTR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 16:23 - Data Hold Cycles"]
    #[inline(always)]
    pub fn hddat(&self) -> HDDAT_R {
        HDDAT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Data Hold Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn hddat(&mut self) -> HDDAT_W<16> {
        HDDAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HS-mode Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstr](index.html) module"]
pub struct HSTR_SPEC;
impl crate::RegisterSpec for HSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstr::R](R) reader structure"]
impl crate::Readable for HSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstr::W](W) writer structure"]
impl crate::Writable for HSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTR to value 0"]
impl crate::Resettable for HSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
