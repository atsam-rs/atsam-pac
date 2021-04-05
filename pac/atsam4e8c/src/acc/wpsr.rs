#[doc = "Reader of register WPSR"]
pub type R = crate::R<u32, super::WPSR>;
#[doc = "Reader of field `WPROTERR`"]
pub type WPROTERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Write PROTection ERRor"]
    #[inline(always)]
    pub fn wproterr(&self) -> WPROTERR_R {
        WPROTERR_R::new((self.bits & 0x01) != 0)
    }
}
