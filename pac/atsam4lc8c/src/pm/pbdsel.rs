#[doc = "Register `PBDSEL` reader"]
pub struct R(crate::R<PBDSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBDSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBDSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBDSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBDSEL` writer"]
pub struct W(crate::W<PBDSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBDSEL_SPEC>;
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
impl From<crate::W<PBDSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBDSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PBSEL` reader - PBD Clock Select"]
pub type PBSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PBSEL` writer - PBD Clock Select"]
pub type PBSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PBDSEL_SPEC, u8, u8, 3, O>;
#[doc = "Field `PBDIV` reader - PBD Division Select"]
pub type PBDIV_R = crate::BitReader<bool>;
#[doc = "Field `PBDIV` writer - PBD Division Select"]
pub type PBDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, PBDSEL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - PBD Clock Select"]
    #[inline(always)]
    pub fn pbsel(&self) -> PBSEL_R {
        PBSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - PBD Division Select"]
    #[inline(always)]
    pub fn pbdiv(&self) -> PBDIV_R {
        PBDIV_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - PBD Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn pbsel(&mut self) -> PBSEL_W<0> {
        PBSEL_W::new(self)
    }
    #[doc = "Bit 7 - PBD Division Select"]
    #[inline(always)]
    #[must_use]
    pub fn pbdiv(&mut self) -> PBDIV_W<7> {
        PBDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBD Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdsel](index.html) module"]
pub struct PBDSEL_SPEC;
impl crate::RegisterSpec for PBDSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbdsel::R](R) reader structure"]
impl crate::Readable for PBDSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbdsel::W](W) writer structure"]
impl crate::Writable for PBDSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBDSEL to value 0"]
impl crate::Resettable for PBDSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
