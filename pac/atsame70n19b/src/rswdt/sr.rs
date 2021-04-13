#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `WDUNF`"]
pub type WDUNF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Watchdog Underflow"]
    #[inline(always)]
    pub fn wdunf(&self) -> WDUNF_R {
        WDUNF_R::new((self.bits & 0x01) != 0)
    }
}
