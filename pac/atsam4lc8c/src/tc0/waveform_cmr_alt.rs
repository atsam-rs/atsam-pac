#[doc = "Register `CMR%s_ALT` reader"]
pub struct R(crate::R<WAVEFORM_CMR_ALT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAVEFORM_CMR_ALT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAVEFORM_CMR_ALT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAVEFORM_CMR_ALT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMR%s_ALT` writer"]
pub struct W(crate::W<WAVEFORM_CMR_ALT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAVEFORM_CMR_ALT_SPEC>;
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
impl From<crate::W<WAVEFORM_CMR_ALT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAVEFORM_CMR_ALT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCCLKS_A {
    #[doc = "0: TIMER_DIV1_CLOCK"]
    TIMER_DIV1_CLOCK = 0,
    #[doc = "1: TIMER_DIV2_CLOCK"]
    TIMER_DIV2_CLOCK = 1,
    #[doc = "2: TIMER_DIV3_CLOCK"]
    TIMER_DIV3_CLOCK = 2,
    #[doc = "3: TIMER_DIV4_CLOCK"]
    TIMER_DIV4_CLOCK = 3,
    #[doc = "4: TIMER_DIV5_CLOCK"]
    TIMER_DIV5_CLOCK = 4,
    #[doc = "5: XC0"]
    XC0 = 5,
    #[doc = "6: XC1"]
    XC1 = 6,
    #[doc = "7: XC2"]
    XC2 = 7,
}
impl From<TCCLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCLKS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TCCLKS` reader - Clock Selection"]
pub struct TCCLKS_R(crate::FieldReader<u8, TCCLKS_A>);
impl TCCLKS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TCCLKS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCCLKS_A {
        match self.bits {
            0 => TCCLKS_A::TIMER_DIV1_CLOCK,
            1 => TCCLKS_A::TIMER_DIV2_CLOCK,
            2 => TCCLKS_A::TIMER_DIV3_CLOCK,
            3 => TCCLKS_A::TIMER_DIV4_CLOCK,
            4 => TCCLKS_A::TIMER_DIV5_CLOCK,
            5 => TCCLKS_A::XC0,
            6 => TCCLKS_A::XC1,
            7 => TCCLKS_A::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV1_CLOCK`"]
    #[inline(always)]
    pub fn is_timer_div1_clock(&self) -> bool {
        **self == TCCLKS_A::TIMER_DIV1_CLOCK
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV2_CLOCK`"]
    #[inline(always)]
    pub fn is_timer_div2_clock(&self) -> bool {
        **self == TCCLKS_A::TIMER_DIV2_CLOCK
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV3_CLOCK`"]
    #[inline(always)]
    pub fn is_timer_div3_clock(&self) -> bool {
        **self == TCCLKS_A::TIMER_DIV3_CLOCK
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV4_CLOCK`"]
    #[inline(always)]
    pub fn is_timer_div4_clock(&self) -> bool {
        **self == TCCLKS_A::TIMER_DIV4_CLOCK
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV5_CLOCK`"]
    #[inline(always)]
    pub fn is_timer_div5_clock(&self) -> bool {
        **self == TCCLKS_A::TIMER_DIV5_CLOCK
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        **self == TCCLKS_A::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        **self == TCCLKS_A::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        **self == TCCLKS_A::XC2
    }
}
impl core::ops::Deref for TCCLKS_R {
    type Target = crate::FieldReader<u8, TCCLKS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCCLKS` writer - Clock Selection"]
pub struct TCCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCLKS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "TIMER_DIV1_CLOCK"]
    #[inline(always)]
    pub fn timer_div1_clock(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_DIV1_CLOCK)
    }
    #[doc = "TIMER_DIV2_CLOCK"]
    #[inline(always)]
    pub fn timer_div2_clock(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_DIV2_CLOCK)
    }
    #[doc = "TIMER_DIV3_CLOCK"]
    #[inline(always)]
    pub fn timer_div3_clock(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_DIV3_CLOCK)
    }
    #[doc = "TIMER_DIV4_CLOCK"]
    #[inline(always)]
    pub fn timer_div4_clock(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_DIV4_CLOCK)
    }
    #[doc = "TIMER_DIV5_CLOCK"]
    #[inline(always)]
    pub fn timer_div5_clock(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_DIV5_CLOCK)
    }
    #[doc = "XC0"]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut W {
        self.variant(TCCLKS_A::XC0)
    }
    #[doc = "XC1"]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut W {
        self.variant(TCCLKS_A::XC1)
    }
    #[doc = "XC2"]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut W {
        self.variant(TCCLKS_A::XC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Clock Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKI_A {
    #[doc = "0: Counter is incremented on rising edge of the clock."]
    _0 = 0,
    #[doc = "1: Counter is incremented on falling edge of the clock."]
    _1 = 1,
}
impl From<CLKI_A> for bool {
    #[inline(always)]
    fn from(variant: CLKI_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKI` reader - Clock Invert"]
pub struct CLKI_R(crate::FieldReader<bool, CLKI_A>);
impl CLKI_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKI_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKI_A {
        match self.bits {
            false => CLKI_A::_0,
            true => CLKI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKI_A::_1
    }
}
impl core::ops::Deref for CLKI_R {
    type Target = crate::FieldReader<bool, CLKI_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKI` writer - Clock Invert"]
pub struct CLKI_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKI_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counter is incremented on rising edge of the clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKI_A::_0)
    }
    #[doc = "Counter is incremented on falling edge of the clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKI_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Burst Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BURST_A {
    #[doc = "0: The clock is not gated by an external signal."]
    NOT_GATED = 0,
    #[doc = "1: XC0 is ANDed with the selected clock."]
    CLK_AND_XC0 = 1,
    #[doc = "2: XC1 is ANDed with the selected clock."]
    CLK_AND_XC1 = 2,
    #[doc = "3: XC2 is ANDed with the selected clock."]
    CLK_AND_XC2 = 3,
}
impl From<BURST_A> for u8 {
    #[inline(always)]
    fn from(variant: BURST_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BURST` reader - Burst Signal Selection"]
pub struct BURST_R(crate::FieldReader<u8, BURST_A>);
impl BURST_R {
    pub(crate) fn new(bits: u8) -> Self {
        BURST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURST_A {
        match self.bits {
            0 => BURST_A::NOT_GATED,
            1 => BURST_A::CLK_AND_XC0,
            2 => BURST_A::CLK_AND_XC1,
            3 => BURST_A::CLK_AND_XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GATED`"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        **self == BURST_A::NOT_GATED
    }
    #[doc = "Checks if the value of the field is `CLK_AND_XC0`"]
    #[inline(always)]
    pub fn is_clk_and_xc0(&self) -> bool {
        **self == BURST_A::CLK_AND_XC0
    }
    #[doc = "Checks if the value of the field is `CLK_AND_XC1`"]
    #[inline(always)]
    pub fn is_clk_and_xc1(&self) -> bool {
        **self == BURST_A::CLK_AND_XC1
    }
    #[doc = "Checks if the value of the field is `CLK_AND_XC2`"]
    #[inline(always)]
    pub fn is_clk_and_xc2(&self) -> bool {
        **self == BURST_A::CLK_AND_XC2
    }
}
impl core::ops::Deref for BURST_R {
    type Target = crate::FieldReader<u8, BURST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURST` writer - Burst Signal Selection"]
pub struct BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURST_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut W {
        self.variant(BURST_A::NOT_GATED)
    }
    #[doc = "XC0 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn clk_and_xc0(self) -> &'a mut W {
        self.variant(BURST_A::CLK_AND_XC0)
    }
    #[doc = "XC1 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn clk_and_xc1(self) -> &'a mut W {
        self.variant(BURST_A::CLK_AND_XC1)
    }
    #[doc = "XC2 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn clk_and_xc2(self) -> &'a mut W {
        self.variant(BURST_A::CLK_AND_XC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Counter Clock Stopped with RC Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPCSTOP_A {
    #[doc = "0: Counter clock is not stopped when counter reaches RC."]
    _0 = 0,
    #[doc = "1: Counter clock is stopped when counter reaches RC."]
    _1 = 1,
}
impl From<CPCSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: CPCSTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPCSTOP` reader - Counter Clock Stopped with RC Compare"]
pub struct CPCSTOP_R(crate::FieldReader<bool, CPCSTOP_A>);
impl CPCSTOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPCSTOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPCSTOP_A {
        match self.bits {
            false => CPCSTOP_A::_0,
            true => CPCSTOP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPCSTOP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPCSTOP_A::_1
    }
}
impl core::ops::Deref for CPCSTOP_R {
    type Target = crate::FieldReader<bool, CPCSTOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPCSTOP` writer - Counter Clock Stopped with RC Compare"]
pub struct CPCSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPCSTOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counter clock is not stopped when counter reaches RC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPCSTOP_A::_0)
    }
    #[doc = "Counter clock is stopped when counter reaches RC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPCSTOP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Counter Clock Disable with RC Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPCDIS_A {
    #[doc = "0: Counter clock is not disabled when counter reaches RC."]
    _0 = 0,
    #[doc = "1: Counter clock is disabled when counter reaches RC."]
    _1 = 1,
}
impl From<CPCDIS_A> for bool {
    #[inline(always)]
    fn from(variant: CPCDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPCDIS` reader - Counter Clock Disable with RC Compare"]
pub struct CPCDIS_R(crate::FieldReader<bool, CPCDIS_A>);
impl CPCDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPCDIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPCDIS_A {
        match self.bits {
            false => CPCDIS_A::_0,
            true => CPCDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CPCDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPCDIS_A::_1
    }
}
impl core::ops::Deref for CPCDIS_R {
    type Target = crate::FieldReader<bool, CPCDIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPCDIS` writer - Counter Clock Disable with RC Compare"]
pub struct CPCDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPCDIS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counter clock is not disabled when counter reaches RC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPCDIS_A::_0)
    }
    #[doc = "Counter clock is disabled when counter reaches RC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPCDIS_A::_1)
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
#[doc = "External Event Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EEVTEDG_A {
    #[doc = "0: none"]
    NO_EDGE = 0,
    #[doc = "1: rising edge"]
    POS_EDGE = 1,
    #[doc = "2: falling edge"]
    NEG_EDGE = 2,
    #[doc = "3: each edge"]
    BOTH_EDGES = 3,
}
impl From<EEVTEDG_A> for u8 {
    #[inline(always)]
    fn from(variant: EEVTEDG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EEVTEDG` reader - External Event Edge Selection"]
pub struct EEVTEDG_R(crate::FieldReader<u8, EEVTEDG_A>);
impl EEVTEDG_R {
    pub(crate) fn new(bits: u8) -> Self {
        EEVTEDG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEVTEDG_A {
        match self.bits {
            0 => EEVTEDG_A::NO_EDGE,
            1 => EEVTEDG_A::POS_EDGE,
            2 => EEVTEDG_A::NEG_EDGE,
            3 => EEVTEDG_A::BOTH_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        **self == EEVTEDG_A::NO_EDGE
    }
    #[doc = "Checks if the value of the field is `POS_EDGE`"]
    #[inline(always)]
    pub fn is_pos_edge(&self) -> bool {
        **self == EEVTEDG_A::POS_EDGE
    }
    #[doc = "Checks if the value of the field is `NEG_EDGE`"]
    #[inline(always)]
    pub fn is_neg_edge(&self) -> bool {
        **self == EEVTEDG_A::NEG_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        **self == EEVTEDG_A::BOTH_EDGES
    }
}
impl core::ops::Deref for EEVTEDG_R {
    type Target = crate::FieldReader<u8, EEVTEDG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEVTEDG` writer - External Event Edge Selection"]
pub struct EEVTEDG_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVTEDG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEVTEDG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(EEVTEDG_A::NO_EDGE)
    }
    #[doc = "rising edge"]
    #[inline(always)]
    pub fn pos_edge(self) -> &'a mut W {
        self.variant(EEVTEDG_A::POS_EDGE)
    }
    #[doc = "falling edge"]
    #[inline(always)]
    pub fn neg_edge(self) -> &'a mut W {
        self.variant(EEVTEDG_A::NEG_EDGE)
    }
    #[doc = "each edge"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EEVTEDG_A::BOTH_EDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "External Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EEVT_A {
    #[doc = "0: TIOB input. If TIOB is chosen as the external event signal, it is configured as an input and no longer generates waveforms."]
    TIOB_INPUT = 0,
    #[doc = "1: XC0 output"]
    XC0_OUTPUT = 1,
    #[doc = "2: XC1 output"]
    XC1_OUTPUT = 2,
    #[doc = "3: XC2 output"]
    XC2_OUTPUT = 3,
}
impl From<EEVT_A> for u8 {
    #[inline(always)]
    fn from(variant: EEVT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EEVT` reader - External Event Selection"]
pub struct EEVT_R(crate::FieldReader<u8, EEVT_A>);
impl EEVT_R {
    pub(crate) fn new(bits: u8) -> Self {
        EEVT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEVT_A {
        match self.bits {
            0 => EEVT_A::TIOB_INPUT,
            1 => EEVT_A::XC0_OUTPUT,
            2 => EEVT_A::XC1_OUTPUT,
            3 => EEVT_A::XC2_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIOB_INPUT`"]
    #[inline(always)]
    pub fn is_tiob_input(&self) -> bool {
        **self == EEVT_A::TIOB_INPUT
    }
    #[doc = "Checks if the value of the field is `XC0_OUTPUT`"]
    #[inline(always)]
    pub fn is_xc0_output(&self) -> bool {
        **self == EEVT_A::XC0_OUTPUT
    }
    #[doc = "Checks if the value of the field is `XC1_OUTPUT`"]
    #[inline(always)]
    pub fn is_xc1_output(&self) -> bool {
        **self == EEVT_A::XC1_OUTPUT
    }
    #[doc = "Checks if the value of the field is `XC2_OUTPUT`"]
    #[inline(always)]
    pub fn is_xc2_output(&self) -> bool {
        **self == EEVT_A::XC2_OUTPUT
    }
}
impl core::ops::Deref for EEVT_R {
    type Target = crate::FieldReader<u8, EEVT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EEVT` writer - External Event Selection"]
pub struct EEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> EEVT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EEVT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "TIOB input. If TIOB is chosen as the external event signal, it is configured as an input and no longer generates waveforms."]
    #[inline(always)]
    pub fn tiob_input(self) -> &'a mut W {
        self.variant(EEVT_A::TIOB_INPUT)
    }
    #[doc = "XC0 output"]
    #[inline(always)]
    pub fn xc0_output(self) -> &'a mut W {
        self.variant(EEVT_A::XC0_OUTPUT)
    }
    #[doc = "XC1 output"]
    #[inline(always)]
    pub fn xc1_output(self) -> &'a mut W {
        self.variant(EEVT_A::XC1_OUTPUT)
    }
    #[doc = "XC2 output"]
    #[inline(always)]
    pub fn xc2_output(self) -> &'a mut W {
        self.variant(EEVT_A::XC2_OUTPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "External Event Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENETRG_A {
    #[doc = "0: The external event has no effect on the counter and its clock. In this case, the selected external event only controls the TIOA output."]
    _0 = 0,
    #[doc = "1: The external event resets the counter and starts the counter clock."]
    _1 = 1,
}
impl From<ENETRG_A> for bool {
    #[inline(always)]
    fn from(variant: ENETRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENETRG` reader - External Event Trigger Enable"]
pub struct ENETRG_R(crate::FieldReader<bool, ENETRG_A>);
impl ENETRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENETRG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENETRG_A {
        match self.bits {
            false => ENETRG_A::_0,
            true => ENETRG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == ENETRG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ENETRG_A::_1
    }
}
impl core::ops::Deref for ENETRG_R {
    type Target = crate::FieldReader<bool, ENETRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENETRG` writer - External Event Trigger Enable"]
pub struct ENETRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ENETRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENETRG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The external event has no effect on the counter and its clock. In this case, the selected external event only controls the TIOA output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENETRG_A::_0)
    }
    #[doc = "The external event resets the counter and starts the counter clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENETRG_A::_1)
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
#[doc = "Waveform Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAVSEL_A {
    #[doc = "0: UP mode without automatic trigger on RC Compare"]
    UP_NO_AUTO = 0,
    #[doc = "1: UPDOWN mode without automatic trigger on RC Compare"]
    UPDOWN_NO_AUTO = 1,
    #[doc = "2: UP mode with automatic trigger on RC Compare"]
    UP_AUTO = 2,
    #[doc = "3: UPDOWN mode with automatic trigger on RC Compare"]
    UPDOWN_AUTO = 3,
}
impl From<WAVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WAVSEL` reader - Waveform Selection"]
pub struct WAVSEL_R(crate::FieldReader<u8, WAVSEL_A>);
impl WAVSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAVSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAVSEL_A {
        match self.bits {
            0 => WAVSEL_A::UP_NO_AUTO,
            1 => WAVSEL_A::UPDOWN_NO_AUTO,
            2 => WAVSEL_A::UP_AUTO,
            3 => WAVSEL_A::UPDOWN_AUTO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UP_NO_AUTO`"]
    #[inline(always)]
    pub fn is_up_no_auto(&self) -> bool {
        **self == WAVSEL_A::UP_NO_AUTO
    }
    #[doc = "Checks if the value of the field is `UPDOWN_NO_AUTO`"]
    #[inline(always)]
    pub fn is_updown_no_auto(&self) -> bool {
        **self == WAVSEL_A::UPDOWN_NO_AUTO
    }
    #[doc = "Checks if the value of the field is `UP_AUTO`"]
    #[inline(always)]
    pub fn is_up_auto(&self) -> bool {
        **self == WAVSEL_A::UP_AUTO
    }
    #[doc = "Checks if the value of the field is `UPDOWN_AUTO`"]
    #[inline(always)]
    pub fn is_updown_auto(&self) -> bool {
        **self == WAVSEL_A::UPDOWN_AUTO
    }
}
impl core::ops::Deref for WAVSEL_R {
    type Target = crate::FieldReader<u8, WAVSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVSEL` writer - Waveform Selection"]
pub struct WAVSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "UP mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn up_no_auto(self) -> &'a mut W {
        self.variant(WAVSEL_A::UP_NO_AUTO)
    }
    #[doc = "UPDOWN mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn updown_no_auto(self) -> &'a mut W {
        self.variant(WAVSEL_A::UPDOWN_NO_AUTO)
    }
    #[doc = "UP mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn up_auto(self) -> &'a mut W {
        self.variant(WAVSEL_A::UP_AUTO)
    }
    #[doc = "UPDOWN mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn updown_auto(self) -> &'a mut W {
        self.variant(WAVSEL_A::UPDOWN_AUTO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 13)) | ((value as u32 & 0x03) << 13);
        self.w
    }
}
#[doc = "WAVE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVE_A {
    #[doc = "0: Waveform Mode is disabled (Capture Mode is enabled)."]
    _0 = 0,
    #[doc = "1: Waveform Mode is enabled."]
    _1 = 1,
}
impl From<WAVE_A> for bool {
    #[inline(always)]
    fn from(variant: WAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAVE` reader - WAVE"]
pub struct WAVE_R(crate::FieldReader<bool, WAVE_A>);
impl WAVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        WAVE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAVE_A {
        match self.bits {
            false => WAVE_A::_0,
            true => WAVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == WAVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == WAVE_A::_1
    }
}
impl core::ops::Deref for WAVE_R {
    type Target = crate::FieldReader<bool, WAVE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAVE` writer - WAVE"]
pub struct WAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Waveform Mode is disabled (Capture Mode is enabled)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAVE_A::_0)
    }
    #[doc = "Waveform Mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAVE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "RA Compare Effect on TIOA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACPA_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<ACPA_A> for u8 {
    #[inline(always)]
    fn from(variant: ACPA_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACPA` reader - RA Compare Effect on TIOA"]
pub struct ACPA_R(crate::FieldReader<u8, ACPA_A>);
impl ACPA_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACPA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACPA_A {
        match self.bits {
            0 => ACPA_A::NONE,
            1 => ACPA_A::SET,
            2 => ACPA_A::CLEAR,
            3 => ACPA_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == ACPA_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == ACPA_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == ACPA_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == ACPA_A::TOGGLE
    }
}
impl core::ops::Deref for ACPA_R {
    type Target = crate::FieldReader<u8, ACPA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACPA` writer - RA Compare Effect on TIOA"]
pub struct ACPA_W<'a> {
    w: &'a mut W,
}
impl<'a> ACPA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACPA_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACPA_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ACPA_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACPA_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ACPA_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "RC Compare Effect on TIOA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACPC_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<ACPC_A> for u8 {
    #[inline(always)]
    fn from(variant: ACPC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ACPC` reader - RC Compare Effect on TIOA"]
pub struct ACPC_R(crate::FieldReader<u8, ACPC_A>);
impl ACPC_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACPC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACPC_A {
        match self.bits {
            0 => ACPC_A::NONE,
            1 => ACPC_A::SET,
            2 => ACPC_A::CLEAR,
            3 => ACPC_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == ACPC_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == ACPC_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == ACPC_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == ACPC_A::TOGGLE
    }
}
impl core::ops::Deref for ACPC_R {
    type Target = crate::FieldReader<u8, ACPC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACPC` writer - RC Compare Effect on TIOA"]
pub struct ACPC_W<'a> {
    w: &'a mut W,
}
impl<'a> ACPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACPC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACPC_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ACPC_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACPC_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ACPC_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "External Event Effect on TIOA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AEEVT_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<AEEVT_A> for u8 {
    #[inline(always)]
    fn from(variant: AEEVT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AEEVT` reader - External Event Effect on TIOA"]
pub struct AEEVT_R(crate::FieldReader<u8, AEEVT_A>);
impl AEEVT_R {
    pub(crate) fn new(bits: u8) -> Self {
        AEEVT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEEVT_A {
        match self.bits {
            0 => AEEVT_A::NONE,
            1 => AEEVT_A::SET,
            2 => AEEVT_A::CLEAR,
            3 => AEEVT_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == AEEVT_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == AEEVT_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == AEEVT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == AEEVT_A::TOGGLE
    }
}
impl core::ops::Deref for AEEVT_R {
    type Target = crate::FieldReader<u8, AEEVT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AEEVT` writer - External Event Effect on TIOA"]
pub struct AEEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> AEEVT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AEEVT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(AEEVT_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(AEEVT_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AEEVT_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(AEEVT_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Software Trigger Effect on TIOA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ASWTRG_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<ASWTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: ASWTRG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ASWTRG` reader - Software Trigger Effect on TIOA"]
pub struct ASWTRG_R(crate::FieldReader<u8, ASWTRG_A>);
impl ASWTRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        ASWTRG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASWTRG_A {
        match self.bits {
            0 => ASWTRG_A::NONE,
            1 => ASWTRG_A::SET,
            2 => ASWTRG_A::CLEAR,
            3 => ASWTRG_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == ASWTRG_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == ASWTRG_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == ASWTRG_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == ASWTRG_A::TOGGLE
    }
}
impl core::ops::Deref for ASWTRG_R {
    type Target = crate::FieldReader<u8, ASWTRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASWTRG` writer - Software Trigger Effect on TIOA"]
pub struct ASWTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ASWTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ASWTRG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ASWTRG_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ASWTRG_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ASWTRG_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ASWTRG_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "RB Compare Effect on TIOB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BCPB_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<BCPB_A> for u8 {
    #[inline(always)]
    fn from(variant: BCPB_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BCPB` reader - RB Compare Effect on TIOB"]
pub struct BCPB_R(crate::FieldReader<u8, BCPB_A>);
impl BCPB_R {
    pub(crate) fn new(bits: u8) -> Self {
        BCPB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCPB_A {
        match self.bits {
            0 => BCPB_A::NONE,
            1 => BCPB_A::SET,
            2 => BCPB_A::CLEAR,
            3 => BCPB_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == BCPB_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == BCPB_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == BCPB_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == BCPB_A::TOGGLE
    }
}
impl core::ops::Deref for BCPB_R {
    type Target = crate::FieldReader<u8, BCPB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCPB` writer - RB Compare Effect on TIOB"]
pub struct BCPB_W<'a> {
    w: &'a mut W,
}
impl<'a> BCPB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCPB_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BCPB_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BCPB_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCPB_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BCPB_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "RC Compare Effect on TIOB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BCPC_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<BCPC_A> for u8 {
    #[inline(always)]
    fn from(variant: BCPC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BCPC` reader - RC Compare Effect on TIOB"]
pub struct BCPC_R(crate::FieldReader<u8, BCPC_A>);
impl BCPC_R {
    pub(crate) fn new(bits: u8) -> Self {
        BCPC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCPC_A {
        match self.bits {
            0 => BCPC_A::NONE,
            1 => BCPC_A::SET,
            2 => BCPC_A::CLEAR,
            3 => BCPC_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == BCPC_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == BCPC_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == BCPC_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == BCPC_A::TOGGLE
    }
}
impl core::ops::Deref for BCPC_R {
    type Target = crate::FieldReader<u8, BCPC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCPC` writer - RC Compare Effect on TIOB"]
pub struct BCPC_W<'a> {
    w: &'a mut W,
}
impl<'a> BCPC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BCPC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BCPC_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BCPC_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCPC_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BCPC_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "External Event Effect on TIOB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BEEVT_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<BEEVT_A> for u8 {
    #[inline(always)]
    fn from(variant: BEEVT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BEEVT` reader - External Event Effect on TIOB"]
pub struct BEEVT_R(crate::FieldReader<u8, BEEVT_A>);
impl BEEVT_R {
    pub(crate) fn new(bits: u8) -> Self {
        BEEVT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEEVT_A {
        match self.bits {
            0 => BEEVT_A::NONE,
            1 => BEEVT_A::SET,
            2 => BEEVT_A::CLEAR,
            3 => BEEVT_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == BEEVT_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == BEEVT_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == BEEVT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == BEEVT_A::TOGGLE
    }
}
impl core::ops::Deref for BEEVT_R {
    type Target = crate::FieldReader<u8, BEEVT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEEVT` writer - External Event Effect on TIOB"]
pub struct BEEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> BEEVT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BEEVT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BEEVT_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BEEVT_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BEEVT_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BEEVT_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Software Trigger Effect on TIOB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BSWTRG_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<BSWTRG_A> for u8 {
    #[inline(always)]
    fn from(variant: BSWTRG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BSWTRG` reader - Software Trigger Effect on TIOB"]
pub struct BSWTRG_R(crate::FieldReader<u8, BSWTRG_A>);
impl BSWTRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        BSWTRG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSWTRG_A {
        match self.bits {
            0 => BSWTRG_A::NONE,
            1 => BSWTRG_A::SET,
            2 => BSWTRG_A::CLEAR,
            3 => BSWTRG_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == BSWTRG_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        **self == BSWTRG_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == BSWTRG_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        **self == BSWTRG_A::TOGGLE
    }
}
impl core::ops::Deref for BSWTRG_R {
    type Target = crate::FieldReader<u8, BSWTRG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSWTRG` writer - Software Trigger Effect on TIOB"]
pub struct BSWTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> BSWTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BSWTRG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BSWTRG_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSWTRG_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BSWTRG_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BSWTRG_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&self) -> TCCLKS_R {
        TCCLKS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&self) -> CLKI_R {
        CLKI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline(always)]
    pub fn cpcstop(&self) -> CPCSTOP_R {
        CPCSTOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Compare"]
    #[inline(always)]
    pub fn cpcdis(&self) -> CPCDIS_R {
        CPCDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline(always)]
    pub fn eevtedg(&self) -> EEVTEDG_R {
        EEVTEDG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline(always)]
    pub fn eevt(&self) -> EEVT_R {
        EEVT_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline(always)]
    pub fn enetrg(&self) -> ENETRG_R {
        ENETRG_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline(always)]
    pub fn wavsel(&self) -> WAVSEL_R {
        WAVSEL_R::new(((self.bits >> 13) & 0x03) as u8)
    }
    #[doc = "Bit 15 - WAVE"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOA"]
    #[inline(always)]
    pub fn acpa(&self) -> ACPA_R {
        ACPA_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOA"]
    #[inline(always)]
    pub fn acpc(&self) -> ACPC_R {
        ACPC_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOA"]
    #[inline(always)]
    pub fn aeevt(&self) -> AEEVT_R {
        AEEVT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOA"]
    #[inline(always)]
    pub fn aswtrg(&self) -> ASWTRG_R {
        ASWTRG_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOB"]
    #[inline(always)]
    pub fn bcpb(&self) -> BCPB_R {
        BCPB_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOB"]
    #[inline(always)]
    pub fn bcpc(&self) -> BCPC_R {
        BCPC_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOB"]
    #[inline(always)]
    pub fn beevt(&self) -> BEEVT_R {
        BEEVT_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOB"]
    #[inline(always)]
    pub fn bswtrg(&self) -> BSWTRG_R {
        BSWTRG_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&mut self) -> TCCLKS_W {
        TCCLKS_W { w: self }
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&mut self) -> CLKI_W {
        CLKI_W { w: self }
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&mut self) -> BURST_W {
        BURST_W { w: self }
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline(always)]
    pub fn cpcstop(&mut self) -> CPCSTOP_W {
        CPCSTOP_W { w: self }
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Compare"]
    #[inline(always)]
    pub fn cpcdis(&mut self) -> CPCDIS_W {
        CPCDIS_W { w: self }
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline(always)]
    pub fn eevtedg(&mut self) -> EEVTEDG_W {
        EEVTEDG_W { w: self }
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline(always)]
    pub fn eevt(&mut self) -> EEVT_W {
        EEVT_W { w: self }
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline(always)]
    pub fn enetrg(&mut self) -> ENETRG_W {
        ENETRG_W { w: self }
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline(always)]
    pub fn wavsel(&mut self) -> WAVSEL_W {
        WAVSEL_W { w: self }
    }
    #[doc = "Bit 15 - WAVE"]
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W {
        WAVE_W { w: self }
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOA"]
    #[inline(always)]
    pub fn acpa(&mut self) -> ACPA_W {
        ACPA_W { w: self }
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOA"]
    #[inline(always)]
    pub fn acpc(&mut self) -> ACPC_W {
        ACPC_W { w: self }
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOA"]
    #[inline(always)]
    pub fn aeevt(&mut self) -> AEEVT_W {
        AEEVT_W { w: self }
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOA"]
    #[inline(always)]
    pub fn aswtrg(&mut self) -> ASWTRG_W {
        ASWTRG_W { w: self }
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOB"]
    #[inline(always)]
    pub fn bcpb(&mut self) -> BCPB_W {
        BCPB_W { w: self }
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOB"]
    #[inline(always)]
    pub fn bcpc(&mut self) -> BCPC_W {
        BCPC_W { w: self }
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOB"]
    #[inline(always)]
    pub fn beevt(&mut self) -> BEEVT_W {
        BEEVT_W { w: self }
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOB"]
    #[inline(always)]
    pub fn bswtrg(&mut self) -> BSWTRG_W {
        BSWTRG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Mode Register Channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waveform_cmr_alt](index.html) module"]
pub struct WAVEFORM_CMR_ALT_SPEC;
impl crate::RegisterSpec for WAVEFORM_CMR_ALT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [waveform_cmr_alt::R](R) reader structure"]
impl crate::Readable for WAVEFORM_CMR_ALT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [waveform_cmr_alt::W](W) writer structure"]
impl crate::Writable for WAVEFORM_CMR_ALT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMR%s_ALT to value 0"]
impl crate::Resettable for WAVEFORM_CMR_ALT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
