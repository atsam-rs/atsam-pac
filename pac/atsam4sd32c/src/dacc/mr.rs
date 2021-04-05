#[doc = "Reader of register MR"]
pub type R = crate::R<u32, super::MR>;
#[doc = "Writer for register MR"]
pub type W = crate::W<u32, super::MR>;
#[doc = "Register MR `reset()`'s with value 0"]
impl crate::ResetValue for super::MR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN_A {
    #[doc = "0: External trigger mode disabled. DACC in free-running mode"]
    DIS = 0,
    #[doc = "1: External trigger mode enabled"]
    EN = 1,
}
impl From<TRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: TRGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TRGEN`"]
pub type TRGEN_R = crate::R<bool, TRGEN_A>;
impl TRGEN_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `TRGEN`"]
pub struct TRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External trigger mode disabled. DACC in free-running mode"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRGEN_A::DIS)
    }
    #[doc = "External trigger mode enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TRGEN_A::EN)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `TRGSEL`"]
pub type TRGSEL_R = crate::R<u8, TRGSEL_A>;
impl TRGSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TRGSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TRGSEL_A::TRGSEL0),
            1 => Val(TRGSEL_A::TRGSEL1),
            2 => Val(TRGSEL_A::TRGSEL2),
            3 => Val(TRGSEL_A::TRGSEL3),
            4 => Val(TRGSEL_A::TRGSEL4),
            5 => Val(TRGSEL_A::TRGSEL5),
            i => Res(i),
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
#[doc = "Write proxy for field `TRGSEL`"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Word Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `WORD`"]
pub type WORD_R = crate::R<bool, WORD_A>;
impl WORD_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `WORD`"]
pub struct WORD_W<'a> {
    w: &'a mut W,
}
impl<'a> WORD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORD_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `ONE`"]
pub type ONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ONE`"]
pub struct ONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "User Channel Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `USER_SEL`"]
pub type USER_SEL_R = crate::R<u8, USER_SEL_A>;
impl USER_SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, USER_SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(USER_SEL_A::CHANNEL0),
            1 => Val(USER_SEL_A::CHANNEL1),
            i => Res(i),
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
#[doc = "Write proxy for field `USER_SEL`"]
pub struct USER_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USER_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USER_SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Tag Selection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TAG_A {
    #[doc = "0: Tag selection mode disabled. Using USER_SEL to select the channel for the conversion"]
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
#[doc = "Reader of field `TAG`"]
pub type TAG_R = crate::R<bool, TAG_A>;
impl TAG_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `TAG`"]
pub struct TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAG_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Tag selection mode disabled. Using USER_SEL to select the channel for the conversion"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TAG_A::DIS)
    }
    #[doc = "Tag selection mode enabled"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TAG_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Maximum Speed Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `MAXS`"]
pub type MAXS_R = crate::R<bool, MAXS_A>;
impl MAXS_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `MAXS`"]
pub struct MAXS_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAXS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Startup Time Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Reader of field `STARTUP`"]
pub type STARTUP_R = crate::R<u8, STARTUP_A>;
impl STARTUP_R {
    #[doc = r"Get enumerated values variant"]
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
#[doc = "Write proxy for field `STARTUP`"]
pub struct STARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTUP_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Trigger Enable"]
    #[inline(always)]
    pub fn trgen(&self) -> TRGEN_R {
        TRGEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Word Transfer"]
    #[inline(always)]
    pub fn word(&self) -> WORD_R {
        WORD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&self) -> ONE_R {
        ONE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - User Channel Selection"]
    #[inline(always)]
    pub fn user_sel(&self) -> USER_SEL_R {
        USER_SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Tag Selection Mode"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Maximum Speed Mode"]
    #[inline(always)]
    pub fn maxs(&self) -> MAXS_R {
        MAXS_R::new(((self.bits >> 21) & 0x01) != 0)
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
    pub fn trgen(&mut self) -> TRGEN_W {
        TRGEN_W { w: self }
    }
    #[doc = "Bits 1:3 - Trigger Selection"]
    #[inline(always)]
    pub fn trgsel(&mut self) -> TRGSEL_W {
        TRGSEL_W { w: self }
    }
    #[doc = "Bit 4 - Word Transfer"]
    #[inline(always)]
    pub fn word(&mut self) -> WORD_W {
        WORD_W { w: self }
    }
    #[doc = "Bit 8 - Must Be Set to 1"]
    #[inline(always)]
    pub fn one(&mut self) -> ONE_W {
        ONE_W { w: self }
    }
    #[doc = "Bits 16:17 - User Channel Selection"]
    #[inline(always)]
    pub fn user_sel(&mut self) -> USER_SEL_W {
        USER_SEL_W { w: self }
    }
    #[doc = "Bit 20 - Tag Selection Mode"]
    #[inline(always)]
    pub fn tag(&mut self) -> TAG_W {
        TAG_W { w: self }
    }
    #[doc = "Bit 21 - Maximum Speed Mode"]
    #[inline(always)]
    pub fn maxs(&mut self) -> MAXS_W {
        MAXS_W { w: self }
    }
    #[doc = "Bits 24:29 - Startup Time Selection"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
}
