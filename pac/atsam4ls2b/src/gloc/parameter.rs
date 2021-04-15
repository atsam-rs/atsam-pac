#[doc = "Reader of register PARAMETER"]
pub type R = crate::R<u32, super::PARAMETER>;
#[doc = "Reader of field `LUTS`"]
pub type LUTS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - LUTs"]
    #[inline(always)]
    pub fn luts(&self) -> LUTS_R {
        LUTS_R::new((self.bits & 0xff) as u8)
    }
}
