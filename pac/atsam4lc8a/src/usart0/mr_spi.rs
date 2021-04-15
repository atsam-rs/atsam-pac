#[doc = "Reader of register MR_SPI"]
pub type R = crate::R<u32, super::MR_SPI>;
#[doc = "Writer for register MR_SPI"]
pub type W = crate::W<u32, super::MR_SPI>;
#[doc = "Register MR_SPI `reset()`'s with value 0"]
impl crate::ResetValue for super::MR_SPI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Usart Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Normal"]
    NORMAL = 0,
    #[doc = "1: RS485"]
    RS485 = 1,
    #[doc = "2: Hardware Handshaking"]
    HARDWARE = 2,
    #[doc = "3: Modem"]
    MODEM = 3,
    #[doc = "4: IS07816 Protocol: T = 0"]
    ISO7816_T0 = 4,
    #[doc = "6: IS07816 Protocol: T = 1"]
    ISO7816_T1 = 6,
    #[doc = "8: IrDA"]
    IRDA = 8,
    #[doc = "10: LIN Master"]
    LIN_MASTER = 10,
    #[doc = "11: LIN Slave"]
    LIN_SLAVE = 11,
    #[doc = "14: SPI Master"]
    SPI_MASTER = 14,
    #[doc = "15: SPI Slave"]
    SPI_SLAVE = 15,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::NORMAL),
            1 => Val(MODE_A::RS485),
            2 => Val(MODE_A::HARDWARE),
            3 => Val(MODE_A::MODEM),
            4 => Val(MODE_A::ISO7816_T0),
            6 => Val(MODE_A::ISO7816_T1),
            8 => Val(MODE_A::IRDA),
            10 => Val(MODE_A::LIN_MASTER),
            11 => Val(MODE_A::LIN_SLAVE),
            14 => Val(MODE_A::SPI_MASTER),
            15 => Val(MODE_A::SPI_SLAVE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RS485`"]
    #[inline(always)]
    pub fn is_rs485(&self) -> bool {
        *self == MODE_A::RS485
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == MODE_A::HARDWARE
    }
    #[doc = "Checks if the value of the field is `MODEM`"]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        *self == MODE_A::MODEM
    }
    #[doc = "Checks if the value of the field is `ISO7816_T0`"]
    #[inline(always)]
    pub fn is_iso7816_t0(&self) -> bool {
        *self == MODE_A::ISO7816_T0
    }
    #[doc = "Checks if the value of the field is `ISO7816_T1`"]
    #[inline(always)]
    pub fn is_iso7816_t1(&self) -> bool {
        *self == MODE_A::ISO7816_T1
    }
    #[doc = "Checks if the value of the field is `IRDA`"]
    #[inline(always)]
    pub fn is_irda(&self) -> bool {
        *self == MODE_A::IRDA
    }
    #[doc = "Checks if the value of the field is `LIN_MASTER`"]
    #[inline(always)]
    pub fn is_lin_master(&self) -> bool {
        *self == MODE_A::LIN_MASTER
    }
    #[doc = "Checks if the value of the field is `LIN_SLAVE`"]
    #[inline(always)]
    pub fn is_lin_slave(&self) -> bool {
        *self == MODE_A::LIN_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == MODE_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODE_A::SPI_SLAVE
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODE_A::NORMAL)
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn rs485(self) -> &'a mut W {
        self.variant(MODE_A::RS485)
    }
    #[doc = "Hardware Handshaking"]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(MODE_A::HARDWARE)
    }
    #[doc = "Modem"]
    #[inline(always)]
    pub fn modem(self) -> &'a mut W {
        self.variant(MODE_A::MODEM)
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline(always)]
    pub fn iso7816_t0(self) -> &'a mut W {
        self.variant(MODE_A::ISO7816_T0)
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline(always)]
    pub fn iso7816_t1(self) -> &'a mut W {
        self.variant(MODE_A::ISO7816_T1)
    }
    #[doc = "IrDA"]
    #[inline(always)]
    pub fn irda(self) -> &'a mut W {
        self.variant(MODE_A::IRDA)
    }
    #[doc = "LIN Master"]
    #[inline(always)]
    pub fn lin_master(self) -> &'a mut W {
        self.variant(MODE_A::LIN_MASTER)
    }
    #[doc = "LIN Slave"]
    #[inline(always)]
    pub fn lin_slave(self) -> &'a mut W {
        self.variant(MODE_A::LIN_SLAVE)
    }
    #[doc = "SPI Master"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODE_A::SPI_MASTER)
    }
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODE_A::SPI_SLAVE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USCLKS_A {
    #[doc = "0: MCK"]
    MCK = 0,
    #[doc = "1: MCK / DIV"]
    MCK_DIV = 1,
    #[doc = "3: SCK"]
    SCK = 3,
}
impl From<USCLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: USCLKS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `USCLKS`"]
pub type USCLKS_R = crate::R<u8, USCLKS_A>;
impl USCLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, USCLKS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(USCLKS_A::MCK),
            1 => Val(USCLKS_A::MCK_DIV),
            3 => Val(USCLKS_A::SCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == USCLKS_A::MCK
    }
    #[doc = "Checks if the value of the field is `MCK_DIV`"]
    #[inline(always)]
    pub fn is_mck_div(&self) -> bool {
        *self == USCLKS_A::MCK_DIV
    }
    #[doc = "Checks if the value of the field is `SCK`"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == USCLKS_A::SCK
    }
}
#[doc = "Write proxy for field `USCLKS`"]
pub struct USCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> USCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USCLKS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MCK"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(USCLKS_A::MCK)
    }
    #[doc = "MCK / DIV"]
    #[inline(always)]
    pub fn mck_div(self) -> &'a mut W {
        self.variant(USCLKS_A::MCK_DIV)
    }
    #[doc = "SCK"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut W {
        self.variant(USCLKS_A::SCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Character Length.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHRL_A {
    #[doc = "0: 5 bits"]
    _5 = 0,
    #[doc = "1: 6 bits"]
    _6 = 1,
    #[doc = "2: 7 bits"]
    _7 = 2,
    #[doc = "3: 8 bits"]
    _8 = 3,
}
impl From<CHRL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHRL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHRL`"]
pub type CHRL_R = crate::R<u8, CHRL_A>;
impl CHRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHRL_A {
        match self.bits {
            0 => CHRL_A::_5,
            1 => CHRL_A::_6,
            2 => CHRL_A::_7,
            3 => CHRL_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == CHRL_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == CHRL_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == CHRL_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == CHRL_A::_8
    }
}
#[doc = "Write proxy for field `CHRL`"]
pub struct CHRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CHRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHRL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(CHRL_A::_5)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(CHRL_A::_6)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(CHRL_A::_7)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(CHRL_A::_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "SPI CLock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPHA_A {
    #[doc = "0: Data is changed on the leading edge of SPCK and captured on the following edge of SPCK"]
    _0 = 0,
    #[doc = "1: Data is captured on the leading edge of SPCK and changed on the following edge of SPCK"]
    _1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPHA`"]
pub type CPHA_R = crate::R<bool, CPHA_A>;
impl CPHA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::_0,
            true => CPHA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPHA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPHA_A::_1
    }
}
#[doc = "Write proxy for field `CPHA`"]
pub struct CPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> CPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPHA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Data is changed on the leading edge of SPCK and captured on the following edge of SPCK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHA_A::_0)
    }
    #[doc = "Data is captured on the leading edge of SPCK and changed on the following edge of SPCK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHA_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAR_A {
    #[doc = "0: Even parity"]
    EVEN = 0,
    #[doc = "1: Odd parity"]
    ODD = 1,
    #[doc = "2: Parity forced to 0 (Space)"]
    SPACE = 2,
    #[doc = "3: Parity forced to 1 (Mark)"]
    MARK = 3,
    #[doc = "4: No Parity"]
    NONE = 4,
    #[doc = "5: No Parity"]
    _5 = 5,
    #[doc = "6: Multi-drop mode"]
    MULTI = 6,
    #[doc = "7: Multi-drop mode"]
    _7 = 7,
}
impl From<PAR_A> for u8 {
    #[inline(always)]
    fn from(variant: PAR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PAR`"]
pub type PAR_R = crate::R<u8, PAR_A>;
impl PAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAR_A {
        match self.bits {
            0 => PAR_A::EVEN,
            1 => PAR_A::ODD,
            2 => PAR_A::SPACE,
            3 => PAR_A::MARK,
            4 => PAR_A::NONE,
            5 => PAR_A::_5,
            6 => PAR_A::MULTI,
            7 => PAR_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PAR_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PAR_A::ODD
    }
    #[doc = "Checks if the value of the field is `SPACE`"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        *self == PAR_A::SPACE
    }
    #[doc = "Checks if the value of the field is `MARK`"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == PAR_A::MARK
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PAR_A::NONE
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == PAR_A::_5
    }
    #[doc = "Checks if the value of the field is `MULTI`"]
    #[inline(always)]
    pub fn is_multi(&self) -> bool {
        *self == PAR_A::MULTI
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == PAR_A::_7
    }
}
#[doc = "Write proxy for field `PAR`"]
pub struct PAR_W<'a> {
    w: &'a mut W,
}
impl<'a> PAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PAR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PAR_A::EVEN)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PAR_A::ODD)
    }
    #[doc = "Parity forced to 0 (Space)"]
    #[inline(always)]
    pub fn space(self) -> &'a mut W {
        self.variant(PAR_A::SPACE)
    }
    #[doc = "Parity forced to 1 (Mark)"]
    #[inline(always)]
    pub fn mark(self) -> &'a mut W {
        self.variant(PAR_A::MARK)
    }
    #[doc = "No Parity"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PAR_A::NONE)
    }
    #[doc = "No Parity"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(PAR_A::_5)
    }
    #[doc = "Multi-drop mode"]
    #[inline(always)]
    pub fn multi(self) -> &'a mut W {
        self.variant(PAR_A::MULTI)
    }
    #[doc = "Multi-drop mode"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(PAR_A::_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | (((value as u32) & 0x07) << 9);
        self.w
    }
}
#[doc = "Number of Stop Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NBSTOP_A {
    #[doc = "0: 1 stop bit"]
    _1 = 0,
    #[doc = "1: 1.5 stop bits (Only valid if SYNC=0)"]
    _1_5 = 1,
    #[doc = "2: 2 stop bits"]
    _2 = 2,
}
impl From<NBSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: NBSTOP_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `NBSTOP`"]
pub type NBSTOP_R = crate::R<u8, NBSTOP_A>;
impl NBSTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NBSTOP_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NBSTOP_A::_1),
            1 => Val(NBSTOP_A::_1_5),
            2 => Val(NBSTOP_A::_2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NBSTOP_A::_1
    }
    #[doc = "Checks if the value of the field is `_1_5`"]
    #[inline(always)]
    pub fn is_1_5(&self) -> bool {
        *self == NBSTOP_A::_1_5
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == NBSTOP_A::_2
    }
}
#[doc = "Write proxy for field `NBSTOP`"]
pub struct NBSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> NBSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NBSTOP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NBSTOP_A::_1)
    }
    #[doc = "1.5 stop bits (Only valid if SYNC=0)"]
    #[inline(always)]
    pub fn _1_5(self) -> &'a mut W {
        self.variant(NBSTOP_A::_1_5)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(NBSTOP_A::_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHMODE_A {
    #[doc = "0: Normal Mode"]
    NORMAL = 0,
    #[doc = "1: Automatic Echo. Receiver input is connected to the TXD pin"]
    ECHO = 1,
    #[doc = "2: Local Loopback. Transmitter output is connected to the Receiver Input"]
    LOCAL_LOOP = 2,
    #[doc = "3: Remote Loopback. RXD pin is internally connected to the TXD pin"]
    REMOTE_LOOP = 3,
}
impl From<CHMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHMODE`"]
pub type CHMODE_R = crate::R<u8, CHMODE_A>;
impl CHMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHMODE_A {
        match self.bits {
            0 => CHMODE_A::NORMAL,
            1 => CHMODE_A::ECHO,
            2 => CHMODE_A::LOCAL_LOOP,
            3 => CHMODE_A::REMOTE_LOOP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CHMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `ECHO`"]
    #[inline(always)]
    pub fn is_echo(&self) -> bool {
        *self == CHMODE_A::ECHO
    }
    #[doc = "Checks if the value of the field is `LOCAL_LOOP`"]
    #[inline(always)]
    pub fn is_local_loop(&self) -> bool {
        *self == CHMODE_A::LOCAL_LOOP
    }
    #[doc = "Checks if the value of the field is `REMOTE_LOOP`"]
    #[inline(always)]
    pub fn is_remote_loop(&self) -> bool {
        *self == CHMODE_A::REMOTE_LOOP
    }
}
#[doc = "Write proxy for field `CHMODE`"]
pub struct CHMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CHMODE_A::NORMAL)
    }
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin"]
    #[inline(always)]
    pub fn echo(self) -> &'a mut W {
        self.variant(CHMODE_A::ECHO)
    }
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input"]
    #[inline(always)]
    pub fn local_loop(self) -> &'a mut W {
        self.variant(CHMODE_A::LOCAL_LOOP)
    }
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin"]
    #[inline(always)]
    pub fn remote_loop(self) -> &'a mut W {
        self.variant(CHMODE_A::REMOTE_LOOP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "SPI Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPOL_A {
    #[doc = "0: The inactive state value of SPCK is logic level zero"]
    ZERO = 0,
    #[doc = "1: The inactive state value of SPCK is logic level one"]
    ONE = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPOL`"]
pub type CPOL_R = crate::R<bool, CPOL_A>;
impl CPOL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::ZERO,
            true => CPOL_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CPOL_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CPOL_A::ONE
    }
}
#[doc = "Write proxy for field `CPOL`"]
pub struct CPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPOL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The inactive state value of SPCK is logic level zero"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CPOL_A::ZERO)
    }
    #[doc = "The inactive state value of SPCK is logic level one"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CPOL_A::ONE)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "9-bit Character Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE9_A {
    #[doc = "0: CHRL defines character length"]
    _0 = 0,
    #[doc = "1: 9-bit character length"]
    _1 = 1,
}
impl From<MODE9_A> for bool {
    #[inline(always)]
    fn from(variant: MODE9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `MODE9`"]
pub type MODE9_R = crate::R<bool, MODE9_A>;
impl MODE9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE9_A {
        match self.bits {
            false => MODE9_A::_0,
            true => MODE9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODE9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODE9_A::_1
    }
}
#[doc = "Write proxy for field `MODE9`"]
pub struct MODE9_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE9_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "CHRL defines character length"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE9_A::_0)
    }
    #[doc = "9-bit character length"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE9_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Clock Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKO_A {
    #[doc = "0: The USART does not drive the SCK pin"]
    _0 = 0,
    #[doc = "1: The USART drives the SCK pin if USCLKS does not select the external clock SCK"]
    _1 = 1,
}
impl From<CLKO_A> for bool {
    #[inline(always)]
    fn from(variant: CLKO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKO`"]
pub type CLKO_R = crate::R<bool, CLKO_A>;
impl CLKO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKO_A {
        match self.bits {
            false => CLKO_A::_0,
            true => CLKO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKO_A::_1
    }
}
#[doc = "Write proxy for field `CLKO`"]
pub struct CLKO_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKO_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The USART does not drive the SCK pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKO_A::_0)
    }
    #[doc = "The USART drives the SCK pin if USCLKS does not select the external clock SCK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKO_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Oversampling Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVER_A {
    #[doc = "0: 16x Oversampling"]
    X16 = 0,
    #[doc = "1: 8x Oversampling"]
    X8 = 1,
}
impl From<OVER_A> for bool {
    #[inline(always)]
    fn from(variant: OVER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVER`"]
pub type OVER_R = crate::R<bool, OVER_A>;
impl OVER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVER_A {
        match self.bits {
            false => OVER_A::X16,
            true => OVER_A::X8,
        }
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == OVER_A::X16
    }
    #[doc = "Checks if the value of the field is `X8`"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == OVER_A::X8
    }
}
#[doc = "Write proxy for field `OVER`"]
pub struct OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> OVER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "16x Oversampling"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut W {
        self.variant(OVER_A::X16)
    }
    #[doc = "8x Oversampling"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut W {
        self.variant(OVER_A::X8)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Inhibit Non Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INACK_A {
    #[doc = "0: The NACK is generated"]
    _0 = 0,
    #[doc = "1: The NACK is not generated"]
    _1 = 1,
}
impl From<INACK_A> for bool {
    #[inline(always)]
    fn from(variant: INACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `INACK`"]
pub type INACK_R = crate::R<bool, INACK_A>;
impl INACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INACK_A {
        match self.bits {
            false => INACK_A::_0,
            true => INACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INACK_A::_1
    }
}
#[doc = "Write proxy for field `INACK`"]
pub struct INACK_W<'a> {
    w: &'a mut W,
}
impl<'a> INACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The NACK is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INACK_A::_0)
    }
    #[doc = "The NACK is not generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INACK_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Disable Successive NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSNACK_A {
    #[doc = "0: NACK is sent on the ISO line as soon as a parity error occurs in the received character (unless INACK is set)"]
    _0 = 0,
    #[doc = "1: Successive parity errors are counted up to the value specified in the MAX_ITERATION field. These parity errors generatea NACK on the ISO line. As soon as this value is reached, no additional NACK is sent on the ISO line. The flag ITERATION is asserted"]
    _1 = 1,
}
impl From<DSNACK_A> for bool {
    #[inline(always)]
    fn from(variant: DSNACK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSNACK`"]
pub type DSNACK_R = crate::R<bool, DSNACK_A>;
impl DSNACK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSNACK_A {
        match self.bits {
            false => DSNACK_A::_0,
            true => DSNACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSNACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSNACK_A::_1
    }
}
#[doc = "Write proxy for field `DSNACK`"]
pub struct DSNACK_W<'a> {
    w: &'a mut W,
}
impl<'a> DSNACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSNACK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "NACK is sent on the ISO line as soon as a parity error occurs in the received character (unless INACK is set)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSNACK_A::_0)
    }
    #[doc = "Successive parity errors are counted up to the value specified in the MAX_ITERATION field. These parity errors generatea NACK on the ISO line. As soon as this value is reached, no additional NACK is sent on the ISO line. The flag ITERATION is asserted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSNACK_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `INVDATA`"]
pub type INVDATA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVDATA`"]
pub struct INVDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> INVDATA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `MAX_ITERATION`"]
pub type MAX_ITERATION_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAX_ITERATION`"]
pub struct MAX_ITERATION_W<'a> {
    w: &'a mut W,
}
impl<'a> MAX_ITERATION_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "Infrared Receive Line Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FILTER_A {
    #[doc = "0: The USART does not filter the receive line"]
    _0 = 0,
    #[doc = "1: The USART filters the receive line using a three-sample filter (1/16-bit clock) (2 over 3 majority)"]
    _1 = 1,
}
impl From<FILTER_A> for bool {
    #[inline(always)]
    fn from(variant: FILTER_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `FILTER`"]
pub type FILTER_R = crate::R<bool, FILTER_A>;
impl FILTER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_A {
        match self.bits {
            false => FILTER_A::_0,
            true => FILTER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FILTER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FILTER_A::_1
    }
}
#[doc = "Write proxy for field `FILTER`"]
pub struct FILTER_W<'a> {
    w: &'a mut W,
}
impl<'a> FILTER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FILTER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The USART does not filter the receive line"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILTER_A::_0)
    }
    #[doc = "The USART filters the receive line using a three-sample filter (1/16-bit clock) (2 over 3 majority)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILTER_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Usart Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&self) -> USCLKS_R {
        USCLKS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Character Length."]
    #[inline(always)]
    pub fn chrl(&self) -> CHRL_R {
        CHRL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 8 - SPI CLock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline(always)]
    pub fn nbstop(&self) -> NBSTOP_R {
        NBSTOP_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&self) -> CHMODE_R {
        CHMODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&self) -> CLKO_R {
        CLKO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline(always)]
    pub fn inack(&self) -> INACK_R {
        INACK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&self) -> DSNACK_R {
        DSNACK_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Inverted data"]
    #[inline(always)]
    pub fn invdata(&self) -> INVDATA_R {
        INVDATA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Max interation"]
    #[inline(always)]
    pub fn max_iteration(&self) -> MAX_ITERATION_R {
        MAX_ITERATION_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 28 - Infrared Receive Line Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Usart Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&mut self) -> USCLKS_W {
        USCLKS_W { w: self }
    }
    #[doc = "Bits 6:7 - Character Length."]
    #[inline(always)]
    pub fn chrl(&mut self) -> CHRL_W {
        CHRL_W { w: self }
    }
    #[doc = "Bit 8 - SPI CLock Phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W {
        CPHA_W { w: self }
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&mut self) -> PAR_W {
        PAR_W { w: self }
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline(always)]
    pub fn nbstop(&mut self) -> NBSTOP_W {
        NBSTOP_W { w: self }
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&mut self) -> CHMODE_W {
        CHMODE_W { w: self }
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W {
        CPOL_W { w: self }
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline(always)]
    pub fn mode9(&mut self) -> MODE9_W {
        MODE9_W { w: self }
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&mut self) -> CLKO_W {
        CLKO_W { w: self }
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline(always)]
    pub fn over(&mut self) -> OVER_W {
        OVER_W { w: self }
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline(always)]
    pub fn inack(&mut self) -> INACK_W {
        INACK_W { w: self }
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&mut self) -> DSNACK_W {
        DSNACK_W { w: self }
    }
    #[doc = "Bit 23 - Inverted data"]
    #[inline(always)]
    pub fn invdata(&mut self) -> INVDATA_W {
        INVDATA_W { w: self }
    }
    #[doc = "Bits 24:26 - Max interation"]
    #[inline(always)]
    pub fn max_iteration(&mut self) -> MAX_ITERATION_W {
        MAX_ITERATION_W { w: self }
    }
    #[doc = "Bit 28 - Infrared Receive Line Filter"]
    #[inline(always)]
    pub fn filter(&mut self) -> FILTER_W {
        FILTER_W { w: self }
    }
}
