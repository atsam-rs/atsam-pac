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
#[doc = "Master/Slave/Controller Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Slave mode (only serial data handled, clocks received from external master or controller)"]
    SLAVE = 0,
    #[doc = "1: Master mode (clocks generated and output by IISC, serial data handled if CR.RXEN and/or CR.TXEN written to 1)"]
    MASTER = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Master/Slave/Controller Mode"]
pub struct MODE_R(crate::FieldReader<bool, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::SLAVE,
            true => MODE_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        **self == MODE_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        **self == MODE_A::MASTER
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Master/Slave/Controller Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Slave mode (only serial data handled, clocks received from external master or controller)"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MODE_A::SLAVE)
    }
    #[doc = "Master mode (clocks generated and output by IISC, serial data handled if CR.RXEN and/or CR.TXEN written to 1)"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MODE_A::MASTER)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Data Word Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATALENGTH_A {
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
impl From<DATALENGTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DATALENGTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DATALENGTH` reader - Data Word Length"]
pub struct DATALENGTH_R(crate::FieldReader<u8, DATALENGTH_A>);
impl DATALENGTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATALENGTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATALENGTH_A {
        match self.bits {
            0 => DATALENGTH_A::_32,
            1 => DATALENGTH_A::_24,
            2 => DATALENGTH_A::_20,
            3 => DATALENGTH_A::_18,
            4 => DATALENGTH_A::_16,
            5 => DATALENGTH_A::_16C,
            6 => DATALENGTH_A::_8,
            7 => DATALENGTH_A::_8C,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        **self == DATALENGTH_A::_32
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        **self == DATALENGTH_A::_24
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        **self == DATALENGTH_A::_20
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        **self == DATALENGTH_A::_18
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == DATALENGTH_A::_16
    }
    #[doc = "Checks if the value of the field is `_16C`"]
    #[inline(always)]
    pub fn is_16c(&self) -> bool {
        **self == DATALENGTH_A::_16C
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == DATALENGTH_A::_8
    }
    #[doc = "Checks if the value of the field is `_8C`"]
    #[inline(always)]
    pub fn is_8c(&self) -> bool {
        **self == DATALENGTH_A::_8C
    }
}
impl core::ops::Deref for DATALENGTH_R {
    type Target = crate::FieldReader<u8, DATALENGTH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATALENGTH` writer - Data Word Length"]
pub struct DATALENGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATALENGTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATALENGTH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_32)
    }
    #[doc = "24 bits"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_24)
    }
    #[doc = "20 bits"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_20)
    }
    #[doc = "18 bits"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_18)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_16)
    }
    #[doc = "16 bits compact stereo"]
    #[inline(always)]
    pub fn _16c(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_16C)
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_8)
    }
    #[doc = "8 bits compact stereo"]
    #[inline(always)]
    pub fn _8c(self) -> &'a mut W {
        self.variant(DATALENGTH_A::_8C)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Receiver Mono\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXMONO_A {
    #[doc = "0: Normal mode"]
    STEREO = 0,
    #[doc = "1: Left channel data is duplicated to right channel"]
    MONO = 1,
}
impl From<RXMONO_A> for bool {
    #[inline(always)]
    fn from(variant: RXMONO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXMONO` reader - Receiver Mono"]
pub struct RXMONO_R(crate::FieldReader<bool, RXMONO_A>);
impl RXMONO_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXMONO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXMONO_A {
        match self.bits {
            false => RXMONO_A::STEREO,
            true => RXMONO_A::MONO,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        **self == RXMONO_A::STEREO
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        **self == RXMONO_A::MONO
    }
}
impl core::ops::Deref for RXMONO_R {
    type Target = crate::FieldReader<bool, RXMONO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMONO` writer - Receiver Mono"]
pub struct RXMONO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMONO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXMONO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(RXMONO_A::STEREO)
    }
    #[doc = "Left channel data is duplicated to right channel"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut W {
        self.variant(RXMONO_A::MONO)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Single or Multiple DMA Channels for Receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXDMA_A {
    #[doc = "0: Single DMA channel"]
    SINGLE = 0,
    #[doc = "1: One DMA channel per data channel"]
    MULTIPLE = 1,
}
impl From<RXDMA_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMA` reader - Single or Multiple DMA Channels for Receiver"]
pub struct RXDMA_R(crate::FieldReader<bool, RXDMA_A>);
impl RXDMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMA_A {
        match self.bits {
            false => RXDMA_A::SINGLE,
            true => RXDMA_A::MULTIPLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == RXDMA_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        **self == RXDMA_A::MULTIPLE
    }
}
impl core::ops::Deref for RXDMA_R {
    type Target = crate::FieldReader<bool, RXDMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDMA` writer - Single or Multiple DMA Channels for Receiver"]
pub struct RXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXDMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single DMA channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(RXDMA_A::SINGLE)
    }
    #[doc = "One DMA channel per data channel"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut W {
        self.variant(RXDMA_A::MULTIPLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Loop-back Test Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLOOP_A {
    #[doc = "0: Normal mode"]
    OFF = 0,
    #[doc = "1: ISDO internally connected to ISDI"]
    ON = 1,
}
impl From<RXLOOP_A> for bool {
    #[inline(always)]
    fn from(variant: RXLOOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXLOOP` reader - Loop-back Test Mode"]
pub struct RXLOOP_R(crate::FieldReader<bool, RXLOOP_A>);
impl RXLOOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXLOOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXLOOP_A {
        match self.bits {
            false => RXLOOP_A::OFF,
            true => RXLOOP_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == RXLOOP_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == RXLOOP_A::ON
    }
}
impl core::ops::Deref for RXLOOP_R {
    type Target = crate::FieldReader<bool, RXLOOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXLOOP` writer - Loop-back Test Mode"]
pub struct RXLOOP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLOOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXLOOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(RXLOOP_A::OFF)
    }
    #[doc = "ISDO internally connected to ISDI"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(RXLOOP_A::ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Transmitter Mono\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXMONO_A {
    #[doc = "0: Normal mode"]
    STEREO = 0,
    #[doc = "1: Left channel data is duplicated to right channel"]
    MONO = 1,
}
impl From<TXMONO_A> for bool {
    #[inline(always)]
    fn from(variant: TXMONO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXMONO` reader - Transmitter Mono"]
pub struct TXMONO_R(crate::FieldReader<bool, TXMONO_A>);
impl TXMONO_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXMONO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXMONO_A {
        match self.bits {
            false => TXMONO_A::STEREO,
            true => TXMONO_A::MONO,
        }
    }
    #[doc = "Checks if the value of the field is `STEREO`"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        **self == TXMONO_A::STEREO
    }
    #[doc = "Checks if the value of the field is `MONO`"]
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        **self == TXMONO_A::MONO
    }
}
impl core::ops::Deref for TXMONO_R {
    type Target = crate::FieldReader<bool, TXMONO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMONO` writer - Transmitter Mono"]
pub struct TXMONO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMONO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXMONO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(TXMONO_A::STEREO)
    }
    #[doc = "Left channel data is duplicated to right channel"]
    #[inline(always)]
    pub fn mono(self) -> &'a mut W {
        self.variant(TXMONO_A::MONO)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Single or Multiple DMA Channels for Transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXDMA_A {
    #[doc = "0: Single DMA channel"]
    SINGLE = 0,
    #[doc = "1: One DMA channel per data channel"]
    MULTIPLE = 1,
}
impl From<TXDMA_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMA` reader - Single or Multiple DMA Channels for Transmitter"]
pub struct TXDMA_R(crate::FieldReader<bool, TXDMA_A>);
impl TXDMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXDMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMA_A {
        match self.bits {
            false => TXDMA_A::SINGLE,
            true => TXDMA_A::MULTIPLE,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == TXDMA_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        **self == TXDMA_A::MULTIPLE
    }
}
impl core::ops::Deref for TXDMA_R {
    type Target = crate::FieldReader<bool, TXDMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXDMA` writer - Single or Multiple DMA Channels for Transmitter"]
pub struct TXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXDMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Single DMA channel"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(TXDMA_A::SINGLE)
    }
    #[doc = "One DMA channel per data channel"]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut W {
        self.variant(TXDMA_A::MULTIPLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Transmit Data when Underrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXSAME_A {
    #[doc = "0: Zero data transmitted in case of underrun"]
    ZERO = 0,
    #[doc = "1: Last data transmitted in case of underrun"]
    SAME = 1,
}
impl From<TXSAME_A> for bool {
    #[inline(always)]
    fn from(variant: TXSAME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXSAME` reader - Transmit Data when Underrun"]
pub struct TXSAME_R(crate::FieldReader<bool, TXSAME_A>);
impl TXSAME_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXSAME_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXSAME_A {
        match self.bits {
            false => TXSAME_A::ZERO,
            true => TXSAME_A::SAME,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == TXSAME_A::ZERO
    }
    #[doc = "Checks if the value of the field is `SAME`"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        **self == TXSAME_A::SAME
    }
}
impl core::ops::Deref for TXSAME_R {
    type Target = crate::FieldReader<bool, TXSAME_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXSAME` writer - Transmit Data when Underrun"]
pub struct TXSAME_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSAME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXSAME_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Zero data transmitted in case of underrun"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(TXSAME_A::ZERO)
    }
    #[doc = "Last data transmitted in case of underrun"]
    #[inline(always)]
    pub fn same(self) -> &'a mut W {
        self.variant(TXSAME_A::SAME)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Master Clock to fs Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IMCKFS_A {
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
impl From<IMCKFS_A> for u8 {
    #[inline(always)]
    fn from(variant: IMCKFS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IMCKFS` reader - Master Clock to fs Ratio"]
pub struct IMCKFS_R(crate::FieldReader<u8, IMCKFS_A>);
impl IMCKFS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IMCKFS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IMCKFS_A> {
        match self.bits {
            0 => Some(IMCKFS_A::_16),
            1 => Some(IMCKFS_A::_32),
            3 => Some(IMCKFS_A::_64),
            7 => Some(IMCKFS_A::_128),
            15 => Some(IMCKFS_A::_256),
            23 => Some(IMCKFS_A::_384),
            31 => Some(IMCKFS_A::_512),
            47 => Some(IMCKFS_A::_768),
            63 => Some(IMCKFS_A::_1024),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == IMCKFS_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        **self == IMCKFS_A::_32
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        **self == IMCKFS_A::_64
    }
    #[doc = "Checks if the value of the field is `_128`"]
    #[inline(always)]
    pub fn is_128(&self) -> bool {
        **self == IMCKFS_A::_128
    }
    #[doc = "Checks if the value of the field is `_256`"]
    #[inline(always)]
    pub fn is_256(&self) -> bool {
        **self == IMCKFS_A::_256
    }
    #[doc = "Checks if the value of the field is `_384`"]
    #[inline(always)]
    pub fn is_384(&self) -> bool {
        **self == IMCKFS_A::_384
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        **self == IMCKFS_A::_512
    }
    #[doc = "Checks if the value of the field is `_768`"]
    #[inline(always)]
    pub fn is_768(&self) -> bool {
        **self == IMCKFS_A::_768
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        **self == IMCKFS_A::_1024
    }
}
impl core::ops::Deref for IMCKFS_R {
    type Target = crate::FieldReader<u8, IMCKFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMCKFS` writer - Master Clock to fs Ratio"]
pub struct IMCKFS_W<'a> {
    w: &'a mut W,
}
impl<'a> IMCKFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMCKFS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "16 fs"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(IMCKFS_A::_16)
    }
    #[doc = "32 fs"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(IMCKFS_A::_32)
    }
    #[doc = "64 fs"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(IMCKFS_A::_64)
    }
    #[doc = "128 fs"]
    #[inline(always)]
    pub fn _128(self) -> &'a mut W {
        self.variant(IMCKFS_A::_128)
    }
    #[doc = "256 fs"]
    #[inline(always)]
    pub fn _256(self) -> &'a mut W {
        self.variant(IMCKFS_A::_256)
    }
    #[doc = "384 fs"]
    #[inline(always)]
    pub fn _384(self) -> &'a mut W {
        self.variant(IMCKFS_A::_384)
    }
    #[doc = "512 fs"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(IMCKFS_A::_512)
    }
    #[doc = "768 fs"]
    #[inline(always)]
    pub fn _768(self) -> &'a mut W {
        self.variant(IMCKFS_A::_768)
    }
    #[doc = "1024 fs"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(IMCKFS_A::_1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "Master Clock Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IMCKMODE_A {
    #[doc = "0: No IMCK generated"]
    NO_IMCK = 0,
    #[doc = "1: IMCK generated"]
    IMCK = 1,
}
impl From<IMCKMODE_A> for bool {
    #[inline(always)]
    fn from(variant: IMCKMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMCKMODE` reader - Master Clock Mode"]
pub struct IMCKMODE_R(crate::FieldReader<bool, IMCKMODE_A>);
impl IMCKMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IMCKMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IMCKMODE_A {
        match self.bits {
            false => IMCKMODE_A::NO_IMCK,
            true => IMCKMODE_A::IMCK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_IMCK`"]
    #[inline(always)]
    pub fn is_no_imck(&self) -> bool {
        **self == IMCKMODE_A::NO_IMCK
    }
    #[doc = "Checks if the value of the field is `IMCK`"]
    #[inline(always)]
    pub fn is_imck(&self) -> bool {
        **self == IMCKMODE_A::IMCK
    }
}
impl core::ops::Deref for IMCKMODE_R {
    type Target = crate::FieldReader<bool, IMCKMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IMCKMODE` writer - Master Clock Mode"]
pub struct IMCKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IMCKMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IMCKMODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No IMCK generated"]
    #[inline(always)]
    pub fn no_imck(self) -> &'a mut W {
        self.variant(IMCKMODE_A::NO_IMCK)
    }
    #[doc = "IMCK generated"]
    #[inline(always)]
    pub fn imck(self) -> &'a mut W {
        self.variant(IMCKMODE_A::IMCK)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "IWS Data Slot Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IWS24_A {
    #[doc = "0: IWS Data Slot is 32-bit wide for DATALENGTH=18/20/24-bit"]
    _32 = 0,
    #[doc = "1: IWS Data Slot is 24-bit wide for DATALENGTH=18/20/24-bit"]
    _24 = 1,
}
impl From<IWS24_A> for bool {
    #[inline(always)]
    fn from(variant: IWS24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IWS24` reader - IWS Data Slot Width"]
pub struct IWS24_R(crate::FieldReader<bool, IWS24_A>);
impl IWS24_R {
    pub(crate) fn new(bits: bool) -> Self {
        IWS24_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IWS24_A {
        match self.bits {
            false => IWS24_A::_32,
            true => IWS24_A::_24,
        }
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        **self == IWS24_A::_32
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        **self == IWS24_A::_24
    }
}
impl core::ops::Deref for IWS24_R {
    type Target = crate::FieldReader<bool, IWS24_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IWS24` writer - IWS Data Slot Width"]
pub struct IWS24_W<'a> {
    w: &'a mut W,
}
impl<'a> IWS24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IWS24_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IWS Data Slot is 32-bit wide for DATALENGTH=18/20/24-bit"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(IWS24_A::_32)
    }
    #[doc = "IWS Data Slot is 24-bit wide for DATALENGTH=18/20/24-bit"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(IWS24_A::_24)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Master/Slave/Controller Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Data Word Length"]
    #[inline(always)]
    pub fn datalength(&self) -> DATALENGTH_R {
        DATALENGTH_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Receiver Mono"]
    #[inline(always)]
    pub fn rxmono(&self) -> RXMONO_R {
        RXMONO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Single or Multiple DMA Channels for Receiver"]
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Loop-back Test Mode"]
    #[inline(always)]
    pub fn rxloop(&self) -> RXLOOP_R {
        RXLOOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Transmitter Mono"]
    #[inline(always)]
    pub fn txmono(&self) -> TXMONO_R {
        TXMONO_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Single or Multiple DMA Channels for Transmitter"]
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Transmit Data when Underrun"]
    #[inline(always)]
    pub fn txsame(&self) -> TXSAME_R {
        TXSAME_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - Master Clock to fs Ratio"]
    #[inline(always)]
    pub fn imckfs(&self) -> IMCKFS_R {
        IMCKFS_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Master Clock Mode"]
    #[inline(always)]
    pub fn imckmode(&self) -> IMCKMODE_R {
        IMCKMODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - IWS Data Slot Width"]
    #[inline(always)]
    pub fn iws24(&self) -> IWS24_R {
        IWS24_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master/Slave/Controller Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 2:4 - Data Word Length"]
    #[inline(always)]
    pub fn datalength(&mut self) -> DATALENGTH_W {
        DATALENGTH_W { w: self }
    }
    #[doc = "Bit 8 - Receiver Mono"]
    #[inline(always)]
    pub fn rxmono(&mut self) -> RXMONO_W {
        RXMONO_W { w: self }
    }
    #[doc = "Bit 9 - Single or Multiple DMA Channels for Receiver"]
    #[inline(always)]
    pub fn rxdma(&mut self) -> RXDMA_W {
        RXDMA_W { w: self }
    }
    #[doc = "Bit 10 - Loop-back Test Mode"]
    #[inline(always)]
    pub fn rxloop(&mut self) -> RXLOOP_W {
        RXLOOP_W { w: self }
    }
    #[doc = "Bit 12 - Transmitter Mono"]
    #[inline(always)]
    pub fn txmono(&mut self) -> TXMONO_W {
        TXMONO_W { w: self }
    }
    #[doc = "Bit 13 - Single or Multiple DMA Channels for Transmitter"]
    #[inline(always)]
    pub fn txdma(&mut self) -> TXDMA_W {
        TXDMA_W { w: self }
    }
    #[doc = "Bit 14 - Transmit Data when Underrun"]
    #[inline(always)]
    pub fn txsame(&mut self) -> TXSAME_W {
        TXSAME_W { w: self }
    }
    #[doc = "Bits 24:29 - Master Clock to fs Ratio"]
    #[inline(always)]
    pub fn imckfs(&mut self) -> IMCKFS_W {
        IMCKFS_W { w: self }
    }
    #[doc = "Bit 30 - Master Clock Mode"]
    #[inline(always)]
    pub fn imckmode(&mut self) -> IMCKMODE_W {
        IMCKMODE_W { w: self }
    }
    #[doc = "Bit 31 - IWS Data Slot Width"]
    #[inline(always)]
    pub fn iws24(&mut self) -> IWS24_W {
        IWS24_W { w: self }
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
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
