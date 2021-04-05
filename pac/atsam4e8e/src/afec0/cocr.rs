#[doc = "Reader of register COCR"]
pub type R = crate::R<u32, super::COCR>;
#[doc = "Reader of field `AOFF`"]
pub type AOFF_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:11 - Analog Offset"]
    #[inline(always)]
    pub fn aoff(&self) -> AOFF_R {
        AOFF_R::new((self.bits & 0x0fff) as u16)
    }
}
