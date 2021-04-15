#[doc = "Reader of register ODATA"]
pub type R = crate::R<u32, super::ODATA>;
#[doc = "Reader of field `ODATA`"]
pub type ODATA_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Output Data"]
    #[inline(always)]
    pub fn odata(&self) -> ODATA_R {
        ODATA_R::new((self.bits & 0x01) != 0)
    }
}
