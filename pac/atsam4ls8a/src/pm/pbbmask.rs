#[doc = "Register `PBBMASK` reader"]
pub struct R(crate::R<PBBMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBBMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBBMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBBMASK` writer"]
pub struct W(crate::W<PBBMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBBMASK_SPEC>;
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
impl From<crate::W<PBBMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBBMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFLASHC_` reader - HFLASHC APB Clock Enable"]
pub type HFLASHC__R = crate::BitReader<bool>;
#[doc = "Field `HFLASHC_` writer - HFLASHC APB Clock Enable"]
pub type HFLASHC__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBBMASK_SPEC, bool, O>;
#[doc = "Field `HCACHE_` reader - HCACHE APB Clock Enable"]
pub type HCACHE__R = crate::BitReader<bool>;
#[doc = "Field `HCACHE_` writer - HCACHE APB Clock Enable"]
pub type HCACHE__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBBMASK_SPEC, bool, O>;
#[doc = "Field `HMATRIX_` reader - HMATRIX APB Clock Enable"]
pub type HMATRIX__R = crate::BitReader<bool>;
#[doc = "Field `HMATRIX_` writer - HMATRIX APB Clock Enable"]
pub type HMATRIX__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBBMASK_SPEC, bool, O>;
#[doc = "Field `PDCA_` reader - PDCA APB Clock Enable"]
pub type PDCA__R = crate::BitReader<bool>;
#[doc = "Field `PDCA_` writer - PDCA APB Clock Enable"]
pub type PDCA__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBBMASK_SPEC, bool, O>;
#[doc = "Field `CRCCU_` reader - CRCCU APB Clock Enable"]
pub type CRCCU__R = crate::BitReader<bool>;
#[doc = "Field `CRCCU_` writer - CRCCU APB Clock Enable"]
pub type CRCCU__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBBMASK_SPEC, bool, O>;
#[doc = "Field `USBC_` reader - USBC APB Clock Enable"]
pub type USBC__R = crate::BitReader<bool>;
#[doc = "Field `USBC_` writer - USBC APB Clock Enable"]
pub type USBC__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBBMASK_SPEC, bool, O>;
#[doc = "Field `PEVC_` reader - PEVC APB Clock Enable"]
pub type PEVC__R = crate::BitReader<bool>;
#[doc = "Field `PEVC_` writer - PEVC APB Clock Enable"]
pub type PEVC__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBBMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - HFLASHC APB Clock Enable"]
    #[inline(always)]
    pub fn hflashc_(&self) -> HFLASHC__R {
        HFLASHC__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HCACHE APB Clock Enable"]
    #[inline(always)]
    pub fn hcache_(&self) -> HCACHE__R {
        HCACHE__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> HMATRIX__R {
        HMATRIX__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDCA APB Clock Enable"]
    #[inline(always)]
    pub fn pdca_(&self) -> PDCA__R {
        PDCA__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRCCU APB Clock Enable"]
    #[inline(always)]
    pub fn crccu_(&self) -> CRCCU__R {
        CRCCU__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USBC APB Clock Enable"]
    #[inline(always)]
    pub fn usbc_(&self) -> USBC__R {
        USBC__R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PEVC APB Clock Enable"]
    #[inline(always)]
    pub fn pevc_(&self) -> PEVC__R {
        PEVC__R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFLASHC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hflashc_(&mut self) -> HFLASHC__W<0> {
        HFLASHC__W::new(self)
    }
    #[doc = "Bit 1 - HCACHE APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hcache_(&mut self) -> HCACHE__W<1> {
        HCACHE__W::new(self)
    }
    #[doc = "Bit 2 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hmatrix_(&mut self) -> HMATRIX__W<2> {
        HMATRIX__W::new(self)
    }
    #[doc = "Bit 3 - PDCA APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdca_(&mut self) -> PDCA__W<3> {
        PDCA__W::new(self)
    }
    #[doc = "Bit 4 - CRCCU APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crccu_(&mut self) -> CRCCU__W<4> {
        CRCCU__W::new(self)
    }
    #[doc = "Bit 5 - USBC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbc_(&mut self) -> USBC__W<5> {
        USBC__W::new(self)
    }
    #[doc = "Bit 6 - PEVC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pevc_(&mut self) -> PEVC__W<6> {
        PEVC__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbbmask](index.html) module"]
pub struct PBBMASK_SPEC;
impl crate::RegisterSpec for PBBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbbmask::R](R) reader structure"]
impl crate::Readable for PBBMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbbmask::W](W) writer structure"]
impl crate::Writable for PBBMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBBMASK to value 0x01"]
impl crate::Resettable for PBBMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
