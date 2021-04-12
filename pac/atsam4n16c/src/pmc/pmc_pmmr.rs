#[doc = "Reader of register PMC_PMMR"]
pub type R = crate::R<u32, super::PMC_PMMR>;
#[doc = "Writer for register PMC_PMMR"]
pub type W = crate::W<u32, super::PMC_PMMR>;
#[doc = "Register PMC_PMMR `reset()`'s with value 0x07ff_07ff"]
impl crate::ResetValue for super::PMC_PMMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x07ff_07ff
    }
}
impl R {}
impl W {}
