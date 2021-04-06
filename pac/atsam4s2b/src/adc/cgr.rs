#[doc = "Reader of register CGR"]
pub type R = crate::R<u32, super::CGR>;
#[doc = "Writer for register CGR"]
pub type W = crate::W<u32, super::CGR>;
#[doc = "Register CGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Gain for Channel 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN0_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN0_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN0_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN0`"]
pub type GAIN0_R = crate::R<u8, GAIN0_A>;
impl GAIN0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN0_A {
        match self.bits {
            0 => GAIN0_A::SE1_DIFF0_5,
            1 => GAIN0_A::SE1_DIFF1,
            2 => GAIN0_A::SE2_DIFF2,
            3 => GAIN0_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN0_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN0_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN0_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN0_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN0`"]
pub struct GAIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN0_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN0_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN0_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN0_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Gain for Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN1_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN1_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN1`"]
pub type GAIN1_R = crate::R<u8, GAIN1_A>;
impl GAIN1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN1_A {
        match self.bits {
            0 => GAIN1_A::SE1_DIFF0_5,
            1 => GAIN1_A::SE1_DIFF1,
            2 => GAIN1_A::SE2_DIFF2,
            3 => GAIN1_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN1_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN1_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN1_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN1_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN1`"]
pub struct GAIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN1_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN1_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN1_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN1_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Gain for Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN2_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN2_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN2`"]
pub type GAIN2_R = crate::R<u8, GAIN2_A>;
impl GAIN2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN2_A {
        match self.bits {
            0 => GAIN2_A::SE1_DIFF0_5,
            1 => GAIN2_A::SE1_DIFF1,
            2 => GAIN2_A::SE2_DIFF2,
            3 => GAIN2_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN2_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN2_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN2_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN2_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN2`"]
pub struct GAIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN2_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN2_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN2_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN2_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Gain for Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN3_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN3_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN3_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN3`"]
pub type GAIN3_R = crate::R<u8, GAIN3_A>;
impl GAIN3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN3_A {
        match self.bits {
            0 => GAIN3_A::SE1_DIFF0_5,
            1 => GAIN3_A::SE1_DIFF1,
            2 => GAIN3_A::SE2_DIFF2,
            3 => GAIN3_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN3_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN3_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN3_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN3_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN3`"]
pub struct GAIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN3_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN3_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN3_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN3_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Gain for Channel 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN4_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN4_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN4_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN4`"]
pub type GAIN4_R = crate::R<u8, GAIN4_A>;
impl GAIN4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN4_A {
        match self.bits {
            0 => GAIN4_A::SE1_DIFF0_5,
            1 => GAIN4_A::SE1_DIFF1,
            2 => GAIN4_A::SE2_DIFF2,
            3 => GAIN4_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN4_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN4_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN4_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN4_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN4`"]
pub struct GAIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN4_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN4_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN4_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN4_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Gain for Channel 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN5_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN5_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN5_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN5`"]
pub type GAIN5_R = crate::R<u8, GAIN5_A>;
impl GAIN5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN5_A {
        match self.bits {
            0 => GAIN5_A::SE1_DIFF0_5,
            1 => GAIN5_A::SE1_DIFF1,
            2 => GAIN5_A::SE2_DIFF2,
            3 => GAIN5_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN5_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN5_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN5_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN5_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN5`"]
pub struct GAIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN5_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN5_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN5_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN5_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Gain for Channel 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN6_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN6_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN6_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN6`"]
pub type GAIN6_R = crate::R<u8, GAIN6_A>;
impl GAIN6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN6_A {
        match self.bits {
            0 => GAIN6_A::SE1_DIFF0_5,
            1 => GAIN6_A::SE1_DIFF1,
            2 => GAIN6_A::SE2_DIFF2,
            3 => GAIN6_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN6_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN6_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN6_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN6_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN6`"]
pub struct GAIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN6_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN6_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN6_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN6_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Gain for Channel 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN7_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN7_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN7_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN7`"]
pub type GAIN7_R = crate::R<u8, GAIN7_A>;
impl GAIN7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN7_A {
        match self.bits {
            0 => GAIN7_A::SE1_DIFF0_5,
            1 => GAIN7_A::SE1_DIFF1,
            2 => GAIN7_A::SE2_DIFF2,
            3 => GAIN7_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN7_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN7_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN7_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN7_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN7`"]
pub struct GAIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN7_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN7_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN7_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN7_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN7_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Gain for Channel 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN8_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN8_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN8`"]
pub type GAIN8_R = crate::R<u8, GAIN8_A>;
impl GAIN8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN8_A {
        match self.bits {
            0 => GAIN8_A::SE1_DIFF0_5,
            1 => GAIN8_A::SE1_DIFF1,
            2 => GAIN8_A::SE2_DIFF2,
            3 => GAIN8_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN8_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN8_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN8_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN8_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN8`"]
pub struct GAIN8_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN8_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN8_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN8_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN8_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN8_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Gain for Channel 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN9_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN9_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN9`"]
pub type GAIN9_R = crate::R<u8, GAIN9_A>;
impl GAIN9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN9_A {
        match self.bits {
            0 => GAIN9_A::SE1_DIFF0_5,
            1 => GAIN9_A::SE1_DIFF1,
            2 => GAIN9_A::SE2_DIFF2,
            3 => GAIN9_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN9_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN9_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN9_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN9_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN9`"]
pub struct GAIN9_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN9_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN9_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN9_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN9_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN9_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Gain for Channel 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN10_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN10_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN10`"]
pub type GAIN10_R = crate::R<u8, GAIN10_A>;
impl GAIN10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN10_A {
        match self.bits {
            0 => GAIN10_A::SE1_DIFF0_5,
            1 => GAIN10_A::SE1_DIFF1,
            2 => GAIN10_A::SE2_DIFF2,
            3 => GAIN10_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN10_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN10_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN10_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN10_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN10`"]
pub struct GAIN10_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN10_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN10_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN10_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN10_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN10_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Gain for Channel 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN11_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN11_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN11_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN11`"]
pub type GAIN11_R = crate::R<u8, GAIN11_A>;
impl GAIN11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN11_A {
        match self.bits {
            0 => GAIN11_A::SE1_DIFF0_5,
            1 => GAIN11_A::SE1_DIFF1,
            2 => GAIN11_A::SE2_DIFF2,
            3 => GAIN11_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN11_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN11_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN11_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN11_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN11`"]
pub struct GAIN11_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN11_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN11_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN11_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN11_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN11_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Gain for Channel 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN12_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN12_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN12_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN12`"]
pub type GAIN12_R = crate::R<u8, GAIN12_A>;
impl GAIN12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN12_A {
        match self.bits {
            0 => GAIN12_A::SE1_DIFF0_5,
            1 => GAIN12_A::SE1_DIFF1,
            2 => GAIN12_A::SE2_DIFF2,
            3 => GAIN12_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN12_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN12_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN12_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN12_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN12`"]
pub struct GAIN12_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN12_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN12_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN12_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN12_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN12_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Gain for Channel 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN13_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN13_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN13_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN13`"]
pub type GAIN13_R = crate::R<u8, GAIN13_A>;
impl GAIN13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN13_A {
        match self.bits {
            0 => GAIN13_A::SE1_DIFF0_5,
            1 => GAIN13_A::SE1_DIFF1,
            2 => GAIN13_A::SE2_DIFF2,
            3 => GAIN13_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN13_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN13_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN13_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN13_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN13`"]
pub struct GAIN13_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN13_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN13_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN13_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN13_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN13_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Gain for Channel 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN14_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN14_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN14_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN14`"]
pub type GAIN14_R = crate::R<u8, GAIN14_A>;
impl GAIN14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN14_A {
        match self.bits {
            0 => GAIN14_A::SE1_DIFF0_5,
            1 => GAIN14_A::SE1_DIFF1,
            2 => GAIN14_A::SE2_DIFF2,
            3 => GAIN14_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN14_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN14_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN14_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN14_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN14`"]
pub struct GAIN14_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN14_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN14_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN14_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN14_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN14_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Gain for Channel 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum GAIN15_A {
    #[doc = "0: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF0_5 = 0,
    #[doc = "1: Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    SE1_DIFF1 = 1,
    #[doc = "2: Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE2_DIFF2 = 2,
    #[doc = "3: Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    SE4_DIFF2 = 3,
}
impl From<GAIN15_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN15_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GAIN15`"]
pub type GAIN15_R = crate::R<u8, GAIN15_A>;
impl GAIN15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GAIN15_A {
        match self.bits {
            0 => GAIN15_A::SE1_DIFF0_5,
            1 => GAIN15_A::SE1_DIFF1,
            2 => GAIN15_A::SE2_DIFF2,
            3 => GAIN15_A::SE4_DIFF2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF0_5`"]
    #[inline(always)]
    pub fn is_se1_diff0_5(&self) -> bool {
        *self == GAIN15_A::SE1_DIFF0_5
    }
    #[doc = "Checks if the value of the field is `SE1_DIFF1`"]
    #[inline(always)]
    pub fn is_se1_diff1(&self) -> bool {
        *self == GAIN15_A::SE1_DIFF1
    }
    #[doc = "Checks if the value of the field is `SE2_DIFF2`"]
    #[inline(always)]
    pub fn is_se2_diff2(&self) -> bool {
        *self == GAIN15_A::SE2_DIFF2
    }
    #[doc = "Checks if the value of the field is `SE4_DIFF2`"]
    #[inline(always)]
    pub fn is_se4_diff2(&self) -> bool {
        *self == GAIN15_A::SE4_DIFF2
    }
}
#[doc = "Write proxy for field `GAIN15`"]
pub struct GAIN15_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GAIN15_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 0.5 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff0_5(self) -> &'a mut W {
        self.variant(GAIN15_A::SE1_DIFF0_5)
    }
    #[doc = "Single-ended gain = 1 (ADC_COR.DIFFx = 0), differential gain = 1 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se1_diff1(self) -> &'a mut W {
        self.variant(GAIN15_A::SE1_DIFF1)
    }
    #[doc = "Single-ended gain = 2 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se2_diff2(self) -> &'a mut W {
        self.variant(GAIN15_A::SE2_DIFF2)
    }
    #[doc = "Single-ended gain = 4 (ADC_COR.DIFFx = 0), differential gain = 2 (ADC_COR.DIFFx = 1)"]
    #[inline(always)]
    pub fn se4_diff2(self) -> &'a mut W {
        self.variant(GAIN15_A::SE4_DIFF2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Gain for Channel 0"]
    #[inline(always)]
    pub fn gain0(&self) -> GAIN0_R {
        GAIN0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Gain for Channel 1"]
    #[inline(always)]
    pub fn gain1(&self) -> GAIN1_R {
        GAIN1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Gain for Channel 2"]
    #[inline(always)]
    pub fn gain2(&self) -> GAIN2_R {
        GAIN2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Gain for Channel 3"]
    #[inline(always)]
    pub fn gain3(&self) -> GAIN3_R {
        GAIN3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Gain for Channel 4"]
    #[inline(always)]
    pub fn gain4(&self) -> GAIN4_R {
        GAIN4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Gain for Channel 5"]
    #[inline(always)]
    pub fn gain5(&self) -> GAIN5_R {
        GAIN5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Gain for Channel 6"]
    #[inline(always)]
    pub fn gain6(&self) -> GAIN6_R {
        GAIN6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Gain for Channel 7"]
    #[inline(always)]
    pub fn gain7(&self) -> GAIN7_R {
        GAIN7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Gain for Channel 8"]
    #[inline(always)]
    pub fn gain8(&self) -> GAIN8_R {
        GAIN8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Gain for Channel 9"]
    #[inline(always)]
    pub fn gain9(&self) -> GAIN9_R {
        GAIN9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Gain for Channel 10"]
    #[inline(always)]
    pub fn gain10(&self) -> GAIN10_R {
        GAIN10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Gain for Channel 11"]
    #[inline(always)]
    pub fn gain11(&self) -> GAIN11_R {
        GAIN11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Gain for Channel 12"]
    #[inline(always)]
    pub fn gain12(&self) -> GAIN12_R {
        GAIN12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Gain for Channel 13"]
    #[inline(always)]
    pub fn gain13(&self) -> GAIN13_R {
        GAIN13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Gain for Channel 14"]
    #[inline(always)]
    pub fn gain14(&self) -> GAIN14_R {
        GAIN14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Gain for Channel 15"]
    #[inline(always)]
    pub fn gain15(&self) -> GAIN15_R {
        GAIN15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Gain for Channel 0"]
    #[inline(always)]
    pub fn gain0(&mut self) -> GAIN0_W {
        GAIN0_W { w: self }
    }
    #[doc = "Bits 2:3 - Gain for Channel 1"]
    #[inline(always)]
    pub fn gain1(&mut self) -> GAIN1_W {
        GAIN1_W { w: self }
    }
    #[doc = "Bits 4:5 - Gain for Channel 2"]
    #[inline(always)]
    pub fn gain2(&mut self) -> GAIN2_W {
        GAIN2_W { w: self }
    }
    #[doc = "Bits 6:7 - Gain for Channel 3"]
    #[inline(always)]
    pub fn gain3(&mut self) -> GAIN3_W {
        GAIN3_W { w: self }
    }
    #[doc = "Bits 8:9 - Gain for Channel 4"]
    #[inline(always)]
    pub fn gain4(&mut self) -> GAIN4_W {
        GAIN4_W { w: self }
    }
    #[doc = "Bits 10:11 - Gain for Channel 5"]
    #[inline(always)]
    pub fn gain5(&mut self) -> GAIN5_W {
        GAIN5_W { w: self }
    }
    #[doc = "Bits 12:13 - Gain for Channel 6"]
    #[inline(always)]
    pub fn gain6(&mut self) -> GAIN6_W {
        GAIN6_W { w: self }
    }
    #[doc = "Bits 14:15 - Gain for Channel 7"]
    #[inline(always)]
    pub fn gain7(&mut self) -> GAIN7_W {
        GAIN7_W { w: self }
    }
    #[doc = "Bits 16:17 - Gain for Channel 8"]
    #[inline(always)]
    pub fn gain8(&mut self) -> GAIN8_W {
        GAIN8_W { w: self }
    }
    #[doc = "Bits 18:19 - Gain for Channel 9"]
    #[inline(always)]
    pub fn gain9(&mut self) -> GAIN9_W {
        GAIN9_W { w: self }
    }
    #[doc = "Bits 20:21 - Gain for Channel 10"]
    #[inline(always)]
    pub fn gain10(&mut self) -> GAIN10_W {
        GAIN10_W { w: self }
    }
    #[doc = "Bits 22:23 - Gain for Channel 11"]
    #[inline(always)]
    pub fn gain11(&mut self) -> GAIN11_W {
        GAIN11_W { w: self }
    }
    #[doc = "Bits 24:25 - Gain for Channel 12"]
    #[inline(always)]
    pub fn gain12(&mut self) -> GAIN12_W {
        GAIN12_W { w: self }
    }
    #[doc = "Bits 26:27 - Gain for Channel 13"]
    #[inline(always)]
    pub fn gain13(&mut self) -> GAIN13_W {
        GAIN13_W { w: self }
    }
    #[doc = "Bits 28:29 - Gain for Channel 14"]
    #[inline(always)]
    pub fn gain14(&mut self) -> GAIN14_W {
        GAIN14_W { w: self }
    }
    #[doc = "Bits 30:31 - Gain for Channel 15"]
    #[inline(always)]
    pub fn gain15(&mut self) -> GAIN15_W {
        GAIN15_W { w: self }
    }
}
