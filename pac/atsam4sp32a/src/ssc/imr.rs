#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXRDY` reader - Transmit Ready Interrupt Mask"]
pub type TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `TXEMPTY` reader - Transmit Empty Interrupt Mask"]
pub type TXEMPTY_R = crate::BitReader<bool>;
#[doc = "Field `ENDTX` reader - End of Transmission Interrupt Mask"]
pub type ENDTX_R = crate::BitReader<bool>;
#[doc = "Field `TXBUFE` reader - Transmit Buffer Empty Interrupt Mask"]
pub type TXBUFE_R = crate::BitReader<bool>;
#[doc = "Field `RXRDY` reader - Receive Ready Interrupt Mask"]
pub type RXRDY_R = crate::BitReader<bool>;
#[doc = "Field `OVRUN` reader - Receive Overrun Interrupt Mask"]
pub type OVRUN_R = crate::BitReader<bool>;
#[doc = "Field `ENDRX` reader - End of Reception Interrupt Mask"]
pub type ENDRX_R = crate::BitReader<bool>;
#[doc = "Field `RXBUFF` reader - Receive Buffer Full Interrupt Mask"]
pub type RXBUFF_R = crate::BitReader<bool>;
#[doc = "Field `CP0` reader - Compare 0 Interrupt Mask"]
pub type CP0_R = crate::BitReader<bool>;
#[doc = "Field `CP1` reader - Compare 1 Interrupt Mask"]
pub type CP1_R = crate::BitReader<bool>;
#[doc = "Field `TXSYN` reader - Tx Sync Interrupt Mask"]
pub type TXSYN_R = crate::BitReader<bool>;
#[doc = "Field `RXSYN` reader - Rx Sync Interrupt Mask"]
pub type RXSYN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Transmit Ready Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - End of Transmission Interrupt Mask"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Buffer Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Ready Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ovrun(&self) -> OVRUN_R {
        OVRUN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - End of Reception Interrupt Mask"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Compare 0 Interrupt Mask"]
    #[inline(always)]
    pub fn cp0(&self) -> CP0_R {
        CP0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Compare 1 Interrupt Mask"]
    #[inline(always)]
    pub fn cp1(&self) -> CP1_R {
        CP1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx Sync Interrupt Mask"]
    #[inline(always)]
    pub fn txsyn(&self) -> TXSYN_R {
        TXSYN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx Sync Interrupt Mask"]
    #[inline(always)]
    pub fn rxsyn(&self) -> RXSYN_R {
        RXSYN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
