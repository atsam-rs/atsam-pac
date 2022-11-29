#[doc = "Register `IMR_LIN` reader"]
pub struct R(crate::R<LIN_MODE_IMR_LIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIN_MODE_IMR_LIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIN_MODE_IMR_LIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIN_MODE_IMR_LIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXRDY` reader - RXRDY Interrupt Mask"]
pub type RXRDY_R = crate::BitReader<RXRDYSELECT_A>;
#[doc = "RXRDY Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXRDYSELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `TXRDY` reader - TXRDY Interrupt Mask"]
pub type TXRDY_R = crate::BitReader<TXRDYSELECT_A>;
#[doc = "TXRDY Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRDYSELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `RXBRK` reader - Receiver Break Interrupt Mask"]
pub type RXBRK_R = crate::BitReader<RXBRKSELECT_A>;
#[doc = "Receiver Break Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXBRKSELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
pub type OVRE_R = crate::BitReader<OVRESELECT_A>;
#[doc = "Overrun Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRESELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `FRAME` reader - Framing Error Interrupt Mask"]
pub type FRAME_R = crate::BitReader<FRAMESELECT_A>;
#[doc = "Framing Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRAMESELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `PARE` reader - Parity Error Interrupt Mask"]
pub type PARE_R = crate::BitReader<PARESELECT_A>;
#[doc = "Parity Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARESELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `TIMEOUT` reader - Time-out Interrupt Mask"]
pub type TIMEOUT_R = crate::BitReader<TIMEOUTSELECT_A>;
#[doc = "Time-out Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUTSELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `TXEMPTY` reader - TXEMPTY Interrupt Mask"]
pub type TXEMPTY_R = crate::BitReader<TXEMPTYSELECT_A>;
#[doc = "TXEMPTY Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEMPTYSELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `ITER` reader - Iteration Interrupt Mask"]
pub type ITER_R = crate::BitReader<ITERSELECT_A>;
#[doc = "Iteration Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITERSELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `TXBUFE` reader - Buffer Empty Interrupt Mask"]
pub type TXBUFE_R = crate::BitReader<TXBUFESELECT_A>;
#[doc = "Buffer Empty Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXBUFESELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `RXBUFF` reader - Buffer Full Interrupt Mask"]
pub type RXBUFF_R = crate::BitReader<RXBUFFSELECT_A>;
#[doc = "Buffer Full Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXBUFFSELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `NACK` reader - Non Acknowledge or LIN Break Sent or LIN Break Received Interrupt Mask"]
pub type NACK_R = crate::BitReader<NACKSELECT_A>;
#[doc = "Non Acknowledge or LIN Break Sent or LIN Break Received Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKSELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `LINID` reader - LIN Identifier Sent or LIN Received Interrupt Mask"]
pub type LINID_R = crate::BitReader<bool>;
#[doc = "Field `LINTC` reader - LIN Transfer Conpleted Interrupt Mask"]
pub type LINTC_R = crate::BitReader<bool>;
#[doc = "Field `RIIC` reader - Ring Indicator Input Change Mask"]
pub type RIIC_R = crate::BitReader<RIICSELECT_A>;
#[doc = "Ring Indicator Input Change Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIICSELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `DSRIC` reader - Data Set Ready Input Change Mask"]
pub type DSRIC_R = crate::BitReader<DSRICSELECT_A>;
#[doc = "Data Set Ready Input Change Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSRICSELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `DCDIC` reader - Data Carrier Detect Input Change Interrupt Mask"]
pub type DCDIC_R = crate::BitReader<DCDICSELECT_A>;
#[doc = "Data Carrier Detect Input Change Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDICSELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `CTSIC` reader - Clear to Send Input Change Interrupt Mask"]
pub type CTSIC_R = crate::BitReader<CTSICSELECT_A>;
#[doc = "Clear to Send Input Change Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSICSELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `LINBE` reader - LIN Bus Error Interrupt Mask"]
pub type LINBE_R = crate::BitReader<bool>;
#[doc = "Field `LINISFE` reader - LIN Inconsistent Synch Field Error Interrupt Mask"]
pub type LINISFE_R = crate::BitReader<bool>;
#[doc = "Field `LINIPE` reader - LIN Identifier Parity Interrupt Mask"]
pub type LINIPE_R = crate::BitReader<bool>;
#[doc = "Field `LINCE` reader - LIN Checksum Error Interrupt Mask"]
pub type LINCE_R = crate::BitReader<bool>;
#[doc = "Field `LINSNRE` reader - LIN Slave Not Responding Error Interrupt Mask"]
pub type LINSNRE_R = crate::BitReader<bool>;
#[doc = "Field `LINSTE` reader - LIN Synch Tolerance Error Interrupt Mask"]
pub type LINSTE_R = crate::BitReader<LINSTESELECT_A>;
#[doc = "LIN Synch Tolerance Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINSTESELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
#[doc = "Field `LINHTE` reader - LIN Header Timeout Error Interrupt Mask"]
pub type LINHTE_R = crate::BitReader<LINHTESELECT_A>;
#[doc = "LIN Header Timeout Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINHTESELECT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
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
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Mask"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RXBRK_R {
        RXBRK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Time-out Interrupt Mask"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Iteration Interrupt Mask"]
    #[inline(always)]
    pub fn iter(&self) -> ITER_R {
        ITER_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Buffer Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Non Acknowledge or LIN Break Sent or LIN Break Received Interrupt Mask"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Received Interrupt Mask"]
    #[inline(always)]
    pub fn linid(&self) -> LINID_R {
        LINID_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LIN Transfer Conpleted Interrupt Mask"]
    #[inline(always)]
    pub fn lintc(&self) -> LINTC_R {
        LINTC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Mask"]
    #[inline(always)]
    pub fn riic(&self) -> RIIC_R {
        RIIC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Mask"]
    #[inline(always)]
    pub fn dsric(&self) -> DSRIC_R {
        DSRIC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Mask"]
    #[inline(always)]
    pub fn dcdic(&self) -> DCDIC_R {
        DCDIC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Mask"]
    #[inline(always)]
    pub fn ctsic(&self) -> CTSIC_R {
        CTSIC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 25 - LIN Bus Error Interrupt Mask"]
    #[inline(always)]
    pub fn linbe(&self) -> LINBE_R {
        LINBE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error Interrupt Mask"]
    #[inline(always)]
    pub fn linisfe(&self) -> LINISFE_R {
        LINISFE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Interrupt Mask"]
    #[inline(always)]
    pub fn linipe(&self) -> LINIPE_R {
        LINIPE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LIN Checksum Error Interrupt Mask"]
    #[inline(always)]
    pub fn lince(&self) -> LINCE_R {
        LINCE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error Interrupt Mask"]
    #[inline(always)]
    pub fn linsnre(&self) -> LINSNRE_R {
        LINSNRE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LIN Synch Tolerance Error Interrupt Mask"]
    #[inline(always)]
    pub fn linste(&self) -> LINSTE_R {
        LINSTE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LIN Header Timeout Error Interrupt Mask"]
    #[inline(always)]
    pub fn linhte(&self) -> LINHTE_R {
        LINHTE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lin_mode_imr_lin](index.html) module"]
pub struct LIN_MODE_IMR_LIN_SPEC;
impl crate::RegisterSpec for LIN_MODE_IMR_LIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lin_mode_imr_lin::R](R) reader structure"]
impl crate::Readable for LIN_MODE_IMR_LIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR_LIN to value 0"]
impl crate::Resettable for LIN_MODE_IMR_LIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
