#[doc = "Register `PBDMASK` reader"]
pub struct R(crate::R<PBDMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBDMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBDMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBDMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBDMASK` writer"]
pub struct W(crate::W<PBDMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBDMASK_SPEC>;
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
impl From<crate::W<PBDMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBDMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BPM_` reader - BPM APB Clock Enable"]
pub type BPM__R = crate::BitReader<bool>;
#[doc = "Field `BPM_` writer - BPM APB Clock Enable"]
pub type BPM__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBDMASK_SPEC, bool, O>;
#[doc = "Field `BSCIF_` reader - BSCIF APB Clock Enable"]
pub type BSCIF__R = crate::BitReader<bool>;
#[doc = "Field `BSCIF_` writer - BSCIF APB Clock Enable"]
pub type BSCIF__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBDMASK_SPEC, bool, O>;
#[doc = "Field `AST_` reader - AST APB Clock Enable"]
pub type AST__R = crate::BitReader<bool>;
#[doc = "Field `AST_` writer - AST APB Clock Enable"]
pub type AST__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBDMASK_SPEC, bool, O>;
#[doc = "Field `WDT_` reader - WDT APB Clock Enable"]
pub type WDT__R = crate::BitReader<bool>;
#[doc = "Field `WDT_` writer - WDT APB Clock Enable"]
pub type WDT__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBDMASK_SPEC, bool, O>;
#[doc = "Field `EIC_` reader - EIC APB Clock Enable"]
pub type EIC__R = crate::BitReader<bool>;
#[doc = "Field `EIC_` writer - EIC APB Clock Enable"]
pub type EIC__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBDMASK_SPEC, bool, O>;
#[doc = "Field `PICOUART_` reader - PICOUART APB Clock Enable"]
pub type PICOUART__R = crate::BitReader<bool>;
#[doc = "Field `PICOUART_` writer - PICOUART APB Clock Enable"]
pub type PICOUART__W<'a, const O: u8> = crate::BitWriter<'a, u32, PBDMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - BPM APB Clock Enable"]
    #[inline(always)]
    pub fn bpm_(&self) -> BPM__R {
        BPM__R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BSCIF APB Clock Enable"]
    #[inline(always)]
    pub fn bscif_(&self) -> BSCIF__R {
        BSCIF__R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AST APB Clock Enable"]
    #[inline(always)]
    pub fn ast_(&self) -> AST__R {
        AST__R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&self) -> WDT__R {
        WDT__R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&self) -> EIC__R {
        EIC__R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PICOUART APB Clock Enable"]
    #[inline(always)]
    pub fn picouart_(&self) -> PICOUART__R {
        PICOUART__R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BPM APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bpm_(&mut self) -> BPM__W<0> {
        BPM__W::new(self)
    }
    #[doc = "Bit 1 - BSCIF APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bscif_(&mut self) -> BSCIF__W<1> {
        BSCIF__W::new(self)
    }
    #[doc = "Bit 2 - AST APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ast_(&mut self) -> AST__W<2> {
        AST__W::new(self)
    }
    #[doc = "Bit 3 - WDT APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_(&mut self) -> WDT__W<3> {
        WDT__W::new(self)
    }
    #[doc = "Bit 4 - EIC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eic_(&mut self) -> EIC__W<4> {
        EIC__W::new(self)
    }
    #[doc = "Bit 5 - PICOUART APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn picouart_(&mut self) -> PICOUART__W<5> {
        PICOUART__W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBD Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdmask](index.html) module"]
pub struct PBDMASK_SPEC;
impl crate::RegisterSpec for PBDMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbdmask::R](R) reader structure"]
impl crate::Readable for PBDMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbdmask::W](W) writer structure"]
impl crate::Writable for PBDMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PBDMASK to value 0x3f"]
impl crate::Resettable for PBDMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
