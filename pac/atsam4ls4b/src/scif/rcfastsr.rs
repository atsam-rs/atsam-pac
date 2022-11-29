#[doc = "Register `RCFASTSR` reader"]
pub struct R(crate::R<RCFASTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCFASTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCFASTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCFASTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCFASTSR` writer"]
pub struct W(crate::W<RCFASTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCFASTSR_SPEC>;
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
impl From<crate::W<RCFASTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCFASTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURTRIM` reader - Current Trim Value"]
pub type CURTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CURTRIM` writer - Current Trim Value"]
pub type CURTRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCFASTSR_SPEC, u8, u8, 7, O>;
#[doc = "Field `CNTERR` reader - Current Count Error"]
pub type CNTERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CNTERR` writer - Current Count Error"]
pub type CNTERR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCFASTSR_SPEC, u8, u8, 5, O>;
#[doc = "Field `SIGN` reader - Sign of Current Count Error"]
pub type SIGN_R = crate::BitReader<bool>;
#[doc = "Field `SIGN` writer - Sign of Current Count Error"]
pub type SIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFASTSR_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - Lock"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFASTSR_SPEC, bool, O>;
#[doc = "Field `LOCKLOST` reader - Lock Lost"]
pub type LOCKLOST_R = crate::BitReader<bool>;
#[doc = "Field `LOCKLOST` writer - Lock Lost"]
pub type LOCKLOST_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFASTSR_SPEC, bool, O>;
#[doc = "Field `UPDATED` reader - Current Trim Value Updated"]
pub type UPDATED_R = crate::BitReader<bool>;
#[doc = "Field `UPDATED` writer - Current Trim Value Updated"]
pub type UPDATED_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCFASTSR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:6 - Current Trim Value"]
    #[inline(always)]
    pub fn curtrim(&self) -> CURTRIM_R {
        CURTRIM_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - Current Count Error"]
    #[inline(always)]
    pub fn cnterr(&self) -> CNTERR_R {
        CNTERR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Sign of Current Count Error"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Lock Lost"]
    #[inline(always)]
    pub fn locklost(&self) -> LOCKLOST_R {
        LOCKLOST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Current Trim Value Updated"]
    #[inline(always)]
    pub fn updated(&self) -> UPDATED_R {
        UPDATED_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Current Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn curtrim(&mut self) -> CURTRIM_W<0> {
        CURTRIM_W::new(self)
    }
    #[doc = "Bits 16:20 - Current Count Error"]
    #[inline(always)]
    #[must_use]
    pub fn cnterr(&mut self) -> CNTERR_W<16> {
        CNTERR_W::new(self)
    }
    #[doc = "Bit 21 - Sign of Current Count Error"]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SIGN_W<21> {
        SIGN_W::new(self)
    }
    #[doc = "Bit 24 - Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<24> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 25 - Lock Lost"]
    #[inline(always)]
    #[must_use]
    pub fn locklost(&mut self) -> LOCKLOST_W<25> {
        LOCKLOST_W::new(self)
    }
    #[doc = "Bit 31 - Current Trim Value Updated"]
    #[inline(always)]
    #[must_use]
    pub fn updated(&mut self) -> UPDATED_W<31> {
        UPDATED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "4/8/12 MHz RC Oscillator Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcfastsr](index.html) module"]
pub struct RCFASTSR_SPEC;
impl crate::RegisterSpec for RCFASTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcfastsr::R](R) reader structure"]
impl crate::Readable for RCFASTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcfastsr::W](W) writer structure"]
impl crate::Writable for RCFASTSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCFASTSR to value 0"]
impl crate::Resettable for RCFASTSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
