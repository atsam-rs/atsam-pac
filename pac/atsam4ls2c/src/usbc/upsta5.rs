#[doc = "Reader of register UPSTA5"]
pub type R = crate::R<u32, super::UPSTA5>;
#[doc = "Reader of field `RXINI`"]
pub type RXINI_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXOUTI`"]
pub type TXOUTI_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXSTPI`"]
pub type TXSTPI_R = crate::R<bool, bool>;
#[doc = "Reader of field `PERRI`"]
pub type PERRI_R = crate::R<bool, bool>;
#[doc = "Reader of field `NAKEDI`"]
pub type NAKEDI_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRORFI`"]
pub type ERRORFI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXSTALLDI`"]
pub type RXSTALLDI_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTSEQ`"]
pub type DTSEQ_R = crate::R<u8, u8>;
#[doc = "Reader of field `RAMACERI`"]
pub type RAMACERI_R = crate::R<bool, bool>;
#[doc = "Reader of field `NBUSYBK`"]
pub type NBUSYBK_R = crate::R<u8, u8>;
#[doc = "Reader of field `CURRBK`"]
pub type CURRBK_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Received IN Data Interrupt"]
    #[inline(always)]
    pub fn rxini(&self) -> RXINI_R {
        RXINI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt"]
    #[inline(always)]
    pub fn txouti(&self) -> TXOUTI_R {
        TXOUTI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt"]
    #[inline(always)]
    pub fn txstpi(&self) -> TXSTPI_R {
        TXSTPI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt"]
    #[inline(always)]
    pub fn perri(&self) -> PERRI_R {
        PERRI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAKed Interrupt"]
    #[inline(always)]
    pub fn nakedi(&self) -> NAKEDI_R {
        NAKEDI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Errorflow Interrupt"]
    #[inline(always)]
    pub fn errorfi(&self) -> ERRORFI_R {
        ERRORFI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt"]
    #[inline(always)]
    pub fn rxstalldi(&self) -> RXSTALLDI_R {
        RXSTALLDI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DTSEQ_R {
        DTSEQ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Ram Access Error Interrupt"]
    #[inline(always)]
    pub fn ramaceri(&self) -> RAMACERI_R {
        RAMACERI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Number of Busy Bank"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NBUSYBK_R {
        NBUSYBK_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CURRBK_R {
        CURRBK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
