#[doc = "Register `CSR_LIN` reader"]
pub struct R(crate::R<LIN_MODE_CSR_LIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIN_MODE_CSR_LIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIN_MODE_CSR_LIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIN_MODE_CSR_LIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - Receiver Ready"]
pub type RXRDY_R = crate::BitReader<RXRDYSELECT_A>;
#[doc = "Receiver Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXRDYSELECT_A {
    #[doc = "0: No complete character has been received since the last read of RHR or the receiver is disabled. If characters werebeing received when the receiver was disabled, RXRDY changes to 1 when the receiver is enabled"]
    _0 = 0,
    #[doc = "1: At least one complete character has been received and RHR has not yet been read"]
    _1 = 1,
}
impl From<RXRDYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RXRDYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRDYSELECT_A {
        match self.bits {
            false => RXRDYSELECT_A::_0,
            true => RXRDYSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXRDYSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXRDYSELECT_A::_1
    }
}
#[doc = "Field `TXRDY` reader - Transmitter Ready"]
pub type TXRDY_R = crate::BitReader<TXRDYSELECT_A>;
#[doc = "Transmitter Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRDYSELECT_A {
    #[doc = "0: A character is in the THR waiting to be transferred to the Transmit Shift Register, or an STTBRK command has been requested, or the transmitter is disabled. As soon as the transmitter is enabled, TXRDY becomes 1"]
    _0 = 0,
    #[doc = "1: There is no character in the THR"]
    _1 = 1,
}
impl From<TXRDYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXRDYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRDYSELECT_A {
        match self.bits {
            false => TXRDYSELECT_A::_0,
            true => TXRDYSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXRDYSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXRDYSELECT_A::_1
    }
}
#[doc = "Field `RXBRK` reader - Break Received/End of Break"]
pub type RXBRK_R = crate::BitReader<RXBRKSELECT_A>;
#[doc = "Break Received/End of Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXBRKSELECT_A {
    #[doc = "0: No Break received or End of Break detected since the last RSTSTA"]
    _0 = 0,
    #[doc = "1: Break Received or End of Break detected since the last RSTSTA"]
    _1 = 1,
}
impl From<RXBRKSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RXBRKSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXBRK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBRKSELECT_A {
        match self.bits {
            false => RXBRKSELECT_A::_0,
            true => RXBRKSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXBRKSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXBRKSELECT_A::_1
    }
}
#[doc = "Field `OVRE` reader - Overrun Error"]
pub type OVRE_R = crate::BitReader<OVRESELECT_A>;
#[doc = "Overrun Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRESELECT_A {
    #[doc = "0: No overrun error has occurred since since the last RSTSTA"]
    _0 = 0,
    #[doc = "1: At least one overrun error has occurred since the last RSTSTA"]
    _1 = 1,
}
impl From<OVRESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: OVRESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRESELECT_A {
        match self.bits {
            false => OVRESELECT_A::_0,
            true => OVRESELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRESELECT_A::_1
    }
}
#[doc = "Field `FRAME` reader - Framing Error"]
pub type FRAME_R = crate::BitReader<FRAMESELECT_A>;
#[doc = "Framing Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAMESELECT_A {
    #[doc = "0: No stop bit has been detected low since the last RSTSTA"]
    _0 = 0,
    #[doc = "1: At least one stop bit has been detected low since the last RSTSTA"]
    _1 = 1,
}
impl From<FRAMESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: FRAMESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FRAME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAMESELECT_A {
        match self.bits {
            false => FRAMESELECT_A::_0,
            true => FRAMESELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRAMESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRAMESELECT_A::_1
    }
}
#[doc = "Field `PARE` reader - Parity Error"]
pub type PARE_R = crate::BitReader<PARESELECT_A>;
#[doc = "Parity Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARESELECT_A {
    #[doc = "0: No parity error has been detected since the last RSTSTA"]
    _0 = 0,
    #[doc = "1: At least one parity error has been detected since the last RSTSTA"]
    _1 = 1,
}
impl From<PARESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PARESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PARE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARESELECT_A {
        match self.bits {
            false => PARESELECT_A::_0,
            true => PARESELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PARESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PARESELECT_A::_1
    }
}
#[doc = "Field `TIMEOUT` reader - Receiver Time-out"]
pub type TIMEOUT_R = crate::BitReader<TIMEOUTSELECT_A>;
#[doc = "Receiver Time-out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUTSELECT_A {
    #[doc = "0: There has not been a time-out since the last Start Time-out command or the Time-out Register is 0"]
    _0 = 0,
    #[doc = "1: There has been a time-out since the last Start Time-out command"]
    _1 = 1,
}
impl From<TIMEOUTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUTSELECT_A {
        match self.bits {
            false => TIMEOUTSELECT_A::_0,
            true => TIMEOUTSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIMEOUTSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIMEOUTSELECT_A::_1
    }
}
#[doc = "Field `TXEMPTY` reader - Transmitter Empty"]
pub type TXEMPTY_R = crate::BitReader<TXEMPTYSELECT_A>;
#[doc = "Transmitter Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEMPTYSELECT_A {
    #[doc = "0: There are characters in either THR or the Transmit Shift Register, or the transmitter is disabled"]
    _0 = 0,
    #[doc = "1: There is at least one character in either THR or the Transmit Shift Register"]
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
#[doc = "Field `ITER` reader - Max number of Repetitions Reached"]
pub type ITER_R = crate::BitReader<ITERSELECT_A>;
#[doc = "Max number of Repetitions Reached\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITERSELECT_A {
    #[doc = "0: Maximum number of repetitions has not been reached since the last RSIT"]
    _0 = 0,
    #[doc = "1: Maximum number of repetitions has been reached since the last RSIT"]
    _1 = 1,
}
impl From<ITERSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ITERSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ITER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITERSELECT_A {
        match self.bits {
            false => ITERSELECT_A::_0,
            true => ITERSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITERSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITERSELECT_A::_1
    }
}
#[doc = "Field `TXBUFE` reader - Transmission Buffer Empty"]
pub type TXBUFE_R = crate::BitReader<TXBUFESELECT_A>;
#[doc = "Transmission Buffer Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXBUFESELECT_A {
    #[doc = "0: The signal Buffer Empty from the Transmit PDC channel is inactive"]
    _0 = 0,
    #[doc = "1: The signal Buffer Empty from the Transmit PDC channel is active"]
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
#[doc = "Field `RXBUFF` reader - Reception Buffer Full"]
pub type RXBUFF_R = crate::BitReader<RXBUFFSELECT_A>;
#[doc = "Reception Buffer Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXBUFFSELECT_A {
    #[doc = "0: The signal Buffer Full from the Receive PDC channel is inactive"]
    _0 = 0,
    #[doc = "1: The signal Buffer Full from the Receive PDC channel is active"]
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
#[doc = "Field `NACK` reader - Non Acknowledge or LIN Break Sent or LIN Break Received"]
pub type NACK_R = crate::BitReader<NACKSELECT_A>;
#[doc = "Non Acknowledge or LIN Break Sent or LIN Break Received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKSELECT_A {
    #[doc = "0: No Non Acknowledge has not been detected since the last RSTNACK"]
    _0 = 0,
    #[doc = "1: At least one Non Acknowledge has been detected since the last RSTNACK"]
    _1 = 1,
}
impl From<NACKSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: NACKSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl NACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKSELECT_A {
        match self.bits {
            false => NACKSELECT_A::_0,
            true => NACKSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NACKSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NACKSELECT_A::_1
    }
}
#[doc = "Field `LINID` reader - LIN Identifier Sent or LIN Identifier Received"]
pub type LINID_R = crate::BitReader<bool>;
#[doc = "Field `LINTC` reader - LIN Transfer Conpleted"]
pub type LINTC_R = crate::BitReader<bool>;
#[doc = "Field `RIIC` reader - Ring Indicator Input Change Flag"]
pub type RIIC_R = crate::BitReader<RIICSELECT_A>;
#[doc = "Ring Indicator Input Change Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIICSELECT_A {
    #[doc = "0: No input change has been detected on the RI pin since the last read of CSR"]
    _0 = 0,
    #[doc = "1: At least one input change has been detected on the RI pin since the last read of CSR"]
    _1 = 1,
}
impl From<RIICSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RIICSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RIIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIICSELECT_A {
        match self.bits {
            false => RIICSELECT_A::_0,
            true => RIICSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIICSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIICSELECT_A::_1
    }
}
#[doc = "Field `DSRIC` reader - Data Set Ready Input Change Flag"]
pub type DSRIC_R = crate::BitReader<DSRICSELECT_A>;
#[doc = "Data Set Ready Input Change Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSRICSELECT_A {
    #[doc = "0: No input change has been detected on the DSR pin since the last read of CSR"]
    _0 = 0,
    #[doc = "1: At least one input change has been detected on the DSR pin since the last read of CSR"]
    _1 = 1,
}
impl From<DSRICSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DSRICSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DSRIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSRICSELECT_A {
        match self.bits {
            false => DSRICSELECT_A::_0,
            true => DSRICSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSRICSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSRICSELECT_A::_1
    }
}
#[doc = "Field `DCDIC` reader - Data Carrier Detect Input Change Flag"]
pub type DCDIC_R = crate::BitReader<DCDICSELECT_A>;
#[doc = "Data Carrier Detect Input Change Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDICSELECT_A {
    #[doc = "0: No input change has been detected on the DCD pin since the last read of CSR"]
    _0 = 0,
    #[doc = "1: At least one input change has been detected on the DCD pin since the last read of CSR"]
    _1 = 1,
}
impl From<DCDICSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DCDICSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDICSELECT_A {
        match self.bits {
            false => DCDICSELECT_A::_0,
            true => DCDICSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCDICSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCDICSELECT_A::_1
    }
}
#[doc = "Field `CTSIC` reader - Clear to Send Input Change Flag"]
pub type CTSIC_R = crate::BitReader<CTSICSELECT_A>;
#[doc = "Clear to Send Input Change Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSICSELECT_A {
    #[doc = "0: No input change has been detected on the CTS pin since the last read of CSR"]
    _0 = 0,
    #[doc = "1: At least one input change has been detected on the CTS pin since the last read of CSR"]
    _1 = 1,
}
impl From<CTSICSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CTSICSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSIC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSICSELECT_A {
        match self.bits {
            false => CTSICSELECT_A::_0,
            true => CTSICSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSICSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSICSELECT_A::_1
    }
}
#[doc = "Field `RI` reader - Image of RI Input"]
pub type RI_R = crate::BitReader<RISELECT_A>;
#[doc = "Image of RI Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RISELECT_A {
    #[doc = "0: RI is at 0"]
    _0 = 0,
    #[doc = "1: RI is at 1"]
    _1 = 1,
}
impl From<RISELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RISELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RISELECT_A {
        match self.bits {
            false => RISELECT_A::_0,
            true => RISELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RISELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RISELECT_A::_1
    }
}
#[doc = "Field `DSR` reader - Image of DSR Input"]
pub type DSR_R = crate::BitReader<DSRSELECT_A>;
#[doc = "Image of DSR Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSRSELECT_A {
    #[doc = "0: DSR is at 0"]
    _0 = 0,
    #[doc = "1: DSR is at 1"]
    _1 = 1,
}
impl From<DSRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DSRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DSR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSRSELECT_A {
        match self.bits {
            false => DSRSELECT_A::_0,
            true => DSRSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSRSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSRSELECT_A::_1
    }
}
#[doc = "Field `DCD` reader - Image of DCD Input"]
pub type DCD_R = crate::BitReader<DCDSELECT_A>;
#[doc = "Image of DCD Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDSELECT_A {
    #[doc = "0: DCD is at 0"]
    _0 = 0,
    #[doc = "1: DCD is at 1"]
    _1 = 1,
}
impl From<DCDSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DCDSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDSELECT_A {
        match self.bits {
            false => DCDSELECT_A::_0,
            true => DCDSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCDSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCDSELECT_A::_1
    }
}
#[doc = "Field `LINBLS` reader - LIN Bus Line Status"]
pub type LINBLS_R = crate::BitReader<LINBLSSELECT_A>;
#[doc = "LIN Bus Line Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINBLSSELECT_A {
    #[doc = "0: CTS is at 0"]
    _0 = 0,
    #[doc = "1: CTS is at 1"]
    _1 = 1,
}
impl From<LINBLSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LINBLSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LINBLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINBLSSELECT_A {
        match self.bits {
            false => LINBLSSELECT_A::_0,
            true => LINBLSSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LINBLSSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LINBLSSELECT_A::_1
    }
}
#[doc = "Field `LINBE` reader - LIN Bit Error"]
pub type LINBE_R = crate::BitReader<bool>;
#[doc = "Field `LINISFE` reader - LIN Inconsistent Synch Field Error"]
pub type LINISFE_R = crate::BitReader<bool>;
#[doc = "Field `LINIPE` reader - LIN Identifier Parity Error"]
pub type LINIPE_R = crate::BitReader<bool>;
#[doc = "Field `LINCE` reader - LIN Checksum Error"]
pub type LINCE_R = crate::BitReader<bool>;
#[doc = "Field `LINSNRE` reader - LIN Slave Not Responding Error"]
pub type LINSNRE_R = crate::BitReader<bool>;
#[doc = "Field `LINSTE` reader - LIN Synch Tolerance Error"]
pub type LINSTE_R = crate::BitReader<LINSTESELECT_A>;
#[doc = "LIN Synch Tolerance Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINSTESELECT_A {
    #[doc = "0: COMM_TX is at 0"]
    _0 = 0,
    #[doc = "1: COMM_TX is at 1"]
    _1 = 1,
}
impl From<LINSTESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LINSTESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LINSTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINSTESELECT_A {
        match self.bits {
            false => LINSTESELECT_A::_0,
            true => LINSTESELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LINSTESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LINSTESELECT_A::_1
    }
}
#[doc = "Field `LINHTE` reader - LIN Header Timeout Error"]
pub type LINHTE_R = crate::BitReader<LINHTESELECT_A>;
#[doc = "LIN Header Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINHTESELECT_A {
    #[doc = "0: COMM_RX is at 0"]
    _0 = 0,
    #[doc = "1: COMM_RX is at 1"]
    _1 = 1,
}
impl From<LINHTESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LINHTESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LINHTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINHTESELECT_A {
        match self.bits {
            false => LINHTESELECT_A::_0,
            true => LINHTESELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LINHTESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LINHTESELECT_A::_1
    }
}
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
    #[doc = "Bit 10 - Max number of Repetitions Reached"]
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
    #[doc = "Bit 13 - Non Acknowledge or LIN Break Sent or LIN Break Received"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received"]
    #[inline(always)]
    pub fn linid(&self) -> LINID_R {
        LINID_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LIN Transfer Conpleted"]
    #[inline(always)]
    pub fn lintc(&self) -> LINTC_R {
        LINTC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Flag"]
    #[inline(always)]
    pub fn riic(&self) -> RIIC_R {
        RIIC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Flag"]
    #[inline(always)]
    pub fn dsric(&self) -> DSRIC_R {
        DSRIC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Flag"]
    #[inline(always)]
    pub fn dcdic(&self) -> DCDIC_R {
        DCDIC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Flag"]
    #[inline(always)]
    pub fn ctsic(&self) -> CTSIC_R {
        CTSIC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Image of RI Input"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Image of DSR Input"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Image of DCD Input"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LIN Bus Line Status"]
    #[inline(always)]
    pub fn linbls(&self) -> LINBLS_R {
        LINBLS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - LIN Bit Error"]
    #[inline(always)]
    pub fn linbe(&self) -> LINBE_R {
        LINBE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error"]
    #[inline(always)]
    pub fn linisfe(&self) -> LINISFE_R {
        LINISFE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Error"]
    #[inline(always)]
    pub fn linipe(&self) -> LINIPE_R {
        LINIPE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LIN Checksum Error"]
    #[inline(always)]
    pub fn lince(&self) -> LINCE_R {
        LINCE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error"]
    #[inline(always)]
    pub fn linsnre(&self) -> LINSNRE_R {
        LINSNRE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LIN Synch Tolerance Error"]
    #[inline(always)]
    pub fn linste(&self) -> LINSTE_R {
        LINSTE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LIN Header Timeout Error"]
    #[inline(always)]
    pub fn linhte(&self) -> LINHTE_R {
        LINHTE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lin_mode_csr_lin](index.html) module"]
pub struct LIN_MODE_CSR_LIN_SPEC;
impl crate::RegisterSpec for LIN_MODE_CSR_LIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lin_mode_csr_lin::R](R) reader structure"]
impl crate::Readable for LIN_MODE_CSR_LIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSR_LIN to value 0"]
impl crate::Resettable for LIN_MODE_CSR_LIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
