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
#[doc = "Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRGEN_A {
    #[doc = "0: External trigger mode disabled. DACC in free running mode."]
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
#[doc = "Field `TRGEN` reader - Trigger Enable"]
pub struct TRGEN_R(crate::FieldReader<bool, TRGEN_A>);
impl TRGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRGEN_R(crate::FieldReader::new(bits))
    }
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
        **self == TRGEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TRGEN_A::EN
    }
}
impl core::ops::Deref for TRGEN_R {
    type Target = crate::FieldReader<bool, TRGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGEN` writer - Trigger Enable"]
pub struct TRGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "External trigger mode disabled. DACC in free running mode."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRGEN_A::DIS)
    }
    #[doc = "External trigger mode enabled."]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub struct TRGSEL_R(crate::FieldReader<u8, u8>);
impl TRGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRGSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Selection"]
pub struct TRGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Word Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WORD_A {
    #[doc = "0: Half-Word transfer"]
    HALF = 0,
    #[doc = "1: Word Transfer"]
    WORD = 1,
}
impl From<WORD_A> for bool {
    #[inline(always)]
    fn from(variant: WORD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WORD` reader - Word Transfer"]
pub struct WORD_R(crate::FieldReader<bool, WORD_A>);
impl WORD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WORD_R(crate::FieldReader::new(bits))
    }
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
        **self == WORD_A::HALF
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        **self == WORD_A::WORD
    }
}
impl core::ops::Deref for WORD_R {
    type Target = crate::FieldReader<bool, WORD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WORD` writer - Word Transfer"]
pub struct WORD_W<'a> {
    w: &'a mut W,
}
impl<'a> WORD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WORD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Half-Word transfer"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(WORD_A::HALF)
    }
    #[doc = "Word Transfer"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `SLEEP` reader - Sleep Mode"]
pub struct SLEEP_R(crate::FieldReader<bool, bool>);
impl SLEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEEP` writer - Sleep Mode"]
pub struct SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEEP_W<'a> {
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
#[doc = "Field `FASTWKUP` reader - Fast Wake up Mode"]
pub struct FASTWKUP_R(crate::FieldReader<bool, bool>);
impl FASTWKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FASTWKUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FASTWKUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FASTWKUP` writer - Fast Wake up Mode"]
pub struct FASTWKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FASTWKUP_W<'a> {
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
#[doc = "Field `REFRESH` reader - Refresh Period"]
pub struct REFRESH_R(crate::FieldReader<u8, u8>);
impl REFRESH_R {
    pub(crate) fn new(bits: u8) -> Self {
        REFRESH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REFRESH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFRESH` writer - Refresh Period"]
pub struct REFRESH_W<'a> {
    w: &'a mut W,
}
impl<'a> REFRESH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
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
#[doc = "Field `USER_SEL` reader - User Channel Selection"]
pub struct USER_SEL_R(crate::FieldReader<u8, USER_SEL_A>);
impl USER_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USER_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
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
        **self == USER_SEL_A::CHANNEL0
    }
    #[doc = "Checks if the value of the field is `CHANNEL1`"]
    #[inline(always)]
    pub fn is_channel1(&self) -> bool {
        **self == USER_SEL_A::CHANNEL1
    }
}
impl core::ops::Deref for USER_SEL_R {
    type Target = crate::FieldReader<u8, USER_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USER_SEL` writer - User Channel Selection"]
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
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Tag Selection Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[doc = "Field `TAG` reader - Tag Selection Mode"]
pub struct TAG_R(crate::FieldReader<bool, TAG_A>);
impl TAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAG_R(crate::FieldReader::new(bits))
    }
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
        **self == TAG_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TAG_A::EN
    }
}
impl core::ops::Deref for TAG_R {
    type Target = crate::FieldReader<bool, TAG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAG` writer - Tag Selection Mode"]
pub struct TAG_W<'a> {
    w: &'a mut W,
}
impl<'a> TAG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TAG_A) -> &'a mut W {
        self.bit(variant.into())
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Max Speed Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXS_A {
    #[doc = "0: Normal Mode"]
    NORMAL = 0,
    #[doc = "1: Max Speed Mode enabled"]
    MAXIMUM = 1,
}
impl From<MAXS_A> for bool {
    #[inline(always)]
    fn from(variant: MAXS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAXS` reader - Max Speed Mode"]
pub struct MAXS_R(crate::FieldReader<bool, MAXS_A>);
impl MAXS_R {
    pub(crate) fn new(bits: bool) -> Self {
        MAXS_R(crate::FieldReader::new(bits))
    }
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
        **self == MAXS_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `MAXIMUM`"]
    #[inline(always)]
    pub fn is_maximum(&self) -> bool {
        **self == MAXS_A::MAXIMUM
    }
}
impl core::ops::Deref for MAXS_R {
    type Target = crate::FieldReader<bool, MAXS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MAXS` writer - Max Speed Mode"]
pub struct MAXS_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MAXS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MAXS_A::NORMAL)
    }
    #[doc = "Max Speed Mode enabled"]
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Startup Time Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 0 periods of DACClock"]
    _0 = 0,
    #[doc = "1: 8 periods of DACClock"]
    _8 = 1,
    #[doc = "2: 16 periods of DACClock"]
    _16 = 2,
    #[doc = "3: 24 periods of DACClock"]
    _24 = 3,
    #[doc = "4: 64 periods of DACClock"]
    _64 = 4,
    #[doc = "5: 80 periods of DACClock"]
    _80 = 5,
    #[doc = "6: 96 periods of DACClock"]
    _96 = 6,
    #[doc = "7: 112 periods of DACClock"]
    _112 = 7,
    #[doc = "8: 512 periods of DACClock"]
    _512 = 8,
    #[doc = "9: 576 periods of DACClock"]
    _576 = 9,
    #[doc = "10: 640 periods of DACClock"]
    _640 = 10,
    #[doc = "11: 704 periods of DACClock"]
    _704 = 11,
    #[doc = "12: 768 periods of DACClock"]
    _768 = 12,
    #[doc = "13: 832 periods of DACClock"]
    _832 = 13,
    #[doc = "14: 896 periods of DACClock"]
    _896 = 14,
    #[doc = "15: 960 periods of DACClock"]
    _960 = 15,
    #[doc = "16: 1024 periods of DACClock"]
    _1024 = 16,
    #[doc = "17: 1088 periods of DACClock"]
    _1088 = 17,
    #[doc = "18: 1152 periods of DACClock"]
    _1152 = 18,
    #[doc = "19: 1216 periods of DACClock"]
    _1216 = 19,
    #[doc = "20: 1280 periods of DACClock"]
    _1280 = 20,
    #[doc = "21: 1344 periods of DACClock"]
    _1344 = 21,
    #[doc = "22: 1408 periods of DACClock"]
    _1408 = 22,
    #[doc = "23: 1472 periods of DACClock"]
    _1472 = 23,
    #[doc = "24: 1536 periods of DACClock"]
    _1536 = 24,
    #[doc = "25: 1600 periods of DACClock"]
    _1600 = 25,
    #[doc = "26: 1664 periods of DACClock"]
    _1664 = 26,
    #[doc = "27: 1728 periods of DACClock"]
    _1728 = 27,
    #[doc = "28: 1792 periods of DACClock"]
    _1792 = 28,
    #[doc = "29: 1856 periods of DACClock"]
    _1856 = 29,
    #[doc = "30: 1920 periods of DACClock"]
    _1920 = 30,
    #[doc = "31: 1984 periods of DACClock"]
    _1984 = 31,
}
impl From<STARTUP_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STARTUP` reader - Startup Time Selection"]
pub struct STARTUP_R(crate::FieldReader<u8, STARTUP_A>);
impl STARTUP_R {
    pub(crate) fn new(bits: u8) -> Self {
        STARTUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STARTUP_A> {
        match self.bits {
            0 => Some(STARTUP_A::_0),
            1 => Some(STARTUP_A::_8),
            2 => Some(STARTUP_A::_16),
            3 => Some(STARTUP_A::_24),
            4 => Some(STARTUP_A::_64),
            5 => Some(STARTUP_A::_80),
            6 => Some(STARTUP_A::_96),
            7 => Some(STARTUP_A::_112),
            8 => Some(STARTUP_A::_512),
            9 => Some(STARTUP_A::_576),
            10 => Some(STARTUP_A::_640),
            11 => Some(STARTUP_A::_704),
            12 => Some(STARTUP_A::_768),
            13 => Some(STARTUP_A::_832),
            14 => Some(STARTUP_A::_896),
            15 => Some(STARTUP_A::_960),
            16 => Some(STARTUP_A::_1024),
            17 => Some(STARTUP_A::_1088),
            18 => Some(STARTUP_A::_1152),
            19 => Some(STARTUP_A::_1216),
            20 => Some(STARTUP_A::_1280),
            21 => Some(STARTUP_A::_1344),
            22 => Some(STARTUP_A::_1408),
            23 => Some(STARTUP_A::_1472),
            24 => Some(STARTUP_A::_1536),
            25 => Some(STARTUP_A::_1600),
            26 => Some(STARTUP_A::_1664),
            27 => Some(STARTUP_A::_1728),
            28 => Some(STARTUP_A::_1792),
            29 => Some(STARTUP_A::_1856),
            30 => Some(STARTUP_A::_1920),
            31 => Some(STARTUP_A::_1984),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == STARTUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        **self == STARTUP_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        **self == STARTUP_A::_16
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        **self == STARTUP_A::_24
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        **self == STARTUP_A::_64
    }
    #[doc = "Checks if the value of the field is `_80`"]
    #[inline(always)]
    pub fn is_80(&self) -> bool {
        **self == STARTUP_A::_80
    }
    #[doc = "Checks if the value of the field is `_96`"]
    #[inline(always)]
    pub fn is_96(&self) -> bool {
        **self == STARTUP_A::_96
    }
    #[doc = "Checks if the value of the field is `_112`"]
    #[inline(always)]
    pub fn is_112(&self) -> bool {
        **self == STARTUP_A::_112
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        **self == STARTUP_A::_512
    }
    #[doc = "Checks if the value of the field is `_576`"]
    #[inline(always)]
    pub fn is_576(&self) -> bool {
        **self == STARTUP_A::_576
    }
    #[doc = "Checks if the value of the field is `_640`"]
    #[inline(always)]
    pub fn is_640(&self) -> bool {
        **self == STARTUP_A::_640
    }
    #[doc = "Checks if the value of the field is `_704`"]
    #[inline(always)]
    pub fn is_704(&self) -> bool {
        **self == STARTUP_A::_704
    }
    #[doc = "Checks if the value of the field is `_768`"]
    #[inline(always)]
    pub fn is_768(&self) -> bool {
        **self == STARTUP_A::_768
    }
    #[doc = "Checks if the value of the field is `_832`"]
    #[inline(always)]
    pub fn is_832(&self) -> bool {
        **self == STARTUP_A::_832
    }
    #[doc = "Checks if the value of the field is `_896`"]
    #[inline(always)]
    pub fn is_896(&self) -> bool {
        **self == STARTUP_A::_896
    }
    #[doc = "Checks if the value of the field is `_960`"]
    #[inline(always)]
    pub fn is_960(&self) -> bool {
        **self == STARTUP_A::_960
    }
    #[doc = "Checks if the value of the field is `_1024`"]
    #[inline(always)]
    pub fn is_1024(&self) -> bool {
        **self == STARTUP_A::_1024
    }
    #[doc = "Checks if the value of the field is `_1088`"]
    #[inline(always)]
    pub fn is_1088(&self) -> bool {
        **self == STARTUP_A::_1088
    }
    #[doc = "Checks if the value of the field is `_1152`"]
    #[inline(always)]
    pub fn is_1152(&self) -> bool {
        **self == STARTUP_A::_1152
    }
    #[doc = "Checks if the value of the field is `_1216`"]
    #[inline(always)]
    pub fn is_1216(&self) -> bool {
        **self == STARTUP_A::_1216
    }
    #[doc = "Checks if the value of the field is `_1280`"]
    #[inline(always)]
    pub fn is_1280(&self) -> bool {
        **self == STARTUP_A::_1280
    }
    #[doc = "Checks if the value of the field is `_1344`"]
    #[inline(always)]
    pub fn is_1344(&self) -> bool {
        **self == STARTUP_A::_1344
    }
    #[doc = "Checks if the value of the field is `_1408`"]
    #[inline(always)]
    pub fn is_1408(&self) -> bool {
        **self == STARTUP_A::_1408
    }
    #[doc = "Checks if the value of the field is `_1472`"]
    #[inline(always)]
    pub fn is_1472(&self) -> bool {
        **self == STARTUP_A::_1472
    }
    #[doc = "Checks if the value of the field is `_1536`"]
    #[inline(always)]
    pub fn is_1536(&self) -> bool {
        **self == STARTUP_A::_1536
    }
    #[doc = "Checks if the value of the field is `_1600`"]
    #[inline(always)]
    pub fn is_1600(&self) -> bool {
        **self == STARTUP_A::_1600
    }
    #[doc = "Checks if the value of the field is `_1664`"]
    #[inline(always)]
    pub fn is_1664(&self) -> bool {
        **self == STARTUP_A::_1664
    }
    #[doc = "Checks if the value of the field is `_1728`"]
    #[inline(always)]
    pub fn is_1728(&self) -> bool {
        **self == STARTUP_A::_1728
    }
    #[doc = "Checks if the value of the field is `_1792`"]
    #[inline(always)]
    pub fn is_1792(&self) -> bool {
        **self == STARTUP_A::_1792
    }
    #[doc = "Checks if the value of the field is `_1856`"]
    #[inline(always)]
    pub fn is_1856(&self) -> bool {
        **self == STARTUP_A::_1856
    }
    #[doc = "Checks if the value of the field is `_1920`"]
    #[inline(always)]
    pub fn is_1920(&self) -> bool {
        **self == STARTUP_A::_1920
    }
    #[doc = "Checks if the value of the field is `_1984`"]
    #[inline(always)]
    pub fn is_1984(&self) -> bool {
        **self == STARTUP_A::_1984
    }
}
impl core::ops::Deref for STARTUP_R {
    type Target = crate::FieldReader<u8, STARTUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTUP` writer - Startup Time Selection"]
pub struct STARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTUP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 periods of DACClock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STARTUP_A::_0)
    }
    #[doc = "8 periods of DACClock"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(STARTUP_A::_8)
    }
    #[doc = "16 periods of DACClock"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(STARTUP_A::_16)
    }
    #[doc = "24 periods of DACClock"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(STARTUP_A::_24)
    }
    #[doc = "64 periods of DACClock"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(STARTUP_A::_64)
    }
    #[doc = "80 periods of DACClock"]
    #[inline(always)]
    pub fn _80(self) -> &'a mut W {
        self.variant(STARTUP_A::_80)
    }
    #[doc = "96 periods of DACClock"]
    #[inline(always)]
    pub fn _96(self) -> &'a mut W {
        self.variant(STARTUP_A::_96)
    }
    #[doc = "112 periods of DACClock"]
    #[inline(always)]
    pub fn _112(self) -> &'a mut W {
        self.variant(STARTUP_A::_112)
    }
    #[doc = "512 periods of DACClock"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(STARTUP_A::_512)
    }
    #[doc = "576 periods of DACClock"]
    #[inline(always)]
    pub fn _576(self) -> &'a mut W {
        self.variant(STARTUP_A::_576)
    }
    #[doc = "640 periods of DACClock"]
    #[inline(always)]
    pub fn _640(self) -> &'a mut W {
        self.variant(STARTUP_A::_640)
    }
    #[doc = "704 periods of DACClock"]
    #[inline(always)]
    pub fn _704(self) -> &'a mut W {
        self.variant(STARTUP_A::_704)
    }
    #[doc = "768 periods of DACClock"]
    #[inline(always)]
    pub fn _768(self) -> &'a mut W {
        self.variant(STARTUP_A::_768)
    }
    #[doc = "832 periods of DACClock"]
    #[inline(always)]
    pub fn _832(self) -> &'a mut W {
        self.variant(STARTUP_A::_832)
    }
    #[doc = "896 periods of DACClock"]
    #[inline(always)]
    pub fn _896(self) -> &'a mut W {
        self.variant(STARTUP_A::_896)
    }
    #[doc = "960 periods of DACClock"]
    #[inline(always)]
    pub fn _960(self) -> &'a mut W {
        self.variant(STARTUP_A::_960)
    }
    #[doc = "1024 periods of DACClock"]
    #[inline(always)]
    pub fn _1024(self) -> &'a mut W {
        self.variant(STARTUP_A::_1024)
    }
    #[doc = "1088 periods of DACClock"]
    #[inline(always)]
    pub fn _1088(self) -> &'a mut W {
        self.variant(STARTUP_A::_1088)
    }
    #[doc = "1152 periods of DACClock"]
    #[inline(always)]
    pub fn _1152(self) -> &'a mut W {
        self.variant(STARTUP_A::_1152)
    }
    #[doc = "1216 periods of DACClock"]
    #[inline(always)]
    pub fn _1216(self) -> &'a mut W {
        self.variant(STARTUP_A::_1216)
    }
    #[doc = "1280 periods of DACClock"]
    #[inline(always)]
    pub fn _1280(self) -> &'a mut W {
        self.variant(STARTUP_A::_1280)
    }
    #[doc = "1344 periods of DACClock"]
    #[inline(always)]
    pub fn _1344(self) -> &'a mut W {
        self.variant(STARTUP_A::_1344)
    }
    #[doc = "1408 periods of DACClock"]
    #[inline(always)]
    pub fn _1408(self) -> &'a mut W {
        self.variant(STARTUP_A::_1408)
    }
    #[doc = "1472 periods of DACClock"]
    #[inline(always)]
    pub fn _1472(self) -> &'a mut W {
        self.variant(STARTUP_A::_1472)
    }
    #[doc = "1536 periods of DACClock"]
    #[inline(always)]
    pub fn _1536(self) -> &'a mut W {
        self.variant(STARTUP_A::_1536)
    }
    #[doc = "1600 periods of DACClock"]
    #[inline(always)]
    pub fn _1600(self) -> &'a mut W {
        self.variant(STARTUP_A::_1600)
    }
    #[doc = "1664 periods of DACClock"]
    #[inline(always)]
    pub fn _1664(self) -> &'a mut W {
        self.variant(STARTUP_A::_1664)
    }
    #[doc = "1728 periods of DACClock"]
    #[inline(always)]
    pub fn _1728(self) -> &'a mut W {
        self.variant(STARTUP_A::_1728)
    }
    #[doc = "1792 periods of DACClock"]
    #[inline(always)]
    pub fn _1792(self) -> &'a mut W {
        self.variant(STARTUP_A::_1792)
    }
    #[doc = "1856 periods of DACClock"]
    #[inline(always)]
    pub fn _1856(self) -> &'a mut W {
        self.variant(STARTUP_A::_1856)
    }
    #[doc = "1920 periods of DACClock"]
    #[inline(always)]
    pub fn _1920(self) -> &'a mut W {
        self.variant(STARTUP_A::_1920)
    }
    #[doc = "1984 periods of DACClock"]
    #[inline(always)]
    pub fn _1984(self) -> &'a mut W {
        self.variant(STARTUP_A::_1984)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
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
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast Wake up Mode"]
    #[inline(always)]
    pub fn fastwkup(&self) -> FASTWKUP_R {
        FASTWKUP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Refresh Period"]
    #[inline(always)]
    pub fn refresh(&self) -> REFRESH_R {
        REFRESH_R::new(((self.bits >> 8) & 0xff) as u8)
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
    #[doc = "Bit 21 - Max Speed Mode"]
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
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 6 - Fast Wake up Mode"]
    #[inline(always)]
    pub fn fastwkup(&mut self) -> FASTWKUP_W {
        FASTWKUP_W { w: self }
    }
    #[doc = "Bits 8:15 - Refresh Period"]
    #[inline(always)]
    pub fn refresh(&mut self) -> REFRESH_W {
        REFRESH_W { w: self }
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
    #[doc = "Bit 21 - Max Speed Mode"]
    #[inline(always)]
    pub fn maxs(&mut self) -> MAXS_W {
        MAXS_W { w: self }
    }
    #[doc = "Bits 24:29 - Startup Time Selection"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
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
