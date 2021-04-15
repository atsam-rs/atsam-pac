#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RCLKBUSY`"]
pub type RCLKBUSY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Frequency measurement on-going"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Reference Clock busy"]
    #[inline(always)]
    pub fn rclkbusy(&self) -> RCLKBUSY_R {
        RCLKBUSY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
