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
#[doc = "Master/Slave Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MSTR_A {
    #[doc = "0: SPI is in Slave mode."]
    _0 = 0,
    #[doc = "1: SPI is in Master mode."]
    _1 = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTR` reader - Master/Slave Mode"]
pub struct MSTR_R(crate::FieldReader<bool, MSTR_A>);
impl MSTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSTR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::_0,
            true => MSTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MSTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MSTR_A::_1
    }
}
impl core::ops::Deref for MSTR_R {
    type Target = crate::FieldReader<bool, MSTR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSTR` writer - Master/Slave Mode"]
pub struct MSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MSTR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MSTR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SPI is in Slave mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTR_A::_0)
    }
    #[doc = "SPI is in Master mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTR_A::_1)
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
#[doc = "Peripheral Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PS_A {
    #[doc = "0: Fixed Peripheral Select."]
    _0 = 0,
    #[doc = "1: Variable Peripheral Select."]
    _1 = 1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS` reader - Peripheral Select"]
pub struct PS_R(crate::FieldReader<bool, PS_A>);
impl PS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::_0,
            true => PS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PS_A::_1
    }
}
impl core::ops::Deref for PS_R {
    type Target = crate::FieldReader<bool, PS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS` writer - Peripheral Select"]
pub struct PS_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Fixed Peripheral Select."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PS_A::_0)
    }
    #[doc = "Variable Peripheral Select."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Chip Select Decode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCSDEC_A {
    #[doc = "0: The chip selects are directly connected to a peripheral device."]
    _0 = 0,
    #[doc = "1: The four chip select lines are connected to a 4- to 16-bit decoder.When PCSDEC equals one, up to 15 Chip Select signals can be generated with the four lines using an external 4- to 16-bitdecoder. The Chip Select Registers define the characteristics of the 16 chip selects according to the following rules:CSR0 defines peripheral chip select signals 0 to 3.CSR1 defines peripheral chip select signals 4 to 7.CSR2 defines peripheral chip select signals 8 to 11.CSR3 defines peripheral chip select signals 12 to 15."]
    _1 = 1,
}
impl From<PCSDEC_A> for bool {
    #[inline(always)]
    fn from(variant: PCSDEC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCSDEC` reader - Chip Select Decode"]
pub struct PCSDEC_R(crate::FieldReader<bool, PCSDEC_A>);
impl PCSDEC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCSDEC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSDEC_A {
        match self.bits {
            false => PCSDEC_A::_0,
            true => PCSDEC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PCSDEC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PCSDEC_A::_1
    }
}
impl core::ops::Deref for PCSDEC_R {
    type Target = crate::FieldReader<bool, PCSDEC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCSDEC` writer - Chip Select Decode"]
pub struct PCSDEC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCSDEC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCSDEC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The chip selects are directly connected to a peripheral device."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSDEC_A::_0)
    }
    #[doc = "The four chip select lines are connected to a 4- to 16-bit decoder.When PCSDEC equals one, up to 15 Chip Select signals can be generated with the four lines using an external 4- to 16-bitdecoder. The Chip Select Registers define the characteristics of the 16 chip selects according to the following rules:CSR0 defines peripheral chip select signals 0 to 3.CSR1 defines peripheral chip select signals 4 to 7.CSR2 defines peripheral chip select signals 8 to 11.CSR3 defines peripheral chip select signals 12 to 15."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSDEC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Mode Fault Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODFDIS_A {
    #[doc = "0: Mode fault detection is enabled."]
    _0 = 0,
    #[doc = "1: Mode fault detection is disabled."]
    _1 = 1,
}
impl From<MODFDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MODFDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODFDIS` reader - Mode Fault Detection"]
pub struct MODFDIS_R(crate::FieldReader<bool, MODFDIS_A>);
impl MODFDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODFDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODFDIS_A {
        match self.bits {
            false => MODFDIS_A::_0,
            true => MODFDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MODFDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MODFDIS_A::_1
    }
}
impl core::ops::Deref for MODFDIS_R {
    type Target = crate::FieldReader<bool, MODFDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODFDIS` writer - Mode Fault Detection"]
pub struct MODFDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> MODFDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODFDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Mode fault detection is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODFDIS_A::_0)
    }
    #[doc = "Mode fault detection is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODFDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `WDRBT` reader - wait data read before transfer"]
pub struct WDRBT_R(crate::FieldReader<bool, bool>);
impl WDRBT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDRBT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDRBT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDRBT` writer - wait data read before transfer"]
pub struct WDRBT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDRBT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RXFIFOEN` reader - FIFO in Reception Enable"]
pub struct RXFIFOEN_R(crate::FieldReader<bool, bool>);
impl RXFIFOEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFIFOEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFIFOEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFIFOEN` writer - FIFO in Reception Enable"]
pub struct RXFIFOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Local Loopback Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LLB_A {
    #[doc = "0: Local loopback path disabled."]
    _0 = 0,
    #[doc = "1: Local loopback path enabled.LLB controls the local loopback on the data serializer for testing in Master Mode only."]
    _1 = 1,
}
impl From<LLB_A> for bool {
    #[inline(always)]
    fn from(variant: LLB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LLB` reader - Local Loopback Enable"]
pub struct LLB_R(crate::FieldReader<bool, LLB_A>);
impl LLB_R {
    pub(crate) fn new(bits: bool) -> Self {
        LLB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LLB_A {
        match self.bits {
            false => LLB_A::_0,
            true => LLB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == LLB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LLB_A::_1
    }
}
impl core::ops::Deref for LLB_R {
    type Target = crate::FieldReader<bool, LLB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LLB` writer - Local Loopback Enable"]
pub struct LLB_W<'a> {
    w: &'a mut W,
}
impl<'a> LLB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LLB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Local loopback path disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LLB_A::_0)
    }
    #[doc = "Local loopback path enabled.LLB controls the local loopback on the data serializer for testing in Master Mode only."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LLB_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub struct PCS_R(crate::FieldReader<u8, u8>);
impl PCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        PCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub struct PCS_W<'a> {
    w: &'a mut W,
}
impl<'a> PCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `DLYBCS` reader - Delay Between Chip Selects"]
pub struct DLYBCS_R(crate::FieldReader<u8, u8>);
impl DLYBCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLYBCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYBCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYBCS` writer - Delay Between Chip Selects"]
pub struct DLYBCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYBCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&self) -> PCSDEC_R {
        PCSDEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&self) -> MODFDIS_R {
        MODFDIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - wait data read before transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WDRBT_R {
        WDRBT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FIFO in Reception Enable"]
    #[inline(always)]
    pub fn rxfifoen(&self) -> RXFIFOEN_R {
        RXFIFOEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LLB_R {
        LLB_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&self) -> DLYBCS_R {
        DLYBCS_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W {
        MSTR_W { w: self }
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W {
        PS_W { w: self }
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&mut self) -> PCSDEC_W {
        PCSDEC_W { w: self }
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&mut self) -> MODFDIS_W {
        MODFDIS_W { w: self }
    }
    #[doc = "Bit 5 - wait data read before transfer"]
    #[inline(always)]
    pub fn wdrbt(&mut self) -> WDRBT_W {
        WDRBT_W { w: self }
    }
    #[doc = "Bit 6 - FIFO in Reception Enable"]
    #[inline(always)]
    pub fn rxfifoen(&mut self) -> RXFIFOEN_W {
        RXFIFOEN_W { w: self }
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&mut self) -> LLB_W {
        LLB_W { w: self }
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    pub fn pcs(&mut self) -> PCS_W {
        PCS_W { w: self }
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    pub fn dlybcs(&mut self) -> DLYBCS_W {
        DLYBCS_W { w: self }
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
