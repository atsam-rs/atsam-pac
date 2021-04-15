#[doc = "Reader of register PARAMETER%s"]
pub type R = crate::R<u32, super::PARAMETER>;
#[doc = "Reader of field `PARAMETER`"]
pub type PARAMETER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Parameter"]
    #[inline(always)]
    pub fn parameter(&self) -> PARAMETER_R {
        PARAMETER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
