#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - Receiver Ready"]
pub type RXRDY_R = crate::BitReader<bool>;
#[doc = "Field `TXRDY` reader - Transmitter Ready"]
pub type TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `RXBRK` reader - Break Received/End of Break"]
pub type RXBRK_R = crate::BitReader<bool>;
#[doc = "Field `ENDRX` reader - End of Receiver Transfer"]
pub type ENDRX_R = crate::BitReader<bool>;
#[doc = "Field `ENDTX` reader - End of Transmitter Transfer"]
pub type ENDTX_R = crate::BitReader<bool>;
#[doc = "Field `OVRE` reader - Overrun Error"]
pub type OVRE_R = crate::BitReader<bool>;
#[doc = "Field `FRAME` reader - Framing Error"]
pub type FRAME_R = crate::BitReader<bool>;
#[doc = "Field `PARE` reader - Parity Error"]
pub type PARE_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUT` reader - Receiver Time-out"]
pub type TIMEOUT_R = crate::BitReader<bool>;
#[doc = "Field `TXEMPTY` reader - Transmitter Empty"]
pub type TXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `ITER` reader - MaxNumber of Repetitions Reached"]
pub type ITER_R = crate::BitReader<bool>;
#[doc = "Field `TXBUFE` reader - Transmission Buffer Empty"]
pub type TXBUFE_R = crate::BitReader<bool>;
#[doc = "Field `RXBUFF` reader - Reception Buffer Full"]
pub type RXBUFF_R = crate::BitReader<bool>;
#[doc = "Field `NACK` reader - Non AcknowledgeInterrupt"]
pub type NACK_R = crate::BitReader<bool>;
#[doc = "Field `CTSIC` reader - Clear to Send Input Change Flag"]
pub type CTSIC_R = crate::BitReader<bool>;
#[doc = "Field `CTS` reader - Image of CTS Input"]
pub type CTS_R = crate::BitReader<bool>;
#[doc = "Field `MANERR` reader - Manchester Error"]
pub type MANERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Receiver Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Break Received/End of Break"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RXBRK_R {
        RXBRK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Receiver Transfer"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of Transmitter Transfer"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing Error"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Error"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receiver Time-out"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MaxNumber of Repetitions Reached"]
    #[inline(always)]
    pub fn iter(&self) -> ITER_R {
        ITER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmission Buffer Empty"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reception Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Non AcknowledgeInterrupt"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Flag"]
    #[inline(always)]
    pub fn ctsic(&self) -> CTSIC_R {
        CTSIC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - Image of CTS Input"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Manchester Error"]
    #[inline(always)]
    pub fn manerr(&self) -> MANERR_R {
        MANERR_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
