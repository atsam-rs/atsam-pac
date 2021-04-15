#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `WINDOW`"]
pub type WINDOW_R = crate::R<bool, bool>;
#[doc = "Reader of field `CLEARED`"]
pub type CLEARED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - WDT in window"]
    #[inline(always)]
    pub fn window(&self) -> WINDOW_R {
        WINDOW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WDT cleared"]
    #[inline(always)]
    pub fn cleared(&self) -> CLEARED_R {
        CLEARED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
