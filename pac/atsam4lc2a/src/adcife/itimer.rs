#[doc = "Register `ITIMER` reader"]
pub struct R(crate::R<ITIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ITIMER` writer"]
pub struct W(crate::W<ITIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ITIMER_SPEC>;
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
impl From<crate::W<ITIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ITIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITMC` reader - Internal timer max counter"]
pub struct ITMC_R(crate::FieldReader<u16, u16>);
impl ITMC_R {
    pub(crate) fn new(bits: u16) -> Self {
        ITMC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITMC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITMC` writer - Internal timer max counter"]
pub struct ITMC_W<'a> {
    w: &'a mut W,
}
impl<'a> ITMC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Internal timer max counter"]
    #[inline(always)]
    pub fn itmc(&self) -> ITMC_R {
        ITMC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Internal timer max counter"]
    #[inline(always)]
    pub fn itmc(&mut self) -> ITMC_W {
        ITMC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itimer](index.html) module"]
pub struct ITIMER_SPEC;
impl crate::RegisterSpec for ITIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itimer::R](R) reader structure"]
impl crate::Readable for ITIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [itimer::W](W) writer structure"]
impl crate::Writable for ITIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ITIMER to value 0"]
impl crate::Resettable for ITIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
