#[doc = "Register `PPCR` reader"]
pub struct R(crate::R<PPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPCR` writer"]
pub struct W(crate::W<PPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPCR_SPEC>;
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
impl From<crate::W<PPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTPUN` reader - Reset Pullup"]
pub type RSTPUN_R = crate::BitReader<bool>;
#[doc = "Field `RSTPUN` writer - Reset Pullup"]
pub type RSTPUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPCR_SPEC, bool, O>;
#[doc = "Field `CATBRCMASK` reader - CAT Request Clock Mask"]
pub type CATBRCMASK_R = crate::BitReader<bool>;
#[doc = "Field `CATBRCMASK` writer - CAT Request Clock Mask"]
pub type CATBRCMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPCR_SPEC, bool, O>;
#[doc = "Field `ACIFCRCMASK` reader - ACIFC Request Clock Mask"]
pub type ACIFCRCMASK_R = crate::BitReader<bool>;
#[doc = "Field `ACIFCRCMASK` writer - ACIFC Request Clock Mask"]
pub type ACIFCRCMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPCR_SPEC, bool, O>;
#[doc = "Field `ASTRCMASK` reader - AST Request Clock Mask"]
pub type ASTRCMASK_R = crate::BitReader<bool>;
#[doc = "Field `ASTRCMASK` writer - AST Request Clock Mask"]
pub type ASTRCMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPCR_SPEC, bool, O>;
#[doc = "Field `TWIS0RCMASK` reader - TWIS0 Request Clock Mask"]
pub type TWIS0RCMASK_R = crate::BitReader<bool>;
#[doc = "Field `TWIS0RCMASK` writer - TWIS0 Request Clock Mask"]
pub type TWIS0RCMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPCR_SPEC, bool, O>;
#[doc = "Field `TWIS1RCMASK` reader - TWIS1 Request Clock Mask"]
pub type TWIS1RCMASK_R = crate::BitReader<bool>;
#[doc = "Field `TWIS1RCMASK` writer - TWIS1 Request Clock Mask"]
pub type TWIS1RCMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPCR_SPEC, bool, O>;
#[doc = "Field `PEVCRCMASK` reader - PEVC Request Clock Mask"]
pub type PEVCRCMASK_R = crate::BitReader<bool>;
#[doc = "Field `PEVCRCMASK` writer - PEVC Request Clock Mask"]
pub type PEVCRCMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPCR_SPEC, bool, O>;
#[doc = "Field `ADCIFERCMASK` reader - ADCIFE Request Clock Mask"]
pub type ADCIFERCMASK_R = crate::BitReader<bool>;
#[doc = "Field `ADCIFERCMASK` writer - ADCIFE Request Clock Mask"]
pub type ADCIFERCMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPCR_SPEC, bool, O>;
#[doc = "Field `VREGRCMASK` reader - VREG Request Clock Mask"]
pub type VREGRCMASK_R = crate::BitReader<bool>;
#[doc = "Field `VREGRCMASK` writer - VREG Request Clock Mask"]
pub type VREGRCMASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPCR_SPEC, bool, O>;
#[doc = "Field `FWBGREF` reader - Flash Wait BGREF"]
pub type FWBGREF_R = crate::BitReader<bool>;
#[doc = "Field `FWBGREF` writer - Flash Wait BGREF"]
pub type FWBGREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPCR_SPEC, bool, O>;
#[doc = "Field `FWBOD18` reader - Flash Wait BOD18"]
pub type FWBOD18_R = crate::BitReader<bool>;
#[doc = "Field `FWBOD18` writer - Flash Wait BOD18"]
pub type FWBOD18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PPCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Reset Pullup"]
    #[inline(always)]
    pub fn rstpun(&self) -> RSTPUN_R {
        RSTPUN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CAT Request Clock Mask"]
    #[inline(always)]
    pub fn catbrcmask(&self) -> CATBRCMASK_R {
        CATBRCMASK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACIFC Request Clock Mask"]
    #[inline(always)]
    pub fn acifcrcmask(&self) -> ACIFCRCMASK_R {
        ACIFCRCMASK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AST Request Clock Mask"]
    #[inline(always)]
    pub fn astrcmask(&self) -> ASTRCMASK_R {
        ASTRCMASK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TWIS0 Request Clock Mask"]
    #[inline(always)]
    pub fn twis0rcmask(&self) -> TWIS0RCMASK_R {
        TWIS0RCMASK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TWIS1 Request Clock Mask"]
    #[inline(always)]
    pub fn twis1rcmask(&self) -> TWIS1RCMASK_R {
        TWIS1RCMASK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PEVC Request Clock Mask"]
    #[inline(always)]
    pub fn pevcrcmask(&self) -> PEVCRCMASK_R {
        PEVCRCMASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADCIFE Request Clock Mask"]
    #[inline(always)]
    pub fn adcifercmask(&self) -> ADCIFERCMASK_R {
        ADCIFERCMASK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VREG Request Clock Mask"]
    #[inline(always)]
    pub fn vregrcmask(&self) -> VREGRCMASK_R {
        VREGRCMASK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash Wait BGREF"]
    #[inline(always)]
    pub fn fwbgref(&self) -> FWBGREF_R {
        FWBGREF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Flash Wait BOD18"]
    #[inline(always)]
    pub fn fwbod18(&self) -> FWBOD18_R {
        FWBOD18_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Pullup"]
    #[inline(always)]
    #[must_use]
    pub fn rstpun(&mut self) -> RSTPUN_W<0> {
        RSTPUN_W::new(self)
    }
    #[doc = "Bit 1 - CAT Request Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn catbrcmask(&mut self) -> CATBRCMASK_W<1> {
        CATBRCMASK_W::new(self)
    }
    #[doc = "Bit 2 - ACIFC Request Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn acifcrcmask(&mut self) -> ACIFCRCMASK_W<2> {
        ACIFCRCMASK_W::new(self)
    }
    #[doc = "Bit 3 - AST Request Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn astrcmask(&mut self) -> ASTRCMASK_W<3> {
        ASTRCMASK_W::new(self)
    }
    #[doc = "Bit 4 - TWIS0 Request Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twis0rcmask(&mut self) -> TWIS0RCMASK_W<4> {
        TWIS0RCMASK_W::new(self)
    }
    #[doc = "Bit 5 - TWIS1 Request Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twis1rcmask(&mut self) -> TWIS1RCMASK_W<5> {
        TWIS1RCMASK_W::new(self)
    }
    #[doc = "Bit 6 - PEVC Request Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pevcrcmask(&mut self) -> PEVCRCMASK_W<6> {
        PEVCRCMASK_W::new(self)
    }
    #[doc = "Bit 7 - ADCIFE Request Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn adcifercmask(&mut self) -> ADCIFERCMASK_W<7> {
        ADCIFERCMASK_W::new(self)
    }
    #[doc = "Bit 8 - VREG Request Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn vregrcmask(&mut self) -> VREGRCMASK_W<8> {
        VREGRCMASK_W::new(self)
    }
    #[doc = "Bit 9 - Flash Wait BGREF"]
    #[inline(always)]
    #[must_use]
    pub fn fwbgref(&mut self) -> FWBGREF_W<9> {
        FWBGREF_W::new(self)
    }
    #[doc = "Bit 10 - Flash Wait BOD18"]
    #[inline(always)]
    #[must_use]
    pub fn fwbod18(&mut self) -> FWBOD18_W<10> {
        FWBOD18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Power Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppcr](index.html) module"]
pub struct PPCR_SPEC;
impl crate::RegisterSpec for PPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppcr::R](R) reader structure"]
impl crate::Readable for PPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppcr::W](W) writer structure"]
impl crate::Writable for PPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPCR to value 0x01fe"]
impl crate::Resettable for PPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01fe;
}
