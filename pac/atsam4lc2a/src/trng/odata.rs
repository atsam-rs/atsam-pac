#[doc = "Register `ODATA` reader"]
pub struct R(crate::R<ODATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ODATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ODATA_SPEC>> for R {
    fn from(reader: crate::R<ODATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ODATA` reader - Output Data"]
pub struct ODATA_R(crate::FieldReader<bool, bool>);
impl ODATA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Output Data"]
    #[inline(always)]
    pub fn odata(&self) -> ODATA_R {
        ODATA_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Output Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odata](index.html) module"]
pub struct ODATA_SPEC;
impl crate::RegisterSpec for ODATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [odata::R](R) reader structure"]
impl crate::Readable for ODATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ODATA to value 0"]
impl crate::Resettable for ODATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
