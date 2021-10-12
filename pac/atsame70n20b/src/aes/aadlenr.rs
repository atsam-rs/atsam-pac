#[doc = "Register `AADLENR` reader"]
pub struct R(crate::R<AADLENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AADLENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AADLENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AADLENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AADLENR` writer"]
pub struct W(crate::W<AADLENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AADLENR_SPEC>;
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
impl From<crate::W<AADLENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AADLENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AADLEN` reader - Additional Authenticated Data Length"]
pub struct AADLEN_R(crate::FieldReader<u32, u32>);
impl AADLEN_R {
    pub(crate) fn new(bits: u32) -> Self {
        AADLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AADLEN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AADLEN` writer - Additional Authenticated Data Length"]
pub struct AADLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AADLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Additional Authenticated Data Length"]
    #[inline(always)]
    pub fn aadlen(&self) -> AADLEN_R {
        AADLEN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Additional Authenticated Data Length"]
    #[inline(always)]
    pub fn aadlen(&mut self) -> AADLEN_W {
        AADLEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Additional Authenticated Data Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aadlenr](index.html) module"]
pub struct AADLENR_SPEC;
impl crate::RegisterSpec for AADLENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aadlenr::R](R) reader structure"]
impl crate::Readable for AADLENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aadlenr::W](W) writer structure"]
impl crate::Writable for AADLENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AADLENR to value 0"]
impl crate::Resettable for AADLENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
