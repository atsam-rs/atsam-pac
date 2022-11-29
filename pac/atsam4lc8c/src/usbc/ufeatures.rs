#[doc = "Register `UFEATURES` reader"]
pub struct R(crate::R<UFEATURES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UFEATURES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UFEATURES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UFEATURES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPTNBRMAX` reader - Maximum Number of Pipes/Endpints"]
pub type EPTNBRMAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UTMIMODE` reader - UTMI Mode"]
pub type UTMIMODE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Maximum Number of Pipes/Endpints"]
    #[inline(always)]
    pub fn eptnbrmax(&self) -> EPTNBRMAX_R {
        EPTNBRMAX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - UTMI Mode"]
    #[inline(always)]
    pub fn utmimode(&self) -> UTMIMODE_R {
        UTMIMODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "IP Features Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ufeatures](index.html) module"]
pub struct UFEATURES_SPEC;
impl crate::RegisterSpec for UFEATURES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ufeatures::R](R) reader structure"]
impl crate::Readable for UFEATURES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UFEATURES to value 0x07"]
impl crate::Resettable for UFEATURES_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
