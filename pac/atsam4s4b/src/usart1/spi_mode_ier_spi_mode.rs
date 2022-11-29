#[doc = "Register `IER_SPI_MODE` writer"]
pub struct W(crate::W<SPI_MODE_IER_SPI_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MODE_IER_SPI_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_MODE_IER_SPI_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MODE_IER_SPI_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXRDY` writer - RXRDY Interrupt Enable"]
pub type RXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_IER_SPI_MODE_SPEC, bool, O>;
#[doc = "Field `TXRDY` writer - TXRDY Interrupt Enable"]
pub type TXRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_IER_SPI_MODE_SPEC, bool, O>;
#[doc = "Field `ENDRX` writer - "]
pub type ENDRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_IER_SPI_MODE_SPEC, bool, O>;
#[doc = "Field `ENDTX` writer - "]
pub type ENDTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_IER_SPI_MODE_SPEC, bool, O>;
#[doc = "Field `OVRE` writer - Overrun Error Interrupt Enable"]
pub type OVRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_IER_SPI_MODE_SPEC, bool, O>;
#[doc = "Field `TXEMPTY` writer - TXEMPTY Interrupt Enable"]
pub type TXEMPTY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_MODE_IER_SPI_MODE_SPEC, bool, O>;
#[doc = "Field `UNRE` writer - SPI Underrun Error Interrupt Enable"]
pub type UNRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_IER_SPI_MODE_SPEC, bool, O>;
#[doc = "Field `TXBUFE` writer - "]
pub type TXBUFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_IER_SPI_MODE_SPEC, bool, O>;
#[doc = "Field `RXBUFF` writer - "]
pub type RXBUFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_IER_SPI_MODE_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - RXRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxrdy(&mut self) -> RXRDY_W<0> {
        RXRDY_W::new(self)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txrdy(&mut self) -> TXRDY_W<1> {
        TXRDY_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn endrx(&mut self) -> ENDRX_W<3> {
        ENDRX_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn endtx(&mut self) -> ENDTX_W<4> {
        ENDTX_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovre(&mut self) -> OVRE_W<5> {
        OVRE_W::new(self)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<9> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn unre(&mut self) -> UNRE_W<10> {
        UNRE_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn txbufe(&mut self) -> TXBUFE_W<11> {
        TXBUFE_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn rxbuff(&mut self) -> RXBUFF_W<12> {
        RXBUFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mode_ier_spi_mode](index.html) module"]
pub struct SPI_MODE_IER_SPI_MODE_SPEC;
impl crate::RegisterSpec for SPI_MODE_IER_SPI_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [spi_mode_ier_spi_mode::W](W) writer structure"]
impl crate::Writable for SPI_MODE_IER_SPI_MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
