#[doc = "Register `CHCTRLA` reader"]
pub struct R(crate::R<CHCTRLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCTRLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCTRLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCTRLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCTRLA` writer"]
pub struct W(crate::W<CHCTRLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCTRLA_SPEC>;
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
impl From<crate::W<CHCTRLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCTRLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Channel Software Reset"]
pub struct SWRST_R(crate::FieldReader<bool, bool>);
impl SWRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWRST` writer - Channel Software Reset"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
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
#[doc = "Field `ENABLE` reader - Channel Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Channel Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RUNSTDBY` reader - Channel Run in Standby"]
pub struct RUNSTDBY_R(crate::FieldReader<bool, bool>);
impl RUNSTDBY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RUNSTDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNSTDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNSTDBY` writer - Channel Run in Standby"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGSRC_A {
    #[doc = "0: Only software/event triggers"]
    DISABLE = 0,
}
impl From<TRIGSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIGSRC` reader - Trigger Source"]
pub struct TRIGSRC_R(crate::FieldReader<u8, TRIGSRC_A>);
impl TRIGSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIGSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGSRC_A> {
        match self.bits {
            0 => Some(TRIGSRC_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TRIGSRC_A::DISABLE
    }
}
impl core::ops::Deref for TRIGSRC_R {
    type Target = crate::FieldReader<u8, TRIGSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGSRC` writer - Trigger Source"]
pub struct TRIGSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGSRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Only software/event triggers"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TRIGSRC_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRIGACT_A {
    #[doc = "0: One trigger required for each block transfer"]
    BLOCK = 0,
    #[doc = "2: One trigger required for each burst transfer"]
    BURST = 2,
    #[doc = "3: One trigger required for each transaction"]
    TRANSACTION = 3,
}
impl From<TRIGACT_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGACT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRIGACT` reader - Trigger Action"]
pub struct TRIGACT_R(crate::FieldReader<u8, TRIGACT_A>);
impl TRIGACT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRIGACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGACT_A> {
        match self.bits {
            0 => Some(TRIGACT_A::BLOCK),
            2 => Some(TRIGACT_A::BURST),
            3 => Some(TRIGACT_A::TRANSACTION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        **self == TRIGACT_A::BLOCK
    }
    #[doc = "Checks if the value of the field is `BURST`"]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        **self == TRIGACT_A::BURST
    }
    #[doc = "Checks if the value of the field is `TRANSACTION`"]
    #[inline(always)]
    pub fn is_transaction(&self) -> bool {
        **self == TRIGACT_A::TRANSACTION
    }
}
impl core::ops::Deref for TRIGACT_R {
    type Target = crate::FieldReader<u8, TRIGACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIGACT` writer - Trigger Action"]
pub struct TRIGACT_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIGACT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRIGACT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "One trigger required for each block transfer"]
    #[inline(always)]
    pub fn block(self) -> &'a mut W {
        self.variant(TRIGACT_A::BLOCK)
    }
    #[doc = "One trigger required for each burst transfer"]
    #[inline(always)]
    pub fn burst(self) -> &'a mut W {
        self.variant(TRIGACT_A::BURST)
    }
    #[doc = "One trigger required for each transaction"]
    #[inline(always)]
    pub fn transaction(self) -> &'a mut W {
        self.variant(TRIGACT_A::TRANSACTION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Burst Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BURSTLEN_A {
    #[doc = "0: Single-beat burst length"]
    SINGLE = 0,
    #[doc = "1: 2-beats burst length"]
    _2BEAT = 1,
    #[doc = "2: 3-beats burst length"]
    _3BEAT = 2,
    #[doc = "3: 4-beats burst length"]
    _4BEAT = 3,
    #[doc = "4: 5-beats burst length"]
    _5BEAT = 4,
    #[doc = "5: 6-beats burst length"]
    _6BEAT = 5,
    #[doc = "6: 7-beats burst length"]
    _7BEAT = 6,
    #[doc = "7: 8-beats burst length"]
    _8BEAT = 7,
    #[doc = "8: 9-beats burst length"]
    _9BEAT = 8,
    #[doc = "9: 10-beats burst length"]
    _10BEAT = 9,
    #[doc = "10: 11-beats burst length"]
    _11BEAT = 10,
    #[doc = "11: 12-beats burst length"]
    _12BEAT = 11,
    #[doc = "12: 13-beats burst length"]
    _13BEAT = 12,
    #[doc = "13: 14-beats burst length"]
    _14BEAT = 13,
    #[doc = "14: 15-beats burst length"]
    _15BEAT = 14,
    #[doc = "15: 16-beats burst length"]
    _16BEAT = 15,
}
impl From<BURSTLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: BURSTLEN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BURSTLEN` reader - Burst Length"]
pub struct BURSTLEN_R(crate::FieldReader<u8, BURSTLEN_A>);
impl BURSTLEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        BURSTLEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BURSTLEN_A {
        match self.bits {
            0 => BURSTLEN_A::SINGLE,
            1 => BURSTLEN_A::_2BEAT,
            2 => BURSTLEN_A::_3BEAT,
            3 => BURSTLEN_A::_4BEAT,
            4 => BURSTLEN_A::_5BEAT,
            5 => BURSTLEN_A::_6BEAT,
            6 => BURSTLEN_A::_7BEAT,
            7 => BURSTLEN_A::_8BEAT,
            8 => BURSTLEN_A::_9BEAT,
            9 => BURSTLEN_A::_10BEAT,
            10 => BURSTLEN_A::_11BEAT,
            11 => BURSTLEN_A::_12BEAT,
            12 => BURSTLEN_A::_13BEAT,
            13 => BURSTLEN_A::_14BEAT,
            14 => BURSTLEN_A::_15BEAT,
            15 => BURSTLEN_A::_16BEAT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        **self == BURSTLEN_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `_2BEAT`"]
    #[inline(always)]
    pub fn is_2beat(&self) -> bool {
        **self == BURSTLEN_A::_2BEAT
    }
    #[doc = "Checks if the value of the field is `_3BEAT`"]
    #[inline(always)]
    pub fn is_3beat(&self) -> bool {
        **self == BURSTLEN_A::_3BEAT
    }
    #[doc = "Checks if the value of the field is `_4BEAT`"]
    #[inline(always)]
    pub fn is_4beat(&self) -> bool {
        **self == BURSTLEN_A::_4BEAT
    }
    #[doc = "Checks if the value of the field is `_5BEAT`"]
    #[inline(always)]
    pub fn is_5beat(&self) -> bool {
        **self == BURSTLEN_A::_5BEAT
    }
    #[doc = "Checks if the value of the field is `_6BEAT`"]
    #[inline(always)]
    pub fn is_6beat(&self) -> bool {
        **self == BURSTLEN_A::_6BEAT
    }
    #[doc = "Checks if the value of the field is `_7BEAT`"]
    #[inline(always)]
    pub fn is_7beat(&self) -> bool {
        **self == BURSTLEN_A::_7BEAT
    }
    #[doc = "Checks if the value of the field is `_8BEAT`"]
    #[inline(always)]
    pub fn is_8beat(&self) -> bool {
        **self == BURSTLEN_A::_8BEAT
    }
    #[doc = "Checks if the value of the field is `_9BEAT`"]
    #[inline(always)]
    pub fn is_9beat(&self) -> bool {
        **self == BURSTLEN_A::_9BEAT
    }
    #[doc = "Checks if the value of the field is `_10BEAT`"]
    #[inline(always)]
    pub fn is_10beat(&self) -> bool {
        **self == BURSTLEN_A::_10BEAT
    }
    #[doc = "Checks if the value of the field is `_11BEAT`"]
    #[inline(always)]
    pub fn is_11beat(&self) -> bool {
        **self == BURSTLEN_A::_11BEAT
    }
    #[doc = "Checks if the value of the field is `_12BEAT`"]
    #[inline(always)]
    pub fn is_12beat(&self) -> bool {
        **self == BURSTLEN_A::_12BEAT
    }
    #[doc = "Checks if the value of the field is `_13BEAT`"]
    #[inline(always)]
    pub fn is_13beat(&self) -> bool {
        **self == BURSTLEN_A::_13BEAT
    }
    #[doc = "Checks if the value of the field is `_14BEAT`"]
    #[inline(always)]
    pub fn is_14beat(&self) -> bool {
        **self == BURSTLEN_A::_14BEAT
    }
    #[doc = "Checks if the value of the field is `_15BEAT`"]
    #[inline(always)]
    pub fn is_15beat(&self) -> bool {
        **self == BURSTLEN_A::_15BEAT
    }
    #[doc = "Checks if the value of the field is `_16BEAT`"]
    #[inline(always)]
    pub fn is_16beat(&self) -> bool {
        **self == BURSTLEN_A::_16BEAT
    }
}
impl core::ops::Deref for BURSTLEN_R {
    type Target = crate::FieldReader<u8, BURSTLEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BURSTLEN` writer - Burst Length"]
pub struct BURSTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTLEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BURSTLEN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single-beat burst length"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(BURSTLEN_A::SINGLE)
    }
    #[doc = "2-beats burst length"]
    #[inline(always)]
    pub fn _2beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_2BEAT)
    }
    #[doc = "3-beats burst length"]
    #[inline(always)]
    pub fn _3beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_3BEAT)
    }
    #[doc = "4-beats burst length"]
    #[inline(always)]
    pub fn _4beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_4BEAT)
    }
    #[doc = "5-beats burst length"]
    #[inline(always)]
    pub fn _5beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_5BEAT)
    }
    #[doc = "6-beats burst length"]
    #[inline(always)]
    pub fn _6beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_6BEAT)
    }
    #[doc = "7-beats burst length"]
    #[inline(always)]
    pub fn _7beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_7BEAT)
    }
    #[doc = "8-beats burst length"]
    #[inline(always)]
    pub fn _8beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_8BEAT)
    }
    #[doc = "9-beats burst length"]
    #[inline(always)]
    pub fn _9beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_9BEAT)
    }
    #[doc = "10-beats burst length"]
    #[inline(always)]
    pub fn _10beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_10BEAT)
    }
    #[doc = "11-beats burst length"]
    #[inline(always)]
    pub fn _11beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_11BEAT)
    }
    #[doc = "12-beats burst length"]
    #[inline(always)]
    pub fn _12beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_12BEAT)
    }
    #[doc = "13-beats burst length"]
    #[inline(always)]
    pub fn _13beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_13BEAT)
    }
    #[doc = "14-beats burst length"]
    #[inline(always)]
    pub fn _14beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_14BEAT)
    }
    #[doc = "15-beats burst length"]
    #[inline(always)]
    pub fn _15beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_15BEAT)
    }
    #[doc = "16-beats burst length"]
    #[inline(always)]
    pub fn _16beat(self) -> &'a mut W {
        self.variant(BURSTLEN_A::_16BEAT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "FIFO Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum THRESHOLD_A {
    #[doc = "0: Destination write starts after each beat source address read"]
    _1BEAT = 0,
    #[doc = "1: Destination write starts after 2-beats source address read"]
    _2BEATS = 1,
    #[doc = "2: Destination write starts after 4-beats source address read"]
    _4BEATS = 2,
    #[doc = "3: Destination write starts after 8-beats source address read"]
    _8BEATS = 3,
}
impl From<THRESHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: THRESHOLD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `THRESHOLD` reader - FIFO Threshold"]
pub struct THRESHOLD_R(crate::FieldReader<u8, THRESHOLD_A>);
impl THRESHOLD_R {
    pub(crate) fn new(bits: u8) -> Self {
        THRESHOLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THRESHOLD_A {
        match self.bits {
            0 => THRESHOLD_A::_1BEAT,
            1 => THRESHOLD_A::_2BEATS,
            2 => THRESHOLD_A::_4BEATS,
            3 => THRESHOLD_A::_8BEATS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1BEAT`"]
    #[inline(always)]
    pub fn is_1beat(&self) -> bool {
        **self == THRESHOLD_A::_1BEAT
    }
    #[doc = "Checks if the value of the field is `_2BEATS`"]
    #[inline(always)]
    pub fn is_2beats(&self) -> bool {
        **self == THRESHOLD_A::_2BEATS
    }
    #[doc = "Checks if the value of the field is `_4BEATS`"]
    #[inline(always)]
    pub fn is_4beats(&self) -> bool {
        **self == THRESHOLD_A::_4BEATS
    }
    #[doc = "Checks if the value of the field is `_8BEATS`"]
    #[inline(always)]
    pub fn is_8beats(&self) -> bool {
        **self == THRESHOLD_A::_8BEATS
    }
}
impl core::ops::Deref for THRESHOLD_R {
    type Target = crate::FieldReader<u8, THRESHOLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `THRESHOLD` writer - FIFO Threshold"]
pub struct THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THRESHOLD_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Destination write starts after each beat source address read"]
    #[inline(always)]
    pub fn _1beat(self) -> &'a mut W {
        self.variant(THRESHOLD_A::_1BEAT)
    }
    #[doc = "Destination write starts after 2-beats source address read"]
    #[inline(always)]
    pub fn _2beats(self) -> &'a mut W {
        self.variant(THRESHOLD_A::_2BEATS)
    }
    #[doc = "Destination write starts after 4-beats source address read"]
    #[inline(always)]
    pub fn _4beats(self) -> &'a mut W {
        self.variant(THRESHOLD_A::_4BEATS)
    }
    #[doc = "Destination write starts after 8-beats source address read"]
    #[inline(always)]
    pub fn _8beats(self) -> &'a mut W {
        self.variant(THRESHOLD_A::_8BEATS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&self) -> TRIGSRC_R {
        TRIGSRC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&self) -> TRIGACT_R {
        TRIGACT_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline(always)]
    pub fn burstlen(&self) -> BURSTLEN_R {
        BURSTLEN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 1 - Channel Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 6 - Channel Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bits 8:14 - Trigger Source"]
    #[inline(always)]
    pub fn trigsrc(&mut self) -> TRIGSRC_W {
        TRIGSRC_W { w: self }
    }
    #[doc = "Bits 20:21 - Trigger Action"]
    #[inline(always)]
    pub fn trigact(&mut self) -> TRIGACT_W {
        TRIGACT_W { w: self }
    }
    #[doc = "Bits 24:27 - Burst Length"]
    #[inline(always)]
    pub fn burstlen(&mut self) -> BURSTLEN_W {
        BURSTLEN_W { w: self }
    }
    #[doc = "Bits 28:29 - FIFO Threshold"]
    #[inline(always)]
    pub fn threshold(&mut self) -> THRESHOLD_W {
        THRESHOLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Control A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chctrla](index.html) module"]
pub struct CHCTRLA_SPEC;
impl crate::RegisterSpec for CHCTRLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chctrla::R](R) reader structure"]
impl crate::Readable for CHCTRLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chctrla::W](W) writer structure"]
impl crate::Writable for CHCTRLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHCTRLA to value 0"]
impl crate::Resettable for CHCTRLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
