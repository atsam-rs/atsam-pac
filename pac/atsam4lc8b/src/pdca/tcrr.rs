#[doc = "Register `TCRR%s` reader"]
pub struct R(crate::R<TCRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCRR%s` writer"]
pub struct W(crate::W<TCRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCRR_SPEC>;
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
impl From<crate::W<TCRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCRV` reader - Transfer Counter Reload Value"]
pub type TCRV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TCRV` writer - Transfer Counter Reload Value"]
pub type TCRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCRR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Transfer Counter Reload Value"]
    #[inline(always)]
    pub fn tcrv(&self) -> TCRV_R {
        TCRV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transfer Counter Reload Value"]
    #[inline(always)]
    #[must_use]
    pub fn tcrv(&mut self) -> TCRV_W<0> {
        TCRV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Counter Reload Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcrr](index.html) module"]
pub struct TCRR_SPEC;
impl crate::RegisterSpec for TCRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcrr::R](R) reader structure"]
impl crate::Readable for TCRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcrr::W](W) writer structure"]
impl crate::Writable for TCRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCRR%s to value 0"]
impl crate::Resettable for TCRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
