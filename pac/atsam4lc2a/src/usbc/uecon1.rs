#[doc = "Register `UECON1` reader"]
pub struct R(crate::R<UECON1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UECON1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UECON1_SPEC>> for R {
    fn from(reader: crate::R<UECON1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXINE` reader - TXIN Interrupt Enable"]
pub struct TXINE_R(crate::FieldReader<bool, bool>);
impl TXINE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXINE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOUTE` reader - RXOUT Interrupt Enable"]
pub struct RXOUTE_R(crate::FieldReader<bool, bool>);
impl RXOUTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTPE` reader - RXSTP Interrupt Enable"]
pub struct RXSTPE_R(crate::FieldReader<bool, bool>);
impl RXSTPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSTPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKOUTE` reader - NAKOUT Interrupt Enable"]
pub struct NAKOUTE_R(crate::FieldReader<bool, bool>);
impl NAKOUTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKOUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKOUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKINE` reader - NAKIN Interrupt Enable"]
pub struct NAKINE_R(crate::FieldReader<bool, bool>);
impl NAKINE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKINE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALLEDE` reader - STALLED Interrupt Enable"]
pub struct STALLEDE_R(crate::FieldReader<bool, bool>);
impl STALLEDE_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALLEDE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLEDE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NREPLY` reader - No Reply"]
pub struct NREPLY_R(crate::FieldReader<bool, bool>);
impl NREPLY_R {
    pub(crate) fn new(bits: bool) -> Self {
        NREPLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NREPLY_R {
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
#[doc = "Field `NBUSYBKE` reader - Number of Busy Banks Interrupt Enable"]
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
#[doc = "Field `KILLBK` reader - Kill IN Bank"]
pub struct KILLBK_R(crate::FieldReader<bool, bool>);
impl KILLBK_R {
    pub(crate) fn new(bits: bool) -> Self {
        KILLBK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KILLBK_R {
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
#[doc = "Field `NYETDIS` reader - NYET Token Enable"]
pub struct NYETDIS_R(crate::FieldReader<bool, bool>);
impl NYETDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NYETDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NYETDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTDT` reader - Reset Data Toggle"]
pub struct RSTDT_R(crate::FieldReader<bool, bool>);
impl RSTDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALLRQ` reader - STALL Request"]
pub struct STALLRQ_R(crate::FieldReader<bool, bool>);
impl STALLRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALLRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY0` reader - Busy Bank1 Enable"]
pub struct BUSY0_R(crate::FieldReader<bool, bool>);
impl BUSY0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY1` reader - Busy Bank0 Enable"]
pub struct BUSY1_R(crate::FieldReader<bool, bool>);
impl BUSY1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Endpoint Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uecon1](index.html) module"]
pub struct UECON1_SPEC;
impl crate::RegisterSpec for UECON1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uecon1::R](R) reader structure"]
impl crate::Readable for UECON1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UECON1 to value 0"]
impl crate::Resettable for UECON1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
