#[doc = "Register `US_CSR_SPI_MODE` reader"]
pub struct R(crate::R<US_CSR_SPI_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<US_CSR_SPI_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<US_CSR_SPI_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<US_CSR_SPI_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - Receiver Ready (cleared by reading US_RHR)"]
pub struct RXRDY_R(crate::FieldReader<bool, bool>);
impl RXRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRDY` reader - Transmitter Ready (cleared by writing US_THR)"]
pub struct TXRDY_R(crate::FieldReader<bool, bool>);
impl TXRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRE` reader - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
pub struct OVRE_R(crate::FieldReader<bool, bool>);
impl OVRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXEMPTY` reader - Transmitter Empty (cleared by writing US_THR)"]
pub struct TXEMPTY_R(crate::FieldReader<bool, bool>);
impl TXEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNRE` reader - SPI Underrun Error"]
pub struct UNRE_R(crate::FieldReader<bool, bool>);
impl UNRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSSE` reader - NSS Line (Driving CTS Pin) Rising or Falling Edge Event"]
pub struct NSSE_R(crate::FieldReader<bool, bool>);
impl NSSE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSSE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSS` reader - Image of NSS Line"]
pub struct NSS_R(crate::FieldReader<bool, bool>);
impl NSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        NSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Ready (cleared by reading US_RHR)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun Error (cleared by writing a one to bit US_CR.RSTSTA)"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty (cleared by writing US_THR)"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SPI Underrun Error"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 19 - NSS Line (Driving CTS Pin) Rising or Falling Edge Event"]
    #[inline(always)]
    pub fn nsse(&self) -> NSSE_R {
        NSSE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Image of NSS Line"]
    #[inline(always)]
    pub fn nss(&self) -> NSS_R {
        NSS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [us_csr_spi_mode](index.html) module"]
pub struct US_CSR_SPI_MODE_SPEC;
impl crate::RegisterSpec for US_CSR_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [us_csr_spi_mode::R](R) reader structure"]
impl crate::Readable for US_CSR_SPI_MODE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets US_CSR_SPI_MODE to value 0"]
impl crate::Resettable for US_CSR_SPI_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
