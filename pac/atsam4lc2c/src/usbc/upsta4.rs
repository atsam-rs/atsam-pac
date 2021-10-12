#[doc = "Register `UPSTA4` reader"]
pub struct R(crate::R<UPSTA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPSTA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPSTA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPSTA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXINI` reader - Received IN Data Interrupt"]
pub struct RXINI_R(crate::FieldReader<bool, bool>);
impl RXINI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXINI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXINI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXOUTI` reader - Transmitted OUT Data Interrupt"]
pub struct TXOUTI_R(crate::FieldReader<bool, bool>);
impl TXOUTI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXOUTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXOUTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSTPI` reader - Transmitted SETUP Interrupt"]
pub struct TXSTPI_R(crate::FieldReader<bool, bool>);
impl TXSTPI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSTPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXSTPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERRI` reader - Pipe Error Interrupt"]
pub struct PERRI_R(crate::FieldReader<bool, bool>);
impl PERRI_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERRI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKEDI` reader - NAKed Interrupt"]
pub struct NAKEDI_R(crate::FieldReader<bool, bool>);
impl NAKEDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKEDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKEDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRORFI` reader - Errorflow Interrupt"]
pub struct ERRORFI_R(crate::FieldReader<bool, bool>);
impl ERRORFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRORFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRORFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTALLDI` reader - Received STALLed Interrupt"]
pub struct RXSTALLDI_R(crate::FieldReader<bool, bool>);
impl RXSTALLDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSTALLDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTALLDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTSEQ` reader - Data Toggle Sequence"]
pub struct DTSEQ_R(crate::FieldReader<u8, u8>);
impl DTSEQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTSEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTSEQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMACERI` reader - Ram Access Error Interrupt"]
pub struct RAMACERI_R(crate::FieldReader<bool, bool>);
impl RAMACERI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAMACERI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMACERI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBUSYBK` reader - Number of Busy Bank"]
pub struct NBUSYBK_R(crate::FieldReader<u8, u8>);
impl NBUSYBK_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBUSYBK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBUSYBK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURRBK` reader - Current Bank"]
pub struct CURRBK_R(crate::FieldReader<u8, u8>);
impl CURRBK_R {
    pub(crate) fn new(bits: u8) -> Self {
        CURRBK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURRBK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta4](index.html) module"]
pub struct UPSTA4_SPEC;
impl crate::RegisterSpec for UPSTA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [upsta4::R](R) reader structure"]
impl crate::Readable for UPSTA4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UPSTA4 to value 0"]
impl crate::Resettable for UPSTA4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
