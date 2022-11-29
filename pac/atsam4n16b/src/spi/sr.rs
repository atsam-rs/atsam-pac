#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDRF` reader - Receive Data Register Full"]
pub type RDRF_R = crate::BitReader<bool>;
#[doc = "Field `TDRE` reader - Transmit Data Register Empty"]
pub type TDRE_R = crate::BitReader<bool>;
#[doc = "Field `MODF` reader - Mode Fault Error"]
pub type MODF_R = crate::BitReader<bool>;
#[doc = "Field `OVRES` reader - Overrun Error Status"]
pub type OVRES_R = crate::BitReader<bool>;
#[doc = "Field `ENDRX` reader - End of RX buffer"]
pub type ENDRX_R = crate::BitReader<bool>;
#[doc = "Field `ENDTX` reader - End of TX buffer"]
pub type ENDTX_R = crate::BitReader<bool>;
#[doc = "Field `RXBUFF` reader - RX Buffer Full"]
pub type RXBUFF_R = crate::BitReader<bool>;
#[doc = "Field `TXBUFE` reader - TX Buffer Empty"]
pub type TXBUFE_R = crate::BitReader<bool>;
#[doc = "Field `NSSR` reader - NSS Rising"]
pub type NSSR_R = crate::BitReader<bool>;
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty"]
pub type TXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `UNDES` reader - Underrun Error Status (Slave Mode Only)"]
pub type UNDES_R = crate::BitReader<bool>;
#[doc = "Field `SPIENS` reader - SPI Enable Status"]
pub type SPIENS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Receive Data Register Full"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Status"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R {
        OVRES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - End of RX buffer"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of TX buffer"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX Buffer Empty"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NSS Rising"]
    #[inline(always)]
    pub fn nssr(&self) -> NSSR_R {
        NSSR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Registers Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Underrun Error Status (Slave Mode Only)"]
    #[inline(always)]
    pub fn undes(&self) -> UNDES_R {
        UNDES_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI Enable Status"]
    #[inline(always)]
    pub fn spiens(&self) -> SPIENS_R {
        SPIENS_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0xf0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0xf0;
}
