#[doc = "Reader of register PRLAT1"]
pub type R = crate::R<u32, super::PRLAT1>;
#[doc = "Reader of field `LAT`"]
pub type LAT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Maximum Transfer initiation cycles counted since last reset"]
    #[inline(always)]
    pub fn lat(&self) -> LAT_R {
        LAT_R::new((self.bits & 0xffff) as u16)
    }
}