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
#[doc = "Field `MSTR` reader - Master/Slave Mode"]
pub type MSTR_R = crate::BitReader<MSTRSELECT_A>;
#[doc = "Master/Slave Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTRSELECT_A {
    #[doc = "0: SPI is in Slave mode."]
    _0 = 0,
    #[doc = "1: SPI is in Master mode."]
    _1 = 1,
}
impl From<MSTRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MSTRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTRSELECT_A {
        match self.bits {
            false => MSTRSELECT_A::_0,
            true => MSTRSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSTRSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSTRSELECT_A::_1
    }
}
#[doc = "Field `MSTR` writer - Master/Slave Mode"]
pub type MSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, MSTRSELECT_A, O>;
impl<'a, const O: u8> MSTR_W<'a, O> {
    #[doc = "SPI is in Slave mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSTRSELECT_A::_0)
    }
    #[doc = "SPI is in Master mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSTRSELECT_A::_1)
    }
}
#[doc = "Field `PS` reader - Peripheral Select"]
pub type PS_R = crate::BitReader<PSSELECT_A>;
#[doc = "Peripheral Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSSELECT_A {
    #[doc = "0: Fixed Peripheral Select."]
    _0 = 0,
    #[doc = "1: Variable Peripheral Select."]
    _1 = 1,
}
impl From<PSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSSELECT_A {
        match self.bits {
            false => PSSELECT_A::_0,
            true => PSSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PSSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PSSELECT_A::_1
    }
}
#[doc = "Field `PS` writer - Peripheral Select"]
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, PSSELECT_A, O>;
impl<'a, const O: u8> PS_W<'a, O> {
    #[doc = "Fixed Peripheral Select."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PSSELECT_A::_0)
    }
    #[doc = "Variable Peripheral Select."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PSSELECT_A::_1)
    }
}
#[doc = "Field `PCSDEC` reader - Chip Select Decode"]
pub type PCSDEC_R = crate::BitReader<PCSDECSELECT_A>;
#[doc = "Chip Select Decode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCSDECSELECT_A {
    #[doc = "0: The chip selects are directly connected to a peripheral device."]
    _0 = 0,
    #[doc = "1: The four chip select lines are connected to a 4- to 16-bit decoder.When PCSDEC equals one, up to 15 Chip Select signals can be generated with the four lines using an external 4- to 16-bitdecoder. The Chip Select Registers define the characteristics of the 16 chip selects according to the following rules:CSR0 defines peripheral chip select signals 0 to 3.CSR1 defines peripheral chip select signals 4 to 7.CSR2 defines peripheral chip select signals 8 to 11.CSR3 defines peripheral chip select signals 12 to 15."]
    _1 = 1,
}
impl From<PCSDECSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PCSDECSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PCSDEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCSDECSELECT_A {
        match self.bits {
            false => PCSDECSELECT_A::_0,
            true => PCSDECSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCSDECSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCSDECSELECT_A::_1
    }
}
#[doc = "Field `PCSDEC` writer - Chip Select Decode"]
pub type PCSDEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, PCSDECSELECT_A, O>;
impl<'a, const O: u8> PCSDEC_W<'a, O> {
    #[doc = "The chip selects are directly connected to a peripheral device."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCSDECSELECT_A::_0)
    }
    #[doc = "The four chip select lines are connected to a 4- to 16-bit decoder.When PCSDEC equals one, up to 15 Chip Select signals can be generated with the four lines using an external 4- to 16-bitdecoder. The Chip Select Registers define the characteristics of the 16 chip selects according to the following rules:CSR0 defines peripheral chip select signals 0 to 3.CSR1 defines peripheral chip select signals 4 to 7.CSR2 defines peripheral chip select signals 8 to 11.CSR3 defines peripheral chip select signals 12 to 15."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCSDECSELECT_A::_1)
    }
}
#[doc = "Field `MODFDIS` reader - Mode Fault Detection"]
pub type MODFDIS_R = crate::BitReader<MODFDISSELECT_A>;
#[doc = "Mode Fault Detection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFDISSELECT_A {
    #[doc = "0: Mode fault detection is enabled."]
    _0 = 0,
    #[doc = "1: Mode fault detection is disabled."]
    _1 = 1,
}
impl From<MODFDISSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MODFDISSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl MODFDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODFDISSELECT_A {
        match self.bits {
            false => MODFDISSELECT_A::_0,
            true => MODFDISSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MODFDISSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MODFDISSELECT_A::_1
    }
}
#[doc = "Field `MODFDIS` writer - Mode Fault Detection"]
pub type MODFDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, MODFDISSELECT_A, O>;
impl<'a, const O: u8> MODFDIS_W<'a, O> {
    #[doc = "Mode fault detection is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MODFDISSELECT_A::_0)
    }
    #[doc = "Mode fault detection is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MODFDISSELECT_A::_1)
    }
}
#[doc = "Field `WDRBT` reader - wait data read before transfer"]
pub type WDRBT_R = crate::BitReader<bool>;
#[doc = "Field `WDRBT` writer - wait data read before transfer"]
pub type WDRBT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `RXFIFOEN` reader - FIFO in Reception Enable"]
pub type RXFIFOEN_R = crate::BitReader<bool>;
#[doc = "Field `RXFIFOEN` writer - FIFO in Reception Enable"]
pub type RXFIFOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `LLB` reader - Local Loopback Enable"]
pub type LLB_R = crate::BitReader<LLBSELECT_A>;
#[doc = "Local Loopback Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LLBSELECT_A {
    #[doc = "0: Local loopback path disabled."]
    _0 = 0,
    #[doc = "1: Local loopback path enabled.LLB controls the local loopback on the data serializer for testing in Master Mode only."]
    _1 = 1,
}
impl From<LLBSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LLBSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LLB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LLBSELECT_A {
        match self.bits {
            false => LLBSELECT_A::_0,
            true => LLBSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LLBSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LLBSELECT_A::_1
    }
}
#[doc = "Field `LLB` writer - Local Loopback Enable"]
pub type LLB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, LLBSELECT_A, O>;
impl<'a, const O: u8> LLB_W<'a, O> {
    #[doc = "Local loopback path disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LLBSELECT_A::_0)
    }
    #[doc = "Local loopback path enabled.LLB controls the local loopback on the data serializer for testing in Master Mode only."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LLBSELECT_A::_1)
    }
}
#[doc = "Field `PCS` reader - Peripheral Chip Select"]
pub type PCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub type PCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 4, O>;
#[doc = "Field `DLYBCS` reader - Delay Between Chip Selects"]
pub type DLYBCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYBCS` writer - Delay Between Chip Selects"]
pub type DLYBCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Master/Slave Mode"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    pub fn pcsdec(&self) -> PCSDEC_R {
        PCSDEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    pub fn modfdis(&self) -> MODFDIS_R {
        MODFDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - wait data read before transfer"]
    #[inline(always)]
    pub fn wdrbt(&self) -> WDRBT_R {
        WDRBT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FIFO in Reception Enable"]
    #[inline(always)]
    pub fn rxfifoen(&self) -> RXFIFOEN_R {
        RXFIFOEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    pub fn llb(&self) -> LLB_R {
        LLB_R::new(((self.bits >> 7) & 1) != 0)
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
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<0> {
        MSTR_W::new(self)
    }
    #[doc = "Bit 1 - Peripheral Select"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<1> {
        PS_W::new(self)
    }
    #[doc = "Bit 2 - Chip Select Decode"]
    #[inline(always)]
    #[must_use]
    pub fn pcsdec(&mut self) -> PCSDEC_W<2> {
        PCSDEC_W::new(self)
    }
    #[doc = "Bit 4 - Mode Fault Detection"]
    #[inline(always)]
    #[must_use]
    pub fn modfdis(&mut self) -> MODFDIS_W<4> {
        MODFDIS_W::new(self)
    }
    #[doc = "Bit 5 - wait data read before transfer"]
    #[inline(always)]
    #[must_use]
    pub fn wdrbt(&mut self) -> WDRBT_W<5> {
        WDRBT_W::new(self)
    }
    #[doc = "Bit 6 - FIFO in Reception Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifoen(&mut self) -> RXFIFOEN_W<6> {
        RXFIFOEN_W::new(self)
    }
    #[doc = "Bit 7 - Local Loopback Enable"]
    #[inline(always)]
    #[must_use]
    pub fn llb(&mut self) -> LLB_W<7> {
        LLB_W::new(self)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcs(&mut self) -> PCS_W<16> {
        PCS_W::new(self)
    }
    #[doc = "Bits 24:31 - Delay Between Chip Selects"]
    #[inline(always)]
    #[must_use]
    pub fn dlybcs(&mut self) -> DLYBCS_W<24> {
        DLYBCS_W::new(self)
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
