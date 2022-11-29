#[doc = "Register `RC32KTUNE` reader"]
pub struct R(crate::R<RC32KTUNE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC32KTUNE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC32KTUNE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC32KTUNE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RC32KTUNE` writer"]
pub struct W(crate::W<RC32KTUNE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC32KTUNE_SPEC>;
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
impl From<crate::W<RC32KTUNE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC32KTUNE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINE` reader - Fine value"]
pub type FINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FINE` writer - Fine value"]
pub type FINE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RC32KTUNE_SPEC, u8, u8, 6, O>;
#[doc = "Field `COARSE` reader - Coarse Value"]
pub type COARSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COARSE` writer - Coarse Value"]
pub type COARSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RC32KTUNE_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:5 - Fine value"]
    #[inline(always)]
    pub fn fine(&self) -> FINE_R {
        FINE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - Coarse Value"]
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fine value"]
    #[inline(always)]
    #[must_use]
    pub fn fine(&mut self) -> FINE_W<0> {
        FINE_W::new(self)
    }
    #[doc = "Bits 16:22 - Coarse Value"]
    #[inline(always)]
    #[must_use]
    pub fn coarse(&mut self) -> COARSE_W<16> {
        COARSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32kHz RC Oscillator Tuning Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32ktune](index.html) module"]
pub struct RC32KTUNE_SPEC;
impl crate::RegisterSpec for RC32KTUNE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc32ktune::R](R) reader structure"]
impl crate::Readable for RC32KTUNE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc32ktune::W](W) writer structure"]
impl crate::Writable for RC32KTUNE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC32KTUNE to value 0"]
impl crate::Resettable for RC32KTUNE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
