#[doc = "Register `MFID2` reader"]
pub struct R(crate::R<MFID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MFID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MFID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MFID2_SPEC>) -> Self {
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
#[doc = "Mailbox Family ID Register (MB = 2)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfid2](index.html) module"]
pub struct MFID2_SPEC;
impl crate::RegisterSpec for MFID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mfid2::R](R) reader structure"]
impl crate::Readable for MFID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MFID2 to value 0"]
impl crate::Resettable for MFID2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
