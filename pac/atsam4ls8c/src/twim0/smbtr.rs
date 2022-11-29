#[doc = "Register `SMBTR` reader"]
pub struct R(crate::R<SMBTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMBTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMBTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMBTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMBTR` writer"]
pub struct W(crate::W<SMBTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMBTR_SPEC>;
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
impl From<crate::W<SMBTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMBTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLOWS` reader - Slave Clock stretch maximum cycles"]
pub type TLOWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TLOWS` writer - Slave Clock stretch maximum cycles"]
pub type TLOWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMBTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `TLOWM` reader - Master Clock stretch maximum cycles"]
pub type TLOWM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TLOWM` writer - Master Clock stretch maximum cycles"]
pub type TLOWM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMBTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `THMAX` reader - Clock High maximum cycles"]
pub type THMAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THMAX` writer - Clock High maximum cycles"]
pub type THMAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMBTR_SPEC, u8, u8, 8, O>;
#[doc = "Field `EXP` reader - SMBus Timeout Clock prescaler"]
pub type EXP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXP` writer - SMBus Timeout Clock prescaler"]
pub type EXP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMBTR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - Slave Clock stretch maximum cycles"]
    #[inline(always)]
    pub fn tlows(&self) -> TLOWS_R {
        TLOWS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Master Clock stretch maximum cycles"]
    #[inline(always)]
    pub fn tlowm(&self) -> TLOWM_R {
        TLOWM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Clock High maximum cycles"]
    #[inline(always)]
    pub fn thmax(&self) -> THMAX_R {
        THMAX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 28:31 - SMBus Timeout Clock prescaler"]
    #[inline(always)]
    pub fn exp(&self) -> EXP_R {
        EXP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Clock stretch maximum cycles"]
    #[inline(always)]
    #[must_use]
    pub fn tlows(&mut self) -> TLOWS_W<0> {
        TLOWS_W::new(self)
    }
    #[doc = "Bits 8:15 - Master Clock stretch maximum cycles"]
    #[inline(always)]
    #[must_use]
    pub fn tlowm(&mut self) -> TLOWM_W<8> {
        TLOWM_W::new(self)
    }
    #[doc = "Bits 16:23 - Clock High maximum cycles"]
    #[inline(always)]
    #[must_use]
    pub fn thmax(&mut self) -> THMAX_W<16> {
        THMAX_W::new(self)
    }
    #[doc = "Bits 28:31 - SMBus Timeout Clock prescaler"]
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
#[doc = "SMBus Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smbtr](index.html) module"]
pub struct SMBTR_SPEC;
impl crate::RegisterSpec for SMBTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smbtr::R](R) reader structure"]
impl crate::Readable for SMBTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smbtr::W](W) writer structure"]
impl crate::Writable for SMBTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMBTR to value 0"]
impl crate::Resettable for SMBTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
