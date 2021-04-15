#[doc = "Reader of register CMR%s"]
pub type R = crate::R<u32, super::CMR>;
#[doc = "Writer for register CMR%s"]
pub type W = crate::W<u32, super::CMR>;
#[doc = "Register CMR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TCCLKS_A {
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
impl From<TCCLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: TCCLKS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TCCLKS`"]
pub type TCCLKS_R = crate::R<u8, TCCLKS_A>;
impl TCCLKS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCCLKS_A {
        match self.bits {
            0 => TCCLKS_A::TIMER_CLOCK1,
            1 => TCCLKS_A::TIMER_CLOCK2,
            2 => TCCLKS_A::TIMER_CLOCK3,
            3 => TCCLKS_A::TIMER_CLOCK4,
            4 => TCCLKS_A::TIMER_CLOCK5,
            5 => TCCLKS_A::XC0,
            6 => TCCLKS_A::XC1,
            7 => TCCLKS_A::XC2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK1`"]
    #[inline(always)]
    pub fn is_timer_clock1(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK1
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK2`"]
    #[inline(always)]
    pub fn is_timer_clock2(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK2
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK3`"]
    #[inline(always)]
    pub fn is_timer_clock3(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK3
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK4`"]
    #[inline(always)]
    pub fn is_timer_clock4(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK4
    }
    #[doc = "Checks if the value of the field is `TIMER_CLOCK5`"]
    #[inline(always)]
    pub fn is_timer_clock5(&self) -> bool {
        *self == TCCLKS_A::TIMER_CLOCK5
    }
    #[doc = "Checks if the value of the field is `XC0`"]
    #[inline(always)]
    pub fn is_xc0(&self) -> bool {
        *self == TCCLKS_A::XC0
    }
    #[doc = "Checks if the value of the field is `XC1`"]
    #[inline(always)]
    pub fn is_xc1(&self) -> bool {
        *self == TCCLKS_A::XC1
    }
    #[doc = "Checks if the value of the field is `XC2`"]
    #[inline(always)]
    pub fn is_xc2(&self) -> bool {
        *self == TCCLKS_A::XC2
    }
}
#[doc = "Write proxy for field `TCCLKS`"]
pub struct TCCLKS_W<'a> {
    w: &'a mut W,
}
impl<'a> TCCLKS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TCCLKS_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "TIMER_CLOCK1"]
    #[inline(always)]
    pub fn timer_clock1(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK1)
    }
    #[doc = "TIMER_CLOCK2"]
    #[inline(always)]
    pub fn timer_clock2(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK2)
    }
    #[doc = "TIMER_CLOCK3"]
    #[inline(always)]
    pub fn timer_clock3(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK3)
    }
    #[doc = "TIMER_CLOCK4"]
    #[inline(always)]
    pub fn timer_clock4(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK4)
    }
    #[doc = "TIMER_CLOCK5"]
    #[inline(always)]
    pub fn timer_clock5(self) -> &'a mut W {
        self.variant(TCCLKS_A::TIMER_CLOCK5)
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
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
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
#[doc = "Reader of field `CLKI`"]
pub type CLKI_R = crate::R<bool, CLKI_A>;
impl CLKI_R {
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
        *self == CLKI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKI_A::_1
    }
}
#[doc = "Write proxy for field `CLKI`"]
pub struct CLKI_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKI_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKI_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
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
#[doc = "Reader of field `BURST`"]
pub type BURST_R = crate::R<u8, BURST_A>;
impl BURST_R {
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
        *self == BURST_A::NOT_GATED
    }
    #[doc = "Checks if the value of the field is `CLK_AND_XC0`"]
    #[inline(always)]
    pub fn is_clk_and_xc0(&self) -> bool {
        *self == BURST_A::CLK_AND_XC0
    }
    #[doc = "Checks if the value of the field is `CLK_AND_XC1`"]
    #[inline(always)]
    pub fn is_clk_and_xc1(&self) -> bool {
        *self == BURST_A::CLK_AND_XC1
    }
    #[doc = "Checks if the value of the field is `CLK_AND_XC2`"]
    #[inline(always)]
    pub fn is_clk_and_xc2(&self) -> bool {
        *self == BURST_A::CLK_AND_XC2
    }
}
#[doc = "Write proxy for field `BURST`"]
pub struct BURST_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURST_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Counter Clock Stopped with RB Loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDBSTOP_A {
    #[doc = "0: Counter clock is not stopped when RB loading occurs."]
    _0 = 0,
    #[doc = "1: Counter clock is stopped when RB loading occurs."]
    _1 = 1,
}
impl From<LDBSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: LDBSTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LDBSTOP`"]
pub type LDBSTOP_R = crate::R<bool, LDBSTOP_A>;
impl LDBSTOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDBSTOP_A {
        match self.bits {
            false => LDBSTOP_A::_0,
            true => LDBSTOP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDBSTOP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDBSTOP_A::_1
    }
}
#[doc = "Write proxy for field `LDBSTOP`"]
pub struct LDBSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> LDBSTOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDBSTOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter clock is not stopped when RB loading occurs."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDBSTOP_A::_0)
    }
    #[doc = "Counter clock is stopped when RB loading occurs."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDBSTOP_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Counter Clock Disable with RB Loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDBDIS_A {
    #[doc = "0: Counter clock is not disabled when RB loading occurs."]
    _0 = 0,
    #[doc = "1: Counter clock is disabled when RB loading occurs."]
    _1 = 1,
}
impl From<LDBDIS_A> for bool {
    #[inline(always)]
    fn from(variant: LDBDIS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LDBDIS`"]
pub type LDBDIS_R = crate::R<bool, LDBDIS_A>;
impl LDBDIS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDBDIS_A {
        match self.bits {
            false => LDBDIS_A::_0,
            true => LDBDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDBDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDBDIS_A::_1
    }
}
#[doc = "Write proxy for field `LDBDIS`"]
pub struct LDBDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LDBDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDBDIS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Counter clock is not disabled when RB loading occurs."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDBDIS_A::_0)
    }
    #[doc = "Counter clock is disabled when RB loading occurs."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDBDIS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "External Trigger Edge Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ETRGEDG_A {
    #[doc = "0: none"]
    NO_EDGE = 0,
    #[doc = "1: rising edge"]
    POS_EDGE = 1,
    #[doc = "2: falling edge"]
    NEG_EDGE = 2,
    #[doc = "3: each edge"]
    BOTH_EDGES = 3,
}
impl From<ETRGEDG_A> for u8 {
    #[inline(always)]
    fn from(variant: ETRGEDG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ETRGEDG`"]
pub type ETRGEDG_R = crate::R<u8, ETRGEDG_A>;
impl ETRGEDG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETRGEDG_A {
        match self.bits {
            0 => ETRGEDG_A::NO_EDGE,
            1 => ETRGEDG_A::POS_EDGE,
            2 => ETRGEDG_A::NEG_EDGE,
            3 => ETRGEDG_A::BOTH_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == ETRGEDG_A::NO_EDGE
    }
    #[doc = "Checks if the value of the field is `POS_EDGE`"]
    #[inline(always)]
    pub fn is_pos_edge(&self) -> bool {
        *self == ETRGEDG_A::POS_EDGE
    }
    #[doc = "Checks if the value of the field is `NEG_EDGE`"]
    #[inline(always)]
    pub fn is_neg_edge(&self) -> bool {
        *self == ETRGEDG_A::NEG_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == ETRGEDG_A::BOTH_EDGES
    }
}
#[doc = "Write proxy for field `ETRGEDG`"]
pub struct ETRGEDG_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRGEDG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETRGEDG_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(ETRGEDG_A::NO_EDGE)
    }
    #[doc = "rising edge"]
    #[inline(always)]
    pub fn pos_edge(self) -> &'a mut W {
        self.variant(ETRGEDG_A::POS_EDGE)
    }
    #[doc = "falling edge"]
    #[inline(always)]
    pub fn neg_edge(self) -> &'a mut W {
        self.variant(ETRGEDG_A::NEG_EDGE)
    }
    #[doc = "each edge"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(ETRGEDG_A::BOTH_EDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "TIOA or TIOB External Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABETRG_A {
    #[doc = "0: TIOB is used as an external trigger."]
    _0 = 0,
    #[doc = "1: TIOA is used as an external trigger."]
    _1 = 1,
}
impl From<ABETRG_A> for bool {
    #[inline(always)]
    fn from(variant: ABETRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ABETRG`"]
pub type ABETRG_R = crate::R<bool, ABETRG_A>;
impl ABETRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABETRG_A {
        match self.bits {
            false => ABETRG_A::_0,
            true => ABETRG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ABETRG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ABETRG_A::_1
    }
}
#[doc = "Write proxy for field `ABETRG`"]
pub struct ABETRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ABETRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABETRG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "TIOB is used as an external trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ABETRG_A::_0)
    }
    #[doc = "TIOA is used as an external trigger."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ABETRG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "RC Compare Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPCTRG_A {
    #[doc = "0: RC Compare has no effect on the counter and its clock."]
    _0 = 0,
    #[doc = "1: RC Compare resets the counter and starts the counter clock."]
    _1 = 1,
}
impl From<CPCTRG_A> for bool {
    #[inline(always)]
    fn from(variant: CPCTRG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPCTRG`"]
pub type CPCTRG_R = crate::R<bool, CPCTRG_A>;
impl CPCTRG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPCTRG_A {
        match self.bits {
            false => CPCTRG_A::_0,
            true => CPCTRG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPCTRG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPCTRG_A::_1
    }
}
#[doc = "Write proxy for field `CPCTRG`"]
pub struct CPCTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPCTRG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RC Compare has no effect on the counter and its clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPCTRG_A::_0)
    }
    #[doc = "RC Compare resets the counter and starts the counter clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPCTRG_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Wave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAVE_A {
    #[doc = "0: Capture Mode is enabled."]
    _0 = 0,
    #[doc = "1: Capture Mode is disabled (Waveform Mode is enabled)."]
    _1 = 1,
}
impl From<WAVE_A> for bool {
    #[inline(always)]
    fn from(variant: WAVE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAVE`"]
pub type WAVE_R = crate::R<bool, WAVE_A>;
impl WAVE_R {
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
        *self == WAVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAVE_A::_1
    }
}
#[doc = "Write proxy for field `WAVE`"]
pub struct WAVE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAVE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Capture Mode is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAVE_A::_0)
    }
    #[doc = "Capture Mode is disabled (Waveform Mode is enabled)."]
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "RA Loading Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LDRA_A {
    #[doc = "0: none"]
    NO_EDGE = 0,
    #[doc = "1: rising edge of TIOA"]
    POS_EDGE_TIOA = 1,
    #[doc = "2: falling edge of TIOA"]
    NEG_EDGE_TIOA = 2,
    #[doc = "3: each edge of TIOA"]
    BOTH_EDGES_TIOA = 3,
}
impl From<LDRA_A> for u8 {
    #[inline(always)]
    fn from(variant: LDRA_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LDRA`"]
pub type LDRA_R = crate::R<u8, LDRA_A>;
impl LDRA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRA_A {
        match self.bits {
            0 => LDRA_A::NO_EDGE,
            1 => LDRA_A::POS_EDGE_TIOA,
            2 => LDRA_A::NEG_EDGE_TIOA,
            3 => LDRA_A::BOTH_EDGES_TIOA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == LDRA_A::NO_EDGE
    }
    #[doc = "Checks if the value of the field is `POS_EDGE_TIOA`"]
    #[inline(always)]
    pub fn is_pos_edge_tioa(&self) -> bool {
        *self == LDRA_A::POS_EDGE_TIOA
    }
    #[doc = "Checks if the value of the field is `NEG_EDGE_TIOA`"]
    #[inline(always)]
    pub fn is_neg_edge_tioa(&self) -> bool {
        *self == LDRA_A::NEG_EDGE_TIOA
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES_TIOA`"]
    #[inline(always)]
    pub fn is_both_edges_tioa(&self) -> bool {
        *self == LDRA_A::BOTH_EDGES_TIOA
    }
}
#[doc = "Write proxy for field `LDRA`"]
pub struct LDRA_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDRA_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(LDRA_A::NO_EDGE)
    }
    #[doc = "rising edge of TIOA"]
    #[inline(always)]
    pub fn pos_edge_tioa(self) -> &'a mut W {
        self.variant(LDRA_A::POS_EDGE_TIOA)
    }
    #[doc = "falling edge of TIOA"]
    #[inline(always)]
    pub fn neg_edge_tioa(self) -> &'a mut W {
        self.variant(LDRA_A::NEG_EDGE_TIOA)
    }
    #[doc = "each edge of TIOA"]
    #[inline(always)]
    pub fn both_edges_tioa(self) -> &'a mut W {
        self.variant(LDRA_A::BOTH_EDGES_TIOA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "RB Loading Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LDRB_A {
    #[doc = "0: none"]
    NO_EDGE = 0,
    #[doc = "1: rising edge of TIOA"]
    POS_EDGE_TIOA = 1,
    #[doc = "2: falling edge of TIOA"]
    NEG_EDGE_TIOA = 2,
    #[doc = "3: each edge of TIOA"]
    BOTH_EDGES_TIOA = 3,
}
impl From<LDRB_A> for u8 {
    #[inline(always)]
    fn from(variant: LDRB_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LDRB`"]
pub type LDRB_R = crate::R<u8, LDRB_A>;
impl LDRB_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRB_A {
        match self.bits {
            0 => LDRB_A::NO_EDGE,
            1 => LDRB_A::POS_EDGE_TIOA,
            2 => LDRB_A::NEG_EDGE_TIOA,
            3 => LDRB_A::BOTH_EDGES_TIOA,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_EDGE`"]
    #[inline(always)]
    pub fn is_no_edge(&self) -> bool {
        *self == LDRB_A::NO_EDGE
    }
    #[doc = "Checks if the value of the field is `POS_EDGE_TIOA`"]
    #[inline(always)]
    pub fn is_pos_edge_tioa(&self) -> bool {
        *self == LDRB_A::POS_EDGE_TIOA
    }
    #[doc = "Checks if the value of the field is `NEG_EDGE_TIOA`"]
    #[inline(always)]
    pub fn is_neg_edge_tioa(&self) -> bool {
        *self == LDRB_A::NEG_EDGE_TIOA
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES_TIOA`"]
    #[inline(always)]
    pub fn is_both_edges_tioa(&self) -> bool {
        *self == LDRB_A::BOTH_EDGES_TIOA
    }
}
#[doc = "Write proxy for field `LDRB`"]
pub struct LDRB_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDRB_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "none"]
    #[inline(always)]
    pub fn no_edge(self) -> &'a mut W {
        self.variant(LDRB_A::NO_EDGE)
    }
    #[doc = "rising edge of TIOA"]
    #[inline(always)]
    pub fn pos_edge_tioa(self) -> &'a mut W {
        self.variant(LDRB_A::POS_EDGE_TIOA)
    }
    #[doc = "falling edge of TIOA"]
    #[inline(always)]
    pub fn neg_edge_tioa(self) -> &'a mut W {
        self.variant(LDRB_A::NEG_EDGE_TIOA)
    }
    #[doc = "each edge of TIOA"]
    #[inline(always)]
    pub fn both_edges_tioa(self) -> &'a mut W {
        self.variant(LDRB_A::BOTH_EDGES_TIOA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
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
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&self) -> LDBSTOP_R {
        LDBSTOP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&self) -> LDBDIS_R {
        LDBDIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&self) -> ETRGEDG_R {
        ETRGEDG_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 10 - TIOA or TIOB External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&self) -> ABETRG_R {
        ABETRG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&self) -> CPCTRG_R {
        CPCTRG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Wave"]
    #[inline(always)]
    pub fn wave(&self) -> WAVE_R {
        WAVE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - RA Loading Selection"]
    #[inline(always)]
    pub fn ldra(&self) -> LDRA_R {
        LDRA_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - RB Loading Selection"]
    #[inline(always)]
    pub fn ldrb(&self) -> LDRB_R {
        LDRB_R::new(((self.bits >> 18) & 0x03) as u8)
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
    #[doc = "Bit 6 - Counter Clock Stopped with RB Loading"]
    #[inline(always)]
    pub fn ldbstop(&mut self) -> LDBSTOP_W {
        LDBSTOP_W { w: self }
    }
    #[doc = "Bit 7 - Counter Clock Disable with RB Loading"]
    #[inline(always)]
    pub fn ldbdis(&mut self) -> LDBDIS_W {
        LDBDIS_W { w: self }
    }
    #[doc = "Bits 8:9 - External Trigger Edge Selection"]
    #[inline(always)]
    pub fn etrgedg(&mut self) -> ETRGEDG_W {
        ETRGEDG_W { w: self }
    }
    #[doc = "Bit 10 - TIOA or TIOB External Trigger Selection"]
    #[inline(always)]
    pub fn abetrg(&mut self) -> ABETRG_W {
        ABETRG_W { w: self }
    }
    #[doc = "Bit 14 - RC Compare Trigger Enable"]
    #[inline(always)]
    pub fn cpctrg(&mut self) -> CPCTRG_W {
        CPCTRG_W { w: self }
    }
    #[doc = "Bit 15 - Wave"]
    #[inline(always)]
    pub fn wave(&mut self) -> WAVE_W {
        WAVE_W { w: self }
    }
    #[doc = "Bits 16:17 - RA Loading Selection"]
    #[inline(always)]
    pub fn ldra(&mut self) -> LDRA_W {
        LDRA_W { w: self }
    }
    #[doc = "Bits 18:19 - RB Loading Selection"]
    #[inline(always)]
    pub fn ldrb(&mut self) -> LDRB_W {
        LDRB_W { w: self }
    }
}
