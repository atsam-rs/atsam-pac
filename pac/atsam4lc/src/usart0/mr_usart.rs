#[doc = "Reader of register MR_USART"]
pub type R = crate::R<u32, super::MR_USART>;
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
#[doc = "SPI Underrun Error Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UNRE_A {
    #[doc = "0: The interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The interrupt is enabled"]
    _1 = 1,
}
impl From<UNRE_A> for bool {
    #[inline(always)]
    fn from(variant: UNRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UNRE`"]
pub type UNRE_R = crate::R<bool, UNRE_A>;
impl UNRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNRE_A {
        match self.bits {
            false => UNRE_A::_0,
            true => UNRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UNRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UNRE_A::_1
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
#[doc = "Non Acknowledge Interrupt Mask\n\nValue on reset: 0"]
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
    #[doc = "Bit 10 - SPI Underrun Error Interrupt Mask"]
    #[inline(always)]
    pub fn unre(&self) -> UNRE_R {
        UNRE_R::new(((self.bits >> 10) & 0x01) != 0)
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
    #[doc = "Bit 13 - Non Acknowledge Interrupt Mask"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 13) & 0x01) != 0)
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
}
