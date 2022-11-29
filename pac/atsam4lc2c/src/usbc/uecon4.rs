#[doc = "Register `UECON4` reader"]
pub struct R(crate::R<UECON4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UECON4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UECON4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UECON4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXINE` reader - TXIN Interrupt Enable"]
pub type TXINE_R = crate::BitReader<bool>;
#[doc = "Field `RXOUTE` reader - RXOUT Interrupt Enable"]
pub type RXOUTE_R = crate::BitReader<bool>;
#[doc = "Field `RXSTPE` reader - RXSTP Interrupt Enable"]
pub type RXSTPE_R = crate::BitReader<bool>;
#[doc = "Field `NAKOUTE` reader - NAKOUT Interrupt Enable"]
pub type NAKOUTE_R = crate::BitReader<bool>;
#[doc = "Field `NAKINE` reader - NAKIN Interrupt Enable"]
pub type NAKINE_R = crate::BitReader<bool>;
#[doc = "Field `STALLEDE` reader - STALLED Interrupt Enable"]
pub type STALLEDE_R = crate::BitReader<bool>;
#[doc = "Field `NREPLY` reader - No Reply"]
pub type NREPLY_R = crate::BitReader<bool>;
#[doc = "Field `RAMACERE` reader - RAMACER Interrupt Enable"]
pub type RAMACERE_R = crate::BitReader<bool>;
#[doc = "Field `NBUSYBKE` reader - Number of Busy Banks Interrupt Enable"]
pub type NBUSYBKE_R = crate::BitReader<bool>;
#[doc = "Field `KILLBK` reader - Kill IN Bank"]
pub type KILLBK_R = crate::BitReader<bool>;
#[doc = "Field `FIFOCON` reader - FIFO Control"]
pub type FIFOCON_R = crate::BitReader<bool>;
#[doc = "Field `NYETDIS` reader - NYET Token Enable"]
pub type NYETDIS_R = crate::BitReader<bool>;
#[doc = "Field `RSTDT` reader - Reset Data Toggle"]
pub type RSTDT_R = crate::BitReader<bool>;
#[doc = "Field `STALLRQ` reader - STALL Request"]
pub type STALLRQ_R = crate::BitReader<bool>;
#[doc = "Field `BUSY0` reader - Busy Bank1 Enable"]
pub type BUSY0_R = crate::BitReader<bool>;
#[doc = "Field `BUSY1` reader - Busy Bank0 Enable"]
pub type BUSY1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - TXIN Interrupt Enable"]
    #[inline(always)]
    pub fn txine(&self) -> TXINE_R {
        TXINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RXOUT Interrupt Enable"]
    #[inline(always)]
    pub fn rxoute(&self) -> RXOUTE_R {
        RXOUTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXSTP Interrupt Enable"]
    #[inline(always)]
    pub fn rxstpe(&self) -> RXSTPE_R {
        RXSTPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NAKOUT Interrupt Enable"]
    #[inline(always)]
    pub fn nakoute(&self) -> NAKOUTE_R {
        NAKOUTE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKIN Interrupt Enable"]
    #[inline(always)]
    pub fn nakine(&self) -> NAKINE_R {
        NAKINE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - STALLED Interrupt Enable"]
    #[inline(always)]
    pub fn stallede(&self) -> STALLEDE_R {
        STALLEDE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - No Reply"]
    #[inline(always)]
    pub fn nreply(&self) -> NREPLY_R {
        NREPLY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - RAMACER Interrupt Enable"]
    #[inline(always)]
    pub fn ramacere(&self) -> RAMACERE_R {
        RAMACERE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Number of Busy Banks Interrupt Enable"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NBUSYBKE_R {
        NBUSYBKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Kill IN Bank"]
    #[inline(always)]
    pub fn killbk(&self) -> KILLBK_R {
        KILLBK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FIFOCON_R {
        FIFOCON_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - NYET Token Enable"]
    #[inline(always)]
    pub fn nyetdis(&self) -> NYETDIS_R {
        NYETDIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset Data Toggle"]
    #[inline(always)]
    pub fn rstdt(&self) -> RSTDT_R {
        RSTDT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - STALL Request"]
    #[inline(always)]
    pub fn stallrq(&self) -> STALLRQ_R {
        STALLRQ_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Busy Bank1 Enable"]
    #[inline(always)]
    pub fn busy0(&self) -> BUSY0_R {
        BUSY0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Busy Bank0 Enable"]
    #[inline(always)]
    pub fn busy1(&self) -> BUSY1_R {
        BUSY1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Endpoint Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon4](index.html) module"]
pub struct UECON4_SPEC;
impl crate::RegisterSpec for UECON4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uecon4::R](R) reader structure"]
impl crate::Readable for UECON4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UECON4 to value 0"]
impl crate::Resettable for UECON4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
