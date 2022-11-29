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
#[doc = "Field `MODE` reader - Master/Slave/Controller Mode"]
pub type MODE_R = crate::BitReader<MODESELECT_A>;
#[doc = "Master/Slave/Controller Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODESELECT_A {
    #[doc = "0: Slave mode (only serial data handled, clocks received from external master or controller)"]
    SLAVE = 0,
    #[doc = "1: Master mode (clocks generated and output by IISC, serial data handled if CR.RXEN and/or CR.TXEN written to 1)"]
    MASTER = 1,
}
impl From<MODESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MODESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODESELECT_A {
        match self.bits {
            false => MODESELECT_A::SLAVE,
            true => MODESELECT_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MODESELECT_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MODESELECT_A::MASTER
    }
}
#[doc = "Field `MODE` writer - Master/Slave/Controller Mode"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, MODESELECT_A, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Slave mode (only serial data handled, clocks received from external master or controller)"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MODESELECT_A::SLAVE)
    }
    #[doc = "Master mode (clocks generated and output by IISC, serial data handled if CR.RXEN and/or CR.TXEN written to 1)"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MODESELECT_A::MASTER)
    }
}
#[doc = "Field `DATALENGTH` reader - Data Word Length"]
pub type DATALENGTH_R = crate::FieldReader<u8, DATALENGTHSELECT_A>;
#[doc = "Data Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATALENGTHSELECT_A {
    #[doc = "0: 32 bits"]
    _32 = 0,
    #[doc = "1: 24 bits"]
    _24 = 1,
    #[doc = "2: 20 bits"]
    _20 = 2,
    #[doc = "3: 18 bits"]
    _18 = 3,
    #[doc = "4: 16 bits"]
    _16 = 4,
    #[doc = "5: 16 bits compact stereo"]
    _16C = 5,
    #[doc = "6: 8 bits"]
    _8 = 6,
    #[doc = "7: 8 bits compact stereo"]
    _8C = 7,
}
impl From<DATALENGTHSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DATALENGTHSELECT_A) -> Self {
        variant as _
    }
}
impl DATALENGTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATALENGTHSELECT_A {
        match self.bits {
            0 => DATALENGTHSELECT_A::_32,
            1 => DATALENGTHSELECT_A::_24,
            2 => DATALENGTHSELECT_A::_20,
            3 => DATALENGTHSELECT_A::_18,
            4 => DATALENGTHSELECT_A::_16,
            5 => DATALENGTHSELECT_A::_16C,
            6 => DATALENGTHSELECT_A::_8,
            7 => DATALENGTHSELECT_A::_8C,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == DATALENGTHSELECT_A::_32
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == DATALENGTHSELECT_A::_24
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == DATALENGTHSELECT_A::_20
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        *self == DATALENGTHSELECT_A::_18
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == DATALENGTHSELECT_A::_16
    }
    #[doc = "Checks if the value of the field is `_16C`"]
    #[inline(always)]
    pub fn is_16c(&self) -> bool {
        *self == DATALENGTHSELECT_A::_16C
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == DATALENGTHSELECT_A::_8
    }
    #[doc = "Checks if the value of the field is `_8C`"]
    #[inline(always)]
    pub fn is_8c(&self) -> bool {
        *self == DATALENGTHSELECT_A::_8C
    }
}
#[doc = "Field `DATALENGTH` writer - Data Word Length"]
pub type DATALENGTH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MR_SPEC, u8, DATALENGTHSELECT_A, 3, O>;
impl<'a, const O: u8> DATALENGTH_W<'a, O> {
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(DATALENGTHSELECT_A::_32)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(DATALENGTHSELECT_A::_24)
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut W {
        self.variant(DATALENGTHSELECT_A::_20)
    }
    #[doc = "18 bits"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut W {
        self.variant(DATALENGTHSELECT_A::_18)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(DATALENGTHSELECT_A::_16)
    }
    #[doc = "16 bits compact stereo"]
    #[inline(always)]
    pub fn _16c(self) -> &'a mut W {
        self.variant(DATALENGTHSELECT_A::_16C)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(DATALENGTHSELECT_A::_8)
    }
    #[doc = "8 bits compact stereo"]
    #[inline(always)]
    pub fn _8c(self) -> &'a mut W {
        self.variant(DATALENGTHSELECT_A::_8C)
    }
}
#[doc = "Field `RXMONO` reader - Receiver Mono"]
pub type RXMONO_R = crate::BitReader<RXMONOSELECT_A>;
#[doc = "Receiver Mono\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXMONOSELECT_A {
    #[doc = "0: Normal mode"]
    STEREO = 0,
    #[doc = "1: Left channel data is duplicated to right channel"]
    MONO = 1,
}
impl From<RXMONOSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RXMONOSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXMONO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXMONOSELECT_A {
        match self.bits {
            false => RXMONOSELECT_A::STEREO,
            true => RXMONOSELECT_A::MONO,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == RXMONOSELECT_A::STEREO
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == RXMONOSELECT_A::MONO
    }
}
#[doc = "Field `RXMONO` writer - Receiver Mono"]
pub type RXMONO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, RXMONOSELECT_A, O>;
impl<'a, const O: u8> RXMONO_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(RXMONOSELECT_A::STEREO)
    }
    #[doc = "Left channel data is duplicated to right channel"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut W {
        self.variant(RXMONOSELECT_A::MONO)
    }
}
#[doc = "Field `RXDMA` reader - Single or Multiple DMA Channels for Receiver"]
pub type RXDMA_R = crate::BitReader<RXDMASELECT_A>;
#[doc = "Single or Multiple DMA Channels for Receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMASELECT_A {
    #[doc = "0: Single DMA channel"]
    SINGLE = 0,
    #[doc = "1: One DMA channel per data channel"]
    MULTIPLE = 1,
}
impl From<RXDMASELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMASELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMASELECT_A {
        match self.bits {
            false => RXDMASELECT_A::SINGLE,
            true => RXDMASELECT_A::MULTIPLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == RXDMASELECT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == RXDMASELECT_A::MULTIPLE
    }
}
#[doc = "Field `RXDMA` writer - Single or Multiple DMA Channels for Receiver"]
pub type RXDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, RXDMASELECT_A, O>;
impl<'a, const O: u8> RXDMA_W<'a, O> {
    #[doc = "Single DMA channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(RXDMASELECT_A::SINGLE)
    }
    #[doc = "One DMA channel per data channel"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut W {
        self.variant(RXDMASELECT_A::MULTIPLE)
    }
}
#[doc = "Field `RXLOOP` reader - Loop-back Test Mode"]
pub type RXLOOP_R = crate::BitReader<RXLOOPSELECT_A>;
#[doc = "Loop-back Test Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXLOOPSELECT_A {
    #[doc = "0: Normal mode"]
    OFF = 0,
    #[doc = "1: ISDO internally connected to ISDI"]
    ON = 1,
}
impl From<RXLOOPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RXLOOPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXLOOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXLOOPSELECT_A {
        match self.bits {
            false => RXLOOPSELECT_A::OFF,
            true => RXLOOPSELECT_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == RXLOOPSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == RXLOOPSELECT_A::ON
    }
}
#[doc = "Field `RXLOOP` writer - Loop-back Test Mode"]
pub type RXLOOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, RXLOOPSELECT_A, O>;
impl<'a, const O: u8> RXLOOP_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(RXLOOPSELECT_A::OFF)
    }
    #[doc = "ISDO internally connected to ISDI"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(RXLOOPSELECT_A::ON)
    }
}
#[doc = "Field `TXMONO` reader - Transmitter Mono"]
pub type TXMONO_R = crate::BitReader<TXMONOSELECT_A>;
#[doc = "Transmitter Mono\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXMONOSELECT_A {
    #[doc = "0: Normal mode"]
    STEREO = 0,
    #[doc = "1: Left channel data is duplicated to right channel"]
    MONO = 1,
}
impl From<TXMONOSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXMONOSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXMONO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXMONOSELECT_A {
        match self.bits {
            false => TXMONOSELECT_A::STEREO,
            true => TXMONOSELECT_A::MONO,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == TXMONOSELECT_A::STEREO
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == TXMONOSELECT_A::MONO
    }
}
#[doc = "Field `TXMONO` writer - Transmitter Mono"]
pub type TXMONO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, TXMONOSELECT_A, O>;
impl<'a, const O: u8> TXMONO_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(TXMONOSELECT_A::STEREO)
    }
    #[doc = "Left channel data is duplicated to right channel"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut W {
        self.variant(TXMONOSELECT_A::MONO)
    }
}
#[doc = "Field `TXDMA` reader - Single or Multiple DMA Channels for Transmitter"]
pub type TXDMA_R = crate::BitReader<TXDMASELECT_A>;
#[doc = "Single or Multiple DMA Channels for Transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMASELECT_A {
    #[doc = "0: Single DMA channel"]
    SINGLE = 0,
    #[doc = "1: One DMA channel per data channel"]
    MULTIPLE = 1,
}
impl From<TXDMASELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMASELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMASELECT_A {
        match self.bits {
            false => TXDMASELECT_A::SINGLE,
            true => TXDMASELECT_A::MULTIPLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == TXDMASELECT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == TXDMASELECT_A::MULTIPLE
    }
}
#[doc = "Field `TXDMA` writer - Single or Multiple DMA Channels for Transmitter"]
pub type TXDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, TXDMASELECT_A, O>;
impl<'a, const O: u8> TXDMA_W<'a, O> {
    #[doc = "Single DMA channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(TXDMASELECT_A::SINGLE)
    }
    #[doc = "One DMA channel per data channel"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut W {
        self.variant(TXDMASELECT_A::MULTIPLE)
    }
}
#[doc = "Field `TXSAME` reader - Transmit Data when Underrun"]
pub type TXSAME_R = crate::BitReader<TXSAMESELECT_A>;
#[doc = "Transmit Data when Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXSAMESELECT_A {
    #[doc = "0: Zero data transmitted in case of underrun"]
    ZERO = 0,
    #[doc = "1: Last data transmitted in case of underrun"]
    SAME = 1,
}
impl From<TXSAMESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TXSAMESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXSAME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSAMESELECT_A {
        match self.bits {
            false => TXSAMESELECT_A::ZERO,
            true => TXSAMESELECT_A::SAME,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == TXSAMESELECT_A::ZERO
    }
    #[doc = "Checks if the value of the field is `SAME`"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        *self == TXSAMESELECT_A::SAME
    }
}
#[doc = "Field `TXSAME` writer - Transmit Data when Underrun"]
pub type TXSAME_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, TXSAMESELECT_A, O>;
impl<'a, const O: u8> TXSAME_W<'a, O> {
    #[doc = "Zero data transmitted in case of underrun"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TXSAMESELECT_A::ZERO)
    }
    #[doc = "Last data transmitted in case of underrun"]
    #[inline(always)]
    pub fn same(self) -> &'a mut W {
        self.variant(TXSAMESELECT_A::SAME)
    }
}
#[doc = "Field `IMCKFS` reader - Master Clock to fs Ratio"]
pub type IMCKFS_R = crate::FieldReader<u8, IMCKFSSELECT_A>;
#[doc = "Master Clock to fs Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IMCKFSSELECT_A {
    #[doc = "0: 16 fs"]
    _16 = 0,
    #[doc = "1: 32 fs"]
    _32 = 1,
    #[doc = "3: 64 fs"]
    _64 = 3,
    #[doc = "7: 128 fs"]
    _128 = 7,
    #[doc = "15: 256 fs"]
    _256 = 15,
    #[doc = "23: 384 fs"]
    _384 = 23,
    #[doc = "31: 512 fs"]
    _512 = 31,
    #[doc = "47: 768 fs"]
    _768 = 47,
    #[doc = "63: 1024 fs"]
    _1024 = 63,
}
impl From<IMCKFSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: IMCKFSSELECT_A) -> Self {
        variant as _
    }
}
impl IMCKFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IMCKFSSELECT_A> {
        match self.bits {
            0 => Some(IMCKFSSELECT_A::_16),
            1 => Some(IMCKFSSELECT_A::_32),
            3 => Some(IMCKFSSELECT_A::_64),
            7 => Some(IMCKFSSELECT_A::_128),
            15 => Some(IMCKFSSELECT_A::_256),
            23 => Some(IMCKFSSELECT_A::_384),
            31 => Some(IMCKFSSELECT_A::_512),
            47 => Some(IMCKFSSELECT_A::_768),
            63 => Some(IMCKFSSELECT_A::_1024),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == IMCKFSSELECT_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == IMCKFSSELECT_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == IMCKFSSELECT_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        *self == IMCKFSSELECT_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        *self == IMCKFSSELECT_A::_256
    }
    #[doc = "Checks if the value of the field is `_384`"]
    #[inline(always)]
    pub fn is_384(&self) -> bool {
        *self == IMCKFSSELECT_A::_384
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == IMCKFSSELECT_A::_512
    }
    #[doc = "Checks if the value of the field is `_768`"]
    #[inline(always)]
    pub fn is_768(&self) -> bool {
        *self == IMCKFSSELECT_A::_768
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == IMCKFSSELECT_A::_1024
    }
}
#[doc = "Field `IMCKFS` writer - Master Clock to fs Ratio"]
pub type IMCKFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, IMCKFSSELECT_A, 6, O>;
impl<'a, const O: u8> IMCKFS_W<'a, O> {
    #[doc = "16 fs"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(IMCKFSSELECT_A::_16)
    }
    #[doc = "32 fs"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(IMCKFSSELECT_A::_32)
    }
    #[doc = "64 fs"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(IMCKFSSELECT_A::_64)
    }
    #[doc = "128 fs"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(IMCKFSSELECT_A::_128)
    }
    #[doc = "256 fs"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(IMCKFSSELECT_A::_256)
    }
    #[doc = "384 fs"]
    #[inline(always)]
    pub fn _384(self) -> &'a mut W {
        self.variant(IMCKFSSELECT_A::_384)
    }
    #[doc = "512 fs"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(IMCKFSSELECT_A::_512)
    }
    #[doc = "768 fs"]
    #[inline(always)]
    pub fn _768(self) -> &'a mut W {
        self.variant(IMCKFSSELECT_A::_768)
    }
    #[doc = "1024 fs"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(IMCKFSSELECT_A::_1024)
    }
}
#[doc = "Field `IMCKMODE` reader - Master Clock Mode"]
pub type IMCKMODE_R = crate::BitReader<IMCKMODESELECT_A>;
#[doc = "Master Clock Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IMCKMODESELECT_A {
    #[doc = "0: No IMCK generated"]
    NO_IMCK = 0,
    #[doc = "1: IMCK generated"]
    IMCK = 1,
}
impl From<IMCKMODESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: IMCKMODESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl IMCKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMCKMODESELECT_A {
        match self.bits {
            false => IMCKMODESELECT_A::NO_IMCK,
            true => IMCKMODESELECT_A::IMCK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_IMCK`"]
    #[inline(always)]
    pub fn is_no_imck(&self) -> bool {
        *self == IMCKMODESELECT_A::NO_IMCK
    }
    #[doc = "Checks if the value of the field is `IMCK`"]
    #[inline(always)]
    pub fn is_imck(&self) -> bool {
        *self == IMCKMODESELECT_A::IMCK
    }
}
#[doc = "Field `IMCKMODE` writer - Master Clock Mode"]
pub type IMCKMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, IMCKMODESELECT_A, O>;
impl<'a, const O: u8> IMCKMODE_W<'a, O> {
    #[doc = "No IMCK generated"]
    #[inline(always)]
    pub fn no_imck(self) -> &'a mut W {
        self.variant(IMCKMODESELECT_A::NO_IMCK)
    }
    #[doc = "IMCK generated"]
    #[inline(always)]
    pub fn imck(self) -> &'a mut W {
        self.variant(IMCKMODESELECT_A::IMCK)
    }
}
#[doc = "Field `IWS24` reader - IWS Data Slot Width"]
pub type IWS24_R = crate::BitReader<IWS24SELECT_A>;
#[doc = "IWS Data Slot Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IWS24SELECT_A {
    #[doc = "0: IWS Data Slot is 32-bit wide for DATALENGTH=18/20/24-bit"]
    _32 = 0,
    #[doc = "1: IWS Data Slot is 24-bit wide for DATALENGTH=18/20/24-bit"]
    _24 = 1,
}
impl From<IWS24SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: IWS24SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl IWS24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWS24SELECT_A {
        match self.bits {
            false => IWS24SELECT_A::_32,
            true => IWS24SELECT_A::_24,
        }
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == IWS24SELECT_A::_32
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == IWS24SELECT_A::_24
    }
}
#[doc = "Field `IWS24` writer - IWS Data Slot Width"]
pub type IWS24_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, IWS24SELECT_A, O>;
impl<'a, const O: u8> IWS24_W<'a, O> {
    #[doc = "IWS Data Slot is 32-bit wide for DATALENGTH=18/20/24-bit"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(IWS24SELECT_A::_32)
    }
    #[doc = "IWS Data Slot is 24-bit wide for DATALENGTH=18/20/24-bit"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(IWS24SELECT_A::_24)
    }
}
impl R {
    #[doc = "Bit 0 - Master/Slave/Controller Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:4 - Data Word Length"]
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 8 - Receiver Mono"]
    #[inline(always)]
    pub fn rxmono(&self) -> RXMONO_R {
        RXMONO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Single or Multiple DMA Channels for Receiver"]
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Loop-back Test Mode"]
    #[inline(always)]
    pub fn rxloop(&self) -> RXLOOP_R {
        RXLOOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmitter Mono"]
    #[inline(always)]
    pub fn txmono(&self) -> TXMONO_R {
        TXMONO_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Single or Multiple DMA Channels for Transmitter"]
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Transmit Data when Underrun"]
    #[inline(always)]
    pub fn txsame(&self) -> TXSAME_R {
        TXSAME_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Master Clock to fs Ratio"]
    #[inline(always)]
    pub fn imckfs(&self) -> IMCKFS_R {
        IMCKFS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Master Clock Mode"]
    #[inline(always)]
    pub fn imckmode(&self) -> IMCKMODE_R {
        IMCKMODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - IWS Data Slot Width"]
    #[inline(always)]
    pub fn iws24(&self) -> IWS24_R {
        IWS24_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master/Slave/Controller Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bits 2:4 - Data Word Length"]
    #[inline(always)]
    #[must_use]
    pub fn datalength(&mut self) -> DATALENGTH_W<2> {
        DATALENGTH_W::new(self)
    }
    #[doc = "Bit 8 - Receiver Mono"]
    #[inline(always)]
    #[must_use]
    pub fn rxmono(&mut self) -> RXMONO_W<8> {
        RXMONO_W::new(self)
    }
    #[doc = "Bit 9 - Single or Multiple DMA Channels for Receiver"]
    #[inline(always)]
    #[must_use]
    pub fn rxdma(&mut self) -> RXDMA_W<9> {
        RXDMA_W::new(self)
    }
    #[doc = "Bit 10 - Loop-back Test Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rxloop(&mut self) -> RXLOOP_W<10> {
        RXLOOP_W::new(self)
    }
    #[doc = "Bit 12 - Transmitter Mono"]
    #[inline(always)]
    #[must_use]
    pub fn txmono(&mut self) -> TXMONO_W<12> {
        TXMONO_W::new(self)
    }
    #[doc = "Bit 13 - Single or Multiple DMA Channels for Transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn txdma(&mut self) -> TXDMA_W<13> {
        TXDMA_W::new(self)
    }
    #[doc = "Bit 14 - Transmit Data when Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn txsame(&mut self) -> TXSAME_W<14> {
        TXSAME_W::new(self)
    }
    #[doc = "Bits 24:29 - Master Clock to fs Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn imckfs(&mut self) -> IMCKFS_W<24> {
        IMCKFS_W::new(self)
    }
    #[doc = "Bit 30 - Master Clock Mode"]
    #[inline(always)]
    #[must_use]
    pub fn imckmode(&mut self) -> IMCKMODE_W<30> {
        IMCKMODE_W::new(self)
    }
    #[doc = "Bit 31 - IWS Data Slot Width"]
    #[inline(always)]
    #[must_use]
    pub fn iws24(&mut self) -> IWS24_W<31> {
        IWS24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
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
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
