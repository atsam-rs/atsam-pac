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
pub struct RXINE_R(crate::FieldReader<bool, bool>);
impl RXINE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXINE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOUTE` reader - TXOUT Interrupt Enable"]
pub struct TXOUTE_R(crate::FieldReader<bool, bool>);
impl TXOUTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSTPE` reader - TXSTP Interrupt Enable"]
pub struct TXSTPE_R(crate::FieldReader<bool, bool>);
impl TXSTPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSTPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSTPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERRE` reader - PERR Interrupt Enable"]
pub struct PERRE_R(crate::FieldReader<bool, bool>);
impl PERRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKEDE` reader - NAKED Interrupt Enable"]
pub struct NAKEDE_R(crate::FieldReader<bool, bool>);
impl NAKEDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKEDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKEDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRORFIE` reader - ERRORFI Interrupt Enable"]
pub struct ERRORFIE_R(crate::FieldReader<bool, bool>);
impl ERRORFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRORFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRORFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTALLDE` reader - RXTALLD Interrupt Enable"]
pub struct RXSTALLDE_R(crate::FieldReader<bool, bool>);
impl RXSTALLDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSTALLDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTALLDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMACERE` reader - RAMACER Interrupt Enable"]
pub struct RAMACERE_R(crate::FieldReader<bool, bool>);
impl RAMACERE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAMACERE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMACERE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBUSYBKE` reader - NBUSYBKInterrupt Enable"]
pub struct NBUSYBKE_R(crate::FieldReader<bool, bool>);
impl NBUSYBKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NBUSYBKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBUSYBKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFOCON` reader - FIFO Control"]
pub struct FIFOCON_R(crate::FieldReader<bool, bool>);
impl FIFOCON_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFOCON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFOCON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PFREEZE` reader - Pipe Freeze"]
pub struct PFREEZE_R(crate::FieldReader<bool, bool>);
impl PFREEZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PFREEZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PFREEZE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITDTGL` reader - Data Toggle Initialization"]
pub struct INITDTGL_R(crate::FieldReader<bool, bool>);
impl INITDTGL_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITDTGL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INITDTGL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INITBK` reader - Bank Initialization"]
pub struct INITBK_R(crate::FieldReader<bool, bool>);
impl INITBK_R {
    pub(crate) fn new(bits: bool) -> Self {
        INITBK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INITBK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
