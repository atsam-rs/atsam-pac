#[doc = "Register `RAB0` reader"]
pub struct R(crate::R<RAB0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAB0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAB0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAB0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAB` reader - Register A or Register B"]
pub struct RAB_R(crate::FieldReader<u32, u32>);
impl RAB_R {
    pub(crate) fn new(bits: u32) -> Self {
        RAB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Register A or Register B"]
    #[inline(always)]
    pub fn rab(&self) -> RAB_R {
        RAB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Register AB (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rab0](index.html) module"]
pub struct RAB0_SPEC;
impl crate::RegisterSpec for RAB0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rab0::R](R) reader structure"]
impl crate::Readable for RAB0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAB0 to value 0"]
impl crate::Resettable for RAB0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
