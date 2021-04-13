#[doc = "Reader of register ODATAR[%s]"]
pub type R = crate::R<u32, super::ODATAR>;
#[doc = "Reader of field `ODATA`"]
pub type ODATA_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Data"]
    #[inline(always)]
    pub fn odata(&self) -> ODATA_R {
        ODATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
