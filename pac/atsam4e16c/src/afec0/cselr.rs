#[doc = "Reader of register CSELR"]
pub type R = crate::R<u32, super::CSELR>;
#[doc = "Reader of field `CSEL`"]
pub type CSEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Channel Selection"]
    #[inline(always)]
    pub fn csel(&self) -> CSEL_R {
        CSEL_R::new((self.bits & 0x0f) as u8)
    }
}
