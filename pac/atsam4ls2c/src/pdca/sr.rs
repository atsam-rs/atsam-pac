#[doc = "Reader of register SR%s"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `TEN`"]
pub type TEN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Transfer Enabled"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new((self.bits & 0x01) != 0)
    }
}
