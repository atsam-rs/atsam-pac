#[doc = "Register `VREGCR` reader"]
pub struct R(crate::R<VREGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREGCR` writer"]
pub struct W(crate::W<VREGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREGCR_SPEC>;
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
impl From<crate::W<VREGCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS` reader - Voltage Regulator disable"]
pub type DIS_R = crate::BitReader<bool>;
#[doc = "Field `DIS` writer - Voltage Regulator disable"]
pub type DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREGCR_SPEC, bool, O>;
#[doc = "Field `SSG` reader - Spread Spectrum Generator Enable"]
pub type SSG_R = crate::BitReader<bool>;
#[doc = "Field `SSG` writer - Spread Spectrum Generator Enable"]
pub type SSG_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREGCR_SPEC, bool, O>;
#[doc = "Field `SSW` reader - Stop Switching"]
pub type SSW_R = crate::BitReader<bool>;
#[doc = "Field `SSW` writer - Stop Switching"]
pub type SSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREGCR_SPEC, bool, O>;
#[doc = "Field `SSWEVT` reader - Stop Switching On Event Enable"]
pub type SSWEVT_R = crate::BitReader<bool>;
#[doc = "Field `SSWEVT` writer - Stop Switching On Event Enable"]
pub type SSWEVT_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREGCR_SPEC, bool, O>;
#[doc = "Field `SFV` reader - Store Final Value"]
pub type SFV_R = crate::BitReader<bool>;
#[doc = "Field `SFV` writer - Store Final Value"]
pub type SFV_W<'a, const O: u8> = crate::BitWriter<'a, u32, VREGCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Voltage Regulator disable"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Spread Spectrum Generator Enable"]
    #[inline(always)]
    pub fn ssg(&self) -> SSG_R {
        SSG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop Switching"]
    #[inline(always)]
    pub fn ssw(&self) -> SSW_R {
        SSW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Stop Switching On Event Enable"]
    #[inline(always)]
    pub fn sswevt(&self) -> SSWEVT_R {
        SSWEVT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 31 - Store Final Value"]
    #[inline(always)]
    pub fn sfv(&self) -> SFV_R {
        SFV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Regulator disable"]
    #[inline(always)]
    #[must_use]
    pub fn dis(&mut self) -> DIS_W<0> {
        DIS_W::new(self)
    }
    #[doc = "Bit 8 - Spread Spectrum Generator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssg(&mut self) -> SSG_W<8> {
        SSG_W::new(self)
    }
    #[doc = "Bit 9 - Stop Switching"]
    #[inline(always)]
    #[must_use]
    pub fn ssw(&mut self) -> SSW_W<9> {
        SSW_W::new(self)
    }
    #[doc = "Bit 10 - Stop Switching On Event Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sswevt(&mut self) -> SSWEVT_W<10> {
        SSWEVT_W::new(self)
    }
    #[doc = "Bit 31 - Store Final Value"]
    #[inline(always)]
    #[must_use]
    pub fn sfv(&mut self) -> SFV_W<31> {
        SFV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Regulator Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vregcr](index.html) module"]
pub struct VREGCR_SPEC;
impl crate::RegisterSpec for VREGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vregcr::R](R) reader structure"]
impl crate::Readable for VREGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vregcr::W](W) writer structure"]
impl crate::Writable for VREGCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VREGCR to value 0"]
impl crate::Resettable for VREGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
