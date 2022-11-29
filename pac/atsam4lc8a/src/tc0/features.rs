#[doc = "Register `FEATURES` reader"]
pub struct R(crate::R<FEATURES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEATURES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEATURES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEATURES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTRSIZE` reader - Counter Size"]
pub type CTRSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPDNIMPL` reader - Up Down is Implemented"]
pub type UPDNIMPL_R = crate::BitReader<bool>;
#[doc = "Field `BRPBHSB` reader - Bridge Type is PB to HSB"]
pub type BRPBHSB_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:7 - Counter Size"]
    #[inline(always)]
    pub fn ctrsize(&self) -> CTRSIZE_R {
        CTRSIZE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Up Down is Implemented"]
    #[inline(always)]
    pub fn updnimpl(&self) -> UPDNIMPL_R {
        UPDNIMPL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bridge Type is PB to HSB"]
    #[inline(always)]
    pub fn brpbhsb(&self) -> BRPBHSB_R {
        BRPBHSB_R::new(((self.bits >> 9) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
