#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `ERRIMR`"]
pub type ERRIMR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn errimr(&self) -> ERRIMR_R {
        ERRIMR_R::new((self.bits & 0x01) != 0)
    }
}
