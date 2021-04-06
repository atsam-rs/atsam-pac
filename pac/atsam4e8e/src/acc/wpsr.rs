#[doc = "Reader of register WPSR"]
pub type R = crate::R<u32, super::WPSR>;
#[doc = "Reader of field `WPVS`"]
pub type WPVS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Write Protection Violation Status"]
    #[inline(always)]
    pub fn wpvs(&self) -> WPVS_R {
        WPVS_R::new((self.bits & 0x01) != 0)
    }
}
