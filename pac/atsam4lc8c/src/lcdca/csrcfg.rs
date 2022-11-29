#[doc = "Register `CSRCFG` reader"]
pub struct R(crate::R<CSRCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSRCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSRCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSRCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSRCFG` writer"]
pub struct W(crate::W<CSRCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSRCFG_SPEC>;
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
impl From<crate::W<CSRCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSRCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSRCFG_SPEC, bool, O>;
#[doc = "Field `FCS` reader - Frame Counter Selection"]
pub type FCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCS` writer - Frame Counter Selection"]
pub type FCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSRCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `SIZE` reader - Size"]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` writer - Size"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSRCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `DATA` reader - Circular Shift Register Value"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - Circular Shift Register Value"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSRCFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Circular Shift Register Value"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<0> {
        DIR_W::new(self)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fcs(&mut self) -> FCS_W<1> {
        FCS_W::new(self)
    }
    #[doc = "Bits 3:5 - Size"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<3> {
        SIZE_W::new(self)
    }
    #[doc = "Bits 8:15 - Circular Shift Register Value"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<8> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Circular Shift Register Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csrcfg](index.html) module"]
pub struct CSRCFG_SPEC;
impl crate::RegisterSpec for CSRCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csrcfg::R](R) reader structure"]
impl crate::Readable for CSRCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csrcfg::W](W) writer structure"]
impl crate::Writable for CSRCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSRCFG to value 0"]
impl crate::Resettable for CSRCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
