#[doc = "Register `MFID0` reader"]
pub struct R(crate::R<MFID0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MFID0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MFID0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MFID0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MFID` reader - Family ID"]
pub type MFID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:28 - Family ID"]
    #[inline(always)]
    pub fn mfid(&self) -> MFID_R {
        MFID_R::new(self.bits & 0x1fff_ffff)
    }
}
#[doc = "Mailbox Family ID Register (MB = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfid0](index.html) module"]
pub struct MFID0_SPEC;
impl crate::RegisterSpec for MFID0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mfid0::R](R) reader structure"]
impl crate::Readable for MFID0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MFID0 to value 0"]
impl crate::Resettable for MFID0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
