#[doc = "Register `ABMCFG` reader"]
pub struct R(crate::R<ABMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ABMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ABMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ABMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ABMCFG` writer"]
pub struct W(crate::W<ABMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ABMCFG_SPEC>;
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
impl From<crate::W<ABMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ABMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ABMCFG_SPEC, bool, O>;
#[doc = "Field `FCS` reader - Frame Counter Selection"]
pub type FCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCS` writer - Frame Counter Selection"]
pub type FCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ABMCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SIZE` reader - Size"]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` writer - Size"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ABMCFG_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 8:12 - Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fcs(&mut self) -> FCS_W<1> {
        FCS_W::new(self)
    }
    #[doc = "Bits 8:12 - Size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<8> {
        SIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automated Bit Mapping Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abmcfg](index.html) module"]
pub struct ABMCFG_SPEC;
impl crate::RegisterSpec for ABMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [abmcfg::R](R) reader structure"]
impl crate::Readable for ABMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [abmcfg::W](W) writer structure"]
impl crate::Writable for ABMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ABMCFG to value 0"]
impl crate::Resettable for ABMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
