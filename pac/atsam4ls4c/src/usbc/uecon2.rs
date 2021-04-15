#[doc = "Reader of register UECON2"]
pub type R = crate::R<u32, super::UECON2>;
#[doc = "Reader of field `TXINE`"]
pub type TXINE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXOUTE`"]
pub type RXOUTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXSTPE`"]
pub type RXSTPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `NAKOUTE`"]
pub type NAKOUTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `NAKINE`"]
pub type NAKINE_R = crate::R<bool, bool>;
#[doc = "Reader of field `STALLEDE`"]
pub type STALLEDE_R = crate::R<bool, bool>;
#[doc = "Reader of field `NREPLY`"]
pub type NREPLY_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAMACERE`"]
pub type RAMACERE_R = crate::R<bool, bool>;
#[doc = "Reader of field `NBUSYBKE`"]
pub type NBUSYBKE_R = crate::R<bool, bool>;
#[doc = "Reader of field `KILLBK`"]
pub type KILLBK_R = crate::R<bool, bool>;
#[doc = "Reader of field `FIFOCON`"]
pub type FIFOCON_R = crate::R<bool, bool>;
#[doc = "Reader of field `NYETDIS`"]
pub type NYETDIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSTDT`"]
pub type RSTDT_R = crate::R<bool, bool>;
#[doc = "Reader of field `STALLRQ`"]
pub type STALLRQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY0`"]
pub type BUSY0_R = crate::R<bool, bool>;
#[doc = "Reader of field `BUSY1`"]
pub type BUSY1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - TXIN Interrupt Enable"]
    #[inline(always)]
    pub fn txine(&self) -> TXINE_R {
        TXINE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RXOUT Interrupt Enable"]
    #[inline(always)]
    pub fn rxoute(&self) -> RXOUTE_R {
        RXOUTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RXSTP Interrupt Enable"]
    #[inline(always)]
    pub fn rxstpe(&self) -> RXSTPE_R {
        RXSTPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NAKOUT Interrupt Enable"]
    #[inline(always)]
    pub fn nakoute(&self) -> NAKOUTE_R {
        NAKOUTE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAKIN Interrupt Enable"]
    #[inline(always)]
    pub fn nakine(&self) -> NAKINE_R {
        NAKINE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - STALLED Interrupt Enable"]
    #[inline(always)]
    pub fn stallede(&self) -> STALLEDE_R {
        STALLEDE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - No Reply"]
    #[inline(always)]
    pub fn nreply(&self) -> NREPLY_R {
        NREPLY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RAMACER Interrupt Enable"]
    #[inline(always)]
    pub fn ramacere(&self) -> RAMACERE_R {
        RAMACERE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NBUSYBKE_R {
        NBUSYBKE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    pub fn killbk(&self) -> KILLBK_R {
        KILLBK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FIFOCON_R {
        FIFOCON_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 17 - NYET Token Enable"]
    #[inline(always)]
    pub fn nyetdis(&self) -> NYETDIS_R {
        NYETDIS_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Reset Data Toggle"]
    #[inline(always)]
    pub fn rstdt(&self) -> RSTDT_R {
        RSTDT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - STALL Request"]
    #[inline(always)]
    pub fn stallrq(&self) -> STALLRQ_R {
        STALLRQ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Busy Bank1 Enable"]
    #[inline(always)]
    pub fn busy0(&self) -> BUSY0_R {
        BUSY0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Busy Bank0 Enable"]
    #[inline(always)]
    pub fn busy1(&self) -> BUSY1_R {
        BUSY1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
