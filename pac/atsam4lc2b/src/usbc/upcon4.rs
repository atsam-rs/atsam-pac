#[doc = "Reader of register UPCON4"]
pub type R = crate::R<u32, super::UPCON4>;
#[doc = "Reader of field `RXINE`"]
pub type RXINE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXOUTE`"]
pub type TXOUTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXSTPE`"]
pub type TXSTPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PERRE`"]
pub type PERRE_R = crate::R<bool, bool>;
#[doc = "Reader of field `NAKEDE`"]
pub type NAKEDE_R = crate::R<bool, bool>;
#[doc = "Reader of field `ERRORFIE`"]
pub type ERRORFIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXSTALLDE`"]
pub type RXSTALLDE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAMACERE`"]
pub type RAMACERE_R = crate::R<bool, bool>;
#[doc = "Reader of field `NBUSYBKE`"]
pub type NBUSYBKE_R = crate::R<bool, bool>;
#[doc = "Reader of field `FIFOCON`"]
pub type FIFOCON_R = crate::R<bool, bool>;
#[doc = "Reader of field `PFREEZE`"]
pub type PFREEZE_R = crate::R<bool, bool>;
#[doc = "Reader of field `INITDTGL`"]
pub type INITDTGL_R = crate::R<bool, bool>;
#[doc = "Reader of field `INITBK`"]
pub type INITBK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RXIN Interrupt Enable"]
    #[inline(always)]
    pub fn rxine(&self) -> RXINE_R {
        RXINE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXOUT Interrupt Enable"]
    #[inline(always)]
    pub fn txoute(&self) -> TXOUTE_R {
        TXOUTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TXSTP Interrupt Enable"]
    #[inline(always)]
    pub fn txstpe(&self) -> TXSTPE_R {
        TXSTPE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PERR Interrupt Enable"]
    #[inline(always)]
    pub fn perre(&self) -> PERRE_R {
        PERRE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAKED Interrupt Enable"]
    #[inline(always)]
    pub fn nakede(&self) -> NAKEDE_R {
        NAKEDE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ERRORFI Interrupt Enable"]
    #[inline(always)]
    pub fn errorfie(&self) -> ERRORFIE_R {
        ERRORFIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RXTALLD Interrupt Enable"]
    #[inline(always)]
    pub fn rxstallde(&self) -> RXSTALLDE_R {
        RXSTALLDE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RAMACER Interrupt Enable"]
    #[inline(always)]
    pub fn ramacere(&self) -> RAMACERE_R {
        RAMACERE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - NBUSYBKInterrupt Enable"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NBUSYBKE_R {
        NBUSYBKE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FIFOCON_R {
        FIFOCON_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Pipe Freeze"]
    #[inline(always)]
    pub fn pfreeze(&self) -> PFREEZE_R {
        PFREEZE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Data Toggle Initialization"]
    #[inline(always)]
    pub fn initdtgl(&self) -> INITDTGL_R {
        INITDTGL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Bank Initialization"]
    #[inline(always)]
    pub fn initbk(&self) -> INITBK_R {
        INITBK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
