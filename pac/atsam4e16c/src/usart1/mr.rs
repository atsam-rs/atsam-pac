#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART_MODE` reader - USART Mode of Operation"]
pub type USART_MODE_R = crate::FieldReader<u8, USART_MODE_A>;
#[doc = "USART Mode of Operation"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USART_MODE_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: RS485"]
    RS485 = 1,
    #[doc = "2: Hardware Handshaking"]
    HW_HANDSHAKING = 2,
    #[doc = "3: Modem"]
    MODEM = 3,
    #[doc = "4: IS07816 Protocol: T = 0"]
    IS07816_T_0 = 4,
    #[doc = "6: IS07816 Protocol: T = 1"]
    IS07816_T_1 = 6,
    #[doc = "8: IrDA"]
    IRDA = 8,
    #[doc = "14: SPI master"]
    SPI_MASTER = 14,
    #[doc = "15: SPI Slave"]
    SPI_SLAVE = 15,
}
impl From<USART_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: USART_MODE_A) -> Self {
        variant as _
    }
}
impl USART_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USART_MODE_A> {
        match self.bits {
            0 => Some(USART_MODE_A::NORMAL),
            1 => Some(USART_MODE_A::RS485),
            2 => Some(USART_MODE_A::HW_HANDSHAKING),
            3 => Some(USART_MODE_A::MODEM),
            4 => Some(USART_MODE_A::IS07816_T_0),
            6 => Some(USART_MODE_A::IS07816_T_1),
            8 => Some(USART_MODE_A::IRDA),
            14 => Some(USART_MODE_A::SPI_MASTER),
            15 => Some(USART_MODE_A::SPI_SLAVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == USART_MODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `RS485`"]
    #[inline(always)]
    pub fn is_rs485(&self) -> bool {
        *self == USART_MODE_A::RS485
    }
    #[doc = "Checks if the value of the field is `HW_HANDSHAKING`"]
    #[inline(always)]
    pub fn is_hw_handshaking(&self) -> bool {
        *self == USART_MODE_A::HW_HANDSHAKING
    }
    #[doc = "Checks if the value of the field is `MODEM`"]
    #[inline(always)]
    pub fn is_modem(&self) -> bool {
        *self == USART_MODE_A::MODEM
    }
    #[doc = "Checks if the value of the field is `IS07816_T_0`"]
    #[inline(always)]
    pub fn is_is07816_t_0(&self) -> bool {
        *self == USART_MODE_A::IS07816_T_0
    }
    #[doc = "Checks if the value of the field is `IS07816_T_1`"]
    #[inline(always)]
    pub fn is_is07816_t_1(&self) -> bool {
        *self == USART_MODE_A::IS07816_T_1
    }
    #[doc = "Checks if the value of the field is `IRDA`"]
    #[inline(always)]
    pub fn is_irda(&self) -> bool {
        *self == USART_MODE_A::IRDA
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == USART_MODE_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `SPI_SLAVE`"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == USART_MODE_A::SPI_SLAVE
    }
}
#[doc = "Field `USART_MODE` writer - USART Mode of Operation"]
pub type USART_MODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MR_SPEC, u8, USART_MODE_A, 4, O>;
impl<'a, const O: u8> USART_MODE_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(USART_MODE_A::NORMAL)
    }
    #[doc = "RS485"]
    #[inline(always)]
    pub fn rs485(self) -> &'a mut W {
        self.variant(USART_MODE_A::RS485)
    }
    #[doc = "Hardware Handshaking"]
    #[inline(always)]
    pub fn hw_handshaking(self) -> &'a mut W {
        self.variant(USART_MODE_A::HW_HANDSHAKING)
    }
    #[doc = "Modem"]
    #[inline(always)]
    pub fn modem(self) -> &'a mut W {
        self.variant(USART_MODE_A::MODEM)
    }
    #[doc = "IS07816 Protocol: T = 0"]
    #[inline(always)]
    pub fn is07816_t_0(self) -> &'a mut W {
        self.variant(USART_MODE_A::IS07816_T_0)
    }
    #[doc = "IS07816 Protocol: T = 1"]
    #[inline(always)]
    pub fn is07816_t_1(self) -> &'a mut W {
        self.variant(USART_MODE_A::IS07816_T_1)
    }
    #[doc = "IrDA"]
    #[inline(always)]
    pub fn irda(self) -> &'a mut W {
        self.variant(USART_MODE_A::IRDA)
    }
    #[doc = "SPI master"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(USART_MODE_A::SPI_MASTER)
    }
    #[doc = "SPI Slave"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut W {
        self.variant(USART_MODE_A::SPI_SLAVE)
    }
}
#[doc = "Field `USCLKS` reader - Clock Selection"]
pub type USCLKS_R = crate::FieldReader<u8, USCLKS_A>;
#[doc = "Clock Selection"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USCLKS_A {
    #[doc = "0: Peripheral clock is selected"]
    MCK = 0,
    #[doc = "1: Peripheral clock divided (DIV=8) is selected"]
    DIV = 1,
    #[doc = "3: Serial clock SCK is selected"]
    SCK = 3,
}
impl From<USCLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: USCLKS_A) -> Self {
        variant as _
    }
}
impl USCLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USCLKS_A> {
        match self.bits {
            0 => Some(USCLKS_A::MCK),
            1 => Some(USCLKS_A::DIV),
            3 => Some(USCLKS_A::SCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == USCLKS_A::MCK
    }
    #[doc = "Checks if the value of the field is `DIV`"]
    #[inline(always)]
    pub fn is_div(&self) -> bool {
        *self == USCLKS_A::DIV
    }
    #[doc = "Checks if the value of the field is `SCK`"]
    #[inline(always)]
    pub fn is_sck(&self) -> bool {
        *self == USCLKS_A::SCK
    }
}
#[doc = "Field `USCLKS` writer - Clock Selection"]
pub type USCLKS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, USCLKS_A, 2, O>;
impl<'a, const O: u8> USCLKS_W<'a, O> {
    #[doc = "Peripheral clock is selected"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(USCLKS_A::MCK)
    }
    #[doc = "Peripheral clock divided (DIV=8) is selected"]
    #[inline(always)]
    pub fn div(self) -> &'a mut W {
        self.variant(USCLKS_A::DIV)
    }
    #[doc = "Serial clock SCK is selected"]
    #[inline(always)]
    pub fn sck(self) -> &'a mut W {
        self.variant(USCLKS_A::SCK)
    }
}
#[doc = "Field `CHRL` reader - Character Length"]
pub type CHRL_R = crate::FieldReader<u8, CHRL_A>;
#[doc = "Character Length"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHRL_A {
    #[doc = "0: Character length is 5 bits"]
    _5_BIT = 0,
    #[doc = "1: Character length is 6 bits"]
    _6_BIT = 1,
    #[doc = "2: Character length is 7 bits"]
    _7_BIT = 2,
    #[doc = "3: Character length is 8 bits"]
    _8_BIT = 3,
}
impl From<CHRL_A> for u8 {
    #[inline(always)]
    fn from(variant: CHRL_A) -> Self {
        variant as _
    }
}
impl CHRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHRL_A {
        match self.bits {
            0 => CHRL_A::_5_BIT,
            1 => CHRL_A::_6_BIT,
            2 => CHRL_A::_7_BIT,
            3 => CHRL_A::_8_BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_5_BIT`"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == CHRL_A::_5_BIT
    }
    #[doc = "Checks if the value of the field is `_6_BIT`"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == CHRL_A::_6_BIT
    }
    #[doc = "Checks if the value of the field is `_7_BIT`"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == CHRL_A::_7_BIT
    }
    #[doc = "Checks if the value of the field is `_8_BIT`"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == CHRL_A::_8_BIT
    }
}
#[doc = "Field `CHRL` writer - Character Length"]
pub type CHRL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MR_SPEC, u8, CHRL_A, 2, O>;
impl<'a, const O: u8> CHRL_W<'a, O> {
    #[doc = "Character length is 5 bits"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut W {
        self.variant(CHRL_A::_5_BIT)
    }
    #[doc = "Character length is 6 bits"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut W {
        self.variant(CHRL_A::_6_BIT)
    }
    #[doc = "Character length is 7 bits"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut W {
        self.variant(CHRL_A::_7_BIT)
    }
    #[doc = "Character length is 8 bits"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut W {
        self.variant(CHRL_A::_8_BIT)
    }
}
#[doc = "Field `SYNC` reader - Synchronous Mode Select"]
pub type SYNC_R = crate::BitReader<bool>;
#[doc = "Field `SYNC` writer - Synchronous Mode Select"]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `PAR` reader - Parity Type"]
pub type PAR_R = crate::FieldReader<u8, PAR_A>;
#[doc = "Parity Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    #[doc = "4: No parity"]
    NO = 4,
    #[doc = "6: Multidrop mode"]
    MULTIDROP = 6,
}
impl From<PAR_A> for u8 {
    #[inline(always)]
    fn from(variant: PAR_A) -> Self {
        variant as _
    }
}
impl PAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PAR_A> {
        match self.bits {
            0 => Some(PAR_A::EVEN),
            1 => Some(PAR_A::ODD),
            2 => Some(PAR_A::SPACE),
            3 => Some(PAR_A::MARK),
            4 => Some(PAR_A::NO),
            6 => Some(PAR_A::MULTIDROP),
            _ => None,
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
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == PAR_A::NO
    }
    #[doc = "Checks if the value of the field is `MULTIDROP`"]
    #[inline(always)]
    pub fn is_multidrop(&self) -> bool {
        *self == PAR_A::MULTIDROP
    }
}
#[doc = "Field `PAR` writer - Parity Type"]
pub type PAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, PAR_A, 3, O>;
impl<'a, const O: u8> PAR_W<'a, O> {
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
    #[doc = "No parity"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(PAR_A::NO)
    }
    #[doc = "Multidrop mode"]
    #[inline(always)]
    pub fn multidrop(self) -> &'a mut W {
        self.variant(PAR_A::MULTIDROP)
    }
}
#[doc = "Field `NBSTOP` reader - Number of Stop Bits"]
pub type NBSTOP_R = crate::FieldReader<u8, NBSTOP_A>;
#[doc = "Number of Stop Bits"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NBSTOP_A {
    #[doc = "0: 1 stop bit"]
    _1_BIT = 0,
    #[doc = "1: 1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    _1_5_BIT = 1,
    #[doc = "2: 2 stop bits"]
    _2_BIT = 2,
}
impl From<NBSTOP_A> for u8 {
    #[inline(always)]
    fn from(variant: NBSTOP_A) -> Self {
        variant as _
    }
}
impl NBSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NBSTOP_A> {
        match self.bits {
            0 => Some(NBSTOP_A::_1_BIT),
            1 => Some(NBSTOP_A::_1_5_BIT),
            2 => Some(NBSTOP_A::_2_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1_BIT`"]
    #[inline(always)]
    pub fn is_1_bit(&self) -> bool {
        *self == NBSTOP_A::_1_BIT
    }
    #[doc = "Checks if the value of the field is `_1_5_BIT`"]
    #[inline(always)]
    pub fn is_1_5_bit(&self) -> bool {
        *self == NBSTOP_A::_1_5_BIT
    }
    #[doc = "Checks if the value of the field is `_2_BIT`"]
    #[inline(always)]
    pub fn is_2_bit(&self) -> bool {
        *self == NBSTOP_A::_2_BIT
    }
}
#[doc = "Field `NBSTOP` writer - Number of Stop Bits"]
pub type NBSTOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, NBSTOP_A, 2, O>;
impl<'a, const O: u8> NBSTOP_W<'a, O> {
    #[doc = "1 stop bit"]
    #[inline(always)]
    pub fn _1_bit(self) -> &'a mut W {
        self.variant(NBSTOP_A::_1_BIT)
    }
    #[doc = "1.5 stop bit (SYNC = 0) or reserved (SYNC = 1)"]
    #[inline(always)]
    pub fn _1_5_bit(self) -> &'a mut W {
        self.variant(NBSTOP_A::_1_5_BIT)
    }
    #[doc = "2 stop bits"]
    #[inline(always)]
    pub fn _2_bit(self) -> &'a mut W {
        self.variant(NBSTOP_A::_2_BIT)
    }
}
#[doc = "Field `CHMODE` reader - Channel Mode"]
pub type CHMODE_R = crate::FieldReader<u8, CHMODE_A>;
#[doc = "Channel Mode"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CHMODE_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Automatic Echo. Receiver input is connected to the TXD pin."]
    AUTOMATIC = 1,
    #[doc = "2: Local Loopback. Transmitter output is connected to the Receiver Input."]
    LOCAL_LOOPBACK = 2,
    #[doc = "3: Remote Loopback. RXD pin is internally connected to the TXD pin."]
    REMOTE_LOOPBACK = 3,
}
impl From<CHMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHMODE_A) -> Self {
        variant as _
    }
}
impl CHMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHMODE_A {
        match self.bits {
            0 => CHMODE_A::NORMAL,
            1 => CHMODE_A::AUTOMATIC,
            2 => CHMODE_A::LOCAL_LOOPBACK,
            3 => CHMODE_A::REMOTE_LOOPBACK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == CHMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == CHMODE_A::AUTOMATIC
    }
    #[doc = "Checks if the value of the field is `LOCAL_LOOPBACK`"]
    #[inline(always)]
    pub fn is_local_loopback(&self) -> bool {
        *self == CHMODE_A::LOCAL_LOOPBACK
    }
    #[doc = "Checks if the value of the field is `REMOTE_LOOPBACK`"]
    #[inline(always)]
    pub fn is_remote_loopback(&self) -> bool {
        *self == CHMODE_A::REMOTE_LOOPBACK
    }
}
#[doc = "Field `CHMODE` writer - Channel Mode"]
pub type CHMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MR_SPEC, u8, CHMODE_A, 2, O>;
impl<'a, const O: u8> CHMODE_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(CHMODE_A::NORMAL)
    }
    #[doc = "Automatic Echo. Receiver input is connected to the TXD pin."]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(CHMODE_A::AUTOMATIC)
    }
    #[doc = "Local Loopback. Transmitter output is connected to the Receiver Input."]
    #[inline(always)]
    pub fn local_loopback(self) -> &'a mut W {
        self.variant(CHMODE_A::LOCAL_LOOPBACK)
    }
    #[doc = "Remote Loopback. RXD pin is internally connected to the TXD pin."]
    #[inline(always)]
    pub fn remote_loopback(self) -> &'a mut W {
        self.variant(CHMODE_A::REMOTE_LOOPBACK)
    }
}
#[doc = "Field `MSBF` reader - Bit Order"]
pub type MSBF_R = crate::BitReader<bool>;
#[doc = "Field `MSBF` writer - Bit Order"]
pub type MSBF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `MODE9` reader - 9-bit Character Length"]
pub type MODE9_R = crate::BitReader<bool>;
#[doc = "Field `MODE9` writer - 9-bit Character Length"]
pub type MODE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `CLKO` reader - Clock Output Select"]
pub type CLKO_R = crate::BitReader<bool>;
#[doc = "Field `CLKO` writer - Clock Output Select"]
pub type CLKO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `OVER` reader - Oversampling Mode"]
pub type OVER_R = crate::BitReader<bool>;
#[doc = "Field `OVER` writer - Oversampling Mode"]
pub type OVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `INACK` reader - Inhibit Non Acknowledge"]
pub type INACK_R = crate::BitReader<bool>;
#[doc = "Field `INACK` writer - Inhibit Non Acknowledge"]
pub type INACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `DSNACK` reader - Disable Successive NACK"]
pub type DSNACK_R = crate::BitReader<bool>;
#[doc = "Field `DSNACK` writer - Disable Successive NACK"]
pub type DSNACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `VAR_SYNC` reader - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
pub type VAR_SYNC_R = crate::BitReader<bool>;
#[doc = "Field `VAR_SYNC` writer - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
pub type VAR_SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `INVDATA` reader - Inverted Data"]
pub type INVDATA_R = crate::BitReader<bool>;
#[doc = "Field `INVDATA` writer - Inverted Data"]
pub type INVDATA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `MAX_ITERATION` reader - Maximum Number of Automatic Iteration"]
pub type MAX_ITERATION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAX_ITERATION` writer - Maximum Number of Automatic Iteration"]
pub type MAX_ITERATION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 3, O>;
#[doc = "Field `FILTER` reader - Receive Line Filter"]
pub type FILTER_R = crate::BitReader<bool>;
#[doc = "Field `FILTER` writer - Receive Line Filter"]
pub type FILTER_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `MAN` reader - Manchester Encoder/Decoder Enable"]
pub type MAN_R = crate::BitReader<bool>;
#[doc = "Field `MAN` writer - Manchester Encoder/Decoder Enable"]
pub type MAN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `MODSYNC` reader - Manchester Synchronization Mode"]
pub type MODSYNC_R = crate::BitReader<bool>;
#[doc = "Field `MODSYNC` writer - Manchester Synchronization Mode"]
pub type MODSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `ONEBIT` reader - Start Frame Delimiter Selector"]
pub type ONEBIT_R = crate::BitReader<bool>;
#[doc = "Field `ONEBIT` writer - Start Frame Delimiter Selector"]
pub type ONEBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    pub fn usart_mode(&self) -> USART_MODE_R {
        USART_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    pub fn usclks(&self) -> USCLKS_R {
        USCLKS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline(always)]
    pub fn chrl(&self) -> CHRL_R {
        CHRL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Synchronous Mode Select"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 8) & 1) != 0)
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
    #[doc = "Bit 16 - Bit Order"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 22 - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
    #[inline(always)]
    pub fn var_sync(&self) -> VAR_SYNC_R {
        VAR_SYNC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Inverted Data"]
    #[inline(always)]
    pub fn invdata(&self) -> INVDATA_R {
        INVDATA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Maximum Number of Automatic Iteration"]
    #[inline(always)]
    pub fn max_iteration(&self) -> MAX_ITERATION_R {
        MAX_ITERATION_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 28 - Receive Line Filter"]
    #[inline(always)]
    pub fn filter(&self) -> FILTER_R {
        FILTER_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Manchester Encoder/Decoder Enable"]
    #[inline(always)]
    pub fn man(&self) -> MAN_R {
        MAN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Manchester Synchronization Mode"]
    #[inline(always)]
    pub fn modsync(&self) -> MODSYNC_R {
        MODSYNC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Start Frame Delimiter Selector"]
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - USART Mode of Operation"]
    #[inline(always)]
    #[must_use]
    pub fn usart_mode(&mut self) -> USART_MODE_W<0> {
        USART_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn usclks(&mut self) -> USCLKS_W<4> {
        USCLKS_W::new(self)
    }
    #[doc = "Bits 6:7 - Character Length"]
    #[inline(always)]
    #[must_use]
    pub fn chrl(&mut self) -> CHRL_W<6> {
        CHRL_W::new(self)
    }
    #[doc = "Bit 8 - Synchronous Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<8> {
        SYNC_W::new(self)
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
    #[doc = "Bit 16 - Bit Order"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<16> {
        MSBF_W::new(self)
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
    #[doc = "Bit 22 - Variable Synchronization of Command/Data Sync Start Frame Delimiter"]
    #[inline(always)]
    #[must_use]
    pub fn var_sync(&mut self) -> VAR_SYNC_W<22> {
        VAR_SYNC_W::new(self)
    }
    #[doc = "Bit 23 - Inverted Data"]
    #[inline(always)]
    #[must_use]
    pub fn invdata(&mut self) -> INVDATA_W<23> {
        INVDATA_W::new(self)
    }
    #[doc = "Bits 24:26 - Maximum Number of Automatic Iteration"]
    #[inline(always)]
    #[must_use]
    pub fn max_iteration(&mut self) -> MAX_ITERATION_W<24> {
        MAX_ITERATION_W::new(self)
    }
    #[doc = "Bit 28 - Receive Line Filter"]
    #[inline(always)]
    #[must_use]
    pub fn filter(&mut self) -> FILTER_W<28> {
        FILTER_W::new(self)
    }
    #[doc = "Bit 29 - Manchester Encoder/Decoder Enable"]
    #[inline(always)]
    #[must_use]
    pub fn man(&mut self) -> MAN_W<29> {
        MAN_W::new(self)
    }
    #[doc = "Bit 30 - Manchester Synchronization Mode"]
    #[inline(always)]
    #[must_use]
    pub fn modsync(&mut self) -> MODSYNC_W<30> {
        MODSYNC_W::new(self)
    }
    #[doc = "Bit 31 - Start Frame Delimiter Selector"]
    #[inline(always)]
    #[must_use]
    pub fn onebit(&mut self) -> ONEBIT_W<31> {
        ONEBIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
