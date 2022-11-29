#[doc = "Register `BOD33SAMPLING` reader"]
pub struct R(crate::R<BOD33SAMPLING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD33SAMPLING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD33SAMPLING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD33SAMPLING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD33SAMPLING` writer"]
pub struct W(crate::W<BOD33SAMPLING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD33SAMPLING_SPEC>;
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
impl From<crate::W<BOD33SAMPLING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD33SAMPLING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEN` reader - Clock Enable"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - Clock Enable"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD33SAMPLING_SPEC, bool, O>;
#[doc = "Field `CSSEL` reader - Clock Source Select"]
pub type CSSEL_R = crate::BitReader<bool>;
#[doc = "Field `CSSEL` writer - Clock Source Select"]
pub type CSSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD33SAMPLING_SPEC, bool, O>;
#[doc = "Field `PSEL` reader - Prescaler Select"]
pub type PSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PSEL` writer - Prescaler Select"]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOD33SAMPLING_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock Source Select"]
    #[inline(always)]
    pub fn cssel(&self) -> CSSEL_R {
        CSSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<0> {
        CEN_W::new(self)
    }
    #[doc = "Bit 1 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn cssel(&mut self) -> CSSEL_W<1> {
        CSSEL_W::new(self)
    }
    #[doc = "Bits 8:11 - Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<8> {
        PSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD33 Sampling Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod33sampling](index.html) module"]
pub struct BOD33SAMPLING_SPEC;
impl crate::RegisterSpec for BOD33SAMPLING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod33sampling::R](R) reader structure"]
impl crate::Readable for BOD33SAMPLING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod33sampling::W](W) writer structure"]
impl crate::Writable for BOD33SAMPLING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOD33SAMPLING to value 0"]
impl crate::Resettable for BOD33SAMPLING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
