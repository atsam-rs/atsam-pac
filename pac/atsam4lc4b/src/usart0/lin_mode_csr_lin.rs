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
#[doc = "Field `RXRDY` reader - Receiver Ready"]
pub struct RXRDY_R(crate::FieldReader<bool, RXRDY_A>);
impl RXRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRDY_R(crate::FieldReader::new(bits))
    }
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
        **self == RXRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXRDY_A::_1
    }
}
impl core::ops::Deref for RXRDY_R {
    type Target = crate::FieldReader<bool, RXRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `TXRDY` reader - Transmitter Ready"]
pub struct TXRDY_R(crate::FieldReader<bool, TXRDY_A>);
impl TXRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRDY_R(crate::FieldReader::new(bits))
    }
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
        **self == TXRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXRDY_A::_1
    }
}
impl core::ops::Deref for TXRDY_R {
    type Target = crate::FieldReader<bool, TXRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `RXBRK` reader - Break Received/End of Break"]
pub struct RXBRK_R(crate::FieldReader<bool, RXBRK_A>);
impl RXBRK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXBRK_R(crate::FieldReader::new(bits))
    }
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
        **self == RXBRK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXBRK_A::_1
    }
}
impl core::ops::Deref for RXBRK_R {
    type Target = crate::FieldReader<bool, RXBRK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `OVRE` reader - Overrun Error"]
pub struct OVRE_R(crate::FieldReader<bool, OVRE_A>);
impl OVRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRE_R(crate::FieldReader::new(bits))
    }
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
        **self == OVRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == OVRE_A::_1
    }
}
impl core::ops::Deref for OVRE_R {
    type Target = crate::FieldReader<bool, OVRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `FRAME` reader - Framing Error"]
pub struct FRAME_R(crate::FieldReader<bool, FRAME_A>);
impl FRAME_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRAME_R(crate::FieldReader::new(bits))
    }
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
        **self == FRAME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == FRAME_A::_1
    }
}
impl core::ops::Deref for FRAME_R {
    type Target = crate::FieldReader<bool, FRAME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `PARE` reader - Parity Error"]
pub struct PARE_R(crate::FieldReader<bool, PARE_A>);
impl PARE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARE_R(crate::FieldReader::new(bits))
    }
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
        **self == PARE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PARE_A::_1
    }
}
impl core::ops::Deref for PARE_R {
    type Target = crate::FieldReader<bool, PARE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `TIMEOUT` reader - Receiver Time-out"]
pub struct TIMEOUT_R(crate::FieldReader<bool, TIMEOUT_A>);
impl TIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
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
        **self == TIMEOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TIMEOUT_A::_1
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<bool, TIMEOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `TXEMPTY` reader - Transmitter Empty"]
pub struct TXEMPTY_R(crate::FieldReader<bool, TXEMPTY_A>);
impl TXEMPTY_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXEMPTY_R(crate::FieldReader::new(bits))
    }
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
        **self == TXEMPTY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXEMPTY_A::_1
    }
}
impl core::ops::Deref for TXEMPTY_R {
    type Target = crate::FieldReader<bool, TXEMPTY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `ITER` reader - Max number of Repetitions Reached"]
pub struct ITER_R(crate::FieldReader<bool, ITER_A>);
impl ITER_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITER_R(crate::FieldReader::new(bits))
    }
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
        **self == ITER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ITER_A::_1
    }
}
impl core::ops::Deref for ITER_R {
    type Target = crate::FieldReader<bool, ITER_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `TXBUFE` reader - Transmission Buffer Empty"]
pub struct TXBUFE_R(crate::FieldReader<bool, TXBUFE_A>);
impl TXBUFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBUFE_R(crate::FieldReader::new(bits))
    }
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
        **self == TXBUFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == TXBUFE_A::_1
    }
}
impl core::ops::Deref for TXBUFE_R {
    type Target = crate::FieldReader<bool, TXBUFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `RXBUFF` reader - Reception Buffer Full"]
pub struct RXBUFF_R(crate::FieldReader<bool, RXBUFF_A>);
impl RXBUFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXBUFF_R(crate::FieldReader::new(bits))
    }
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
        **self == RXBUFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RXBUFF_A::_1
    }
}
impl core::ops::Deref for RXBUFF_R {
    type Target = crate::FieldReader<bool, RXBUFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Non Acknowledge or LIN Break Sent or LIN Break Received\n\nValue on reset: 0"]
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
#[doc = "Field `NACK` reader - Non Acknowledge or LIN Break Sent or LIN Break Received"]
pub struct NACK_R(crate::FieldReader<bool, NACK_A>);
impl NACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NACK_R(crate::FieldReader::new(bits))
    }
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
        **self == NACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == NACK_A::_1
    }
}
impl core::ops::Deref for NACK_R {
    type Target = crate::FieldReader<bool, NACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINID` reader - LIN Identifier Sent or LIN Identifier Received"]
pub struct LINID_R(crate::FieldReader<bool, bool>);
impl LINID_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINTC` reader - LIN Transfer Conpleted"]
pub struct LINTC_R(crate::FieldReader<bool, bool>);
impl LINTC_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINTC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINTC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `RIIC` reader - Ring Indicator Input Change Flag"]
pub struct RIIC_R(crate::FieldReader<bool, RIIC_A>);
impl RIIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIIC_R(crate::FieldReader::new(bits))
    }
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
        **self == RIIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RIIC_A::_1
    }
}
impl core::ops::Deref for RIIC_R {
    type Target = crate::FieldReader<bool, RIIC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `DSRIC` reader - Data Set Ready Input Change Flag"]
pub struct DSRIC_R(crate::FieldReader<bool, DSRIC_A>);
impl DSRIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSRIC_R(crate::FieldReader::new(bits))
    }
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
        **self == DSRIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DSRIC_A::_1
    }
}
impl core::ops::Deref for DSRIC_R {
    type Target = crate::FieldReader<bool, DSRIC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `DCDIC` reader - Data Carrier Detect Input Change Flag"]
pub struct DCDIC_R(crate::FieldReader<bool, DCDIC_A>);
impl DCDIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDIC_R(crate::FieldReader::new(bits))
    }
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
        **self == DCDIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DCDIC_A::_1
    }
}
impl core::ops::Deref for DCDIC_R {
    type Target = crate::FieldReader<bool, DCDIC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CTSIC` reader - Clear to Send Input Change Flag"]
pub struct CTSIC_R(crate::FieldReader<bool, CTSIC_A>);
impl CTSIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTSIC_R(crate::FieldReader::new(bits))
    }
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
        **self == CTSIC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CTSIC_A::_1
    }
}
impl core::ops::Deref for CTSIC_R {
    type Target = crate::FieldReader<bool, CTSIC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `RI` reader - Image of RI Input"]
pub struct RI_R(crate::FieldReader<bool, RI_A>);
impl RI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RI_R(crate::FieldReader::new(bits))
    }
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
        **self == RI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == RI_A::_1
    }
}
impl core::ops::Deref for RI_R {
    type Target = crate::FieldReader<bool, RI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `DSR` reader - Image of DSR Input"]
pub struct DSR_R(crate::FieldReader<bool, DSR_A>);
impl DSR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSR_R(crate::FieldReader::new(bits))
    }
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
        **self == DSR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DSR_A::_1
    }
}
impl core::ops::Deref for DSR_R {
    type Target = crate::FieldReader<bool, DSR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `DCD` reader - Image of DCD Input"]
pub struct DCD_R(crate::FieldReader<bool, DCD_A>);
impl DCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCD_R(crate::FieldReader::new(bits))
    }
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
        **self == DCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DCD_A::_1
    }
}
impl core::ops::Deref for DCD_R {
    type Target = crate::FieldReader<bool, DCD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LIN Bus Line Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINBLS_A {
    #[doc = "0: CTS is at 0"]
    _0 = 0,
    #[doc = "1: CTS is at 1"]
    _1 = 1,
}
impl From<LINBLS_A> for bool {
    #[inline(always)]
    fn from(variant: LINBLS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINBLS` reader - LIN Bus Line Status"]
pub struct LINBLS_R(crate::FieldReader<bool, LINBLS_A>);
impl LINBLS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINBLS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINBLS_A {
        match self.bits {
            false => LINBLS_A::_0,
            true => LINBLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LINBLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LINBLS_A::_1
    }
}
impl core::ops::Deref for LINBLS_R {
    type Target = crate::FieldReader<bool, LINBLS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINBE` reader - LIN Bit Error"]
pub struct LINBE_R(crate::FieldReader<bool, bool>);
impl LINBE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINBE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINBE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINISFE` reader - LIN Inconsistent Synch Field Error"]
pub struct LINISFE_R(crate::FieldReader<bool, bool>);
impl LINISFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINISFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINISFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINIPE` reader - LIN Identifier Parity Error"]
pub struct LINIPE_R(crate::FieldReader<bool, bool>);
impl LINIPE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINIPE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINIPE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINCE` reader - LIN Checksum Error"]
pub struct LINCE_R(crate::FieldReader<bool, bool>);
impl LINCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINSNRE` reader - LIN Slave Not Responding Error"]
pub struct LINSNRE_R(crate::FieldReader<bool, bool>);
impl LINSNRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINSNRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINSNRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LIN Synch Tolerance Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINSTE_A {
    #[doc = "0: COMM_TX is at 0"]
    _0 = 0,
    #[doc = "1: COMM_TX is at 1"]
    _1 = 1,
}
impl From<LINSTE_A> for bool {
    #[inline(always)]
    fn from(variant: LINSTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINSTE` reader - LIN Synch Tolerance Error"]
pub struct LINSTE_R(crate::FieldReader<bool, LINSTE_A>);
impl LINSTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINSTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINSTE_A {
        match self.bits {
            false => LINSTE_A::_0,
            true => LINSTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LINSTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LINSTE_A::_1
    }
}
impl core::ops::Deref for LINSTE_R {
    type Target = crate::FieldReader<bool, LINSTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "LIN Header Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINHTE_A {
    #[doc = "0: COMM_RX is at 0"]
    _0 = 0,
    #[doc = "1: COMM_RX is at 1"]
    _1 = 1,
}
impl From<LINHTE_A> for bool {
    #[inline(always)]
    fn from(variant: LINHTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINHTE` reader - LIN Header Timeout Error"]
pub struct LINHTE_R(crate::FieldReader<bool, LINHTE_A>);
impl LINHTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LINHTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINHTE_A {
        match self.bits {
            false => LINHTE_A::_0,
            true => LINHTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LINHTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LINHTE_A::_1
    }
}
impl core::ops::Deref for LINHTE_R {
    type Target = crate::FieldReader<bool, LINHTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[doc = "Bit 13 - Non Acknowledge or LIN Break Sent or LIN Break Received"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Identifier Received"]
    #[inline(always)]
    pub fn linid(&self) -> LINID_R {
        LINID_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LIN Transfer Conpleted"]
    #[inline(always)]
    pub fn lintc(&self) -> LINTC_R {
        LINTC_R::new(((self.bits >> 15) & 0x01) != 0)
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
    #[doc = "Bit 23 - LIN Bus Line Status"]
    #[inline(always)]
    pub fn linbls(&self) -> LINBLS_R {
        LINBLS_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LIN Bit Error"]
    #[inline(always)]
    pub fn linbe(&self) -> LINBE_R {
        LINBE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error"]
    #[inline(always)]
    pub fn linisfe(&self) -> LINISFE_R {
        LINISFE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Error"]
    #[inline(always)]
    pub fn linipe(&self) -> LINIPE_R {
        LINIPE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LIN Checksum Error"]
    #[inline(always)]
    pub fn lince(&self) -> LINCE_R {
        LINCE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error"]
    #[inline(always)]
    pub fn linsnre(&self) -> LINSNRE_R {
        LINSNRE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LIN Synch Tolerance Error"]
    #[inline(always)]
    pub fn linste(&self) -> LINSTE_R {
        LINSTE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LIN Header Timeout Error"]
    #[inline(always)]
    pub fn linhte(&self) -> LINHTE_R {
        LINHTE_R::new(((self.bits >> 31) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
