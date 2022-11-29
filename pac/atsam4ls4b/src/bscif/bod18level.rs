#[doc = "Register `BOD18LEVEL` reader"]
pub struct R(crate::R<BOD18LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD18LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD18LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD18LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD18LEVEL` writer"]
pub struct W(crate::W<BOD18LEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD18LEVEL_SPEC>;
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
impl From<crate::W<BOD18LEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD18LEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - BOD Value"]
pub type VAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VAL` writer - BOD Value"]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BOD18LEVEL_SPEC, u8, u8, 6, O>;
#[doc = "Field `RANGE` reader - BOD Threshold Range"]
pub type RANGE_R = crate::BitReader<bool>;
#[doc = "Field `RANGE` writer - BOD Threshold Range"]
pub type RANGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOD18LEVEL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - BOD Value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 31 - BOD Threshold Range"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - BOD Value"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
    }
    #[doc = "Bit 31 - BOD Threshold Range"]
    #[inline(always)]
    #[must_use]
    pub fn range(&mut self) -> RANGE_W<31> {
        RANGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD18 Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod18level](index.html) module"]
pub struct BOD18LEVEL_SPEC;
impl crate::RegisterSpec for BOD18LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod18level::R](R) reader structure"]
impl crate::Readable for BOD18LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod18level::W](W) writer structure"]
impl crate::Writable for BOD18LEVEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOD18LEVEL to value 0"]
impl crate::Resettable for BOD18LEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
