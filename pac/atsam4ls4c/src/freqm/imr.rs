#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `DONE`"]
pub type DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCLKRDY`"]
pub type RCLKRDY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Frequency measurment done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reference Clock ready"]
    #[inline(always)]
    pub fn rclkrdy(&self) -> RCLKRDY_R {
        RCLKRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
