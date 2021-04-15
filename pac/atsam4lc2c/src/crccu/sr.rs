#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `CRC`"]
pub type CRC_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Cyclic Redundancy Check Value"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
