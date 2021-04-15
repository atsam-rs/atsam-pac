#[doc = "Reader of register PARAMETER"]
pub type R = crate::R<u32, super::PARAMETER>;
#[doc = "Reader of field `NPINS`"]
pub type NPINS_R = crate::R<u8, u8>;
#[doc = "Reader of field `NSTATUS`"]
pub type NSTATUS_R = crate::R<u8, u8>;
#[doc = "Reader of field `FRACTIONAL`"]
pub type FRACTIONAL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of Pins"]
    #[inline(always)]
    pub fn npins(&self) -> NPINS_R {
        NPINS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of Status bits"]
    #[inline(always)]
    pub fn nstatus(&self) -> NSTATUS_R {
        NSTATUS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Number of Fractional bits"]
    #[inline(always)]
    pub fn fractional(&self) -> FRACTIONAL_R {
        FRACTIONAL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
