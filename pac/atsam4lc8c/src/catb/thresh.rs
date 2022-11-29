#[doc = "Register `THRESH` reader"]
pub struct R(crate::R<THRESH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRESH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRESH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRESH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRESH` writer"]
pub struct W(crate::W<THRESH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRESH_SPEC>;
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
impl From<crate::W<THRESH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRESH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTHRESH` reader - Fractional part of Threshold Value"]
pub type FTHRESH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `FTHRESH` writer - Fractional part of Threshold Value"]
pub type FTHRESH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, THRESH_SPEC, u16, u16, 12, O>;
#[doc = "Field `RTHRESH` reader - Rational part of Threshold Value"]
pub type RTHRESH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTHRESH` writer - Rational part of Threshold Value"]
pub type RTHRESH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, THRESH_SPEC, u8, u8, 8, O>;
#[doc = "Field `DIR` reader - Threshold Direction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Threshold Direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, THRESH_SPEC, bool, O>;
#[doc = "Field `LENGTH` reader - Threshold Length"]
pub type LENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LENGTH` writer - Threshold Length"]
pub type LENGTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, THRESH_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:11 - Fractional part of Threshold Value"]
    #[inline(always)]
    pub fn fthresh(&self) -> FTHRESH_R {
        FTHRESH_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - Rational part of Threshold Value"]
    #[inline(always)]
    pub fn rthresh(&self) -> RTHRESH_R {
        RTHRESH_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bit 23 - Threshold Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Threshold Length"]
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Fractional part of Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn fthresh(&mut self) -> FTHRESH_W<0> {
        FTHRESH_W::new(self)
    }
    #[doc = "Bits 12:19 - Rational part of Threshold Value"]
    #[inline(always)]
    #[must_use]
    pub fn rthresh(&mut self) -> RTHRESH_W<12> {
        RTHRESH_W::new(self)
    }
    #[doc = "Bit 23 - Threshold Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<23> {
        DIR_W::new(self)
    }
    #[doc = "Bits 24:28 - Threshold Length"]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LENGTH_W<24> {
        LENGTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thresh](index.html) module"]
pub struct THRESH_SPEC;
impl crate::RegisterSpec for THRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thresh::R](R) reader structure"]
impl crate::Readable for THRESH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thresh::W](W) writer structure"]
impl crate::Writable for THRESH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets THRESH to value 0"]
impl crate::Resettable for THRESH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
