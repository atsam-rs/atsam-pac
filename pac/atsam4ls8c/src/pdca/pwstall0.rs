#[doc = "Reader of register PWSTALL0"]
pub type R = crate::R<u32, super::PWSTALL0>;
#[doc = "Reader of field `STALL`"]
pub type STALL_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Stall cycles counted since last reset"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
