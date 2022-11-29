#[doc = "Register `GCCTRL%s` reader"]
pub struct R(crate::R<GCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCCTRL%s` writer"]
pub struct W(crate::W<GCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCCTRL_SPEC>;
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
impl From<crate::W<GCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEN` reader - Clock Enable"]
pub type CEN_R = crate::BitReader<bool>;
#[doc = "Field `CEN` writer - Clock Enable"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCTRL_SPEC, bool, O>;
#[doc = "Field `DIVEN` reader - Divide Enable"]
pub type DIVEN_R = crate::BitReader<bool>;
#[doc = "Field `DIVEN` writer - Divide Enable"]
pub type DIVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GCCTRL_SPEC, bool, O>;
#[doc = "Field `OSCSEL` reader - Clock Select"]
pub type OSCSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSCSEL` writer - Clock Select"]
pub type OSCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DIV` reader - Division Factor"]
pub type DIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIV` writer - Division Factor"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GCCTRL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Divide Enable"]
    #[inline(always)]
    pub fn diven(&self) -> DIVEN_R {
        DIVEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Clock Select"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<0> {
        CEN_W::new(self)
    }
    #[doc = "Bit 1 - Divide Enable"]
    #[inline(always)]
    #[must_use]
    pub fn diven(&mut self) -> DIVEN_W<1> {
        DIVEN_W::new(self)
    }
    #[doc = "Bits 8:12 - Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn oscsel(&mut self) -> OSCSEL_W<8> {
        OSCSEL_W::new(self)
    }
    #[doc = "Bits 16:31 - Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<16> {
        DIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Generic Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcctrl](index.html) module"]
pub struct GCCTRL_SPEC;
impl crate::RegisterSpec for GCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcctrl::R](R) reader structure"]
impl crate::Readable for GCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcctrl::W](W) writer structure"]
impl crate::Writable for GCCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GCCTRL%s to value 0"]
impl crate::Resettable for GCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
