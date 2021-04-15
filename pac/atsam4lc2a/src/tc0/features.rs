#[doc = "Register `FEATURES` reader"]
pub struct R(crate::R<FEATURES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEATURES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FEATURES_SPEC>> for R {
    fn from(reader: crate::R<FEATURES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTRSIZE` reader - Counter Size"]
pub struct CTRSIZE_R(crate::FieldReader<u8, u8>);
impl CTRSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTRSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDNIMPL` reader - Up Down is Implemented"]
pub struct UPDNIMPL_R(crate::FieldReader<bool, bool>);
impl UPDNIMPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPDNIMPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPDNIMPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BRPBHSB` reader - Bridge Type is PB to HSB"]
pub struct BRPBHSB_R(crate::FieldReader<bool, bool>);
impl BRPBHSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        BRPBHSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRPBHSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Counter Size"]
    #[inline(always)]
    pub fn ctrsize(&self) -> CTRSIZE_R {
        CTRSIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Up Down is Implemented"]
    #[inline(always)]
    pub fn updnimpl(&self) -> UPDNIMPL_R {
        UPDNIMPL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Bridge Type is PB to HSB"]
    #[inline(always)]
    pub fn brpbhsb(&self) -> BRPBHSB_R {
        BRPBHSB_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "Features Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [features](index.html) module"]
pub struct FEATURES_SPEC;
impl crate::RegisterSpec for FEATURES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [features::R](R) reader structure"]
impl crate::Readable for FEATURES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FEATURES to value 0"]
impl crate::Resettable for FEATURES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
