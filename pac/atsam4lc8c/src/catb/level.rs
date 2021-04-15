#[doc = "Reader of register LEVEL"]
pub type R = crate::R<u32, super::LEVEL>;
#[doc = "Reader of field `FLEVEL`"]
pub type FLEVEL_R = crate::R<u16, u16>;
#[doc = "Reader of field `RLEVEL`"]
pub type RLEVEL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:11 - Fractional Sensor Level"]
    #[inline(always)]
    pub fn flevel(&self) -> FLEVEL_R {
        FLEVEL_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - Integer Sensor Level"]
    #[inline(always)]
    pub fn rlevel(&self) -> RLEVEL_R {
        RLEVEL_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
