#[doc = "Reader of register BUSY"]
pub type R = crate::R<u32, super::BUSY>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Channel Status"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
