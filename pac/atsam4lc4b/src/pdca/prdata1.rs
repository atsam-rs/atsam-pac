#[doc = "Register `PRDATA1` reader"]
pub struct R(crate::R<PRDATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRDATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRDATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRDATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - Data Cycles Counted Since Last reset"]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Data Cycles Counted Since Last reset"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channel 1 Read Data Cycles\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prdata1](index.html) module"]
pub struct PRDATA1_SPEC;
impl crate::RegisterSpec for PRDATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prdata1::R](R) reader structure"]
impl crate::Readable for PRDATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRDATA1 to value 0"]
impl crate::Resettable for PRDATA1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
