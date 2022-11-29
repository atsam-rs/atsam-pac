#[doc = "Register `MR_SPI` reader"]
pub struct R(crate::R<SPI_MODE_MR_SPI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MODE_MR_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MODE_MR_SPI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MODE_MR_SPI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR_SPI` writer"]
pub struct W(crate::W<SPI_MODE_MR_SPI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MODE_MR_SPI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SPI_MODE_MR_SPI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MODE_MR_SPI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Usart Mode"]
pub type MODE_R = crate::FieldReader<u8, MODESELECT_A>;
#[doc = "Usart Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODESELECT_A {
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
impl From<MODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODESELECT_A> {
        match self.bits {
            0 => Some(MODESELECT_A::NORMAL),
            1 => Some(MODESELECT_A::RS485),
            2 => Some(MODESELECT_A::HARDWARE),
            3 => Some(MODESELECT_A::MODEM),
            4 => Some(MODESELECT_A::ISO7816_T0),
            6 => Some(MODESELECT_A::ISO7816_T1),
            8 => Some(MODESELECT_A::IRDA),
            10 => Some(MODESELECT_A::LIN_MASTER),
            11 => Some(MODESELECT_A::LIN_SLAVE),
            14 => Some(MODESELECT_A::SPI_MASTER),
            15 => Some(MODESELECT_A::SPI_SLAVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MODESELECT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RS485`"]
    #[inline(always)]
    pub fn is_rs485(&self) -> bool {
        *self == MODESELECT_A::RS485
    }
    #[doc = "Checks if the value of the field is `HARDWARE`"]
    #[inline(always)]
    pub fn is_hardware(&self) -> bool {
        *self == MODESELECT_A::HARDWARE
    }
    #[doc = "Checks if the value of the field is `MODEM`"]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        *self == MODESELECT_A::MODEM
    }
    #[doc = "Checks if the value of the field is `ISO7816_T0`"]
    #[inline(always)]
    pub fn is_iso7816_t0(&self) -> bool {
        *self == MODESELECT_A::ISO7816_T0
    }
    #[doc = "Checks if the value of the field is `ISO7816_T1`"]
    #[inline(always)]
    pub fn is_iso7816_t1(&self) -> bool {
        *self == MODESELECT_A::ISO7816_T1
    }
    #[doc = "Checks if the value of the field is `IRDA`"]
    #[inline(always)]
    pub fn is_irda(&self) -> bool {
        *self == MODESELECT_A::IRDA
    }
    #[doc = "Checks if the value of the field is `LIN_MASTER`"]
    #[inline(always)]
    pub fn is_lin_master(&self) -> bool {
        *self == MODESELECT_A::LIN_MASTER
    }
    #[doc = "Checks if the value of the field is `LIN_SLAVE`"]
    #[inline(always)]
    pub fn is_lin_slave(&self) -> bool {
        *self == MODESELECT_A::LIN_SLAVE
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == MODESELECT_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == MODESELECT_A::SPI_SLAVE
    }
}
#[doc = "Field `MODE` writer - Usart Mode"]
pub type MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_MODE_MR_SPI_SPEC, u8, MODESELECT_A, 4, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Normal"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODESELECT_A::NORMAL)
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn rs485(self) -> &'a mut W {
        self.variant(MODESELECT_A::RS485)
    }
    #[doc = "Hardware Handshaking"]
    #[inline(always)]
    pub fn hardware(self) -> &'a mut W {
        self.variant(MODESELECT_A::HARDWARE)
    }
    #[doc = "Modem"]
    #[inline(always)]
    pub fn modem(self) -> &'a mut W {
        self.variant(MODESELECT_A::MODEM)
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline(always)]
    pub fn iso7816_t0(self) -> &'a mut W {
        self.variant(MODESELECT_A::ISO7816_T0)
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline(always)]
    pub fn iso7816_t1(self) -> &'a mut W {
        self.variant(MODESELECT_A::ISO7816_T1)
    }
    #[doc = "IrDA"]
    #[inline(always)]
    pub fn irda(self) -> &'a mut W {
        self.variant(MODESELECT_A::IRDA)
    }
    #[doc = "LIN Master"]
    #[inline(always)]
    pub fn lin_master(self) -> &'a mut W {
        self.variant(MODESELECT_A::LIN_MASTER)
    }
    #[doc = "LIN Slave"]
    #[inline(always)]
    pub fn lin_slave(self) -> &'a mut W {
        self.variant(MODESELECT_A::LIN_SLAVE)
    }
    #[doc = "SPI Master"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(MODESELECT_A::SPI_MASTER)
    }
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(MODESELECT_A::SPI_SLAVE)
    }
}
#[doc = "Field `USCLKS` reader - Clock Selection"]
pub type USCLKS_R = crate::FieldReader<u8, USCLKSSELECT_A>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USCLKSSELECT_A {
    #[doc = "0: MCK"]
    MCK = 0,
    #[doc = "1: MCK / DIV"]
    MCK_DIV = 1,
    #[doc = "3: SCK"]
    SCK = 3,
}
impl From<USCLKSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: USCLKSSELECT_A) -> Self {
        variant as _
    }
}
impl USCLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USCLKSSELECT_A> {
        match self.bits {
            0 => Some(USCLKSSELECT_A::MCK),
            1 => Some(USCLKSSELECT_A::MCK_DIV),
            3 => Some(USCLKSSELECT_A::SCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == USCLKSSELECT_A::MCK
    }
    #[doc = "Checks if the value of the field is `MCK_DIV`"]
    #[inline(always)]
    pub fn is_mck_div(&self) -> bool {
        *self == USCLKSSELECT_A::MCK_DIV
    }
    #[doc = "Checks if the value of the field is `SCK`"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == USCLKSSELECT_A::SCK
    }
}
#[doc = "Field `USCLKS` writer - Clock Selection"]
pub type USCLKS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_MODE_MR_SPI_SPEC, u8, USCLKSSELECT_A, 2, O>;
impl<'a, const O: u8> USCLKS_W<'a, O> {
    #[doc = "MCK"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(USCLKSSELECT_A::MCK)
    }
    #[doc = "MCK / DIV"]
    #[inline(always)]
    pub fn mck_div(self) -> &'a mut W {
        self.variant(USCLKSSELECT_A::MCK_DIV)
    }
    #[doc = "SCK"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut W {
        self.variant(USCLKSSELECT_A::SCK)
    }
}
#[doc = "Field `CHRL` reader - Character Length."]
pub type CHRL_R = crate::FieldReader<u8, CHRLSELECT_A>;
#[doc = "Character Length.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHRLSELECT_A {
    #[doc = "0: 5 bits"]
    _5 = 0,
    #[doc = "1: 6 bits"]
    _6 = 1,
    #[doc = "2: 7 bits"]
    _7 = 2,
    #[doc = "3: 8 bits"]
    _8 = 3,
}
impl From<CHRLSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CHRLSELECT_A) -> Self {
        variant as _
    }
}
impl CHRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHRLSELECT_A {
        match self.bits {
            0 => CHRLSELECT_A::_5,
            1 => CHRLSELECT_A::_6,
            2 => CHRLSELECT_A::_7,
            3 => CHRLSELECT_A::_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == CHRLSELECT_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == CHRLSELECT_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == CHRLSELECT_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == CHRLSELECT_A::_8
    }
}
#[doc = "Field `CHRL` writer - Character Length."]
pub type CHRL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SPI_MODE_MR_SPI_SPEC, u8, CHRLSELECT_A, 2, O>;
impl<'a, const O: u8> CHRL_W<'a, O> {
    #[doc = "5 bits"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(CHRLSELECT_A::_5)
    }
    #[doc = "6 bits"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(CHRLSELECT_A::_6)
    }
    #[doc = "7 bits"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(CHRLSELECT_A::_7)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(CHRLSELECT_A::_8)
    }
}
#[doc = "Field `CPHA` reader - SPI CLock Phase"]
pub type CPHA_R = crate::BitReader<CPHASELECT_A>;
#[doc = "SPI CLock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHASELECT_A {
    #[doc = "0: Data is changed on the leading edge of SPCK and captured on the following edge of SPCK"]
    _0 = 0,
    #[doc = "1: Data is captured on the leading edge of SPCK and changed on the following edge of SPCK"]
    _1 = 1,
}
impl From<CPHASELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPHASELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHASELECT_A {
        match self.bits {
            false => CPHASELECT_A::_0,
            true => CPHASELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPHASELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPHASELECT_A::_1
    }
}
#[doc = "Field `CPHA` writer - SPI CLock Phase"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_MR_SPI_SPEC, CPHASELECT_A, O>;
impl<'a, const O: u8> CPHA_W<'a, O> {
    #[doc = "Data is changed on the leading edge of SPCK and captured on the following edge of SPCK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHASELECT_A::_0)
    }
    #[doc = "Data is captured on the leading edge of SPCK and changed on the following edge of SPCK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHASELECT_A::_1)
    }
}
#[doc = "Field `PAR` reader - Parity Type"]
pub type PAR_R = crate::FieldReader<u8, PARSELECT_A>;
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PARSELECT_A {
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
impl From<PARSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PARSELECT_A) -> Self {
        variant as _
    }
}
impl PAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARSELECT_A {
        match self.bits {
            0 => PARSELECT_A::EVEN,
            1 => PARSELECT_A::ODD,
            2 => PARSELECT_A::SPACE,
            3 => PARSELECT_A::MARK,
            4 => PARSELECT_A::NONE,
            5 => PARSELECT_A::_5,
            6 => PARSELECT_A::MULTI,
            7 => PARSELECT_A::_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PARSELECT_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PARSELECT_A::ODD
    }
    #[doc = "Checks if the value of the field is `SPACE`"]
    #[inline(always)]
    pub fn is_space(&self) -> bool {
        *self == PARSELECT_A::SPACE
    }
    #[doc = "Checks if the value of the field is `MARK`"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == PARSELECT_A::MARK
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PARSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == PARSELECT_A::_5
    }
    #[doc = "Checks if the value of the field is `MULTI`"]
    #[inline(always)]
    pub fn is_multi(&self) -> bool {
        *self == PARSELECT_A::MULTI
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == PARSELECT_A::_7
    }
}
#[doc = "Field `PAR` writer - Parity Type"]
pub type PAR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SPI_MODE_MR_SPI_SPEC, u8, PARSELECT_A, 3, O>;
impl<'a, const O: u8> PAR_W<'a, O> {
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PARSELECT_A::EVEN)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PARSELECT_A::ODD)
    }
    #[doc = "Parity forced to 0 (Space)"]
    #[inline(always)]
    pub fn space(self) -> &'a mut W {
        self.variant(PARSELECT_A::SPACE)
    }
    #[doc = "Parity forced to 1 (Mark)"]
    #[inline(always)]
    pub fn mark(self) -> &'a mut W {
        self.variant(PARSELECT_A::MARK)
    }
    #[doc = "No Parity"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(PARSELECT_A::NONE)
    }
    #[doc = "No Parity"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(PARSELECT_A::_5)
    }
    #[doc = "Multi-drop mode"]
    #[inline(always)]
    pub fn multi(self) -> &'a mut W {
        self.variant(PARSELECT_A::MULTI)
    }
    #[doc = "Multi-drop mode"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(PARSELECT_A::_7)
    }
}
#[doc = "Field `NBSTOP` reader - Number of Stop Bits"]
pub type NBSTOP_R = crate::FieldReader<u8, NBSTOPSELECT_A>;
#[doc = "Number of Stop Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NBSTOPSELECT_A {
    #[doc = "0: 1 stop bit"]
    _1 = 0,
    #[doc = "1: 1.5 stop bits (Only valid if SYNC=0)"]
    _1_5 = 1,
    #[doc = "2: 2 stop bits"]
    _2 = 2,
}
impl From<NBSTOPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NBSTOPSELECT_A) -> Self {
        variant as _
    }
}
impl NBSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NBSTOPSELECT_A> {
        match self.bits {
            0 => Some(NBSTOPSELECT_A::_1),
            1 => Some(NBSTOPSELECT_A::_1_5),
            2 => Some(NBSTOPSELECT_A::_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NBSTOPSELECT_A::_1
    }
    #[doc = "Checks if the value of the field is `_1_5`"]
    #[inline(always)]
    pub fn is_1_5(&self) -> bool {
        *self == NBSTOPSELECT_A::_1_5
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == NBSTOPSELECT_A::_2
    }
}
#[doc = "Field `NBSTOP` writer - Number of Stop Bits"]
pub type NBSTOP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_MODE_MR_SPI_SPEC, u8, NBSTOPSELECT_A, 2, O>;
impl<'a, const O: u8> NBSTOP_W<'a, O> {
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NBSTOPSELECT_A::_1)
    }
    #[doc = "1.5 stop bits (Only valid if SYNC=0)"]
    #[inline(always)]
    pub fn _1_5(self) -> &'a mut W {
        self.variant(NBSTOPSELECT_A::_1_5)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(NBSTOPSELECT_A::_2)
    }
}
#[doc = "Field `CHMODE` reader - Channel Mode"]
pub type CHMODE_R = crate::FieldReader<u8, CHMODESELECT_A>;
#[doc = "Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHMODESELECT_A {
    #[doc = "0: Normal Mode"]
    NORMAL = 0,
    #[doc = "1: Automatic Echo. Receiver input is connected to the TXD pin"]
    ECHO = 1,
    #[doc = "2: Local Loopback. Transmitter output is connected to the Receiver Input"]
    LOCAL_LOOP = 2,
    #[doc = "3: Remote Loopback. RXD pin is internally connected to the TXD pin"]
    REMOTE_LOOP = 3,
}
impl From<CHMODESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: CHMODESELECT_A) -> Self {
        variant as _
    }
}
impl CHMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHMODESELECT_A {
        match self.bits {
            0 => CHMODESELECT_A::NORMAL,
            1 => CHMODESELECT_A::ECHO,
            2 => CHMODESELECT_A::LOCAL_LOOP,
            3 => CHMODESELECT_A::REMOTE_LOOP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CHMODESELECT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `ECHO`"]
    #[inline(always)]
    pub fn is_echo(&self) -> bool {
        *self == CHMODESELECT_A::ECHO
    }
    #[doc = "Checks if the value of the field is `LOCAL_LOOP`"]
    #[inline(always)]
    pub fn is_local_loop(&self) -> bool {
        *self == CHMODESELECT_A::LOCAL_LOOP
    }
    #[doc = "Checks if the value of the field is `REMOTE_LOOP`"]
    #[inline(always)]
    pub fn is_remote_loop(&self) -> bool {
        *self == CHMODESELECT_A::REMOTE_LOOP
    }
}
#[doc = "Field `CHMODE` writer - Channel Mode"]
pub type CHMODE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SPI_MODE_MR_SPI_SPEC, u8, CHMODESELECT_A, 2, O>;
impl<'a, const O: u8> CHMODE_W<'a, O> {
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CHMODESELECT_A::NORMAL)
    }
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin"]
    #[inline(always)]
    pub fn echo(self) -> &'a mut W {
        self.variant(CHMODESELECT_A::ECHO)
    }
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input"]
    #[inline(always)]
    pub fn local_loop(self) -> &'a mut W {
        self.variant(CHMODESELECT_A::LOCAL_LOOP)
    }
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin"]
    #[inline(always)]
    pub fn remote_loop(self) -> &'a mut W {
        self.variant(CHMODESELECT_A::REMOTE_LOOP)
    }
}
#[doc = "Field `CPOL` reader - SPI Clock Polarity"]
pub type CPOL_R = crate::BitReader<CPOLSELECT_A>;
#[doc = "SPI Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOLSELECT_A {
    #[doc = "0: The inactive state value of SPCK is logic level zero"]
    ZERO = 0,
    #[doc = "1: The inactive state value of SPCK is logic level one"]
    ONE = 1,
}
impl From<CPOLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPOLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOLSELECT_A {
        match self.bits {
            false => CPOLSELECT_A::ZERO,
            true => CPOLSELECT_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CPOLSELECT_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CPOLSELECT_A::ONE
    }
}
#[doc = "Field `CPOL` writer - SPI Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_MR_SPI_SPEC, CPOLSELECT_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "The inactive state value of SPCK is logic level zero"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CPOLSELECT_A::ZERO)
    }
    #[doc = "The inactive state value of SPCK is logic level one"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CPOLSELECT_A::ONE)
    }
}
#[doc = "Field `MODE9` reader - 9-bit Character Length"]
pub type MODE9_R = crate::BitReader<MODE9SELECT_A>;
#[doc = "9-bit Character Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODE9SELECT_A {
    #[doc = "0: CHRL defines character length"]
    _0 = 0,
    #[doc = "1: 9-bit character length"]
    _1 = 1,
}
impl From<MODE9SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MODE9SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE9SELECT_A {
        match self.bits {
            false => MODE9SELECT_A::_0,
            true => MODE9SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODE9SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODE9SELECT_A::_1
    }
}
#[doc = "Field `MODE9` writer - 9-bit Character Length"]
pub type MODE9_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_MODE_MR_SPI_SPEC, MODE9SELECT_A, O>;
impl<'a, const O: u8> MODE9_W<'a, O> {
    #[doc = "CHRL defines character length"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODE9SELECT_A::_0)
    }
    #[doc = "9-bit character length"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODE9SELECT_A::_1)
    }
}
#[doc = "Field `CLKO` reader - Clock Output Select"]
pub type CLKO_R = crate::BitReader<CLKOSELECT_A>;
#[doc = "Clock Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKOSELECT_A {
    #[doc = "0: The USART does not drive the SCK pin"]
    _0 = 0,
    #[doc = "1: The USART drives the SCK pin if USCLKS does not select the external clock SCK"]
    _1 = 1,
}
impl From<CLKOSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CLKOSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKOSELECT_A {
        match self.bits {
            false => CLKOSELECT_A::_0,
            true => CLKOSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKOSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKOSELECT_A::_1
    }
}
#[doc = "Field `CLKO` writer - Clock Output Select"]
pub type CLKO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_MR_SPI_SPEC, CLKOSELECT_A, O>;
impl<'a, const O: u8> CLKO_W<'a, O> {
    #[doc = "The USART does not drive the SCK pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKOSELECT_A::_0)
    }
    #[doc = "The USART drives the SCK pin if USCLKS does not select the external clock SCK"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKOSELECT_A::_1)
    }
}
#[doc = "Field `OVER` reader - Oversampling Mode"]
pub type OVER_R = crate::BitReader<OVERSELECT_A>;
#[doc = "Oversampling Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVERSELECT_A {
    #[doc = "0: 16x Oversampling"]
    X16 = 0,
    #[doc = "1: 8x Oversampling"]
    X8 = 1,
}
impl From<OVERSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: OVERSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl OVER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVERSELECT_A {
        match self.bits {
            false => OVERSELECT_A::X16,
            true => OVERSELECT_A::X8,
        }
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == OVERSELECT_A::X16
    }
    #[doc = "Checks if the value of the field is `X8`"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == OVERSELECT_A::X8
    }
}
#[doc = "Field `OVER` writer - Oversampling Mode"]
pub type OVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_MR_SPI_SPEC, OVERSELECT_A, O>;
impl<'a, const O: u8> OVER_W<'a, O> {
    #[doc = "16x Oversampling"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut W {
        self.variant(OVERSELECT_A::X16)
    }
    #[doc = "8x Oversampling"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut W {
        self.variant(OVERSELECT_A::X8)
    }
}
#[doc = "Field `INACK` reader - Inhibit Non Acknowledge"]
pub type INACK_R = crate::BitReader<INACKSELECT_A>;
#[doc = "Inhibit Non Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INACKSELECT_A {
    #[doc = "0: The NACK is generated"]
    _0 = 0,
    #[doc = "1: The NACK is not generated"]
    _1 = 1,
}
impl From<INACKSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: INACKSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl INACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INACKSELECT_A {
        match self.bits {
            false => INACKSELECT_A::_0,
            true => INACKSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INACKSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INACKSELECT_A::_1
    }
}
#[doc = "Field `INACK` writer - Inhibit Non Acknowledge"]
pub type INACK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_MODE_MR_SPI_SPEC, INACKSELECT_A, O>;
impl<'a, const O: u8> INACK_W<'a, O> {
    #[doc = "The NACK is generated"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INACKSELECT_A::_0)
    }
    #[doc = "The NACK is not generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INACKSELECT_A::_1)
    }
}
#[doc = "Field `DSNACK` reader - Disable Successive NACK"]
pub type DSNACK_R = crate::BitReader<DSNACKSELECT_A>;
#[doc = "Disable Successive NACK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DSNACKSELECT_A {
    #[doc = "0: NACK is sent on the ISO line as soon as a parity error occurs in the received character (unless INACK is set)"]
    _0 = 0,
    #[doc = "1: Successive parity errors are counted up to the value specified in the MAX_ITERATION field. These parity errors generatea NACK on the ISO line. As soon as this value is reached, no additional NACK is sent on the ISO line. The flag ITERATION is asserted"]
    _1 = 1,
}
impl From<DSNACKSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DSNACKSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DSNACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSNACKSELECT_A {
        match self.bits {
            false => DSNACKSELECT_A::_0,
            true => DSNACKSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSNACKSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSNACKSELECT_A::_1
    }
}
#[doc = "Field `DSNACK` writer - Disable Successive NACK"]
pub type DSNACK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_MODE_MR_SPI_SPEC, DSNACKSELECT_A, O>;
impl<'a, const O: u8> DSNACK_W<'a, O> {
    #[doc = "NACK is sent on the ISO line as soon as a parity error occurs in the received character (unless INACK is set)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DSNACKSELECT_A::_0)
    }
    #[doc = "Successive parity errors are counted up to the value specified in the MAX_ITERATION field. These parity errors generatea NACK on the ISO line. As soon as this value is reached, no additional NACK is sent on the ISO line. The flag ITERATION is asserted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DSNACKSELECT_A::_1)
    }
}
#[doc = "Field `INVDATA` reader - Inverted data"]
pub type INVDATA_R = crate::BitReader<bool>;
#[doc = "Field `INVDATA` writer - Inverted data"]
pub type INVDATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI_MODE_MR_SPI_SPEC, bool, O>;
#[doc = "Field `MAX_ITERATION` reader - Max interation"]
pub type MAX_ITERATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_ITERATION` writer - Max interation"]
pub type MAX_ITERATION_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI_MODE_MR_SPI_SPEC, u8, u8, 3, O>;
#[doc = "Field `FILTER` reader - Infrared Receive Line Filter"]
pub type FILTER_R = crate::BitReader<FILTERSELECT_A>;
#[doc = "Infrared Receive Line Filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FILTERSELECT_A {
    #[doc = "0: The USART does not filter the receive line"]
    _0 = 0,
    #[doc = "1: The USART filters the receive line using a three-sample filter (1/16-bit clock) (2 over 3 majority)"]
    _1 = 1,
}
impl From<FILTERSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: FILTERSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FILTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTERSELECT_A {
        match self.bits {
            false => FILTERSELECT_A::_0,
            true => FILTERSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FILTERSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FILTERSELECT_A::_1
    }
}
#[doc = "Field `FILTER` writer - Infrared Receive Line Filter"]
pub type FILTER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_MODE_MR_SPI_SPEC, FILTERSELECT_A, O>;
impl<'a, const O: u8> FILTER_W<'a, O> {
    #[doc = "The USART does not filter the receive line"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILTERSELECT_A::_0)
    }
    #[doc = "The USART filters the receive line using a three-sample filter (1/16-bit clock) (2 over 3 majority)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILTERSELECT_A::_1)
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
        USCLKS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Character Length."]
    #[inline(always)]
    pub fn chrl(&self) -> CHRL_R {
        CHRL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SPI CLock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline(always)]
    pub fn nbstop(&self) -> NBSTOP_R {
        NBSTOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    pub fn chmode(&self) -> CHMODE_R {
        CHMODE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline(always)]
    pub fn mode9(&self) -> MODE9_R {
        MODE9_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    pub fn clko(&self) -> CLKO_R {
        CLKO_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline(always)]
    pub fn over(&self) -> OVER_R {
        OVER_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline(always)]
    pub fn inack(&self) -> INACK_R {
        INACK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline(always)]
    pub fn dsnack(&self) -> DSNACK_R {
        DSNACK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Inverted data"]
    #[inline(always)]
    pub fn invdata(&self) -> INVDATA_R {
        INVDATA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Max interation"]
    #[inline(always)]
    pub fn max_iteration(&self) -> MAX_ITERATION_R {
        MAX_ITERATION_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Infrared Receive Line Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Usart Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn usclks(&mut self) -> USCLKS_W<4> {
        USCLKS_W::new(self)
    }
    #[doc = "Bits 6:7 - Character Length."]
    #[inline(always)]
    #[must_use]
    pub fn chrl(&mut self) -> CHRL_W<6> {
        CHRL_W::new(self)
    }
    #[doc = "Bit 8 - SPI CLock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<8> {
        CPHA_W::new(self)
    }
    #[doc = "Bits 9:11 - Parity Type"]
    #[inline(always)]
    #[must_use]
    pub fn par(&mut self) -> PAR_W<9> {
        PAR_W::new(self)
    }
    #[doc = "Bits 12:13 - Number of Stop Bits"]
    #[inline(always)]
    #[must_use]
    pub fn nbstop(&mut self) -> NBSTOP_W<12> {
        NBSTOP_W::new(self)
    }
    #[doc = "Bits 14:15 - Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn chmode(&mut self) -> CHMODE_W<14> {
        CHMODE_W::new(self)
    }
    #[doc = "Bit 16 - SPI Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<16> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 17 - 9-bit Character Length"]
    #[inline(always)]
    #[must_use]
    pub fn mode9(&mut self) -> MODE9_W<17> {
        MODE9_W::new(self)
    }
    #[doc = "Bit 18 - Clock Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn clko(&mut self) -> CLKO_W<18> {
        CLKO_W::new(self)
    }
    #[doc = "Bit 19 - Oversampling Mode"]
    #[inline(always)]
    #[must_use]
    pub fn over(&mut self) -> OVER_W<19> {
        OVER_W::new(self)
    }
    #[doc = "Bit 20 - Inhibit Non Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn inack(&mut self) -> INACK_W<20> {
        INACK_W::new(self)
    }
    #[doc = "Bit 21 - Disable Successive NACK"]
    #[inline(always)]
    #[must_use]
    pub fn dsnack(&mut self) -> DSNACK_W<21> {
        DSNACK_W::new(self)
    }
    #[doc = "Bit 23 - Inverted data"]
    #[inline(always)]
    #[must_use]
    pub fn invdata(&mut self) -> INVDATA_W<23> {
        INVDATA_W::new(self)
    }
    #[doc = "Bits 24:26 - Max interation"]
    #[inline(always)]
    #[must_use]
    pub fn max_iteration(&mut self) -> MAX_ITERATION_W<24> {
        MAX_ITERATION_W::new(self)
    }
    #[doc = "Bit 28 - Infrared Receive Line Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<28> {
        FILTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mode_mr_spi](index.html) module"]
pub struct SPI_MODE_MR_SPI_SPEC;
impl crate::RegisterSpec for SPI_MODE_MR_SPI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mode_mr_spi::R](R) reader structure"]
impl crate::Readable for SPI_MODE_MR_SPI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mode_mr_spi::W](W) writer structure"]
impl crate::Writable for SPI_MODE_MR_SPI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR_SPI to value 0"]
impl crate::Resettable for SPI_MODE_MR_SPI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
