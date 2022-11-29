#[doc = "Register `UPCON7` reader"]
pub struct R(crate::R<UPCON7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPCON7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPCON7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPCON7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXINE` reader - RXIN Interrupt Enable"]
pub type RXINE_R = crate::BitReader<bool>;
#[doc = "Field `TXOUTE` reader - TXOUT Interrupt Enable"]
pub type TXOUTE_R = crate::BitReader<bool>;
#[doc = "Field `TXSTPE` reader - TXSTP Interrupt Enable"]
pub type TXSTPE_R = crate::BitReader<bool>;
#[doc = "Field `PERRE` reader - PERR Interrupt Enable"]
pub type PERRE_R = crate::BitReader<bool>;
#[doc = "Field `NAKEDE` reader - NAKED Interrupt Enable"]
pub type NAKEDE_R = crate::BitReader<bool>;
#[doc = "Field `ERRORFIE` reader - ERRORFI Interrupt Enable"]
pub type ERRORFIE_R = crate::BitReader<bool>;
#[doc = "Field `RXSTALLDE` reader - RXTALLD Interrupt Enable"]
pub type RXSTALLDE_R = crate::BitReader<bool>;
#[doc = "Field `RAMACERE` reader - RAMACER Interrupt Enable"]
pub type RAMACERE_R = crate::BitReader<bool>;
#[doc = "Field `NBUSYBKE` reader - NBUSYBKInterrupt Enable"]
pub type NBUSYBKE_R = crate::BitReader<bool>;
#[doc = "Field `FIFOCON` reader - FIFO Control"]
pub type FIFOCON_R = crate::BitReader<bool>;
#[doc = "Field `PFREEZE` reader - Pipe Freeze"]
pub type PFREEZE_R = crate::BitReader<bool>;
#[doc = "Field `INITDTGL` reader - Data Toggle Initialization"]
pub type INITDTGL_R = crate::BitReader<bool>;
#[doc = "Field `INITBK` reader - Bank Initialization"]
pub type INITBK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - RXIN Interrupt Enable"]
    #[inline(always)]
    pub fn rxine(&self) -> RXINE_R {
        RXINE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXOUT Interrupt Enable"]
    #[inline(always)]
    pub fn txoute(&self) -> TXOUTE_R {
        TXOUTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TXSTP Interrupt Enable"]
    #[inline(always)]
    pub fn txstpe(&self) -> TXSTPE_R {
        TXSTPE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PERR Interrupt Enable"]
    #[inline(always)]
    pub fn perre(&self) -> PERRE_R {
        PERRE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKED Interrupt Enable"]
    #[inline(always)]
    pub fn nakede(&self) -> NAKEDE_R {
        NAKEDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ERRORFI Interrupt Enable"]
    #[inline(always)]
    pub fn errorfie(&self) -> ERRORFIE_R {
        ERRORFIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXTALLD Interrupt Enable"]
    #[inline(always)]
    pub fn rxstallde(&self) -> RXSTALLDE_R {
        RXSTALLDE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - RAMACER Interrupt Enable"]
    #[inline(always)]
    pub fn ramacere(&self) -> RAMACERE_R {
        RAMACERE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - NBUSYBKInterrupt Enable"]
    #[inline(always)]
    pub fn nbusybke(&self) -> NBUSYBKE_R {
        NBUSYBKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - FIFO Control"]
    #[inline(always)]
    pub fn fifocon(&self) -> FIFOCON_R {
        FIFOCON_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - Pipe Freeze"]
    #[inline(always)]
    pub fn pfreeze(&self) -> PFREEZE_R {
        PFREEZE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data Toggle Initialization"]
    #[inline(always)]
    pub fn initdtgl(&self) -> INITDTGL_R {
        INITDTGL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank Initialization"]
    #[inline(always)]
    pub fn initbk(&self) -> INITBK_R {
        INITBK_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Pipe Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upcon7](index.html) module"]
pub struct UPCON7_SPEC;
impl crate::RegisterSpec for UPCON7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [upcon7::R](R) reader structure"]
impl crate::Readable for UPCON7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UPCON7 to value 0"]
impl crate::Resettable for UPCON7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
