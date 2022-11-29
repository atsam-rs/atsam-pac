#[doc = "Register `CMR%s` reader"]
pub struct R(crate::R<CAPTURE_CMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAPTURE_CMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAPTURE_CMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAPTURE_CMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMR%s` writer"]
pub struct W(crate::W<CAPTURE_CMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAPTURE_CMR_SPEC>;
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
impl From<crate::W<CAPTURE_CMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAPTURE_CMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCCLKS` reader - Clock Selection"]
pub type TCCLKS_R = crate::FieldReader<u8, TCCLKSSELECT_A>;
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCCLKSSELECT_A {
    #[doc = "0: TIMER_CLOCK1"]
    TIMER_CLOCK1 = 0,
    #[doc = "1: TIMER_CLOCK2"]
    TIMER_CLOCK2 = 1,
    #[doc = "2: TIMER_CLOCK3"]
    TIMER_CLOCK3 = 2,
    #[doc = "3: TIMER_CLOCK4"]
    TIMER_CLOCK4 = 3,
    #[doc = "4: TIMER_CLOCK5"]
    TIMER_CLOCK5 = 4,
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
            0 => TCCLKSSELECT_A::TIMER_CLOCK1,
            1 => TCCLKSSELECT_A::TIMER_CLOCK2,
            2 => TCCLKSSELECT_A::TIMER_CLOCK3,
            3 => TCCLKSSELECT_A::TIMER_CLOCK4,
            4 => TCCLKSSELECT_A::TIMER_CLOCK5,
            5 => TCCLKSSELECT_A::XC0,
            6 => TCCLKSSELECT_A::XC1,
            7 => TCCLKSSELECT_A::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK1`"]
    #[inline(always)]
    pub fn is_timer_clock1(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_CLOCK1
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK2`"]
    #[inline(always)]
    pub fn is_timer_clock2(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_CLOCK2
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK3`"]
    #[inline(always)]
    pub fn is_timer_clock3(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_CLOCK3
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK4`"]
    #[inline(always)]
    pub fn is_timer_clock4(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_CLOCK4
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK5`"]
    #[inline(always)]
    pub fn is_timer_clock5(&self) -> bool {
        *self == TCCLKSSELECT_A::TIMER_CLOCK5
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
    crate::FieldWriterSafe<'a, u32, CAPTURE_CMR_SPEC, u8, TCCLKSSELECT_A, 3, O>;
impl<'a, const O: u8> TCCLKS_W<'a, O> {
    #[doc = "TIMER_CLOCK1"]
    #[inline(always)]
    pub fn timer_clock1(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_CLOCK1)
    }
    #[doc = "TIMER_CLOCK2"]
    #[inline(always)]
    pub fn timer_clock2(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_CLOCK2)
    }
    #[doc = "TIMER_CLOCK3"]
    #[inline(always)]
    pub fn timer_clock3(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_CLOCK3)
    }
    #[doc = "TIMER_CLOCK4"]
    #[inline(always)]
    pub fn timer_clock4(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_CLOCK4)
    }
    #[doc = "TIMER_CLOCK5"]
    #[inline(always)]
    pub fn timer_clock5(self) -> &'a mut W {
        self.variant(TCCLKSSELECT_A::TIMER_CLOCK5)
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
pub type CLKI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPTURE_CMR_SPEC, CLKISELECT_A, O>;
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
    crate::FieldWriterSafe<'a, u32, CAPTURE_CMR_SPEC, u8, BURSTSELECT_A, 2, O>;
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
#[doc = "Field `LDBSTOP` reader - Counter Clock Stopped with RB Loading"]
pub type LDBSTOP_R = crate::BitReader<LDBSTOPSELECT_A>;
#[doc = "Counter Clock Stopped with RB Loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDBSTOPSELECT_A {
    #[doc = "0: Counter clock is not stopped when RB loading occurs."]
    _0 = 0,
    #[doc = "1: Counter clock is stopped when RB loading occurs."]
    _1 = 1,
}
impl From<LDBSTOPSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LDBSTOPSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LDBSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDBSTOPSELECT_A {
        match self.bits {
            false => LDBSTOPSELECT_A::_0,
            true => LDBSTOPSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDBSTOPSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDBSTOPSELECT_A::_1
    }
}
#[doc = "Field `LDBSTOP` writer - Counter Clock Stopped with RB Loading"]
pub type LDBSTOP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CAPTURE_CMR_SPEC, LDBSTOPSELECT_A, O>;
impl<'a, const O: u8> LDBSTOP_W<'a, O> {
    #[doc = "Counter clock is not stopped when RB loading occurs."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDBSTOPSELECT_A::_0)
    }
    #[doc = "Counter clock is stopped when RB loading occurs."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDBSTOPSELECT_A::_1)
    }
}
#[doc = "Field `LDBDIS` reader - Counter Clock Disable with RB Loading"]
pub type LDBDIS_R = crate::BitReader<LDBDISSELECT_A>;
#[doc = "Counter Clock Disable with RB Loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDBDISSELECT_A {
    #[doc = "0: Counter clock is not disabled when RB loading occurs."]
    _0 = 0,
    #[doc = "1: Counter clock is disabled when RB loading occurs."]
    _1 = 1,
}
impl From<LDBDISSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: LDBDISSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl LDBDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDBDISSELECT_A {
        match self.bits {
            false => LDBDISSELECT_A::_0,
            true => LDBDISSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDBDISSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDBDISSELECT_A::_1
    }
}
#[doc = "Field `LDBDIS` writer - Counter Clock Disable with RB Loading"]
pub type LDBDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPTURE_CMR_SPEC, LDBDISSELECT_A, O>;
impl<'a, const O: u8> LDBDIS_W<'a, O> {
    #[doc = "Counter clock is not disabled when RB loading occurs."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDBDISSELECT_A::_0)
    }
    #[doc = "Counter clock is disabled when RB loading occurs."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDBDISSELECT_A::_1)
    }
}
#[doc = "Field `ETRGEDG` reader - External Trigger Edge Selection"]
pub type ETRGEDG_R = crate::FieldReader<u8, ETRGEDGSELECT_A>;
#[doc = "External Trigger Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETRGEDGSELECT_A {
    #[doc = "0: none"]
    NO_EDGE = 0,
    #[doc = "1: rising edge"]
    POS_EDGE = 1,
    #[doc = "2: falling edge"]
    NEG_EDGE = 2,
    #[doc = "3: each edge"]
    BOTH_EDGES = 3,
}
impl From<ETRGEDGSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ETRGEDGSELECT_A) -> Self {
        variant as _
    }
}
impl ETRGEDG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETRGEDGSELECT_A {
        match self.bits {
            0 => ETRGEDGSELECT_A::NO_EDGE,
            1 => ETRGEDGSELECT_A::POS_EDGE,
            2 => ETRGEDGSELECT_A::NEG_EDGE,
            3 => ETRGEDGSELECT_A::BOTH_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == ETRGEDGSELECT_A::NO_EDGE
    }
    #[doc = "Checks if the value of the field is `POS_EDGE`"]
    #[inline(always)]
    pub fn is_pos_edge(&self) -> bool {
        *self == ETRGEDGSELECT_A::POS_EDGE
    }
    #[doc = "Checks if the value of the field is `NEG_EDGE`"]
    #[inline(always)]
    pub fn is_neg_edge(&self) -> bool {
        *self == ETRGEDGSELECT_A::NEG_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == ETRGEDGSELECT_A::BOTH_EDGES
    }
}
#[doc = "Field `ETRGEDG` writer - External Trigger Edge Selection"]
pub type ETRGEDG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CAPTURE_CMR_SPEC, u8, ETRGEDGSELECT_A, 2, O>;
impl<'a, const O: u8> ETRGEDG_W<'a, O> {
    #[doc = "none"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(ETRGEDGSELECT_A::NO_EDGE)
    }
    #[doc = "rising edge"]
    #[inline(always)]
    pub fn pos_edge(self) -> &'a mut W {
        self.variant(ETRGEDGSELECT_A::POS_EDGE)
    }
    #[doc = "falling edge"]
    #[inline(always)]
    pub fn neg_edge(self) -> &'a mut W {
        self.variant(ETRGEDGSELECT_A::NEG_EDGE)
    }
    #[doc = "each edge"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(ETRGEDGSELECT_A::BOTH_EDGES)
    }
}
#[doc = "Field `ABETRG` reader - TIOA or TIOB External Trigger Selection"]
pub type ABETRG_R = crate::BitReader<ABETRGSELECT_A>;
#[doc = "TIOA or TIOB External Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ABETRGSELECT_A {
    #[doc = "0: TIOB is used as an external trigger."]
    _0 = 0,
    #[doc = "1: TIOA is used as an external trigger."]
    _1 = 1,
}
impl From<ABETRGSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ABETRGSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ABETRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABETRGSELECT_A {
        match self.bits {
            false => ABETRGSELECT_A::_0,
            true => ABETRGSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABETRGSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABETRGSELECT_A::_1
    }
}
#[doc = "Field `ABETRG` writer - TIOA or TIOB External Trigger Selection"]
pub type ABETRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPTURE_CMR_SPEC, ABETRGSELECT_A, O>;
impl<'a, const O: u8> ABETRG_W<'a, O> {
    #[doc = "TIOB is used as an external trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABETRGSELECT_A::_0)
    }
    #[doc = "TIOA is used as an external trigger."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABETRGSELECT_A::_1)
    }
}
#[doc = "Field `CPCTRG` reader - RC Compare Trigger Enable"]
pub type CPCTRG_R = crate::BitReader<CPCTRGSELECT_A>;
#[doc = "RC Compare Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPCTRGSELECT_A {
    #[doc = "0: RC Compare has no effect on the counter and its clock."]
    _0 = 0,
    #[doc = "1: RC Compare resets the counter and starts the counter clock."]
    _1 = 1,
}
impl From<CPCTRGSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CPCTRGSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CPCTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPCTRGSELECT_A {
        match self.bits {
            false => CPCTRGSELECT_A::_0,
            true => CPCTRGSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPCTRGSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPCTRGSELECT_A::_1
    }
}
#[doc = "Field `CPCTRG` writer - RC Compare Trigger Enable"]
pub type CPCTRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPTURE_CMR_SPEC, CPCTRGSELECT_A, O>;
impl<'a, const O: u8> CPCTRG_W<'a, O> {
    #[doc = "RC Compare has no effect on the counter and its clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPCTRGSELECT_A::_0)
    }
    #[doc = "RC Compare resets the counter and starts the counter clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPCTRGSELECT_A::_1)
    }
}
#[doc = "Field `WAVE` reader - Wave"]
pub type WAVE_R = crate::BitReader<WAVESELECT_A>;
#[doc = "Wave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAVESELECT_A {
    #[doc = "0: Capture Mode is enabled."]
    _0 = 0,
    #[doc = "1: Capture Mode is disabled (Waveform Mode is enabled)."]
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
#[doc = "Field `WAVE` writer - Wave"]
pub type WAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAPTURE_CMR_SPEC, WAVESELECT_A, O>;
impl<'a, const O: u8> WAVE_W<'a, O> {
    #[doc = "Capture Mode is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAVESELECT_A::_0)
    }
    #[doc = "Capture Mode is disabled (Waveform Mode is enabled)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAVESELECT_A::_1)
    }
}
#[doc = "Field `LDRA` reader - RA Loading Selection"]
pub type LDRA_R = crate::FieldReader<u8, LDRASELECT_A>;
#[doc = "RA Loading Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDRASELECT_A {
    #[doc = "0: none"]
    NO_EDGE = 0,
    #[doc = "1: rising edge of TIOA"]
    POS_EDGE_TIOA = 1,
    #[doc = "2: falling edge of TIOA"]
    NEG_EDGE_TIOA = 2,
    #[doc = "3: each edge of TIOA"]
    BOTH_EDGES_TIOA = 3,
}
impl From<LDRASELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LDRASELECT_A) -> Self {
        variant as _
    }
}
impl LDRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRASELECT_A {
        match self.bits {
            0 => LDRASELECT_A::NO_EDGE,
            1 => LDRASELECT_A::POS_EDGE_TIOA,
            2 => LDRASELECT_A::NEG_EDGE_TIOA,
            3 => LDRASELECT_A::BOTH_EDGES_TIOA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == LDRASELECT_A::NO_EDGE
    }
    #[doc = "Checks if the value of the field is `POS_EDGE_TIOA`"]
    #[inline(always)]
    pub fn is_pos_edge_tioa(&self) -> bool {
        *self == LDRASELECT_A::POS_EDGE_TIOA
    }
    #[doc = "Checks if the value of the field is `NEG_EDGE_TIOA`"]
    #[inline(always)]
    pub fn is_neg_edge_tioa(&self) -> bool {
        *self == LDRASELECT_A::NEG_EDGE_TIOA
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES_TIOA`"]
    #[inline(always)]
    pub fn is_both_edges_tioa(&self) -> bool {
        *self == LDRASELECT_A::BOTH_EDGES_TIOA
    }
}
#[doc = "Field `LDRA` writer - RA Loading Selection"]
pub type LDRA_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CAPTURE_CMR_SPEC, u8, LDRASELECT_A, 2, O>;
impl<'a, const O: u8> LDRA_W<'a, O> {
    #[doc = "none"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(LDRASELECT_A::NO_EDGE)
    }
    #[doc = "rising edge of TIOA"]
    #[inline(always)]
    pub fn pos_edge_tioa(self) -> &'a mut W {
        self.variant(LDRASELECT_A::POS_EDGE_TIOA)
    }
    #[doc = "falling edge of TIOA"]
    #[inline(always)]
    pub fn neg_edge_tioa(self) -> &'a mut W {
        self.variant(LDRASELECT_A::NEG_EDGE_TIOA)
    }
    #[doc = "each edge of TIOA"]
    #[inline(always)]
    pub fn both_edges_tioa(self) -> &'a mut W {
        self.variant(LDRASELECT_A::BOTH_EDGES_TIOA)
    }
}
#[doc = "Field `LDRB` reader - RB Loading Selection"]
pub type LDRB_R = crate::FieldReader<u8, LDRBSELECT_A>;
#[doc = "RB Loading Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDRBSELECT_A {
    #[doc = "0: none"]
    NO_EDGE = 0,
    #[doc = "1: rising edge of TIOA"]
    POS_EDGE_TIOA = 1,
    #[doc = "2: falling edge of TIOA"]
    NEG_EDGE_TIOA = 2,
    #[doc = "3: each edge of TIOA"]
    BOTH_EDGES_TIOA = 3,
}
impl From<LDRBSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: LDRBSELECT_A) -> Self {
        variant as _
    }
}
impl LDRB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRBSELECT_A {
        match self.bits {
            0 => LDRBSELECT_A::NO_EDGE,
            1 => LDRBSELECT_A::POS_EDGE_TIOA,
            2 => LDRBSELECT_A::NEG_EDGE_TIOA,
            3 => LDRBSELECT_A::BOTH_EDGES_TIOA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == LDRBSELECT_A::NO_EDGE
    }
    #[doc = "Checks if the value of the field is `POS_EDGE_TIOA`"]
    #[inline(always)]
    pub fn is_pos_edge_tioa(&self) -> bool {
        *self == LDRBSELECT_A::POS_EDGE_TIOA
    }
    #[doc = "Checks if the value of the field is `NEG_EDGE_TIOA`"]
    #[inline(always)]
    pub fn is_neg_edge_tioa(&self) -> bool {
        *self == LDRBSELECT_A::NEG_EDGE_TIOA
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES_TIOA`"]
    #[inline(always)]
    pub fn is_both_edges_tioa(&self) -> bool {
        *self == LDRBSELECT_A::BOTH_EDGES_TIOA
    }
}
#[doc = "Field `LDRB` writer - RB Loading Selection"]
pub type LDRB_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CAPTURE_CMR_SPEC, u8, LDRBSELECT_A, 2, O>;
impl<'a, const O: u8> LDRB_W<'a, O> {
    #[doc = "none"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(LDRBSELECT_A::NO_EDGE)
    }
    #[doc = "rising edge of TIOA"]
    #[inline(always)]
    pub fn pos_edge_tioa(self) -> &'a mut W {
        self.variant(LDRBSELECT_A::POS_EDGE_TIOA)
    }
    #[doc = "falling edge of TIOA"]
    #[inline(always)]
    pub fn neg_edge_tioa(self) -> &'a mut W {
        self.variant(LDRBSELECT_A::NEG_EDGE_TIOA)
    }
    #[doc = "each edge of TIOA"]
    #[inline(always)]
    pub fn both_edges_tioa(self) -> &'a mut W {
        self.variant(LDRBSELECT_A::BOTH_EDGES_TIOA)
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
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&self) -> LDBSTOP_R {
        LDBSTOP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&self) -> LDBDIS_R {
        LDBDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&self) -> ETRGEDG_R {
        ETRGEDG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - TIOA or TIOB External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&self) -> ABETRG_R {
        ABETRG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&self) -> CPCTRG_R {
        CPCTRG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Wave"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - RA Loading Selection"]
    #[inline(always)]
    pub fn ldra(&self) -> LDRA_R {
        LDRA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - RB Loading Selection"]
    #[inline(always)]
    pub fn ldrb(&self) -> LDRB_R {
        LDRB_R::new(((self.bits >> 18) & 3) as u8)
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
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldbstop(&mut self) -> LDBSTOP_W<6> {
        LDBSTOP_W::new(self)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldbdis(&mut self) -> LDBDIS_W<7> {
        LDBDIS_W::new(self)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn etrgedg(&mut self) -> ETRGEDG_W<8> {
        ETRGEDG_W::new(self)
    }
    #[doc = "Bit 10 - TIOA or TIOB External Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn abetrg(&mut self) -> ABETRG_W<10> {
        ABETRG_W::new(self)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cpctrg(&mut self) -> CPCTRG_W<14> {
        CPCTRG_W::new(self)
    }
    #[doc = "Bit 15 - Wave"]
    #[inline(always)]
    #[must_use]
    pub fn wave(&mut self) -> WAVE_W<15> {
        WAVE_W::new(self)
    }
    #[doc = "Bits 16:17 - RA Loading Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ldra(&mut self) -> LDRA_W<16> {
        LDRA_W::new(self)
    }
    #[doc = "Bits 18:19 - RB Loading Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ldrb(&mut self) -> LDRB_W<18> {
        LDRB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Mode Register Channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [capture_cmr](index.html) module"]
pub struct CAPTURE_CMR_SPEC;
impl crate::RegisterSpec for CAPTURE_CMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [capture_cmr::R](R) reader structure"]
impl crate::Readable for CAPTURE_CMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [capture_cmr::W](W) writer structure"]
impl crate::Writable for CAPTURE_CMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMR%s to value 0"]
impl crate::Resettable for CAPTURE_CMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
