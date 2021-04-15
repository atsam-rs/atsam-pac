#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `PSOK`"]
pub type PSOK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Power Scaling OK Status"]
    #[inline(always)]
    pub fn psok(&self) -> PSOK_R {
        PSOK_R::new((self.bits & 0x01) != 0)
    }
}
