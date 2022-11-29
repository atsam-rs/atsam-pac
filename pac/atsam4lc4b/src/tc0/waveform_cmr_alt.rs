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
#[doc = "Field `TCCLKS` reader - Clock Selection"]
pub type TCCLKS_R = crate::FieldReader<u8, TCCLKSSELECT_A>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCLKSSELECT_A {
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
impl From<TCCLKSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCLKSSELECT_A) -> Self {
        variant as _
    }
}
impl TCCLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCCLKSSELECT_A {
        match self.bits {
            0 => TCCLKSSELECT_A::TIMER_DIV1_CLOCK,
            1 => TCCLKSSELECT_A::TIMER_DIV2_CLOCK,
            2 => TCCLKSSELECT_A::TIMER_DIV3_CLOCK,
            3 => TCCLKSSELECT_A::TIMER_DIV4_CLOCK,
            4 => TCCLKSSELECT_A::TIMER_DIV5_CLOCK,
            5 => TCCLKSSELECT_A::XC0,
            6 => TCCLKSSELECT_A::XC1,
            7 => TCCLKSSELECT_A::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV1_CLOCK`"]
    #[inline(always)]
    pub fn is_timer_div1_clock(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_DIV1_CLOCK
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV2_CLOCK`"]
    #[inline(always)]
    pub fn is_timer_div2_clock(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_DIV2_CLOCK
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV3_CLOCK`"]
    #[inline(always)]
    pub fn is_timer_div3_clock(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_DIV3_CLOCK
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV4_CLOCK`"]
    #[inline(always)]
    pub fn is_timer_div4_clock(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_DIV4_CLOCK
    }
    #[doc = "Checks if the value of the field is `TIMER_DIV5_CLOCK`"]
    #[inline(always)]
    pub fn is_timer_div5_clock(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_DIV5_CLOCK
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == TCCLKSSELECT_A::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == TCCLKSSELECT_A::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == TCCLKSSELECT_A::XC2
    }
}
#[doc = "Field `TCCLKS` writer - Clock Selection"]
pub type TCCLKS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAVEFORM_CMR_ALT_SPEC, u8, TCCLKSSELECT_A, 3, O>;
impl<'a, const O: u8> TCCLKS_W<'a, O> {
    #[doc = "TIMER_DIV1_CLOCK"]
    #[inline(always)]
    pub fn timer_div1_clock(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_DIV1_CLOCK)
    }
    #[doc = "TIMER_DIV2_CLOCK"]
    #[inline(always)]
    pub fn timer_div2_clock(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_DIV2_CLOCK)
    }
    #[doc = "TIMER_DIV3_CLOCK"]
    #[inline(always)]
    pub fn timer_div3_clock(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_DIV3_CLOCK)
    }
    #[doc = "TIMER_DIV4_CLOCK"]
    #[inline(always)]
    pub fn timer_div4_clock(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_DIV4_CLOCK)
    }
    #[doc = "TIMER_DIV5_CLOCK"]
    #[inline(always)]
    pub fn timer_div5_clock(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_DIV5_CLOCK)
    }
    #[doc = "XC0"]
    #[inline(always)]
    pub fn xc0(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::XC0)
    }
    #[doc = "XC1"]
    #[inline(always)]
    pub fn xc1(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::XC1)
    }
    #[doc = "XC2"]
    #[inline(always)]
    pub fn xc2(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::XC2)
    }
}
#[doc = "Field `CLKI` reader - Clock Invert"]
pub type CLKI_R = crate::BitReader<CLKISELECT_A>;
#[doc = "Clock Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKISELECT_A {
    #[doc = "0: Counter is incremented on rising edge of the clock."]
    _0 = 0,
    #[doc = "1: Counter is incremented on falling edge of the clock."]
    _1 = 1,
}
impl From<CLKISELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CLKISELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKISELECT_A {
        match self.bits {
            false => CLKISELECT_A::_0,
            true => CLKISELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKISELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKISELECT_A::_1
    }
}
#[doc = "Field `CLKI` writer - Clock Invert"]
pub type CLKI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAVEFORM_CMR_ALT_SPEC, CLKISELECT_A, O>;
impl<'a, const O: u8> CLKI_W<'a, O> {
    #[doc = "Counter is incremented on rising edge of the clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKISELECT_A::_0)
    }
    #[doc = "Counter is incremented on falling edge of the clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKISELECT_A::_1)
    }
}
#[doc = "Field `BURST` reader - Burst Signal Selection"]
pub type BURST_R = crate::FieldReader<u8, BURSTSELECT_A>;
#[doc = "Burst Signal Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BURSTSELECT_A {
    #[doc = "0: The clock is not gated by an external signal."]
    NOT_GATED = 0,
    #[doc = "1: XC0 is ANDed with the selected clock."]
    CLK_AND_XC0 = 1,
    #[doc = "2: XC1 is ANDed with the selected clock."]
    CLK_AND_XC1 = 2,
    #[doc = "3: XC2 is ANDed with the selected clock."]
    CLK_AND_XC2 = 3,
}
impl From<BURSTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BURSTSELECT_A) -> Self {
        variant as _
    }
}
impl BURST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTSELECT_A {
        match self.bits {
            0 => BURSTSELECT_A::NOT_GATED,
            1 => BURSTSELECT_A::CLK_AND_XC0,
            2 => BURSTSELECT_A::CLK_AND_XC1,
            3 => BURSTSELECT_A::CLK_AND_XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOT_GATED`"]
    #[inline(always)]
    pub fn is_not_gated(&self) -> bool {
        *self == BURSTSELECT_A::NOT_GATED
    }
    #[doc = "Checks if the value of the field is `CLK_AND_XC0`"]
    #[inline(always)]
    pub fn is_clk_and_xc0(&self) -> bool {
        *self == BURSTSELECT_A::CLK_AND_XC0
    }
    #[doc = "Checks if the value of the field is `CLK_AND_XC1`"]
    #[inline(always)]
    pub fn is_clk_and_xc1(&self) -> bool {
        *self == BURSTSELECT_A::CLK_AND_XC1
    }
    #[doc = "Checks if the value of the field is `CLK_AND_XC2`"]
    #[inline(always)]
    pub fn is_clk_and_xc2(&self) -> bool {
        *self == BURSTSELECT_A::CLK_AND_XC2
    }
}
#[doc = "Field `BURST` writer - Burst Signal Selection"]
pub type BURST_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAVEFORM_CMR_ALT_SPEC, u8, BURSTSELECT_A, 2, O>;
impl<'a, const O: u8> BURST_W<'a, O> {
    #[doc = "The clock is not gated by an external signal."]
    #[inline(always)]
    pub fn not_gated(self) -> &'a mut W {
        self.variant(BURSTSELECT_A::NOT_GATED)
    }
    #[doc = "XC0 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn clk_and_xc0(self) -> &'a mut W {
        self.variant(BURSTSELECT_A::CLK_AND_XC0)
    }
    #[doc = "XC1 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn clk_and_xc1(self) -> &'a mut W {
        self.variant(BURSTSELECT_A::CLK_AND_XC1)
    }
    #[doc = "XC2 is ANDed with the selected clock."]
    #[inline(always)]
    pub fn clk_and_xc2(self) -> &'a mut W {
        self.variant(BURSTSELECT_A::CLK_AND_XC2)
    }
}
#[doc = "Field `CPCSTOP` reader - Counter Clock Stopped with RC Compare"]
pub type CPCSTOP_R = crate::BitReader<CPCSTOPSELECT_A>;
#[doc = "Counter Clock Stopped with RC Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPCSTOPSELECT_A {
    #[doc = "0: Counter clock is not stopped when counter reaches RC."]
    _0 = 0,
    #[doc = "1: Counter clock is stopped when counter reaches RC."]
    _1 = 1,
}
impl From<CPCSTOPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPCSTOPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPCSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPCSTOPSELECT_A {
        match self.bits {
            false => CPCSTOPSELECT_A::_0,
            true => CPCSTOPSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPCSTOPSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPCSTOPSELECT_A::_1
    }
}
#[doc = "Field `CPCSTOP` writer - Counter Clock Stopped with RC Compare"]
pub type CPCSTOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAVEFORM_CMR_ALT_SPEC, CPCSTOPSELECT_A, O>;
impl<'a, const O: u8> CPCSTOP_W<'a, O> {
    #[doc = "Counter clock is not stopped when counter reaches RC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPCSTOPSELECT_A::_0)
    }
    #[doc = "Counter clock is stopped when counter reaches RC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPCSTOPSELECT_A::_1)
    }
}
#[doc = "Field `CPCDIS` reader - Counter Clock Disable with RC Compare"]
pub type CPCDIS_R = crate::BitReader<CPCDISSELECT_A>;
#[doc = "Counter Clock Disable with RC Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPCDISSELECT_A {
    #[doc = "0: Counter clock is not disabled when counter reaches RC."]
    _0 = 0,
    #[doc = "1: Counter clock is disabled when counter reaches RC."]
    _1 = 1,
}
impl From<CPCDISSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPCDISSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPCDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPCDISSELECT_A {
        match self.bits {
            false => CPCDISSELECT_A::_0,
            true => CPCDISSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPCDISSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPCDISSELECT_A::_1
    }
}
#[doc = "Field `CPCDIS` writer - Counter Clock Disable with RC Compare"]
pub type CPCDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAVEFORM_CMR_ALT_SPEC, CPCDISSELECT_A, O>;
impl<'a, const O: u8> CPCDIS_W<'a, O> {
    #[doc = "Counter clock is not disabled when counter reaches RC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPCDISSELECT_A::_0)
    }
    #[doc = "Counter clock is disabled when counter reaches RC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPCDISSELECT_A::_1)
    }
}
#[doc = "Field `EEVTEDG` reader - External Event Edge Selection"]
pub type EEVTEDG_R = crate::FieldReader<u8, EEVTEDGSELECT_A>;
#[doc = "External Event Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EEVTEDGSELECT_A {
    #[doc = "0: none"]
    NO_EDGE = 0,
    #[doc = "1: rising edge"]
    POS_EDGE = 1,
    #[doc = "2: falling edge"]
    NEG_EDGE = 2,
    #[doc = "3: each edge"]
    BOTH_EDGES = 3,
}
impl From<EEVTEDGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EEVTEDGSELECT_A) -> Self {
        variant as _
    }
}
impl EEVTEDG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEVTEDGSELECT_A {
        match self.bits {
            0 => EEVTEDGSELECT_A::NO_EDGE,
            1 => EEVTEDGSELECT_A::POS_EDGE,
            2 => EEVTEDGSELECT_A::NEG_EDGE,
            3 => EEVTEDGSELECT_A::BOTH_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == EEVTEDGSELECT_A::NO_EDGE
    }
    #[doc = "Checks if the value of the field is `POS_EDGE`"]
    #[inline(always)]
    pub fn is_pos_edge(&self) -> bool {
        *self == EEVTEDGSELECT_A::POS_EDGE
    }
    #[doc = "Checks if the value of the field is `NEG_EDGE`"]
    #[inline(always)]
    pub fn is_neg_edge(&self) -> bool {
        *self == EEVTEDGSELECT_A::NEG_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EEVTEDGSELECT_A::BOTH_EDGES
    }
}
#[doc = "Field `EEVTEDG` writer - External Event Edge Selection"]
pub type EEVTEDG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAVEFORM_CMR_ALT_SPEC, u8, EEVTEDGSELECT_A, 2, O>;
impl<'a, const O: u8> EEVTEDG_W<'a, O> {
    #[doc = "none"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(EEVTEDGSELECT_A::NO_EDGE)
    }
    #[doc = "rising edge"]
    #[inline(always)]
    pub fn pos_edge(self) -> &'a mut W {
        self.variant(EEVTEDGSELECT_A::POS_EDGE)
    }
    #[doc = "falling edge"]
    #[inline(always)]
    pub fn neg_edge(self) -> &'a mut W {
        self.variant(EEVTEDGSELECT_A::NEG_EDGE)
    }
    #[doc = "each edge"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EEVTEDGSELECT_A::BOTH_EDGES)
    }
}
#[doc = "Field `EEVT` reader - External Event Selection"]
pub type EEVT_R = crate::FieldReader<u8, EEVTSELECT_A>;
#[doc = "External Event Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EEVTSELECT_A {
    #[doc = "0: TIOB input. If TIOB is chosen as the external event signal, it is configured as an input and no longer generates waveforms."]
    TIOB_INPUT = 0,
    #[doc = "1: XC0 output"]
    XC0_OUTPUT = 1,
    #[doc = "2: XC1 output"]
    XC1_OUTPUT = 2,
    #[doc = "3: XC2 output"]
    XC2_OUTPUT = 3,
}
impl From<EEVTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EEVTSELECT_A) -> Self {
        variant as _
    }
}
impl EEVT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEVTSELECT_A {
        match self.bits {
            0 => EEVTSELECT_A::TIOB_INPUT,
            1 => EEVTSELECT_A::XC0_OUTPUT,
            2 => EEVTSELECT_A::XC1_OUTPUT,
            3 => EEVTSELECT_A::XC2_OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIOB_INPUT`"]
    #[inline(always)]
    pub fn is_tiob_input(&self) -> bool {
        *self == EEVTSELECT_A::TIOB_INPUT
    }
    #[doc = "Checks if the value of the field is `XC0_OUTPUT`"]
    #[inline(always)]
    pub fn is_xc0_output(&self) -> bool {
        *self == EEVTSELECT_A::XC0_OUTPUT
    }
    #[doc = "Checks if the value of the field is `XC1_OUTPUT`"]
    #[inline(always)]
    pub fn is_xc1_output(&self) -> bool {
        *self == EEVTSELECT_A::XC1_OUTPUT
    }
    #[doc = "Checks if the value of the field is `XC2_OUTPUT`"]
    #[inline(always)]
    pub fn is_xc2_output(&self) -> bool {
        *self == EEVTSELECT_A::XC2_OUTPUT
    }
}
#[doc = "Field `EEVT` writer - External Event Selection"]
pub type EEVT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAVEFORM_CMR_ALT_SPEC, u8, EEVTSELECT_A, 2, O>;
impl<'a, const O: u8> EEVT_W<'a, O> {
    #[doc = "TIOB input. If TIOB is chosen as the external event signal, it is configured as an input and no longer generates waveforms."]
    #[inline(always)]
    pub fn tiob_input(self) -> &'a mut W {
        self.variant(EEVTSELECT_A::TIOB_INPUT)
    }
    #[doc = "XC0 output"]
    #[inline(always)]
    pub fn xc0_output(self) -> &'a mut W {
        self.variant(EEVTSELECT_A::XC0_OUTPUT)
    }
    #[doc = "XC1 output"]
    #[inline(always)]
    pub fn xc1_output(self) -> &'a mut W {
        self.variant(EEVTSELECT_A::XC1_OUTPUT)
    }
    #[doc = "XC2 output"]
    #[inline(always)]
    pub fn xc2_output(self) -> &'a mut W {
        self.variant(EEVTSELECT_A::XC2_OUTPUT)
    }
}
#[doc = "Field `ENETRG` reader - External Event Trigger Enable"]
pub type ENETRG_R = crate::BitReader<ENETRGSELECT_A>;
#[doc = "External Event Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENETRGSELECT_A {
    #[doc = "0: The external event has no effect on the counter and its clock. In this case, the selected external event only controls the TIOA output."]
    _0 = 0,
    #[doc = "1: The external event resets the counter and starts the counter clock."]
    _1 = 1,
}
impl From<ENETRGSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ENETRGSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ENETRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENETRGSELECT_A {
        match self.bits {
            false => ENETRGSELECT_A::_0,
            true => ENETRGSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENETRGSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENETRGSELECT_A::_1
    }
}
#[doc = "Field `ENETRG` writer - External Event Trigger Enable"]
pub type ENETRG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAVEFORM_CMR_ALT_SPEC, ENETRGSELECT_A, O>;
impl<'a, const O: u8> ENETRG_W<'a, O> {
    #[doc = "The external event has no effect on the counter and its clock. In this case, the selected external event only controls the TIOA output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENETRGSELECT_A::_0)
    }
    #[doc = "The external event resets the counter and starts the counter clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENETRGSELECT_A::_1)
    }
}
#[doc = "Field `WAVSEL` reader - Waveform Selection"]
pub type WAVSEL_R = crate::FieldReader<u8, WAVSELSELECT_A>;
#[doc = "Waveform Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAVSELSELECT_A {
    #[doc = "0: UP mode without automatic trigger on RC Compare"]
    UP_NO_AUTO = 0,
    #[doc = "1: UPDOWN mode without automatic trigger on RC Compare"]
    UPDOWN_NO_AUTO = 1,
    #[doc = "2: UP mode with automatic trigger on RC Compare"]
    UP_AUTO = 2,
    #[doc = "3: UPDOWN mode with automatic trigger on RC Compare"]
    UPDOWN_AUTO = 3,
}
impl From<WAVSELSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: WAVSELSELECT_A) -> Self {
        variant as _
    }
}
impl WAVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAVSELSELECT_A {
        match self.bits {
            0 => WAVSELSELECT_A::UP_NO_AUTO,
            1 => WAVSELSELECT_A::UPDOWN_NO_AUTO,
            2 => WAVSELSELECT_A::UP_AUTO,
            3 => WAVSELSELECT_A::UPDOWN_AUTO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UP_NO_AUTO`"]
    #[inline(always)]
    pub fn is_up_no_auto(&self) -> bool {
        *self == WAVSELSELECT_A::UP_NO_AUTO
    }
    #[doc = "Checks if the value of the field is `UPDOWN_NO_AUTO`"]
    #[inline(always)]
    pub fn is_updown_no_auto(&self) -> bool {
        *self == WAVSELSELECT_A::UPDOWN_NO_AUTO
    }
    #[doc = "Checks if the value of the field is `UP_AUTO`"]
    #[inline(always)]
    pub fn is_up_auto(&self) -> bool {
        *self == WAVSELSELECT_A::UP_AUTO
    }
    #[doc = "Checks if the value of the field is `UPDOWN_AUTO`"]
    #[inline(always)]
    pub fn is_updown_auto(&self) -> bool {
        *self == WAVSELSELECT_A::UPDOWN_AUTO
    }
}
#[doc = "Field `WAVSEL` writer - Waveform Selection"]
pub type WAVSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAVEFORM_CMR_ALT_SPEC, u8, WAVSELSELECT_A, 2, O>;
impl<'a, const O: u8> WAVSEL_W<'a, O> {
    #[doc = "UP mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn up_no_auto(self) -> &'a mut W {
        self.variant(WAVSELSELECT_A::UP_NO_AUTO)
    }
    #[doc = "UPDOWN mode without automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn updown_no_auto(self) -> &'a mut W {
        self.variant(WAVSELSELECT_A::UPDOWN_NO_AUTO)
    }
    #[doc = "UP mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn up_auto(self) -> &'a mut W {
        self.variant(WAVSELSELECT_A::UP_AUTO)
    }
    #[doc = "UPDOWN mode with automatic trigger on RC Compare"]
    #[inline(always)]
    pub fn updown_auto(self) -> &'a mut W {
        self.variant(WAVSELSELECT_A::UPDOWN_AUTO)
    }
}
#[doc = "Field `WAVE` reader - WAVE"]
pub type WAVE_R = crate::BitReader<WAVESELECT_A>;
#[doc = "WAVE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAVESELECT_A {
    #[doc = "0: Waveform Mode is disabled (Capture Mode is enabled)."]
    _0 = 0,
    #[doc = "1: Waveform Mode is enabled."]
    _1 = 1,
}
impl From<WAVESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: WAVESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl WAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAVESELECT_A {
        match self.bits {
            false => WAVESELECT_A::_0,
            true => WAVESELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAVESELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAVESELECT_A::_1
    }
}
#[doc = "Field `WAVE` writer - WAVE"]
pub type WAVE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WAVEFORM_CMR_ALT_SPEC, WAVESELECT_A, O>;
impl<'a, const O: u8> WAVE_W<'a, O> {
    #[doc = "Waveform Mode is disabled (Capture Mode is enabled)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAVESELECT_A::_0)
    }
    #[doc = "Waveform Mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAVESELECT_A::_1)
    }
}
#[doc = "Field `ACPA` reader - RA Compare Effect on TIOA"]
pub type ACPA_R = crate::FieldReader<u8, ACPASELECT_A>;
#[doc = "RA Compare Effect on TIOA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACPASELECT_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<ACPASELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACPASELECT_A) -> Self {
        variant as _
    }
}
impl ACPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACPASELECT_A {
        match self.bits {
            0 => ACPASELECT_A::NONE,
            1 => ACPASELECT_A::SET,
            2 => ACPASELECT_A::CLEAR,
            3 => ACPASELECT_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ACPASELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ACPASELECT_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ACPASELECT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == ACPASELECT_A::TOGGLE
    }
}
#[doc = "Field `ACPA` writer - RA Compare Effect on TIOA"]
pub type ACPA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAVEFORM_CMR_ALT_SPEC, u8, ACPASELECT_A, 2, O>;
impl<'a, const O: u8> ACPA_W<'a, O> {
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACPASELECT_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ACPASELECT_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACPASELECT_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ACPASELECT_A::TOGGLE)
    }
}
#[doc = "Field `ACPC` reader - RC Compare Effect on TIOA"]
pub type ACPC_R = crate::FieldReader<u8, ACPCSELECT_A>;
#[doc = "RC Compare Effect on TIOA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ACPCSELECT_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<ACPCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ACPCSELECT_A) -> Self {
        variant as _
    }
}
impl ACPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACPCSELECT_A {
        match self.bits {
            0 => ACPCSELECT_A::NONE,
            1 => ACPCSELECT_A::SET,
            2 => ACPCSELECT_A::CLEAR,
            3 => ACPCSELECT_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ACPCSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ACPCSELECT_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ACPCSELECT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == ACPCSELECT_A::TOGGLE
    }
}
#[doc = "Field `ACPC` writer - RC Compare Effect on TIOA"]
pub type ACPC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAVEFORM_CMR_ALT_SPEC, u8, ACPCSELECT_A, 2, O>;
impl<'a, const O: u8> ACPC_W<'a, O> {
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ACPCSELECT_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ACPCSELECT_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACPCSELECT_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ACPCSELECT_A::TOGGLE)
    }
}
#[doc = "Field `AEEVT` reader - External Event Effect on TIOA"]
pub type AEEVT_R = crate::FieldReader<u8, AEEVTSELECT_A>;
#[doc = "External Event Effect on TIOA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AEEVTSELECT_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<AEEVTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: AEEVTSELECT_A) -> Self {
        variant as _
    }
}
impl AEEVT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEEVTSELECT_A {
        match self.bits {
            0 => AEEVTSELECT_A::NONE,
            1 => AEEVTSELECT_A::SET,
            2 => AEEVTSELECT_A::CLEAR,
            3 => AEEVTSELECT_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == AEEVTSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == AEEVTSELECT_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == AEEVTSELECT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == AEEVTSELECT_A::TOGGLE
    }
}
#[doc = "Field `AEEVT` writer - External Event Effect on TIOA"]
pub type AEEVT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAVEFORM_CMR_ALT_SPEC, u8, AEEVTSELECT_A, 2, O>;
impl<'a, const O: u8> AEEVT_W<'a, O> {
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(AEEVTSELECT_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(AEEVTSELECT_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AEEVTSELECT_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(AEEVTSELECT_A::TOGGLE)
    }
}
#[doc = "Field `ASWTRG` reader - Software Trigger Effect on TIOA"]
pub type ASWTRG_R = crate::FieldReader<u8, ASWTRGSELECT_A>;
#[doc = "Software Trigger Effect on TIOA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASWTRGSELECT_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<ASWTRGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ASWTRGSELECT_A) -> Self {
        variant as _
    }
}
impl ASWTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASWTRGSELECT_A {
        match self.bits {
            0 => ASWTRGSELECT_A::NONE,
            1 => ASWTRGSELECT_A::SET,
            2 => ASWTRGSELECT_A::CLEAR,
            3 => ASWTRGSELECT_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ASWTRGSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ASWTRGSELECT_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ASWTRGSELECT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == ASWTRGSELECT_A::TOGGLE
    }
}
#[doc = "Field `ASWTRG` writer - Software Trigger Effect on TIOA"]
pub type ASWTRG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAVEFORM_CMR_ALT_SPEC, u8, ASWTRGSELECT_A, 2, O>;
impl<'a, const O: u8> ASWTRG_W<'a, O> {
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ASWTRGSELECT_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ASWTRGSELECT_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ASWTRGSELECT_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(ASWTRGSELECT_A::TOGGLE)
    }
}
#[doc = "Field `BCPB` reader - RB Compare Effect on TIOB"]
pub type BCPB_R = crate::FieldReader<u8, BCPBSELECT_A>;
#[doc = "RB Compare Effect on TIOB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCPBSELECT_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<BCPBSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BCPBSELECT_A) -> Self {
        variant as _
    }
}
impl BCPB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCPBSELECT_A {
        match self.bits {
            0 => BCPBSELECT_A::NONE,
            1 => BCPBSELECT_A::SET,
            2 => BCPBSELECT_A::CLEAR,
            3 => BCPBSELECT_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BCPBSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == BCPBSELECT_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == BCPBSELECT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == BCPBSELECT_A::TOGGLE
    }
}
#[doc = "Field `BCPB` writer - RB Compare Effect on TIOB"]
pub type BCPB_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAVEFORM_CMR_ALT_SPEC, u8, BCPBSELECT_A, 2, O>;
impl<'a, const O: u8> BCPB_W<'a, O> {
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BCPBSELECT_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BCPBSELECT_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCPBSELECT_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BCPBSELECT_A::TOGGLE)
    }
}
#[doc = "Field `BCPC` reader - RC Compare Effect on TIOB"]
pub type BCPC_R = crate::FieldReader<u8, BCPCSELECT_A>;
#[doc = "RC Compare Effect on TIOB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BCPCSELECT_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<BCPCSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BCPCSELECT_A) -> Self {
        variant as _
    }
}
impl BCPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCPCSELECT_A {
        match self.bits {
            0 => BCPCSELECT_A::NONE,
            1 => BCPCSELECT_A::SET,
            2 => BCPCSELECT_A::CLEAR,
            3 => BCPCSELECT_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BCPCSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == BCPCSELECT_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == BCPCSELECT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == BCPCSELECT_A::TOGGLE
    }
}
#[doc = "Field `BCPC` writer - RC Compare Effect on TIOB"]
pub type BCPC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAVEFORM_CMR_ALT_SPEC, u8, BCPCSELECT_A, 2, O>;
impl<'a, const O: u8> BCPC_W<'a, O> {
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BCPCSELECT_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BCPCSELECT_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BCPCSELECT_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BCPCSELECT_A::TOGGLE)
    }
}
#[doc = "Field `BEEVT` reader - External Event Effect on TIOB"]
pub type BEEVT_R = crate::FieldReader<u8, BEEVTSELECT_A>;
#[doc = "External Event Effect on TIOB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BEEVTSELECT_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<BEEVTSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BEEVTSELECT_A) -> Self {
        variant as _
    }
}
impl BEEVT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEEVTSELECT_A {
        match self.bits {
            0 => BEEVTSELECT_A::NONE,
            1 => BEEVTSELECT_A::SET,
            2 => BEEVTSELECT_A::CLEAR,
            3 => BEEVTSELECT_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BEEVTSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == BEEVTSELECT_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == BEEVTSELECT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == BEEVTSELECT_A::TOGGLE
    }
}
#[doc = "Field `BEEVT` writer - External Event Effect on TIOB"]
pub type BEEVT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAVEFORM_CMR_ALT_SPEC, u8, BEEVTSELECT_A, 2, O>;
impl<'a, const O: u8> BEEVT_W<'a, O> {
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BEEVTSELECT_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BEEVTSELECT_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BEEVTSELECT_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BEEVTSELECT_A::TOGGLE)
    }
}
#[doc = "Field `BSWTRG` reader - Software Trigger Effect on TIOB"]
pub type BSWTRG_R = crate::FieldReader<u8, BSWTRGSELECT_A>;
#[doc = "Software Trigger Effect on TIOB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BSWTRGSELECT_A {
    #[doc = "0: none"]
    NONE = 0,
    #[doc = "1: set"]
    SET = 1,
    #[doc = "2: clear"]
    CLEAR = 2,
    #[doc = "3: toggle"]
    TOGGLE = 3,
}
impl From<BSWTRGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: BSWTRGSELECT_A) -> Self {
        variant as _
    }
}
impl BSWTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSWTRGSELECT_A {
        match self.bits {
            0 => BSWTRGSELECT_A::NONE,
            1 => BSWTRGSELECT_A::SET,
            2 => BSWTRGSELECT_A::CLEAR,
            3 => BSWTRGSELECT_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BSWTRGSELECT_A::NONE
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == BSWTRGSELECT_A::SET
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == BSWTRGSELECT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == BSWTRGSELECT_A::TOGGLE
    }
}
#[doc = "Field `BSWTRG` writer - Software Trigger Effect on TIOB"]
pub type BSWTRG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, WAVEFORM_CMR_ALT_SPEC, u8, BSWTRGSELECT_A, 2, O>;
impl<'a, const O: u8> BSWTRG_W<'a, O> {
    #[doc = "none"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BSWTRGSELECT_A::NONE)
    }
    #[doc = "set"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(BSWTRGSELECT_A::SET)
    }
    #[doc = "clear"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(BSWTRGSELECT_A::CLEAR)
    }
    #[doc = "toggle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(BSWTRGSELECT_A::TOGGLE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    pub fn tcclks(&self) -> TCCLKS_R {
        TCCLKS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    pub fn clki(&self) -> CLKI_R {
        CLKI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    pub fn burst(&self) -> BURST_R {
        BURST_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline(always)]
    pub fn cpcstop(&self) -> CPCSTOP_R {
        CPCSTOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Compare"]
    #[inline(always)]
    pub fn cpcdis(&self) -> CPCDIS_R {
        CPCDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline(always)]
    pub fn eevtedg(&self) -> EEVTEDG_R {
        EEVTEDG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline(always)]
    pub fn eevt(&self) -> EEVT_R {
        EEVT_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline(always)]
    pub fn enetrg(&self) -> ENETRG_R {
        ENETRG_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline(always)]
    pub fn wavsel(&self) -> WAVSEL_R {
        WAVSEL_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - WAVE"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOA"]
    #[inline(always)]
    pub fn acpa(&self) -> ACPA_R {
        ACPA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOA"]
    #[inline(always)]
    pub fn acpc(&self) -> ACPC_R {
        ACPC_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOA"]
    #[inline(always)]
    pub fn aeevt(&self) -> AEEVT_R {
        AEEVT_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOA"]
    #[inline(always)]
    pub fn aswtrg(&self) -> ASWTRG_R {
        ASWTRG_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOB"]
    #[inline(always)]
    pub fn bcpb(&self) -> BCPB_R {
        BCPB_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOB"]
    #[inline(always)]
    pub fn bcpc(&self) -> BCPC_R {
        BCPC_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOB"]
    #[inline(always)]
    pub fn beevt(&self) -> BEEVT_R {
        BEEVT_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOB"]
    #[inline(always)]
    pub fn bswtrg(&self) -> BSWTRG_R {
        BSWTRG_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Selection"]
    #[inline(always)]
    #[must_use]
    pub fn tcclks(&mut self) -> TCCLKS_W<0> {
        TCCLKS_W::new(self)
    }
    #[doc = "Bit 3 - Clock Invert"]
    #[inline(always)]
    #[must_use]
    pub fn clki(&mut self) -> CLKI_W<3> {
        CLKI_W::new(self)
    }
    #[doc = "Bits 4:5 - Burst Signal Selection"]
    #[inline(always)]
    #[must_use]
    pub fn burst(&mut self) -> BURST_W<4> {
        BURST_W::new(self)
    }
    #[doc = "Bit 6 - Counter Clock Stopped with RC Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpcstop(&mut self) -> CPCSTOP_W<6> {
        CPCSTOP_W::new(self)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RC Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpcdis(&mut self) -> CPCDIS_W<7> {
        CPCDIS_W::new(self)
    }
    #[doc = "Bits 8:9 - External Event Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn eevtedg(&mut self) -> EEVTEDG_W<8> {
        EEVTEDG_W::new(self)
    }
    #[doc = "Bits 10:11 - External Event Selection"]
    #[inline(always)]
    #[must_use]
    pub fn eevt(&mut self) -> EEVT_W<10> {
        EEVT_W::new(self)
    }
    #[doc = "Bit 12 - External Event Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enetrg(&mut self) -> ENETRG_W<12> {
        ENETRG_W::new(self)
    }
    #[doc = "Bits 13:14 - Waveform Selection"]
    #[inline(always)]
    #[must_use]
    pub fn wavsel(&mut self) -> WAVSEL_W<13> {
        WAVSEL_W::new(self)
    }
    #[doc = "Bit 15 - WAVE"]
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WAVE_W<15> {
        WAVE_W::new(self)
    }
    #[doc = "Bits 16:17 - RA Compare Effect on TIOA"]
    #[inline(always)]
    #[must_use]
    pub fn acpa(&mut self) -> ACPA_W<16> {
        ACPA_W::new(self)
    }
    #[doc = "Bits 18:19 - RC Compare Effect on TIOA"]
    #[inline(always)]
    #[must_use]
    pub fn acpc(&mut self) -> ACPC_W<18> {
        ACPC_W::new(self)
    }
    #[doc = "Bits 20:21 - External Event Effect on TIOA"]
    #[inline(always)]
    #[must_use]
    pub fn aeevt(&mut self) -> AEEVT_W<20> {
        AEEVT_W::new(self)
    }
    #[doc = "Bits 22:23 - Software Trigger Effect on TIOA"]
    #[inline(always)]
    #[must_use]
    pub fn aswtrg(&mut self) -> ASWTRG_W<22> {
        ASWTRG_W::new(self)
    }
    #[doc = "Bits 24:25 - RB Compare Effect on TIOB"]
    #[inline(always)]
    #[must_use]
    pub fn bcpb(&mut self) -> BCPB_W<24> {
        BCPB_W::new(self)
    }
    #[doc = "Bits 26:27 - RC Compare Effect on TIOB"]
    #[inline(always)]
    #[must_use]
    pub fn bcpc(&mut self) -> BCPC_W<26> {
        BCPC_W::new(self)
    }
    #[doc = "Bits 28:29 - External Event Effect on TIOB"]
    #[inline(always)]
    #[must_use]
    pub fn beevt(&mut self) -> BEEVT_W<28> {
        BEEVT_W::new(self)
    }
    #[doc = "Bits 30:31 - Software Trigger Effect on TIOB"]
    #[inline(always)]
    #[must_use]
    pub fn bswtrg(&mut self) -> BSWTRG_W<30> {
        BSWTRG_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMR%s_ALT to value 0"]
impl crate::Resettable for WAVEFORM_CMR_ALT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
