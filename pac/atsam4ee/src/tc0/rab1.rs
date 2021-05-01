#[doc = "Reader of register RAB1"]
pub type R = crate::R<u32, super::RAB1>;
#[doc = "Reader of field `RAB`"]
pub type RAB_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Register A or Register B"]
    #[inline(always)]
    pub fn rab(&self) -> RAB_R {
        RAB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
