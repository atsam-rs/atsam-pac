#[doc = "Register `MARR%s` reader"]
pub struct R(crate::R<MARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MARR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MARR%s` writer"]
pub struct W(crate::W<MARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MARR_SPEC>;
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
impl From<crate::W<MARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MARR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MARV` reader - Memory Address Reload Value"]
pub type MARV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MARV` writer - Memory Address Reload Value"]
pub type MARV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MARR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Memory Address Reload Value"]
    #[inline(always)]
    pub fn marv(&self) -> MARV_R {
        MARV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory Address Reload Value"]
    #[inline(always)]
    #[must_use]
    pub fn marv(&mut self) -> MARV_W<0> {
        MARV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Address Reload Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [marr](index.html) module"]
pub struct MARR_SPEC;
impl crate::RegisterSpec for MARR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [marr::R](R) reader structure"]
impl crate::Readable for MARR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [marr::W](W) writer structure"]
impl crate::Writable for MARR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MARR%s to value 0"]
impl crate::Resettable for MARR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
