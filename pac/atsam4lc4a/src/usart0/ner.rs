#[doc = "Register `NER` reader"]
pub struct R(crate::R<NER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NB_ERRORS` reader - Error number during ISO7816 transfers"]
pub struct NB_ERRORS_R(crate::FieldReader<u8, u8>);
impl NB_ERRORS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NB_ERRORS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NB_ERRORS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Error number during ISO7816 transfers"]
    #[inline(always)]
    pub fn nb_errors(&self) -> NB_ERRORS_R {
        NB_ERRORS_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Number of Errors Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ner](index.html) module"]
pub struct NER_SPEC;
impl crate::RegisterSpec for NER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ner::R](R) reader structure"]
impl crate::Readable for NER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets NER to value 0"]
impl crate::Resettable for NER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
