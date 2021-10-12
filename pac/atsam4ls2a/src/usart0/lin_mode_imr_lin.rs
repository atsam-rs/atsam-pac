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
#[doc = "RXRDY Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXRDY_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<RXRDY_A> for bool {
    #[inline(always)]
    fn from(variant: RXRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXRDY` reader - RXRDY Interrupt Mask"]
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
#[doc = "TXRDY Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXRDY_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<TXRDY_A> for bool {
    #[inline(always)]
    fn from(variant: TXRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXRDY` reader - TXRDY Interrupt Mask"]
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
#[doc = "Receiver Break Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBRK_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<RXBRK_A> for bool {
    #[inline(always)]
    fn from(variant: RXBRK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXBRK` reader - Receiver Break Interrupt Mask"]
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
#[doc = "Overrun Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRE_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<OVRE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRE` reader - Overrun Error Interrupt Mask"]
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
#[doc = "Framing Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FRAME_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<FRAME_A> for bool {
    #[inline(always)]
    fn from(variant: FRAME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRAME` reader - Framing Error Interrupt Mask"]
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
#[doc = "Parity Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PARE_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<PARE_A> for bool {
    #[inline(always)]
    fn from(variant: PARE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARE` reader - Parity Error Interrupt Mask"]
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
#[doc = "Time-out Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` reader - Time-out Interrupt Mask"]
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
#[doc = "TXEMPTY Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXEMPTY_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<TXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEMPTY` reader - TXEMPTY Interrupt Mask"]
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
#[doc = "Iteration Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ITER_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<ITER_A> for bool {
    #[inline(always)]
    fn from(variant: ITER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITER` reader - Iteration Interrupt Mask"]
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
#[doc = "Buffer Empty Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXBUFE_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<TXBUFE_A> for bool {
    #[inline(always)]
    fn from(variant: TXBUFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXBUFE` reader - Buffer Empty Interrupt Mask"]
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
#[doc = "Buffer Full Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXBUFF_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<RXBUFF_A> for bool {
    #[inline(always)]
    fn from(variant: RXBUFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXBUFF` reader - Buffer Full Interrupt Mask"]
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
#[doc = "Non Acknowledge or LIN Break Sent or LIN Break Received Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACK_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACK` reader - Non Acknowledge or LIN Break Sent or LIN Break Received Interrupt Mask"]
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
#[doc = "Field `LINID` reader - LIN Identifier Sent or LIN Received Interrupt Mask"]
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
#[doc = "Field `LINTC` reader - LIN Transfer Conpleted Interrupt Mask"]
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
#[doc = "Ring Indicator Input Change Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RIIC_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<RIIC_A> for bool {
    #[inline(always)]
    fn from(variant: RIIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIIC` reader - Ring Indicator Input Change Mask"]
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
#[doc = "Data Set Ready Input Change Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSRIC_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<DSRIC_A> for bool {
    #[inline(always)]
    fn from(variant: DSRIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSRIC` reader - Data Set Ready Input Change Mask"]
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
#[doc = "Data Carrier Detect Input Change Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDIC_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<DCDIC_A> for bool {
    #[inline(always)]
    fn from(variant: DCDIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDIC` reader - Data Carrier Detect Input Change Interrupt Mask"]
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
#[doc = "Clear to Send Input Change Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIC_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<CTSIC_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSIC` reader - Clear to Send Input Change Interrupt Mask"]
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
#[doc = "Field `LINBE` reader - LIN Bus Error Interrupt Mask"]
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
#[doc = "Field `LINISFE` reader - LIN Inconsistent Synch Field Error Interrupt Mask"]
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
#[doc = "Field `LINIPE` reader - LIN Identifier Parity Interrupt Mask"]
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
#[doc = "Field `LINCE` reader - LIN Checksum Error Interrupt Mask"]
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
#[doc = "Field `LINSNRE` reader - LIN Slave Not Responding Error Interrupt Mask"]
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
#[doc = "LIN Synch Tolerance Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINSTE_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<LINSTE_A> for bool {
    #[inline(always)]
    fn from(variant: LINSTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINSTE` reader - LIN Synch Tolerance Error Interrupt Mask"]
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
#[doc = "LIN Header Timeout Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LINHTE_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<LINHTE_A> for bool {
    #[inline(always)]
    fn from(variant: LINHTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LINHTE` reader - LIN Header Timeout Error Interrupt Mask"]
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
    #[doc = "Bit 0 - RXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXRDY Interrupt Mask"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Receiver Break Interrupt Mask"]
    #[inline(always)]
    pub fn rxbrk(&self) -> RXBRK_R {
        RXBRK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Framing Error Interrupt Mask"]
    #[inline(always)]
    pub fn frame(&self) -> FRAME_R {
        FRAME_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Parity Error Interrupt Mask"]
    #[inline(always)]
    pub fn pare(&self) -> PARE_R {
        PARE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Time-out Interrupt Mask"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TXEMPTY Interrupt Mask"]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Iteration Interrupt Mask"]
    #[inline(always)]
    pub fn iter(&self) -> ITER_R {
        ITER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Buffer Empty Interrupt Mask"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Buffer Full Interrupt Mask"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Non Acknowledge or LIN Break Sent or LIN Break Received Interrupt Mask"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - LIN Identifier Sent or LIN Received Interrupt Mask"]
    #[inline(always)]
    pub fn linid(&self) -> LINID_R {
        LINID_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - LIN Transfer Conpleted Interrupt Mask"]
    #[inline(always)]
    pub fn lintc(&self) -> LINTC_R {
        LINTC_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Ring Indicator Input Change Mask"]
    #[inline(always)]
    pub fn riic(&self) -> RIIC_R {
        RIIC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Data Set Ready Input Change Mask"]
    #[inline(always)]
    pub fn dsric(&self) -> DSRIC_R {
        DSRIC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Data Carrier Detect Input Change Interrupt Mask"]
    #[inline(always)]
    pub fn dcdic(&self) -> DCDIC_R {
        DCDIC_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Clear to Send Input Change Interrupt Mask"]
    #[inline(always)]
    pub fn ctsic(&self) -> CTSIC_R {
        CTSIC_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 25 - LIN Bus Error Interrupt Mask"]
    #[inline(always)]
    pub fn linbe(&self) -> LINBE_R {
        LINBE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - LIN Inconsistent Synch Field Error Interrupt Mask"]
    #[inline(always)]
    pub fn linisfe(&self) -> LINISFE_R {
        LINISFE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - LIN Identifier Parity Interrupt Mask"]
    #[inline(always)]
    pub fn linipe(&self) -> LINIPE_R {
        LINIPE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - LIN Checksum Error Interrupt Mask"]
    #[inline(always)]
    pub fn lince(&self) -> LINCE_R {
        LINCE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - LIN Slave Not Responding Error Interrupt Mask"]
    #[inline(always)]
    pub fn linsnre(&self) -> LINSNRE_R {
        LINSNRE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - LIN Synch Tolerance Error Interrupt Mask"]
    #[inline(always)]
    pub fn linste(&self) -> LINSTE_R {
        LINSTE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - LIN Header Timeout Error Interrupt Mask"]
    #[inline(always)]
    pub fn linhte(&self) -> LINHTE_R {
        LINHTE_R::new(((self.bits >> 31) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
