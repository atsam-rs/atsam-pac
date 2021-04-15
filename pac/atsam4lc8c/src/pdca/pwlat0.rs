#[doc = "Reader of register PWLAT0"]
pub type R = crate::R<u32, super::PWLAT0>;
#[doc = "Reader of field `LAT`"]
pub type LAT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Maximum transfer initiation cycles counted since last reset"]
    #[inline(always)]
    pub fn lat(&self) -> LAT_R {
        LAT_R::new((self.bits & 0xffff) as u16)
    }
}
