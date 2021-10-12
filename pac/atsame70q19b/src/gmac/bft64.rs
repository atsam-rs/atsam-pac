#[doc = "Register `BFT64` reader"]
pub struct R(crate::R<BFT64_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFT64_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFT64_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFT64_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NFTX` reader - 64 Byte Frames Transmitted without Error"]
pub struct NFTX_R(crate::FieldReader<u32, u32>);
impl NFTX_R {
    pub(crate) fn new(bits: u32) -> Self {
        NFTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NFTX_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - 64 Byte Frames Transmitted without Error"]
    #[inline(always)]
    pub fn nftx(&self) -> NFTX_R {
        NFTX_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "64 Byte Frames Transmitted Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bft64](index.html) module"]
pub struct BFT64_SPEC;
impl crate::RegisterSpec for BFT64_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bft64::R](R) reader structure"]
impl crate::Readable for BFT64_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BFT64 to value 0"]
impl crate::Resettable for BFT64_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
