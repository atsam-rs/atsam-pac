#[doc = "Register `VERSION` reader"]
pub struct R(crate::R<VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VERSION` reader - VERSION"]
pub type VERSION_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MFN` reader - MFN"]
pub type MFN_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - VERSION"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - MFN"]
    #[inline(always)]
    pub fn mfn(&self) -> MFN_R {
        MFN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Version Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](index.html) module"]
pub struct VERSION_SPEC;
impl crate::RegisterSpec for VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [version::R](R) reader structure"]
impl crate::Readable for VERSION_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VERSION to value 0x0101"]
impl crate::Resettable for VERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0101;
}
