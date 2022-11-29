#[doc = "Register `PBCMASK` reader"]
pub struct R(crate::R<PBCMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBCMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBCMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBCMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBCMASK` writer"]
pub struct W(crate::W<PBCMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBCMASK_SPEC>;
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
impl From<crate::W<PBCMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBCMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PM_` reader - PM APB Clock Enable"]
pub type PM__R = crate::BitReader<bool>;
#[doc = "Field `PM_` writer - PM APB Clock Enable"]
pub type PM__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBCMASK_SPEC, bool, O>;
#[doc = "Field `CHIPID_` reader - CHIPID APB Clock Enable"]
pub type CHIPID__R = crate::BitReader<bool>;
#[doc = "Field `CHIPID_` writer - CHIPID APB Clock Enable"]
pub type CHIPID__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBCMASK_SPEC, bool, O>;
#[doc = "Field `SCIF_` reader - SCIF APB Clock Enable"]
pub type SCIF__R = crate::BitReader<bool>;
#[doc = "Field `SCIF_` writer - SCIF APB Clock Enable"]
pub type SCIF__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBCMASK_SPEC, bool, O>;
#[doc = "Field `FREQM_` reader - FREQM APB Clock Enable"]
pub type FREQM__R = crate::BitReader<bool>;
#[doc = "Field `FREQM_` writer - FREQM APB Clock Enable"]
pub type FREQM__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBCMASK_SPEC, bool, O>;
#[doc = "Field `GPIO_` reader - GPIO APB Clock Enable"]
pub type GPIO__R = crate::BitReader<bool>;
#[doc = "Field `GPIO_` writer - GPIO APB Clock Enable"]
pub type GPIO__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBCMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&self) -> PM__R {
        PM__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CHIPID APB Clock Enable"]
    #[inline(always)]
    pub fn chipid_(&self) -> CHIPID__R {
        CHIPID__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCIF APB Clock Enable"]
    #[inline(always)]
    pub fn scif_(&self) -> SCIF__R {
        SCIF__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FREQM APB Clock Enable"]
    #[inline(always)]
    pub fn freqm_(&self) -> FREQM__R {
        FREQM__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO APB Clock Enable"]
    #[inline(always)]
    pub fn gpio_(&self) -> GPIO__R {
        GPIO__R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PM APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pm_(&mut self) -> PM__W<0> {
        PM__W::new(self)
    }
    #[doc = "Bit 1 - CHIPID APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chipid_(&mut self) -> CHIPID__W<1> {
        CHIPID__W::new(self)
    }
    #[doc = "Bit 2 - SCIF APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn scif_(&mut self) -> SCIF__W<2> {
        SCIF__W::new(self)
    }
    #[doc = "Bit 3 - FREQM APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn freqm_(&mut self) -> FREQM__W<3> {
        FREQM__W::new(self)
    }
    #[doc = "Bit 4 - GPIO APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_(&mut self) -> GPIO__W<4> {
        GPIO__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBC Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbcmask](index.html) module"]
pub struct PBCMASK_SPEC;
impl crate::RegisterSpec for PBCMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbcmask::R](R) reader structure"]
impl crate::Readable for PBCMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbcmask::W](W) writer structure"]
impl crate::Writable for PBCMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBCMASK to value 0x1f"]
impl crate::Resettable for PBCMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
