#[doc = "Register `HSCWGR` reader"]
pub struct R(crate::R<HSCWGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCWGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCWGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCWGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCWGR` writer"]
pub struct W(crate::W<HSCWGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCWGR_SPEC>;
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
impl From<crate::W<HSCWGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCWGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOW` reader - Clock Low Cycles"]
pub type LOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOW` writer - Clock Low Cycles"]
pub type LOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSCWGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `HIGH` reader - Clock High Cycles"]
pub type HIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HIGH` writer - Clock High Cycles"]
pub type HIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSCWGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `STASTO` reader - START and STOP Cycles"]
pub type STASTO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STASTO` writer - START and STOP Cycles"]
pub type STASTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSCWGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA` reader - Data Setup and Hold Cycles"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - Data Setup and Hold Cycles"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSCWGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `EXP` reader - Clock Prescaler"]
pub type EXP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXP` writer - Clock Prescaler"]
pub type EXP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSCWGR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:7 - Clock Low Cycles"]
    #[inline(always)]
    pub fn low(&self) -> LOW_R {
        LOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clock High Cycles"]
    #[inline(always)]
    pub fn high(&self) -> HIGH_R {
        HIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - START and STOP Cycles"]
    #[inline(always)]
    pub fn stasto(&self) -> STASTO_R {
        STASTO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Data Setup and Hold Cycles"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Clock Prescaler"]
    #[inline(always)]
    pub fn exp(&self) -> EXP_R {
        EXP_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock Low Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn low(&mut self) -> LOW_W<0> {
        LOW_W::new(self)
    }
    #[doc = "Bits 8:15 - Clock High Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn high(&mut self) -> HIGH_W<8> {
        HIGH_W::new(self)
    }
    #[doc = "Bits 16:23 - START and STOP Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn stasto(&mut self) -> STASTO_W<16> {
        STASTO_W::new(self)
    }
    #[doc = "Bits 24:27 - Data Setup and Hold Cycles"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<24> {
        DATA_W::new(self)
    }
    #[doc = "Bits 28:30 - Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn exp(&mut self) -> EXP_W<28> {
        EXP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HS-mode Clock Waveform Generator\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hscwgr](index.html) module"]
pub struct HSCWGR_SPEC;
impl crate::RegisterSpec for HSCWGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hscwgr::R](R) reader structure"]
impl crate::Readable for HSCWGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hscwgr::W](W) writer structure"]
impl crate::Writable for HSCWGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSCWGR to value 0"]
impl crate::Resettable for HSCWGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
