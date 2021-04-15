#[doc = "Reader of register OVSR"]
pub type R = crate::R<u32, super::OVSR>;
#[doc = "Reader of field `OVS`"]
pub type OVS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Overrun Interrupt Status"]
    #[inline(always)]
    pub fn ovs(&self) -> OVS_R {
        OVS_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
