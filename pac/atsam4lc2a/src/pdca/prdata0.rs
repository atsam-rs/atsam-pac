#[doc = "Register `PRDATA0` reader"]
pub struct R(crate::R<PRDATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRDATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PRDATA0_SPEC>> for R {
    fn from(reader: crate::R<PRDATA0_SPEC>) -> Self {
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
#[doc = "Channel 0 Read Data Cycles\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prdata0](index.html) module"]
pub struct PRDATA0_SPEC;
impl crate::RegisterSpec for PRDATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prdata0::R](R) reader structure"]
impl crate::Readable for PRDATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRDATA0 to value 0"]
impl crate::Resettable for PRDATA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
