#[doc = "Reader of register RHR"]
pub type R = crate::R<u32, super::RHR>;
#[doc = "Reader of field `CDATA`"]
pub type CDATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Captured Data"]
    #[inline(always)]
    pub fn cdata(&self) -> CDATA_R {
        CDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
