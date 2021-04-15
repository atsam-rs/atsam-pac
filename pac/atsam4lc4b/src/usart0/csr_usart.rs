#[doc = "Reader of register CSR_USART"]
pub type R = crate::R<u32, super::CSR_USART>;
#[doc = "Receiver Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRDY_A {
    #[doc = "0: No complete character has been received since the last read of RHR or the receiver is disabled. If characters werebeing received when the receiver was disabled, RXRDY changes to 1 when the receiver is enabled"]
    _0 = 0,
    #[doc = "1: At least one complete character has been received and RHR has not yet been read"]
    _1 = 1,
}
impl From<RXRDY_A> for bool {
    #[inline(always)]
    fn from(variant: RXRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXRDY`"]
pub type RXRDY_R = crate::R<bool, RXRDY_A>;
impl RXRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXRDY_A {
        match self.bits {
            false => RXRDY_A::_0,
            true => RXRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXRDY_A::_1
    }
}
#[doc = "Transmitter Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDY_A {
    #[doc = "0: A character is in the THR waiting to be transferred to the Transmit Shift Register, or an STTBRK command has been requested, or the transmitter is disabled. As soon as the transmitter is enabled, TXRDY becomes 1"]
    _0 = 0,
    #[doc = "1: There is no character in the THR"]
    _1 = 1,
}
impl From<TXRDY_A> for bool {
    #[inline(always)]
    fn from(variant: TXRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TXRDY`"]
pub type TXRDY_R = crate::R<bool, TXRDY_A>;
impl TXRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRDY_A {
        match self.bits {
            false => TXRDY_A::_0,
            true => TXRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXRDY_A::_1
    }
}
#[doc = "Break Received/End of Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBRK_A {
    #[doc = "0: No Break received or End of Break detected since the last RSTSTA"]
    _0 = 0,
    #[doc = "1: Break Received or End of Break detected since the last RSTSTA"]
    _1 = 1,
}
impl From<RXBRK_A> for bool {
    #[inline(always)]
    fn from(variant: RXBRK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RXBRK`"]
pub type RXBRK_R = crate::R<bool, RXBRK_A>;
impl RXBRK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXBRK_A {
        match self.bits {
            false => RXBRK_A::_0,
            true => RXBRK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXBRK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXBRK_A::_1
    }
}
#[doc = "Overrun Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRE_A {
    #[doc = "0: No overrun error has occurred since since the last RSTSTA"]
    _0 = 0,
    #[doc = "1: At least one overrun error has occurred since the last RSTSTA"]
    _1 = 1,
}
impl From<OVRE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVRE`"]
pub type OVRE_R = crate::R<bool, OVRE_A>;
impl OVRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRE_A {
        match self.bits {
            false => OVRE_A::_0,
            true => OVRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRE_A::_1
    }
}
#[doc = "Framing Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAME_A {
    #[doc = "0: No stop bit has been detected low since the last RSTSTA"]
    _0 = 0,
    #[doc = "1: At least one stop bit has been detected low since the last RSTSTA"]
    _1 = 1,
}
impl From<FRAME_A> for bool {
    #[inline(always)]
    fn from(variant: FRAME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FRAME`"]
pub type FRAME_R = crate::R<bool, FRAME_A>;
impl FRAME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRAME_A {
        match self.bits {
            false => FRAME_A::_0,
            true => FRAME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRAME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRAME_A::_1
    }
}
#[doc = "Parity Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARE_A {
    #[doc = "0: No parity error has been detected since the last RSTSTA"]
    _0 = 0,
    #[doc = "1: At least one parity error has been detected since the last RSTSTA"]
    _1 = 1,
}
impl From<PARE_A> for bool {
    #[inline(always)]
    fn from(variant: PARE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PARE`"]
pub type PARE_R = crate::R<bool, PARE_A>;
impl PARE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARE_A {
        match self.bits {
            false => PARE_A::_0,
            true => PARE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PARE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PARE_A::_1
    }
}
#[doc = "Receiver Time-out\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_A {
    #[doc = "0: There has not been a time-out since the last Start Time-out command or the Time-out Register is 0"]
    _0 = 0,
    #[doc = "1: There has been a time-out since the last Start Time-out command"]
    _1 = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMEOUT`"]
pub type TIMEOUT_R = crate::R<bool, TIMEOUT_A>;
impl TIMEOUT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            false => TIMEOUT_A::_0,
            true => TIMEOUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIMEOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIMEOUT_A::_1
    }
}
#[doc = "Transmitter Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTY_A {
    #[doc = "0: There are characters in either THR or the Transmit Shift Register, or the transmitter is disabled"]
    _0 = 0,
    #[doc = "1: There is at least one character in either THR or the Transmit Shift Register"]
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
#[doc = "Max number of Repetitions Reached\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITER_A {
    #[doc = "0: Maximum number of repetitions has not been reached since the last RSIT"]
    _0 = 0,
    #[doc = "1: Maximum number of repetitions has been reached since the last RSIT"]
    _1 = 1,
}
impl From<ITER_A> for bool {
    #[inline(always)]
    fn from(variant: ITER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ITER`"]
pub type ITER_R = crate::R<bool, ITER_A>;
impl ITER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITER_A {
        match self.bits {
            false => ITER_A::_0,
            true => ITER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITER_A::_1
    }
}
#[doc = "Transmission Buffer Empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBUFE_A {
    #[doc = "0: The signal Buffer Empty from the Transmit PDC channel is inactive"]
    _0 = 0,
    #[doc = "1: The signal Buffer Empty from the Transmit PDC channel is active"]
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
#[doc = "Reception Buffer Full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBUFF_A {
    #[doc = "0: The signal Buffer Full from the Receive PDC channel is inactive"]
    _0 = 0,
    #[doc = "1: The signal Buffer Full from the Receive PDC channel is active"]
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
#[doc = "Non Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACK_A {
    #[doc = "0: No Non Acknowledge has not been detected since the last RSTNACK"]
    _0 = 0,
    #[doc = "1: At least one Non Acknowledge has been detected since the last RSTNACK"]
    _1 = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `NACK`"]
pub type NACK_R = crate::R<bool, NACK_A>;
impl NACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACK_A {
        match self.bits {
            false => NACK_A::_0,
            true => NACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NACK_A::_1
    }
}
#[doc = "Ring Indicator Input Change Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIIC_A {
    #[doc = "0: No input change has been detected on the RI pin since the last read of CSR"]
    _0 = 0,
    #[doc = "1: At least one input change has been detected on the RI pin since the last read of CSR"]
    _1 = 1,
}
impl From<RIIC_A> for bool {
    #[inline(always)]
    fn from(variant: RIIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RIIC`"]
pub type RIIC_R = crate::R<bool, RIIC_A>;
impl RIIC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIIC_A {
        match self.bits {
            false => RIIC_A::_0,
            true => RIIC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIIC_A::_1
    }
}
#[doc = "Data Set Ready Input Change Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSRIC_A {
    #[doc = "0: No input change has been detected on the DSR pin since the last read of CSR"]
    _0 = 0,
    #[doc = "1: At least one input change has been detected on the DSR pin since the last read of CSR"]
    _1 = 1,
}
impl From<DSRIC_A> for bool {
    #[inline(always)]
    fn from(variant: DSRIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSRIC`"]
pub type DSRIC_R = crate::R<bool, DSRIC_A>;
impl DSRIC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSRIC_A {
        match self.bits {
            false => DSRIC_A::_0,
            true => DSRIC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSRIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSRIC_A::_1
    }
}
#[doc = "Data Carrier Detect Input Change Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDIC_A {
    #[doc = "0: No input change has been detected on the DCD pin since the last read of CSR"]
    _0 = 0,
    #[doc = "1: At least one input change has been detected on the DCD pin since the last read of CSR"]
    _1 = 1,
}
impl From<DCDIC_A> for bool {
    #[inline(always)]
    fn from(variant: DCDIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCDIC`"]
pub type DCDIC_R = crate::R<bool, DCDIC_A>;
impl DCDIC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDIC_A {
        match self.bits {
            false => DCDIC_A::_0,
            true => DCDIC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCDIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCDIC_A::_1
    }
}
#[doc = "Clear to Send Input Change Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIC_A {
    #[doc = "0: No input change has been detected on the CTS pin since the last read of CSR"]
    _0 = 0,
    #[doc = "1: At least one input change has been detected on the CTS pin since the last read of CSR"]
    _1 = 1,
}
impl From<CTSIC_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTSIC`"]
pub type CTSIC_R = crate::R<bool, CTSIC_A>;
impl CTSIC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSIC_A {
        match self.bits {
            false => CTSIC_A::_0,
            true => CTSIC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSIC_A::_1
    }
}
#[doc = "Image of RI Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RI_A {
    #[doc = "0: RI is at 0"]
    _0 = 0,
    #[doc = "1: RI is at 1"]
    _1 = 1,
}
impl From<RI_A> for bool {
    #[inline(always)]
    fn from(variant: RI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RI`"]
pub type RI_R = crate::R<bool, RI_A>;
impl RI_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RI_A {
        match self.bits {
            false => RI_A::_0,
            true => RI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RI_A::_1
    }
}
#[doc = "Image of DSR Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSR_A {
    #[doc = "0: DSR is at 0"]
    _0 = 0,
    #[doc = "1: DSR is at 1"]
    _1 = 1,
}
impl From<DSR_A> for bool {
    #[inline(always)]
    fn from(variant: DSR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSR`"]
pub type DSR_R = crate::R<bool, DSR_A>;
impl DSR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSR_A {
        match self.bits {
            false => DSR_A::_0,
            true => DSR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSR_A::_1
    }
}
#[doc = "Image of DCD Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCD_A {
    #[doc = "0: DCD is at 0"]
    _0 = 0,
    #[doc = "1: DCD is at 1"]
    _1 = 1,
}
impl From<DCD_A> for bool {
    #[inline(always)]
    fn from(variant: DCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DCD`"]
pub type DCD_R = crate::R<bool, DCD_A>;
impl DCD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCD_A {
        match self.bits {
            false => DCD_A::_0,
            true => DCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCD_A::_1
    }
}
#[doc = "Image of CTS Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTS_A {
    #[doc = "0: CTS is at 0"]
    _0 = 0,
    #[doc = "1: CTS is at 1"]
    _1 = 1,
}
impl From<CTS_A> for bool {
    #[inline(always)]
    fn from(variant: CTS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CTS`"]
pub type CTS_R = crate::R<bool, CTS_A>;
impl CTS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTS_A {
        match self.bits {
            false => CTS_A::_0,
            true => CTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTS_A::_1
    }
}
#[doc = "Manchester Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MANERR_A {
    #[doc = "0: No Manchester error has been detected since the last RSTSTA"]
    _0 = 0,
    #[doc = "1: At least one Manchester error has been detected since the last RSTSTA"]
    _1 = 1,
}
impl From<MANERR_A> for bool {
    #[inline(always)]
    fn from(variant: MANERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MANERR`"]
pub type MANERR_R = crate::R<bool, MANERR_A>;
impl MANERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MANERR_A {
        match self.bits {
            false => MANERR_A::_0,
            true => MANERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MANERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MANERR_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Ready"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter Ready"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Break Received/End of Break"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RXBRK_R {
        RXBRK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun Error"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Framing Error"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Parity Error"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receiver Time-out"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmitter Empty"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Max number of Repetitions Reached"]
    #[inline(always)]
    pub fn iter(&self) -> ITER_R {
        ITER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Transmission Buffer Empty"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Reception Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Non Acknowledge"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Flag"]
    #[inline(always)]
    pub fn riic(&self) -> RIIC_R {
        RIIC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Flag"]
    #[inline(always)]
    pub fn dsric(&self) -> DSRIC_R {
        DSRIC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Flag"]
    #[inline(always)]
    pub fn dcdic(&self) -> DCDIC_R {
        DCDIC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Flag"]
    #[inline(always)]
    pub fn ctsic(&self) -> CTSIC_R {
        CTSIC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Image of RI Input"]
    #[inline(always)]
    pub fn ri(&self) -> RI_R {
        RI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Image of DSR Input"]
    #[inline(always)]
    pub fn dsr(&self) -> DSR_R {
        DSR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Image of DCD Input"]
    #[inline(always)]
    pub fn dcd(&self) -> DCD_R {
        DCD_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Image of CTS Input"]
    #[inline(always)]
    pub fn cts(&self) -> CTS_R {
        CTS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Manchester Error"]
    #[inline(always)]
    pub fn manerr(&self) -> MANERR_R {
        MANERR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
