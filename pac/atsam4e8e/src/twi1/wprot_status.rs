#[doc = "Reader of register WPROT_STATUS"]
pub type R = crate::R<u32, super::WPROT_STATUS>;
#[doc = "Reader of field `WPROTERR`"]
pub type WPROTERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `WPROTADDR`"]
pub type WPROTADDR_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bit 0 - Write Protection Error"]
    #[inline(always)]
    pub fn wproterr(&self) -> WPROTERR_R {
        WPROTERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Error Address"]
    #[inline(always)]
    pub fn wprotaddr(&self) -> WPROTADDR_R {
        WPROTADDR_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
