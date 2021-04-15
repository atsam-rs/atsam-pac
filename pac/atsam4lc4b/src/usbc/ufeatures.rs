#[doc = "Reader of register UFEATURES"]
pub type R = crate::R<u32, super::UFEATURES>;
#[doc = "Reader of field `EPTNBRMAX`"]
pub type EPTNBRMAX_R = crate::R<u8, u8>;
#[doc = "Reader of field `UTMIMODE`"]
pub type UTMIMODE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Maximum Number of Pipes/Endpints"]
    #[inline(always)]
    pub fn eptnbrmax(&self) -> EPTNBRMAX_R {
        EPTNBRMAX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - UTMI Mode"]
    #[inline(always)]
    pub fn utmimode(&self) -> UTMIMODE_R {
        UTMIMODE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
