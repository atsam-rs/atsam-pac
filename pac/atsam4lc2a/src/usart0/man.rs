#[doc = "Register `MAN` reader"]
pub struct R(crate::R<MAN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAN` writer"]
pub struct W(crate::W<MAN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAN_SPEC>;
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
impl From<crate::W<MAN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_PL` reader - Transmitter Preamble Length"]
pub type TX_PL_R = crate::FieldReader<u8, TX_PLSELECT_A>;
#[doc = "Transmitter Preamble Length\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_PLSELECT_A {
    #[doc = "0: The Transmitter Preamble pattern generation is disabled"]
    _0 = 0,
}
impl From<TX_PLSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PLSELECT_A) -> Self {
        variant as _
    }
}
impl TX_PL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_PLSELECT_A> {
        match self.bits {
            0 => Some(TX_PLSELECT_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TX_PLSELECT_A::_0
    }
}
#[doc = "Field `TX_PL` writer - Transmitter Preamble Length"]
pub type TX_PL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAN_SPEC, u8, TX_PLSELECT_A, 4, O>;
impl<'a, const O: u8> TX_PL_W<'a, O> {
    #[doc = "The Transmitter Preamble pattern generation is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_PLSELECT_A::_0)
    }
}
#[doc = "Field `TX_PP` reader - Transmitter Preamble Pattern"]
pub type TX_PP_R = crate::FieldReader<u8, TX_PPSELECT_A>;
#[doc = "Transmitter Preamble Pattern\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_PPSELECT_A {
    #[doc = "0: ALL_ONE"]
    _0 = 0,
    #[doc = "1: ALL_ZERO"]
    _1 = 1,
    #[doc = "2: ZERO_ONE"]
    _2 = 2,
    #[doc = "3: ONE_ZERO"]
    _3 = 3,
}
impl From<TX_PPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_PPSELECT_A) -> Self {
        variant as _
    }
}
impl TX_PP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_PPSELECT_A {
        match self.bits {
            0 => TX_PPSELECT_A::_0,
            1 => TX_PPSELECT_A::_1,
            2 => TX_PPSELECT_A::_2,
            3 => TX_PPSELECT_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TX_PPSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TX_PPSELECT_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == TX_PPSELECT_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == TX_PPSELECT_A::_3
    }
}
#[doc = "Field `TX_PP` writer - Transmitter Preamble Pattern"]
pub type TX_PP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MAN_SPEC, u8, TX_PPSELECT_A, 2, O>;
impl<'a, const O: u8> TX_PP_W<'a, O> {
    #[doc = "ALL_ONE"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_PPSELECT_A::_0)
    }
    #[doc = "ALL_ZERO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_PPSELECT_A::_1)
    }
    #[doc = "ZERO_ONE"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(TX_PPSELECT_A::_2)
    }
    #[doc = "ONE_ZERO"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(TX_PPSELECT_A::_3)
    }
}
#[doc = "Field `TX_MPOL` reader - Transmitter Manchester Polarity"]
pub type TX_MPOL_R = crate::BitReader<TX_MPOLSELECT_A>;
#[doc = "Transmitter Manchester Polarity\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_MPOLSELECT_A {
    #[doc = "0: Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    _0 = 0,
    #[doc = "1: Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    _1 = 1,
}
impl From<TX_MPOLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: TX_MPOLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_MPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_MPOLSELECT_A {
        match self.bits {
            false => TX_MPOLSELECT_A::_0,
            true => TX_MPOLSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TX_MPOLSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TX_MPOLSELECT_A::_1
    }
}
#[doc = "Field `TX_MPOL` writer - Transmitter Manchester Polarity"]
pub type TX_MPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAN_SPEC, TX_MPOLSELECT_A, O>;
impl<'a, const O: u8> TX_MPOL_W<'a, O> {
    #[doc = "Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TX_MPOLSELECT_A::_0)
    }
    #[doc = "Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TX_MPOLSELECT_A::_1)
    }
}
#[doc = "Field `RX_PL` reader - Receiver Preamble Length"]
pub type RX_PL_R = crate::FieldReader<u8, RX_PLSELECT_A>;
#[doc = "Receiver Preamble Length\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_PLSELECT_A {
    #[doc = "0: The receiver preamble pattern detection is disabled"]
    _0 = 0,
}
impl From<RX_PLSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PLSELECT_A) -> Self {
        variant as _
    }
}
impl RX_PL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RX_PLSELECT_A> {
        match self.bits {
            0 => Some(RX_PLSELECT_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX_PLSELECT_A::_0
    }
}
#[doc = "Field `RX_PL` writer - Receiver Preamble Length"]
pub type RX_PL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MAN_SPEC, u8, RX_PLSELECT_A, 4, O>;
impl<'a, const O: u8> RX_PL_W<'a, O> {
    #[doc = "The receiver preamble pattern detection is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_PLSELECT_A::_0)
    }
}
#[doc = "Field `RX_PP` reader - Receiver Preamble Pattern detected"]
pub type RX_PP_R = crate::FieldReader<u8, RX_PPSELECT_A>;
#[doc = "Receiver Preamble Pattern detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RX_PPSELECT_A {
    #[doc = "0: ALL_ONE"]
    _0 = 0,
    #[doc = "1: ALL_ZERO"]
    _1 = 1,
    #[doc = "2: ZERO_ONE"]
    _2 = 2,
    #[doc = "3: ONE_ZERO"]
    _3 = 3,
}
impl From<RX_PPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: RX_PPSELECT_A) -> Self {
        variant as _
    }
}
impl RX_PP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_PPSELECT_A {
        match self.bits {
            0 => RX_PPSELECT_A::_0,
            1 => RX_PPSELECT_A::_1,
            2 => RX_PPSELECT_A::_2,
            3 => RX_PPSELECT_A::_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX_PPSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX_PPSELECT_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == RX_PPSELECT_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == RX_PPSELECT_A::_3
    }
}
#[doc = "Field `RX_PP` writer - Receiver Preamble Pattern detected"]
pub type RX_PP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MAN_SPEC, u8, RX_PPSELECT_A, 2, O>;
impl<'a, const O: u8> RX_PP_W<'a, O> {
    #[doc = "ALL_ONE"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_PPSELECT_A::_0)
    }
    #[doc = "ALL_ZERO"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_PPSELECT_A::_1)
    }
    #[doc = "ZERO_ONE"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(RX_PPSELECT_A::_2)
    }
    #[doc = "ONE_ZERO"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(RX_PPSELECT_A::_3)
    }
}
#[doc = "Field `RX_MPOL` reader - Receiver Manchester Polarity"]
pub type RX_MPOL_R = crate::BitReader<RX_MPOLSELECT_A>;
#[doc = "Receiver Manchester Polarity\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_MPOLSELECT_A {
    #[doc = "0: Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    _0 = 0,
    #[doc = "1: Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    _1 = 1,
}
impl From<RX_MPOLSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RX_MPOLSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_MPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_MPOLSELECT_A {
        match self.bits {
            false => RX_MPOLSELECT_A::_0,
            true => RX_MPOLSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RX_MPOLSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RX_MPOLSELECT_A::_1
    }
}
#[doc = "Field `RX_MPOL` writer - Receiver Manchester Polarity"]
pub type RX_MPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAN_SPEC, RX_MPOLSELECT_A, O>;
impl<'a, const O: u8> RX_MPOL_W<'a, O> {
    #[doc = "Logic Zero is coded as a zero-to-one transition, Logic One is coded as a one-to-zero transition"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RX_MPOLSELECT_A::_0)
    }
    #[doc = "Logic Zero is coded as a one-to-zero transition, Logic One is coded as a zero-to-one transition"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RX_MPOLSELECT_A::_1)
    }
}
#[doc = "Field `DRIFT` reader - Drift compensation"]
pub type DRIFT_R = crate::BitReader<DRIFTSELECT_A>;
#[doc = "Drift compensation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRIFTSELECT_A {
    #[doc = "0: The USART can not recover from an important clock drift"]
    _0 = 0,
    #[doc = "1: The USART can recover from clock drift. The 16X clock mode must be enabled"]
    _1 = 1,
}
impl From<DRIFTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DRIFTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DRIFT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DRIFTSELECT_A {
        match self.bits {
            false => DRIFTSELECT_A::_0,
            true => DRIFTSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DRIFTSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DRIFTSELECT_A::_1
    }
}
#[doc = "Field `DRIFT` writer - Drift compensation"]
pub type DRIFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MAN_SPEC, DRIFTSELECT_A, O>;
impl<'a, const O: u8> DRIFT_W<'a, O> {
    #[doc = "The USART can not recover from an important clock drift"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DRIFTSELECT_A::_0)
    }
    #[doc = "The USART can recover from clock drift. The 16X clock mode must be enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DRIFTSELECT_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    pub fn tx_pl(&self) -> TX_PL_R {
        TX_PL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    pub fn tx_pp(&self) -> TX_PP_R {
        TX_PP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    pub fn tx_mpol(&self) -> TX_MPOL_R {
        TX_MPOL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    pub fn rx_pl(&self) -> RX_PL_R {
        RX_PL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    pub fn rx_pp(&self) -> RX_PP_R {
        RX_PP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    pub fn rx_mpol(&self) -> RX_MPOL_R {
        RX_MPOL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Drift compensation"]
    #[inline(always)]
    pub fn drift(&self) -> DRIFT_R {
        DRIFT_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transmitter Preamble Length"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pl(&mut self) -> TX_PL_W<0> {
        TX_PL_W::new(self)
    }
    #[doc = "Bits 8:9 - Transmitter Preamble Pattern"]
    #[inline(always)]
    #[must_use]
    pub fn tx_pp(&mut self) -> TX_PP_W<8> {
        TX_PP_W::new(self)
    }
    #[doc = "Bit 12 - Transmitter Manchester Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn tx_mpol(&mut self) -> TX_MPOL_W<12> {
        TX_MPOL_W::new(self)
    }
    #[doc = "Bits 16:19 - Receiver Preamble Length"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pl(&mut self) -> RX_PL_W<16> {
        RX_PL_W::new(self)
    }
    #[doc = "Bits 24:25 - Receiver Preamble Pattern detected"]
    #[inline(always)]
    #[must_use]
    pub fn rx_pp(&mut self) -> RX_PP_W<24> {
        RX_PP_W::new(self)
    }
    #[doc = "Bit 28 - Receiver Manchester Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn rx_mpol(&mut self) -> RX_MPOL_W<28> {
        RX_MPOL_W::new(self)
    }
    #[doc = "Bit 30 - Drift compensation"]
    #[inline(always)]
    #[must_use]
    pub fn drift(&mut self) -> DRIFT_W<30> {
        DRIFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Manchester Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [man](index.html) module"]
pub struct MAN_SPEC;
impl crate::RegisterSpec for MAN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [man::R](R) reader structure"]
impl crate::Readable for MAN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [man::W](W) writer structure"]
impl crate::Writable for MAN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MAN to value 0x3001_1004"]
impl crate::Resettable for MAN_SPEC {
    const RESET_VALUE: Self::Ux = 0x3001_1004;
}
