#[doc = "Register `RAW` reader"]
pub struct R(crate::R<RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAWA` reader - Current Sensor Raw Value"]
pub struct RAWA_R(crate::FieldReader<u8, u8>);
impl RAWA_R {
    pub(crate) fn new(bits: u8) -> Self {
        RAWA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAWB` reader - Last Sensor Raw Value"]
pub struct RAWB_R(crate::FieldReader<u8, u8>);
impl RAWB_R {
    pub(crate) fn new(bits: u8) -> Self {
        RAWB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAWB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 16:23 - Current Sensor Raw Value"]
    #[inline(always)]
    pub fn rawa(&self) -> RAWA_R {
        RAWA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Last Sensor Raw Value"]
    #[inline(always)]
    pub fn rawb(&self) -> RAWB_R {
        RAWB_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Sensor Raw Value\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [raw](index.html) module"]
pub struct RAW_SPEC;
impl crate::RegisterSpec for RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [raw::R](R) reader structure"]
impl crate::Readable for RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RAW to value 0"]
impl crate::Resettable for RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
