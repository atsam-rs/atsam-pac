#[doc = "Register `TIMING` reader"]
pub struct R(crate::R<TIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMING` writer"]
pub struct W(crate::W<TIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMING_SPEC>;
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
impl From<crate::W<TIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLEVEL` reader - Relative Level Smoothing"]
pub type TLEVEL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TLEVEL` writer - Relative Level Smoothing"]
pub type TLEVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMING_SPEC, u16, u16, 12, O>;
#[doc = "Field `TIDLE` reader - Idle Smoothening"]
pub type TIDLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIDLE` writer - Idle Smoothening"]
pub type TIDLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMING_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Relative Level Smoothing"]
    #[inline(always)]
    pub fn tlevel(&self) -> TLEVEL_R {
        TLEVEL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Idle Smoothening"]
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Relative Level Smoothing"]
    #[inline(always)]
    #[must_use]
    pub fn tlevel(&mut self) -> TLEVEL_W<0> {
        TLEVEL_W::new(self)
    }
    #[doc = "Bits 16:27 - Idle Smoothening"]
    #[inline(always)]
    #[must_use]
    pub fn tidle(&mut self) -> TIDLE_W<16> {
        TIDLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Filter Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timing](index.html) module"]
pub struct TIMING_SPEC;
impl crate::RegisterSpec for TIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timing::R](R) reader structure"]
impl crate::Readable for TIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timing::W](W) writer structure"]
impl crate::Writable for TIMING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMING to value 0"]
impl crate::Resettable for TIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
