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
#[doc = "Field `TRGEN` reader - Trigger Enable"]
pub type TRGEN_R = crate::BitReader<TRGEN_A>;
#[doc = "Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGEN_A {
    #[doc = "0: External trigger mode disabled. DACC in free-running mode."]
    DIS = 0,
    #[doc = "1: External trigger mode enabled."]
    EN = 1,
}
impl From<TRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGEN_A {
        match self.bits {
            false => TRGEN_A::DIS,
            true => TRGEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TRGEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TRGEN_A::EN
    }
}
#[doc = "Field `TRGEN` writer - Trigger Enable"]
pub type TRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, TRGEN_A, O>;
impl<'a, const O: u8> TRGEN_W<'a, O> {
    #[doc = "External trigger mode disabled. DACC in free-running mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRGEN_A::DIS)
    }
    #[doc = "External trigger mode enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TRGEN_A::EN)
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub type TRGSEL_R = crate::FieldReader<u8, TRGSEL_A>;
#[doc = "Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGSEL_A {
    #[doc = "0: External trigger"]
    TRGSEL0 = 0,
    #[doc = "1: TIO Output of the Timer Counter Channel 0"]
    TRGSEL1 = 1,
    #[doc = "2: TIO Output of the Timer Counter Channel 1"]
    TRGSEL2 = 2,
    #[doc = "3: TIO Output of the Timer Counter Channel 2"]
    TRGSEL3 = 3,
    #[doc = "4: PWM Event Line 0"]
    TRGSEL4 = 4,
    #[doc = "5: PWM Event Line 1"]
    TRGSEL5 = 5,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        variant as _
    }
}
impl TRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRGSEL_A> {
        match self.bits {
            0 => Some(TRGSEL_A::TRGSEL0),
            1 => Some(TRGSEL_A::TRGSEL1),
            2 => Some(TRGSEL_A::TRGSEL2),
            3 => Some(TRGSEL_A::TRGSEL3),
            4 => Some(TRGSEL_A::TRGSEL4),
            5 => Some(TRGSEL_A::TRGSEL5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRGSEL0`"]
    #[inline(always)]
    pub fn is_trgsel0(&self) -> bool {
        *self == TRGSEL_A::TRGSEL0
    }
    #[doc = "Checks if the value of the field is `TRGSEL1`"]
    #[inline(always)]
    pub fn is_trgsel1(&self) -> bool {
        *self == TRGSEL_A::TRGSEL1
    }
    #[doc = "Checks if the value of the field is `TRGSEL2`"]
    #[inline(always)]
    pub fn is_trgsel2(&self) -> bool {
        *self == TRGSEL_A::TRGSEL2
    }
    #[doc = "Checks if the value of the field is `TRGSEL3`"]
    #[inline(always)]
    pub fn is_trgsel3(&self) -> bool {
        *self == TRGSEL_A::TRGSEL3
    }
    #[doc = "Checks if the value of the field is `TRGSEL4`"]
    #[inline(always)]
    pub fn is_trgsel4(&self) -> bool {
        *self == TRGSEL_A::TRGSEL4
    }
    #[doc = "Checks if the value of the field is `TRGSEL5`"]
    #[inline(always)]
    pub fn is_trgsel5(&self) -> bool {
        *self == TRGSEL_A::TRGSEL5
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Selection"]
pub type TRGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, TRGSEL_A, 3, O>;
impl<'a, const O: u8> TRGSEL_W<'a, O> {
    #[doc = "External trigger"]
    #[inline(always)]
    pub fn trgsel0(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL0)
    }
    #[doc = "TIO Output of the Timer Counter Channel 0"]
    #[inline(always)]
    pub fn trgsel1(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL1)
    }
    #[doc = "TIO Output of the Timer Counter Channel 1"]
    #[inline(always)]
    pub fn trgsel2(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL2)
    }
    #[doc = "TIO Output of the Timer Counter Channel 2"]
    #[inline(always)]
    pub fn trgsel3(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL3)
    }
    #[doc = "PWM Event Line 0"]
    #[inline(always)]
    pub fn trgsel4(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL4)
    }
    #[doc = "PWM Event Line 1"]
    #[inline(always)]
    pub fn trgsel5(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRGSEL5)
    }
}
#[doc = "Field `WORD` reader - Word Transfer"]
pub type WORD_R = crate::BitReader<WORD_A>;
#[doc = "Word Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WORD_A {
    #[doc = "0: Half-word transfer"]
    HALF = 0,
    #[doc = "1: Word transfer"]
    WORD = 1,
}
impl From<WORD_A> for bool {
    #[inline(always)]
    fn from(variant: WORD_A) -> Self {
        variant as u8 != 0
    }
}
impl WORD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WORD_A {
        match self.bits {
            false => WORD_A::HALF,
            true => WORD_A::WORD,
        }
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == WORD_A::HALF
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == WORD_A::WORD
    }
}
#[doc = "Field `WORD` writer - Word Transfer"]
pub type WORD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, WORD_A, O>;
impl<'a, const O: u8> WORD_W<'a, O> {
    #[doc = "Half-word transfer"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(WORD_A::HALF)
    }
    #[doc = "Word transfer"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(WORD_A::WORD)
    }
}
#[doc = "Field `ONE` reader - Must Be Set to 1"]
pub type ONE_R = crate::BitReader<bool>;
#[doc = "Field `ONE` writer - Must Be Set to 1"]
pub type ONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `USER_SEL` reader - User Channel Selection"]
pub type USER_SEL_R = crate::FieldReader<u8, USER_SEL_A>;
#[doc = "User Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum USER_SEL_A {
    #[doc = "0: Channel 0"]
    CHANNEL0 = 0,
    #[doc = "1: Channel 1"]
    CHANNEL1 = 1,
}
impl From<USER_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USER_SEL_A) -> Self {
        variant as _
    }
}
impl USER_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USER_SEL_A> {
        match self.bits {
            0 => Some(USER_SEL_A::CHANNEL0),
            1 => Some(USER_SEL_A::CHANNEL1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CHANNEL0`"]
    #[inline(always)]
    pub fn is_channel0(&self) -> bool {
        *self == USER_SEL_A::CHANNEL0
    }
    #[doc = "Checks if the value of the field is `CHANNEL1`"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        *self == USER_SEL_A::CHANNEL1
    }
}
#[doc = "Field `USER_SEL` writer - User Channel Selection"]
pub type USER_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, USER_SEL_A, 2, O>;
impl<'a, const O: u8> USER_SEL_W<'a, O> {
    #[doc = "Channel 0"]
    #[inline(always)]
    pub fn channel0(self) -> &'a mut W {
        self.variant(USER_SEL_A::CHANNEL0)
    }
    #[doc = "Channel 1"]
    #[inline(always)]
    pub fn channel1(self) -> &'a mut W {
        self.variant(USER_SEL_A::CHANNEL1)
    }
}
#[doc = "Field `TAG` reader - Tag Selection Mode"]
pub type TAG_R = crate::BitReader<TAG_A>;
#[doc = "Tag Selection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAG_A {
    #[doc = "0: Tag selection mode disabled. Using USER_SEL to select the channel for the conversion."]
    DIS = 0,
    #[doc = "1: Tag selection mode enabled"]
    EN = 1,
}
impl From<TAG_A> for bool {
    #[inline(always)]
    fn from(variant: TAG_A) -> Self {
        variant as u8 != 0
    }
}
impl TAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAG_A {
        match self.bits {
            false => TAG_A::DIS,
            true => TAG_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == TAG_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == TAG_A::EN
    }
}
#[doc = "Field `TAG` writer - Tag Selection Mode"]
pub type TAG_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, TAG_A, O>;
impl<'a, const O: u8> TAG_W<'a, O> {
    #[doc = "Tag selection mode disabled. Using USER_SEL to select the channel for the conversion."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAG_A::DIS)
    }
    #[doc = "Tag selection mode enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TAG_A::EN)
    }
}
#[doc = "Field `MAXS` reader - Maximum Speed Mode"]
pub type MAXS_R = crate::BitReader<MAXS_A>;
#[doc = "Maximum Speed Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAXS_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Maximum speed mode enabled"]
    MAXIMUM = 1,
}
impl From<MAXS_A> for bool {
    #[inline(always)]
    fn from(variant: MAXS_A) -> Self {
        variant as u8 != 0
    }
}
impl MAXS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAXS_A {
        match self.bits {
            false => MAXS_A::NORMAL,
            true => MAXS_A::MAXIMUM,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MAXS_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        *self == MAXS_A::MAXIMUM
    }
}
#[doc = "Field `MAXS` writer - Maximum Speed Mode"]
pub type MAXS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, MAXS_A, O>;
impl<'a, const O: u8> MAXS_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MAXS_A::NORMAL)
    }
    #[doc = "Maximum speed mode enabled"]
    #[inline(always)]
    pub fn maximum(self) -> &'a mut W {
        self.variant(MAXS_A::MAXIMUM)
    }
}
#[doc = "Field `CLKDIV` reader - Clock Divider"]
pub type CLKDIV_R = crate::BitReader<CLKDIV_A>;
#[doc = "Clock Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKDIV_A {
    #[doc = "0: DAC clock is peripheral clock divided by 2"]
    DIV_2 = 0,
    #[doc = "1: DAC clock is peripheral clock divided by 4 (to be used when peripheral clock frequency is above 100 MHz)"]
    DIV_4 = 1,
}
impl From<CLKDIV_A> for bool {
    #[inline(always)]
    fn from(variant: CLKDIV_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKDIV_A {
        match self.bits {
            false => CLKDIV_A::DIV_2,
            true => CLKDIV_A::DIV_4,
        }
    }
    #[doc = "Checks if the value of the field is `DIV_2`"]
    #[inline(always)]
    pub fn is_div_2(&self) -> bool {
        *self == CLKDIV_A::DIV_2
    }
    #[doc = "Checks if the value of the field is `DIV_4`"]
    #[inline(always)]
    pub fn is_div_4(&self) -> bool {
        *self == CLKDIV_A::DIV_4
    }
}
#[doc = "Field `CLKDIV` writer - Clock Divider"]
pub type CLKDIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, CLKDIV_A, O>;
impl<'a, const O: u8> CLKDIV_W<'a, O> {
    #[doc = "DAC clock is peripheral clock divided by 2"]
    #[inline(always)]
    pub fn div_2(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_2)
    }
    #[doc = "DAC clock is peripheral clock divided by 4 (to be used when peripheral clock frequency is above 100 MHz)"]
    #[inline(always)]
    pub fn div_4(self) -> &'a mut W {
        self.variant(CLKDIV_A::DIV_4)
    }
}
#[doc = "Field `STARTUP` reader - Startup Time Selection"]
pub type STARTUP_R = crate::FieldReader<u8, STARTUP_A>;
#[doc = "Startup Time Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 0 periods of peripheral clock"]
    _0 = 0,
    #[doc = "1: 8 periods of peripheral clock"]
    _8 = 1,
    #[doc = "2: 16 periods of peripheral clock"]
    _16 = 2,
    #[doc = "3: 24 periods of peripheral clock"]
    _24 = 3,
    #[doc = "4: 64 periods of peripheral clock"]
    _64 = 4,
    #[doc = "5: 80 periods of peripheral clock"]
    _80 = 5,
    #[doc = "6: 96 periods of peripheral clock"]
    _96 = 6,
    #[doc = "7: 112 periods of peripheral clock"]
    _112 = 7,
    #[doc = "8: 512 periods of peripheral clock"]
    _512 = 8,
    #[doc = "9: 576 periods of peripheral clock"]
    _576 = 9,
    #[doc = "10: 640 periods of peripheral clock"]
    _640 = 10,
    #[doc = "11: 704 periods of peripheral clock"]
    _704 = 11,
    #[doc = "12: 768 periods of peripheral clock"]
    _768 = 12,
    #[doc = "13: 832 periods of peripheral clock"]
    _832 = 13,
    #[doc = "14: 896 periods of peripheral clock"]
    _896 = 14,
    #[doc = "15: 960 periods of peripheral clock"]
    _960 = 15,
    #[doc = "16: 1024 periods of peripheral clock"]
    _1024 = 16,
    #[doc = "17: 1088 periods of peripheral clock"]
    _1088 = 17,
    #[doc = "18: 1152 periods of peripheral clock"]
    _1152 = 18,
    #[doc = "19: 1216 periods of peripheral clock"]
    _1216 = 19,
    #[doc = "20: 1280 periods of peripheral clock"]
    _1280 = 20,
    #[doc = "21: 1344 periods of peripheral clock"]
    _1344 = 21,
    #[doc = "22: 1408 periods of peripheral clock"]
    _1408 = 22,
    #[doc = "23: 1472 periods of peripheral clock"]
    _1472 = 23,
    #[doc = "24: 1536 periods of peripheral clock"]
    _1536 = 24,
    #[doc = "25: 1600 periods of peripheral clock"]
    _1600 = 25,
    #[doc = "26: 1664 periods of peripheral clock"]
    _1664 = 26,
    #[doc = "27: 1728 periods of peripheral clock"]
    _1728 = 27,
    #[doc = "28: 1792 periods of peripheral clock"]
    _1792 = 28,
    #[doc = "29: 1856 periods of peripheral clock"]
    _1856 = 29,
    #[doc = "30: 1920 periods of peripheral clock"]
    _1920 = 30,
    #[doc = "31: 1984 periods of peripheral clock"]
    _1984 = 31,
    #[doc = "32: 2048 periods of peripheral clock"]
    _2048 = 32,
    #[doc = "33: 2112 periods of peripheral clock"]
    _2112 = 33,
    #[doc = "34: 2176 periods of peripheral clock"]
    _2176 = 34,
    #[doc = "35: 2240 periods of peripheral clock"]
    _2240 = 35,
    #[doc = "36: 2304 periods of peripheral clock"]
    _2304 = 36,
    #[doc = "37: 2368 periods of peripheral clock"]
    _2368 = 37,
    #[doc = "38: 2432 periods of peripheral clock"]
    _2432 = 38,
    #[doc = "39: 2496 periods of peripheral clock"]
    _2496 = 39,
    #[doc = "40: 2560 periods of peripheral clock"]
    _2560 = 40,
    #[doc = "41: 2624 periods of peripheral clock"]
    _2624 = 41,
    #[doc = "42: 2688 periods of peripheral clock"]
    _2688 = 42,
    #[doc = "43: 2752 periods of peripheral clock"]
    _2752 = 43,
    #[doc = "44: 2816 periods of peripheral clock"]
    _2816 = 44,
    #[doc = "45: 2880 periods of peripheral clock"]
    _2880 = 45,
    #[doc = "46: 2944 periods of peripheral clock"]
    _2944 = 46,
    #[doc = "47: 3008 periods of peripheral clock"]
    _3008 = 47,
    #[doc = "48: 3072 periods of peripheral clock"]
    _3072 = 48,
    #[doc = "49: 3136 periods of peripheral clock"]
    _3136 = 49,
    #[doc = "50: 3200 periods of peripheral clock"]
    _3200 = 50,
    #[doc = "51: 3264 periods of peripheral clock"]
    _3264 = 51,
    #[doc = "52: 3328 periods of peripheral clock"]
    _3328 = 52,
    #[doc = "53: 3392 periods of peripheral clock"]
    _3392 = 53,
    #[doc = "54: 3456 periods of peripheral clock"]
    _3456 = 54,
    #[doc = "55: 3520 periods of peripheral clock"]
    _3520 = 55,
    #[doc = "56: 3584 periods of peripheral clock"]
    _3584 = 56,
    #[doc = "57: 3648 periods of peripheral clock"]
    _3648 = 57,
    #[doc = "58: 3712 periods of peripheral clock"]
    _3712 = 58,
    #[doc = "59: 3776 periods of peripheral clock"]
    _3776 = 59,
    #[doc = "60: 3840 periods of peripheral clock"]
    _3840 = 60,
    #[doc = "61: 3904 periods of peripheral clock"]
    _3904 = 61,
    #[doc = "62: 3968 periods of peripheral clock"]
    _3968 = 62,
    #[doc = "63: 4032 periods of peripheral clock"]
    _4032 = 63,
}
impl From<STARTUP_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUP_A) -> Self {
        variant as _
    }
}
impl STARTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTUP_A {
        match self.bits {
            0 => STARTUP_A::_0,
            1 => STARTUP_A::_8,
            2 => STARTUP_A::_16,
            3 => STARTUP_A::_24,
            4 => STARTUP_A::_64,
            5 => STARTUP_A::_80,
            6 => STARTUP_A::_96,
            7 => STARTUP_A::_112,
            8 => STARTUP_A::_512,
            9 => STARTUP_A::_576,
            10 => STARTUP_A::_640,
            11 => STARTUP_A::_704,
            12 => STARTUP_A::_768,
            13 => STARTUP_A::_832,
            14 => STARTUP_A::_896,
            15 => STARTUP_A::_960,
            16 => STARTUP_A::_1024,
            17 => STARTUP_A::_1088,
            18 => STARTUP_A::_1152,
            19 => STARTUP_A::_1216,
            20 => STARTUP_A::_1280,
            21 => STARTUP_A::_1344,
            22 => STARTUP_A::_1408,
            23 => STARTUP_A::_1472,
            24 => STARTUP_A::_1536,
            25 => STARTUP_A::_1600,
            26 => STARTUP_A::_1664,
            27 => STARTUP_A::_1728,
            28 => STARTUP_A::_1792,
            29 => STARTUP_A::_1856,
            30 => STARTUP_A::_1920,
            31 => STARTUP_A::_1984,
            32 => STARTUP_A::_2048,
            33 => STARTUP_A::_2112,
            34 => STARTUP_A::_2176,
            35 => STARTUP_A::_2240,
            36 => STARTUP_A::_2304,
            37 => STARTUP_A::_2368,
            38 => STARTUP_A::_2432,
            39 => STARTUP_A::_2496,
            40 => STARTUP_A::_2560,
            41 => STARTUP_A::_2624,
            42 => STARTUP_A::_2688,
            43 => STARTUP_A::_2752,
            44 => STARTUP_A::_2816,
            45 => STARTUP_A::_2880,
            46 => STARTUP_A::_2944,
            47 => STARTUP_A::_3008,
            48 => STARTUP_A::_3072,
            49 => STARTUP_A::_3136,
            50 => STARTUP_A::_3200,
            51 => STARTUP_A::_3264,
            52 => STARTUP_A::_3328,
            53 => STARTUP_A::_3392,
            54 => STARTUP_A::_3456,
            55 => STARTUP_A::_3520,
            56 => STARTUP_A::_3584,
            57 => STARTUP_A::_3648,
            58 => STARTUP_A::_3712,
            59 => STARTUP_A::_3776,
            60 => STARTUP_A::_3840,
            61 => STARTUP_A::_3904,
            62 => STARTUP_A::_3968,
            63 => STARTUP_A::_4032,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STARTUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == STARTUP_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == STARTUP_A::_16
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == STARTUP_A::_24
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == STARTUP_A::_64
    }
    #[doc = "Checks if the value of the field is `_80`"]
    #[inline(always)]
    pub fn is_80(&self) -> bool {
        *self == STARTUP_A::_80
    }
    #[doc = "Checks if the value of the field is `_96`"]
    #[inline(always)]
    pub fn is_96(&self) -> bool {
        *self == STARTUP_A::_96
    }
    #[doc = "Checks if the value of the field is `_112`"]
    #[inline(always)]
    pub fn is_112(&self) -> bool {
        *self == STARTUP_A::_112
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == STARTUP_A::_512
    }
    #[doc = "Checks if the value of the field is `_576`"]
    #[inline(always)]
    pub fn is_576(&self) -> bool {
        *self == STARTUP_A::_576
    }
    #[doc = "Checks if the value of the field is `_640`"]
    #[inline(always)]
    pub fn is_640(&self) -> bool {
        *self == STARTUP_A::_640
    }
    #[doc = "Checks if the value of the field is `_704`"]
    #[inline(always)]
    pub fn is_704(&self) -> bool {
        *self == STARTUP_A::_704
    }
    #[doc = "Checks if the value of the field is `_768`"]
    #[inline(always)]
    pub fn is_768(&self) -> bool {
        *self == STARTUP_A::_768
    }
    #[doc = "Checks if the value of the field is `_832`"]
    #[inline(always)]
    pub fn is_832(&self) -> bool {
        *self == STARTUP_A::_832
    }
    #[doc = "Checks if the value of the field is `_896`"]
    #[inline(always)]
    pub fn is_896(&self) -> bool {
        *self == STARTUP_A::_896
    }
    #[doc = "Checks if the value of the field is `_960`"]
    #[inline(always)]
    pub fn is_960(&self) -> bool {
        *self == STARTUP_A::_960
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        *self == STARTUP_A::_1024
    }
    #[doc = "Checks if the value of the field is `_1088`"]
    #[inline(always)]
    pub fn is_1088(&self) -> bool {
        *self == STARTUP_A::_1088
    }
    #[doc = "Checks if the value of the field is `_1152`"]
    #[inline(always)]
    pub fn is_1152(&self) -> bool {
        *self == STARTUP_A::_1152
    }
    #[doc = "Checks if the value of the field is `_1216`"]
    #[inline(always)]
    pub fn is_1216(&self) -> bool {
        *self == STARTUP_A::_1216
    }
    #[doc = "Checks if the value of the field is `_1280`"]
    #[inline(always)]
    pub fn is_1280(&self) -> bool {
        *self == STARTUP_A::_1280
    }
    #[doc = "Checks if the value of the field is `_1344`"]
    #[inline(always)]
    pub fn is_1344(&self) -> bool {
        *self == STARTUP_A::_1344
    }
    #[doc = "Checks if the value of the field is `_1408`"]
    #[inline(always)]
    pub fn is_1408(&self) -> bool {
        *self == STARTUP_A::_1408
    }
    #[doc = "Checks if the value of the field is `_1472`"]
    #[inline(always)]
    pub fn is_1472(&self) -> bool {
        *self == STARTUP_A::_1472
    }
    #[doc = "Checks if the value of the field is `_1536`"]
    #[inline(always)]
    pub fn is_1536(&self) -> bool {
        *self == STARTUP_A::_1536
    }
    #[doc = "Checks if the value of the field is `_1600`"]
    #[inline(always)]
    pub fn is_1600(&self) -> bool {
        *self == STARTUP_A::_1600
    }
    #[doc = "Checks if the value of the field is `_1664`"]
    #[inline(always)]
    pub fn is_1664(&self) -> bool {
        *self == STARTUP_A::_1664
    }
    #[doc = "Checks if the value of the field is `_1728`"]
    #[inline(always)]
    pub fn is_1728(&self) -> bool {
        *self == STARTUP_A::_1728
    }
    #[doc = "Checks if the value of the field is `_1792`"]
    #[inline(always)]
    pub fn is_1792(&self) -> bool {
        *self == STARTUP_A::_1792
    }
    #[doc = "Checks if the value of the field is `_1856`"]
    #[inline(always)]
    pub fn is_1856(&self) -> bool {
        *self == STARTUP_A::_1856
    }
    #[doc = "Checks if the value of the field is `_1920`"]
    #[inline(always)]
    pub fn is_1920(&self) -> bool {
        *self == STARTUP_A::_1920
    }
    #[doc = "Checks if the value of the field is `_1984`"]
    #[inline(always)]
    pub fn is_1984(&self) -> bool {
        *self == STARTUP_A::_1984
    }
    #[doc = "Checks if the value of the field is `_2048`"]
    #[inline(always)]
    pub fn is_2048(&self) -> bool {
        *self == STARTUP_A::_2048
    }
    #[doc = "Checks if the value of the field is `_2112`"]
    #[inline(always)]
    pub fn is_2112(&self) -> bool {
        *self == STARTUP_A::_2112
    }
    #[doc = "Checks if the value of the field is `_2176`"]
    #[inline(always)]
    pub fn is_2176(&self) -> bool {
        *self == STARTUP_A::_2176
    }
    #[doc = "Checks if the value of the field is `_2240`"]
    #[inline(always)]
    pub fn is_2240(&self) -> bool {
        *self == STARTUP_A::_2240
    }
    #[doc = "Checks if the value of the field is `_2304`"]
    #[inline(always)]
    pub fn is_2304(&self) -> bool {
        *self == STARTUP_A::_2304
    }
    #[doc = "Checks if the value of the field is `_2368`"]
    #[inline(always)]
    pub fn is_2368(&self) -> bool {
        *self == STARTUP_A::_2368
    }
    #[doc = "Checks if the value of the field is `_2432`"]
    #[inline(always)]
    pub fn is_2432(&self) -> bool {
        *self == STARTUP_A::_2432
    }
    #[doc = "Checks if the value of the field is `_2496`"]
    #[inline(always)]
    pub fn is_2496(&self) -> bool {
        *self == STARTUP_A::_2496
    }
    #[doc = "Checks if the value of the field is `_2560`"]
    #[inline(always)]
    pub fn is_2560(&self) -> bool {
        *self == STARTUP_A::_2560
    }
    #[doc = "Checks if the value of the field is `_2624`"]
    #[inline(always)]
    pub fn is_2624(&self) -> bool {
        *self == STARTUP_A::_2624
    }
    #[doc = "Checks if the value of the field is `_2688`"]
    #[inline(always)]
    pub fn is_2688(&self) -> bool {
        *self == STARTUP_A::_2688
    }
    #[doc = "Checks if the value of the field is `_2752`"]
    #[inline(always)]
    pub fn is_2752(&self) -> bool {
        *self == STARTUP_A::_2752
    }
    #[doc = "Checks if the value of the field is `_2816`"]
    #[inline(always)]
    pub fn is_2816(&self) -> bool {
        *self == STARTUP_A::_2816
    }
    #[doc = "Checks if the value of the field is `_2880`"]
    #[inline(always)]
    pub fn is_2880(&self) -> bool {
        *self == STARTUP_A::_2880
    }
    #[doc = "Checks if the value of the field is `_2944`"]
    #[inline(always)]
    pub fn is_2944(&self) -> bool {
        *self == STARTUP_A::_2944
    }
    #[doc = "Checks if the value of the field is `_3008`"]
    #[inline(always)]
    pub fn is_3008(&self) -> bool {
        *self == STARTUP_A::_3008
    }
    #[doc = "Checks if the value of the field is `_3072`"]
    #[inline(always)]
    pub fn is_3072(&self) -> bool {
        *self == STARTUP_A::_3072
    }
    #[doc = "Checks if the value of the field is `_3136`"]
    #[inline(always)]
    pub fn is_3136(&self) -> bool {
        *self == STARTUP_A::_3136
    }
    #[doc = "Checks if the value of the field is `_3200`"]
    #[inline(always)]
    pub fn is_3200(&self) -> bool {
        *self == STARTUP_A::_3200
    }
    #[doc = "Checks if the value of the field is `_3264`"]
    #[inline(always)]
    pub fn is_3264(&self) -> bool {
        *self == STARTUP_A::_3264
    }
    #[doc = "Checks if the value of the field is `_3328`"]
    #[inline(always)]
    pub fn is_3328(&self) -> bool {
        *self == STARTUP_A::_3328
    }
    #[doc = "Checks if the value of the field is `_3392`"]
    #[inline(always)]
    pub fn is_3392(&self) -> bool {
        *self == STARTUP_A::_3392
    }
    #[doc = "Checks if the value of the field is `_3456`"]
    #[inline(always)]
    pub fn is_3456(&self) -> bool {
        *self == STARTUP_A::_3456
    }
    #[doc = "Checks if the value of the field is `_3520`"]
    #[inline(always)]
    pub fn is_3520(&self) -> bool {
        *self == STARTUP_A::_3520
    }
    #[doc = "Checks if the value of the field is `_3584`"]
    #[inline(always)]
    pub fn is_3584(&self) -> bool {
        *self == STARTUP_A::_3584
    }
    #[doc = "Checks if the value of the field is `_3648`"]
    #[inline(always)]
    pub fn is_3648(&self) -> bool {
        *self == STARTUP_A::_3648
    }
    #[doc = "Checks if the value of the field is `_3712`"]
    #[inline(always)]
    pub fn is_3712(&self) -> bool {
        *self == STARTUP_A::_3712
    }
    #[doc = "Checks if the value of the field is `_3776`"]
    #[inline(always)]
    pub fn is_3776(&self) -> bool {
        *self == STARTUP_A::_3776
    }
    #[doc = "Checks if the value of the field is `_3840`"]
    #[inline(always)]
    pub fn is_3840(&self) -> bool {
        *self == STARTUP_A::_3840
    }
    #[doc = "Checks if the value of the field is `_3904`"]
    #[inline(always)]
    pub fn is_3904(&self) -> bool {
        *self == STARTUP_A::_3904
    }
    #[doc = "Checks if the value of the field is `_3968`"]
    #[inline(always)]
    pub fn is_3968(&self) -> bool {
        *self == STARTUP_A::_3968
    }
    #[doc = "Checks if the value of the field is `_4032`"]
    #[inline(always)]
    pub fn is_4032(&self) -> bool {
        *self == STARTUP_A::_4032
    }
}
#[doc = "Field `STARTUP` writer - Startup Time Selection"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MR_SPEC, u8, STARTUP_A, 6, O>;
impl<'a, const O: u8> STARTUP_W<'a, O> {
    #[doc = "0 periods of peripheral clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STARTUP_A::_0)
    }
    #[doc = "8 periods of peripheral clock"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(STARTUP_A::_8)
    }
    #[doc = "16 periods of peripheral clock"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(STARTUP_A::_16)
    }
    #[doc = "24 periods of peripheral clock"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(STARTUP_A::_24)
    }
    #[doc = "64 periods of peripheral clock"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(STARTUP_A::_64)
    }
    #[doc = "80 periods of peripheral clock"]
    #[inline(always)]
    pub fn _80(self) -> &'a mut W {
        self.variant(STARTUP_A::_80)
    }
    #[doc = "96 periods of peripheral clock"]
    #[inline(always)]
    pub fn _96(self) -> &'a mut W {
        self.variant(STARTUP_A::_96)
    }
    #[doc = "112 periods of peripheral clock"]
    #[inline(always)]
    pub fn _112(self) -> &'a mut W {
        self.variant(STARTUP_A::_112)
    }
    #[doc = "512 periods of peripheral clock"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(STARTUP_A::_512)
    }
    #[doc = "576 periods of peripheral clock"]
    #[inline(always)]
    pub fn _576(self) -> &'a mut W {
        self.variant(STARTUP_A::_576)
    }
    #[doc = "640 periods of peripheral clock"]
    #[inline(always)]
    pub fn _640(self) -> &'a mut W {
        self.variant(STARTUP_A::_640)
    }
    #[doc = "704 periods of peripheral clock"]
    #[inline(always)]
    pub fn _704(self) -> &'a mut W {
        self.variant(STARTUP_A::_704)
    }
    #[doc = "768 periods of peripheral clock"]
    #[inline(always)]
    pub fn _768(self) -> &'a mut W {
        self.variant(STARTUP_A::_768)
    }
    #[doc = "832 periods of peripheral clock"]
    #[inline(always)]
    pub fn _832(self) -> &'a mut W {
        self.variant(STARTUP_A::_832)
    }
    #[doc = "896 periods of peripheral clock"]
    #[inline(always)]
    pub fn _896(self) -> &'a mut W {
        self.variant(STARTUP_A::_896)
    }
    #[doc = "960 periods of peripheral clock"]
    #[inline(always)]
    pub fn _960(self) -> &'a mut W {
        self.variant(STARTUP_A::_960)
    }
    #[doc = "1024 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(STARTUP_A::_1024)
    }
    #[doc = "1088 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1088(self) -> &'a mut W {
        self.variant(STARTUP_A::_1088)
    }
    #[doc = "1152 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1152(self) -> &'a mut W {
        self.variant(STARTUP_A::_1152)
    }
    #[doc = "1216 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1216(self) -> &'a mut W {
        self.variant(STARTUP_A::_1216)
    }
    #[doc = "1280 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1280(self) -> &'a mut W {
        self.variant(STARTUP_A::_1280)
    }
    #[doc = "1344 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1344(self) -> &'a mut W {
        self.variant(STARTUP_A::_1344)
    }
    #[doc = "1408 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1408(self) -> &'a mut W {
        self.variant(STARTUP_A::_1408)
    }
    #[doc = "1472 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1472(self) -> &'a mut W {
        self.variant(STARTUP_A::_1472)
    }
    #[doc = "1536 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1536(self) -> &'a mut W {
        self.variant(STARTUP_A::_1536)
    }
    #[doc = "1600 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1600(self) -> &'a mut W {
        self.variant(STARTUP_A::_1600)
    }
    #[doc = "1664 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1664(self) -> &'a mut W {
        self.variant(STARTUP_A::_1664)
    }
    #[doc = "1728 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1728(self) -> &'a mut W {
        self.variant(STARTUP_A::_1728)
    }
    #[doc = "1792 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1792(self) -> &'a mut W {
        self.variant(STARTUP_A::_1792)
    }
    #[doc = "1856 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1856(self) -> &'a mut W {
        self.variant(STARTUP_A::_1856)
    }
    #[doc = "1920 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1920(self) -> &'a mut W {
        self.variant(STARTUP_A::_1920)
    }
    #[doc = "1984 periods of peripheral clock"]
    #[inline(always)]
    pub fn _1984(self) -> &'a mut W {
        self.variant(STARTUP_A::_1984)
    }
    #[doc = "2048 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2048(self) -> &'a mut W {
        self.variant(STARTUP_A::_2048)
    }
    #[doc = "2112 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2112(self) -> &'a mut W {
        self.variant(STARTUP_A::_2112)
    }
    #[doc = "2176 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2176(self) -> &'a mut W {
        self.variant(STARTUP_A::_2176)
    }
    #[doc = "2240 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2240(self) -> &'a mut W {
        self.variant(STARTUP_A::_2240)
    }
    #[doc = "2304 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2304(self) -> &'a mut W {
        self.variant(STARTUP_A::_2304)
    }
    #[doc = "2368 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2368(self) -> &'a mut W {
        self.variant(STARTUP_A::_2368)
    }
    #[doc = "2432 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2432(self) -> &'a mut W {
        self.variant(STARTUP_A::_2432)
    }
    #[doc = "2496 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2496(self) -> &'a mut W {
        self.variant(STARTUP_A::_2496)
    }
    #[doc = "2560 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2560(self) -> &'a mut W {
        self.variant(STARTUP_A::_2560)
    }
    #[doc = "2624 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2624(self) -> &'a mut W {
        self.variant(STARTUP_A::_2624)
    }
    #[doc = "2688 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2688(self) -> &'a mut W {
        self.variant(STARTUP_A::_2688)
    }
    #[doc = "2752 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2752(self) -> &'a mut W {
        self.variant(STARTUP_A::_2752)
    }
    #[doc = "2816 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2816(self) -> &'a mut W {
        self.variant(STARTUP_A::_2816)
    }
    #[doc = "2880 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2880(self) -> &'a mut W {
        self.variant(STARTUP_A::_2880)
    }
    #[doc = "2944 periods of peripheral clock"]
    #[inline(always)]
    pub fn _2944(self) -> &'a mut W {
        self.variant(STARTUP_A::_2944)
    }
    #[doc = "3008 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3008(self) -> &'a mut W {
        self.variant(STARTUP_A::_3008)
    }
    #[doc = "3072 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3072(self) -> &'a mut W {
        self.variant(STARTUP_A::_3072)
    }
    #[doc = "3136 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3136(self) -> &'a mut W {
        self.variant(STARTUP_A::_3136)
    }
    #[doc = "3200 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3200(self) -> &'a mut W {
        self.variant(STARTUP_A::_3200)
    }
    #[doc = "3264 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3264(self) -> &'a mut W {
        self.variant(STARTUP_A::_3264)
    }
    #[doc = "3328 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3328(self) -> &'a mut W {
        self.variant(STARTUP_A::_3328)
    }
    #[doc = "3392 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3392(self) -> &'a mut W {
        self.variant(STARTUP_A::_3392)
    }
    #[doc = "3456 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3456(self) -> &'a mut W {
        self.variant(STARTUP_A::_3456)
    }
    #[doc = "3520 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3520(self) -> &'a mut W {
        self.variant(STARTUP_A::_3520)
    }
    #[doc = "3584 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3584(self) -> &'a mut W {
        self.variant(STARTUP_A::_3584)
    }
    #[doc = "3648 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3648(self) -> &'a mut W {
        self.variant(STARTUP_A::_3648)
    }
    #[doc = "3712 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3712(self) -> &'a mut W {
        self.variant(STARTUP_A::_3712)
    }
    #[doc = "3776 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3776(self) -> &'a mut W {
        self.variant(STARTUP_A::_3776)
    }
    #[doc = "3840 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3840(self) -> &'a mut W {
        self.variant(STARTUP_A::_3840)
    }
    #[doc = "3904 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3904(self) -> &'a mut W {
        self.variant(STARTUP_A::_3904)
    }
    #[doc = "3968 periods of peripheral clock"]
    #[inline(always)]
    pub fn _3968(self) -> &'a mut W {
        self.variant(STARTUP_A::_3968)
    }
    #[doc = "4032 periods of peripheral clock"]
    #[inline(always)]
    pub fn _4032(self) -> &'a mut W {
        self.variant(STARTUP_A::_4032)
    }
}
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TRGEN_R {
        TRGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Word Transfer"]
    #[inline(always)]
    pub fn word(&self) -> WORD_R {
        WORD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - User Channel Selection"]
    #[inline(always)]
    pub fn user_sel(&self) -> USER_SEL_R {
        USER_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Tag Selection Mode"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Maximum Speed Mode"]
    #[inline(always)]
    pub fn maxs(&self) -> MAXS_R {
        MAXS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Clock Divider"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Startup Time Selection"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trgen(&mut self) -> TRGEN_W<0> {
        TRGEN_W::new(self)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TRGSEL_W<1> {
        TRGSEL_W::new(self)
    }
    #[doc = "Bit 4 - Word Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn word(&mut self) -> WORD_W<4> {
        WORD_W::new(self)
    }
    #[doc = "Bit 8 - Must Be Set to 1"]
    #[inline(always)]
    #[must_use]
    pub fn one(&mut self) -> ONE_W<8> {
        ONE_W::new(self)
    }
    #[doc = "Bits 16:17 - User Channel Selection"]
    #[inline(always)]
    #[must_use]
    pub fn user_sel(&mut self) -> USER_SEL_W<16> {
        USER_SEL_W::new(self)
    }
    #[doc = "Bit 20 - Tag Selection Mode"]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TAG_W<20> {
        TAG_W::new(self)
    }
    #[doc = "Bit 21 - Maximum Speed Mode"]
    #[inline(always)]
    #[must_use]
    pub fn maxs(&mut self) -> MAXS_W<21> {
        MAXS_W::new(self)
    }
    #[doc = "Bit 22 - Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<22> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bits 24:29 - Startup Time Selection"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<24> {
        STARTUP_W::new(self)
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
