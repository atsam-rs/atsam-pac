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
pub type RDRF_R = crate::BitReader<RDRFSELECT_A>;
#[doc = "Receive Data Register Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDRFSELECT_A {
    #[doc = "0: No data has been received since the last read of RDR"]
    _0 = 0,
    #[doc = "1: Data has been received and the received data has been transferred from the serializer to RDR since the last readof RDR."]
    _1 = 1,
}
impl From<RDRFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RDRFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RDRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDRFSELECT_A {
        match self.bits {
            false => RDRFSELECT_A::_0,
            true => RDRFSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDRFSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDRFSELECT_A::_1
    }
}
#[doc = "Field `TDRE` reader - Transmit Data Register Empty"]
pub type TDRE_R = crate::BitReader<TDRESELECT_A>;
#[doc = "Transmit Data Register Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDRESELECT_A {
    #[doc = "0: Data has been written to TDR and not yet transferred to the serializer."]
    _0 = 0,
    #[doc = "1: The last data written in the Transmit Data Register has been transferred to the serializer.TDRE equals zero when the SPI is disabled or at reset. The SPI enable command sets this bit to one."]
    _1 = 1,
}
impl From<TDRESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TDRESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRESELECT_A {
        match self.bits {
            false => TDRESELECT_A::_0,
            true => TDRESELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDRESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDRESELECT_A::_1
    }
}
#[doc = "Field `MODF` reader - Mode Fault Error"]
pub type MODF_R = crate::BitReader<MODFSELECT_A>;
#[doc = "Mode Fault Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFSELECT_A {
    #[doc = "0: No Mode Fault has been detected since the last read of SR."]
    _0 = 0,
    #[doc = "1: A Mode Fault occurred since the last read of the SR."]
    _1 = 1,
}
impl From<MODFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MODFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODFSELECT_A {
        match self.bits {
            false => MODFSELECT_A::_0,
            true => MODFSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODFSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODFSELECT_A::_1
    }
}
#[doc = "Field `OVRES` reader - Overrun Error Status"]
pub type OVRES_R = crate::BitReader<OVRESSELECT_A>;
#[doc = "Overrun Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRESSELECT_A {
    #[doc = "0: No overrun has been detected since the last read of SR."]
    _0 = 0,
    #[doc = "1: An overrun has occurred since the last read of SR."]
    _1 = 1,
}
impl From<OVRESSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: OVRESSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRESSELECT_A {
        match self.bits {
            false => OVRESSELECT_A::_0,
            true => OVRESSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRESSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRESSELECT_A::_1
    }
}
#[doc = "Field `ENDRX` reader - End of RX buffer"]
pub type ENDRX_R = crate::BitReader<ENDRXSELECT_A>;
#[doc = "End of RX buffer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDRXSELECT_A {
    #[doc = "0: The Receive Counter Register has not reached 0 since the last write in RCR or RNCR."]
    _0 = 0,
    #[doc = "1: The Receive Counter Register has reached 0 since the last write in RCR or RNCR."]
    _1 = 1,
}
impl From<ENDRXSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRXSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDRX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRXSELECT_A {
        match self.bits {
            false => ENDRXSELECT_A::_0,
            true => ENDRXSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDRXSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDRXSELECT_A::_1
    }
}
#[doc = "Field `ENDTX` reader - End of TX buffer"]
pub type ENDTX_R = crate::BitReader<ENDTXSELECT_A>;
#[doc = "End of TX buffer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDTXSELECT_A {
    #[doc = "0: The Transmit Counter Register has not reached 0 since the last write in TCR or TNCR."]
    _0 = 0,
    #[doc = "1: The Transmit Counter Register has reached 0 since the last write in TCR or TNCR."]
    _1 = 1,
}
impl From<ENDTXSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDTXSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDTX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDTXSELECT_A {
        match self.bits {
            false => ENDTXSELECT_A::_0,
            true => ENDTXSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDTXSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDTXSELECT_A::_1
    }
}
#[doc = "Field `RXBUFF` reader - RX Buffer Full"]
pub type RXBUFF_R = crate::BitReader<RXBUFFSELECT_A>;
#[doc = "RX Buffer Full\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXBUFFSELECT_A {
    #[doc = "0: RCR or RNCR has a value other than 0."]
    _0 = 0,
    #[doc = "1: Both RCR and RNCR has a value of 0."]
    _1 = 1,
}
impl From<RXBUFFSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RXBUFFSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXBUFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBUFFSELECT_A {
        match self.bits {
            false => RXBUFFSELECT_A::_0,
            true => RXBUFFSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXBUFFSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXBUFFSELECT_A::_1
    }
}
#[doc = "Field `TXBUFE` reader - TX Buffer Empty"]
pub type TXBUFE_R = crate::BitReader<TXBUFESELECT_A>;
#[doc = "TX Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXBUFESELECT_A {
    #[doc = "0: TCR or TNCR has a value other than 0."]
    _0 = 0,
    #[doc = "1: Both TCR and TNCR has a value of 0."]
    _1 = 1,
}
impl From<TXBUFESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXBUFESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXBUFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXBUFESELECT_A {
        match self.bits {
            false => TXBUFESELECT_A::_0,
            true => TXBUFESELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXBUFESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXBUFESELECT_A::_1
    }
}
#[doc = "Field `NSSR` reader - NSS Rising"]
pub type NSSR_R = crate::BitReader<NSSRSELECT_A>;
#[doc = "NSS Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSSRSELECT_A {
    #[doc = "0: No rising edge detected on NSS pin since last read."]
    _0 = 0,
    #[doc = "1: A rising edge occurred on NSS pin since last read."]
    _1 = 1,
}
impl From<NSSRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: NSSRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl NSSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSSRSELECT_A {
        match self.bits {
            false => NSSRSELECT_A::_0,
            true => NSSRSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NSSRSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NSSRSELECT_A::_1
    }
}
#[doc = "Field `TXEMPTY` reader - Transmission Registers Empty"]
pub type TXEMPTY_R = crate::BitReader<TXEMPTYSELECT_A>;
#[doc = "Transmission Registers Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEMPTYSELECT_A {
    #[doc = "0: As soon as data is written in TDR."]
    _0 = 0,
    #[doc = "1: TDR and internal shifter are empty. If a transfer delay has been defined, TXEMPTY is set after the completion ofsuch delay."]
    _1 = 1,
}
impl From<TXEMPTYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPTYSELECT_A {
        match self.bits {
            false => TXEMPTYSELECT_A::_0,
            true => TXEMPTYSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXEMPTYSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXEMPTYSELECT_A::_1
    }
}
#[doc = "Field `UNDES` reader - Underrun Error Status (Slave Mode Only)"]
pub type UNDES_R = crate::BitReader<bool>;
#[doc = "Field `SPIENS` reader - SPI Enable Status"]
pub type SPIENS_R = crate::BitReader<SPIENSSELECT_A>;
#[doc = "SPI Enable Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIENSSELECT_A {
    #[doc = "0: SPI is disabled."]
    _0 = 0,
    #[doc = "1: SPI is enabled."]
    _1 = 1,
}
impl From<SPIENSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SPIENSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SPIENS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIENSSELECT_A {
        match self.bits {
            false => SPIENSSELECT_A::_0,
            true => SPIENSSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPIENSSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPIENSSELECT_A::_1
    }
}
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
