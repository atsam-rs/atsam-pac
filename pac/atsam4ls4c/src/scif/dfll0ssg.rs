#[doc = "Register `DFLL0SSG` reader"]
pub struct R(crate::R<DFLL0SSG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLL0SSG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLL0SSG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLL0SSG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLL0SSG` writer"]
pub struct W(crate::W<DFLL0SSG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLL0SSG_SPEC>;
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
impl From<crate::W<DFLL0SSG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLL0SSG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFLL0SSG_SPEC, bool, O>;
#[doc = "Field `PRBS` writer - Pseudo Random Bit Sequence"]
pub type PRBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFLL0SSG_SPEC, bool, O>;
#[doc = "Field `AMPLITUDE` writer - SSG Amplitude"]
pub type AMPLITUDE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFLL0SSG_SPEC, u8, u8, 5, O>;
#[doc = "Field `STEPSIZE` writer - SSG Step Size"]
pub type STEPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFLL0SSG_SPEC, u8, u8, 5, O>;
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Pseudo Random Bit Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn prbs(&mut self) -> PRBS_W<1> {
        PRBS_W::new(self)
    }
    #[doc = "Bits 8:12 - SSG Amplitude"]
    #[inline(always)]
    #[must_use]
    pub fn amplitude(&mut self) -> AMPLITUDE_W<8> {
        AMPLITUDE_W::new(self)
    }
    #[doc = "Bits 16:20 - SSG Step Size"]
    #[inline(always)]
    #[must_use]
    pub fn stepsize(&mut self) -> STEPSIZE_W<16> {
        STEPSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL0 Spread Spectrum Generator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0ssg](index.html) module"]
pub struct DFLL0SSG_SPEC;
impl crate::RegisterSpec for DFLL0SSG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfll0ssg::R](R) reader structure"]
impl crate::Readable for DFLL0SSG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfll0ssg::W](W) writer structure"]
impl crate::Writable for DFLL0SSG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFLL0SSG to value 0"]
impl crate::Resettable for DFLL0SSG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
