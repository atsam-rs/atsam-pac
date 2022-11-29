#[doc = "Register `CALV` reader"]
pub struct R(crate::R<CALV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALV` writer"]
pub struct W(crate::W<CALV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALV_SPEC>;
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
impl From<crate::W<CALV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC` reader - Second"]
pub type SEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC` writer - Second"]
pub type SEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALV_SPEC, u8, u8, 6, O>;
#[doc = "Field `MIN` reader - Minute"]
pub type MIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN` writer - Minute"]
pub type MIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALV_SPEC, u8, u8, 6, O>;
#[doc = "Field `HOUR` reader - Hour"]
pub type HOUR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOUR` writer - Hour"]
pub type HOUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALV_SPEC, u8, u8, 5, O>;
#[doc = "Field `DAY` reader - Day"]
pub type DAY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAY` writer - Day"]
pub type DAY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALV_SPEC, u8, u8, 5, O>;
#[doc = "Field `MONTH` reader - Month"]
pub type MONTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MONTH` writer - Month"]
pub type MONTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALV_SPEC, u8, u8, 4, O>;
#[doc = "Field `YEAR` reader - Year"]
pub type YEAR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YEAR` writer - Year"]
pub type YEAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CALV_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Second"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:16 - Hour"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Second"]
    #[inline(always)]
    #[must_use]
    pub fn sec(&mut self) -> SEC_W<0> {
        SEC_W::new(self)
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline(always)]
    #[must_use]
    pub fn min(&mut self) -> MIN_W<6> {
        MIN_W::new(self)
    }
    #[doc = "Bits 12:16 - Hour"]
    #[inline(always)]
    #[must_use]
    pub fn hour(&mut self) -> HOUR_W<12> {
        HOUR_W::new(self)
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline(always)]
    #[must_use]
    pub fn day(&mut self) -> DAY_W<17> {
        DAY_W::new(self)
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline(always)]
    #[must_use]
    pub fn month(&mut self) -> MONTH_W<22> {
        MONTH_W::new(self)
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline(always)]
    #[must_use]
    pub fn year(&mut self) -> YEAR_W<26> {
        YEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calendar Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calv](index.html) module"]
pub struct CALV_SPEC;
impl crate::RegisterSpec for CALV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calv::R](R) reader structure"]
impl crate::Readable for CALV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calv::W](W) writer structure"]
impl crate::Writable for CALV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CALV to value 0"]
impl crate::Resettable for CALV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
