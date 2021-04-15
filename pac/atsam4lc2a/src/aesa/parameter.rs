#[doc = "Reader of register PARAMETER"]
pub type R = crate::R<u32, super::PARAMETER>;
#[doc = "Reader of field `KEYSIZE`"]
pub type KEYSIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `OPMODE`"]
pub type OPMODE_R = crate::R<u8, u8>;
#[doc = "Reader of field `CTRMEAS`"]
pub type CTRMEAS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - Maximum Key Size"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Maximum Number of Confidentiality Modes of Operation"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Countermeasures"]
    #[inline(always)]
    pub fn ctrmeas(&self) -> CTRMEAS_R {
        CTRMEAS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
