#[doc = "Reader of register UESTA7"]
pub type R = crate::R<u32, super::UESTA7>;
#[doc = "Reader of field `TXINI`"]
pub type TXINI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOUTI`"]
pub type RXOUTI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXSTPI`"]
pub type RXSTPI_R = crate::R<bool, bool>;
#[doc = "Reader of field `NAKOUTI`"]
pub type NAKOUTI_R = crate::R<bool, bool>;
#[doc = "Reader of field `NAKINI`"]
pub type NAKINI_R = crate::R<bool, bool>;
#[doc = "Reader of field `STALLEDI`"]
pub type STALLEDI_R = crate::R<bool, bool>;
#[doc = "Reader of field `DTSEQ`"]
pub type DTSEQ_R = crate::R<u8, u8>;
#[doc = "Reader of field `RAMACERI`"]
pub type RAMACERI_R = crate::R<bool, bool>;
#[doc = "Reader of field `NBUSYBK`"]
pub type NBUSYBK_R = crate::R<u8, u8>;
#[doc = "Reader of field `CURRBK`"]
pub type CURRBK_R = crate::R<u8, u8>;
#[doc = "Control Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRLDIR_A {
    #[doc = "0: `0`"]
    OUT = 0,
    #[doc = "1: `1`"]
    IN = 1,
}
impl From<CTRLDIR_A> for bool {
    #[inline(always)]
    fn from(variant: CTRLDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTRLDIR`"]
pub type CTRLDIR_R = crate::R<bool, CTRLDIR_A>;
impl CTRLDIR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRLDIR_A {
        match self.bits {
            false => CTRLDIR_A::OUT,
            true => CTRLDIR_A::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == CTRLDIR_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in_(&self) -> bool {
        *self == CTRLDIR_A::IN
    }
}
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txini(&self) -> TXINI_R {
        TXINI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxouti(&self) -> RXOUTI_R {
        RXOUTI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt"]
    #[inline(always)]
    pub fn rxstpi(&self) -> RXSTPI_R {
        RXSTPI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt"]
    #[inline(always)]
    pub fn nakouti(&self) -> NAKOUTI_R {
        NAKOUTI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt"]
    #[inline(always)]
    pub fn nakini(&self) -> NAKINI_R {
        NAKINI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - STALLed Interrupt"]
    #[inline(always)]
    pub fn stalledi(&self) -> STALLEDI_R {
        STALLEDI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DTSEQ_R {
        DTSEQ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Ram Access Error Interrupt"]
    #[inline(always)]
    pub fn ramaceri(&self) -> RAMACERI_R {
        RAMACERI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Number Of Busy Banks"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NBUSYBK_R {
        NBUSYBK_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CURRBK_R {
        CURRBK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Control Direction"]
    #[inline(always)]
    pub fn ctrldir(&self) -> CTRLDIR_R {
        CTRLDIR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
