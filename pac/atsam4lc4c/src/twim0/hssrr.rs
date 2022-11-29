#[doc = "Register `HSSRR` reader"]
pub struct R(crate::R<HSSRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSSRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSSRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSSRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSSRR` writer"]
pub struct W(crate::W<HSSRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSSRR_SPEC>;
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
impl From<crate::W<HSSRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSSRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DADRIVEL` reader - Data Drive Strength LOW"]
pub type DADRIVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DADRIVEL` writer - Data Drive Strength LOW"]
pub type DADRIVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSSRR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DASLEW` reader - Data Slew Limit"]
pub type DASLEW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DASLEW` writer - Data Slew Limit"]
pub type DASLEW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSSRR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLDRIVEL` reader - Clock Drive Strength LOW"]
pub type CLDRIVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLDRIVEL` writer - Clock Drive Strength LOW"]
pub type CLDRIVEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSSRR_SPEC, u8, u8, 3, O>;
#[doc = "Field `CLDRIVEH` reader - Clock Drive Strength HIGH"]
pub type CLDRIVEH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLDRIVEH` writer - Clock Drive Strength HIGH"]
pub type CLDRIVEH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSSRR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLSLEW` reader - Clock Slew Limit"]
pub type CLSLEW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLSLEW` writer - Clock Slew Limit"]
pub type CLSLEW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSSRR_SPEC, u8, u8, 2, O>;
#[doc = "Field `FILTER` reader - Input Spike Filter Control"]
pub type FILTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTER` writer - Input Spike Filter Control"]
pub type FILTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSSRR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:2 - Data Drive Strength LOW"]
    #[inline(always)]
    pub fn dadrivel(&self) -> DADRIVEL_R {
        DADRIVEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - Data Slew Limit"]
    #[inline(always)]
    pub fn daslew(&self) -> DASLEW_R {
        DASLEW_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Clock Drive Strength LOW"]
    #[inline(always)]
    pub fn cldrivel(&self) -> CLDRIVEL_R {
        CLDRIVEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Clock Drive Strength HIGH"]
    #[inline(always)]
    pub fn cldriveh(&self) -> CLDRIVEH_R {
        CLDRIVEH_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Clock Slew Limit"]
    #[inline(always)]
    pub fn clslew(&self) -> CLSLEW_R {
        CLSLEW_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Input Spike Filter Control"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data Drive Strength LOW"]
    #[inline(always)]
    #[must_use]
    pub fn dadrivel(&mut self) -> DADRIVEL_W<0> {
        DADRIVEL_W::new(self)
    }
    #[doc = "Bits 8:9 - Data Slew Limit"]
    #[inline(always)]
    #[must_use]
    pub fn daslew(&mut self) -> DASLEW_W<8> {
        DASLEW_W::new(self)
    }
    #[doc = "Bits 16:18 - Clock Drive Strength LOW"]
    #[inline(always)]
    #[must_use]
    pub fn cldrivel(&mut self) -> CLDRIVEL_W<16> {
        CLDRIVEL_W::new(self)
    }
    #[doc = "Bits 20:21 - Clock Drive Strength HIGH"]
    #[inline(always)]
    #[must_use]
    pub fn cldriveh(&mut self) -> CLDRIVEH_W<20> {
        CLDRIVEH_W::new(self)
    }
    #[doc = "Bits 24:25 - Clock Slew Limit"]
    #[inline(always)]
    #[must_use]
    pub fn clslew(&mut self) -> CLSLEW_W<24> {
        CLSLEW_W::new(self)
    }
    #[doc = "Bits 28:29 - Input Spike Filter Control"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<28> {
        FILTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HS-mode Slew Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hssrr](index.html) module"]
pub struct HSSRR_SPEC;
impl crate::RegisterSpec for HSSRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hssrr::R](R) reader structure"]
impl crate::Readable for HSSRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hssrr::W](W) writer structure"]
impl crate::Writable for HSSRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSSRR to value 0"]
impl crate::Resettable for HSSRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
