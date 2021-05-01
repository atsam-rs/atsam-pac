#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Receive Data Register Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDRF_A {
    #[doc = "0: No data has been received since the last read of RDR"]
    _0 = 0,
    #[doc = "1: Data has been received and the received data has been transferred from the serializer to RDR since the last readof RDR."]
    _1 = 1,
}
impl From<RDRF_A> for bool {
    #[inline(always)]
    fn from(variant: RDRF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RDRF`"]
pub type RDRF_R = crate::R<bool, RDRF_A>;
impl RDRF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDRF_A {
        match self.bits {
            false => RDRF_A::_0,
            true => RDRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDRF_A::_1
    }
}
#[doc = "Transmit Data Register Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TDRE_A {
    #[doc = "0: Data has been written to TDR and not yet transferred to the serializer."]
    _0 = 0,
    #[doc = "1: The last data written in the Transmit Data Register has been transferred to the serializer.TDRE equals zero when the SPI is disabled or at reset. The SPI enable command sets this bit to one."]
    _1 = 1,
}
impl From<TDRE_A> for bool {
    #[inline(always)]
    fn from(variant: TDRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TDRE`"]
pub type TDRE_R = crate::R<bool, TDRE_A>;
impl TDRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDRE_A {
        match self.bits {
            false => TDRE_A::_0,
            true => TDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDRE_A::_1
    }
}
#[doc = "Mode Fault Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODF_A {
    #[doc = "0: No Mode Fault has been detected since the last read of SR."]
    _0 = 0,
    #[doc = "1: A Mode Fault occurred since the last read of the SR."]
    _1 = 1,
}
impl From<MODF_A> for bool {
    #[inline(always)]
    fn from(variant: MODF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODF`"]
pub type MODF_R = crate::R<bool, MODF_A>;
impl MODF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODF_A {
        match self.bits {
            false => MODF_A::_0,
            true => MODF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODF_A::_1
    }
}
#[doc = "Overrun Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRES_A {
    #[doc = "0: No overrun has been detected since the last read of SR."]
    _0 = 0,
    #[doc = "1: An overrun has occurred since the last read of SR."]
    _1 = 1,
}
impl From<OVRES_A> for bool {
    #[inline(always)]
    fn from(variant: OVRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVRES`"]
pub type OVRES_R = crate::R<bool, OVRES_A>;
impl OVRES_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRES_A {
        match self.bits {
            false => OVRES_A::_0,
            true => OVRES_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRES_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRES_A::_1
    }
}
#[doc = "End of RX buffer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDRX_A {
    #[doc = "0: The Receive Counter Register has not reached 0 since the last write in RCR or RNCR."]
    _0 = 0,
    #[doc = "1: The Receive Counter Register has reached 0 since the last write in RCR or RNCR."]
    _1 = 1,
}
impl From<ENDRX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDRX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDRX`"]
pub type ENDRX_R = crate::R<bool, ENDRX_A>;
impl ENDRX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDRX_A {
        match self.bits {
            false => ENDRX_A::_0,
            true => ENDRX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDRX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDRX_A::_1
    }
}
#[doc = "End of TX buffer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDTX_A {
    #[doc = "0: The Transmit Counter Register has not reached 0 since the last write in TCR or TNCR."]
    _0 = 0,
    #[doc = "1: The Transmit Counter Register has reached 0 since the last write in TCR or TNCR."]
    _1 = 1,
}
impl From<ENDTX_A> for bool {
    #[inline(always)]
    fn from(variant: ENDTX_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENDTX`"]
pub type ENDTX_R = crate::R<bool, ENDTX_A>;
impl ENDTX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDTX_A {
        match self.bits {
            false => ENDTX_A::_0,
            true => ENDTX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDTX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDTX_A::_1
    }
}
#[doc = "RX Buffer Full\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBUFF_A {
    #[doc = "0: RCR or RNCR has a value other than 0."]
    _0 = 0,
    #[doc = "1: Both RCR and RNCR has a value of 0."]
    _1 = 1,
}
impl From<RXBUFF_A> for bool {
    #[inline(always)]
    fn from(variant: RXBUFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXBUFF`"]
pub type RXBUFF_R = crate::R<bool, RXBUFF_A>;
impl RXBUFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBUFF_A {
        match self.bits {
            false => RXBUFF_A::_0,
            true => RXBUFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXBUFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXBUFF_A::_1
    }
}
#[doc = "TX Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBUFE_A {
    #[doc = "0: TCR or TNCR has a value other than 0."]
    _0 = 0,
    #[doc = "1: Both TCR and TNCR has a value of 0."]
    _1 = 1,
}
impl From<TXBUFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXBUFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXBUFE`"]
pub type TXBUFE_R = crate::R<bool, TXBUFE_A>;
impl TXBUFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXBUFE_A {
        match self.bits {
            false => TXBUFE_A::_0,
            true => TXBUFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXBUFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXBUFE_A::_1
    }
}
#[doc = "NSS Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NSSR_A {
    #[doc = "0: No rising edge detected on NSS pin since last read."]
    _0 = 0,
    #[doc = "1: A rising edge occurred on NSS pin since last read."]
    _1 = 1,
}
impl From<NSSR_A> for bool {
    #[inline(always)]
    fn from(variant: NSSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NSSR`"]
pub type NSSR_R = crate::R<bool, NSSR_A>;
impl NSSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSSR_A {
        match self.bits {
            false => NSSR_A::_0,
            true => NSSR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NSSR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NSSR_A::_1
    }
}
#[doc = "Transmission Registers Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTY_A {
    #[doc = "0: As soon as data is written in TDR."]
    _0 = 0,
    #[doc = "1: TDR and internal shifter are empty. If a transfer delay has been defined, TXEMPTY is set after the completion ofsuch delay."]
    _1 = 1,
}
impl From<TXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXEMPTY`"]
pub type TXEMPTY_R = crate::R<bool, TXEMPTY_A>;
impl TXEMPTY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPTY_A {
        match self.bits {
            false => TXEMPTY_A::_0,
            true => TXEMPTY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXEMPTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXEMPTY_A::_1
    }
}
#[doc = "Reader of field `UNDES`"]
pub type UNDES_R = crate::R<bool, bool>;
#[doc = "SPI Enable Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIENS_A {
    #[doc = "0: SPI is disabled."]
    _0 = 0,
    #[doc = "1: SPI is enabled."]
    _1 = 1,
}
impl From<SPIENS_A> for bool {
    #[inline(always)]
    fn from(variant: SPIENS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SPIENS`"]
pub type SPIENS_R = crate::R<bool, SPIENS_A>;
impl SPIENS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIENS_A {
        match self.bits {
            false => SPIENS_A::_0,
            true => SPIENS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPIENS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPIENS_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Receive Data Register Full"]
    #[inline(always)]
    pub fn rdrf(&self) -> RDRF_R {
        RDRF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit Data Register Empty"]
    #[inline(always)]
    pub fn tdre(&self) -> TDRE_R {
        TDRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mode Fault Error"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Status"]
    #[inline(always)]
    pub fn ovres(&self) -> OVRES_R {
        OVRES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End of RX buffer"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of TX buffer"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RX Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TX Buffer Empty"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - NSS Rising"]
    #[inline(always)]
    pub fn nssr(&self) -> NSSR_R {
        NSSR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Registers Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Underrun Error Status (Slave Mode Only)"]
    #[inline(always)]
    pub fn undes(&self) -> UNDES_R {
        UNDES_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SPI Enable Status"]
    #[inline(always)]
    pub fn spiens(&self) -> SPIENS_R {
        SPIENS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
