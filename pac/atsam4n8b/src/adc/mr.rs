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
    #[doc = "0: Hardware triggers are disabled. Starting a conversion is only possible by software."]
    DIS = 0,
    #[doc = "1: Hardware trigger selected by TRGSEL field is enabled."]
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
    #[doc = "Hardware triggers are disabled. Starting a conversion is only possible by software."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TRGEN_A::DIS)
    }
    #[doc = "Hardware trigger selected by TRGSEL field is enabled."]
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
#[doc = "Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGSEL_A {
    #[doc = "0: External trigger"]
    ADC_TRIG0 = 0,
    #[doc = "1: TIO Output of the Timer Counter Channel 0"]
    ADC_TRIG1 = 1,
    #[doc = "2: TIO Output of the Timer Counter Channel 1"]
    ADC_TRIG2 = 2,
    #[doc = "3: TIO Output of the Timer Counter Channel 2"]
    ADC_TRIG3 = 3,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Selection"]
pub struct TRGSEL_R(crate::FieldReader<u8, TRGSEL_A>);
impl TRGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRGSEL_A> {
        match self.bits {
            0 => Some(TRGSEL_A::ADC_TRIG0),
            1 => Some(TRGSEL_A::ADC_TRIG1),
            2 => Some(TRGSEL_A::ADC_TRIG2),
            3 => Some(TRGSEL_A::ADC_TRIG3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC_TRIG0`"]
    #[inline(always)]
    pub fn is_adc_trig0(&self) -> bool {
        **self == TRGSEL_A::ADC_TRIG0
    }
    #[doc = "Checks if the value of the field is `ADC_TRIG1`"]
    #[inline(always)]
    pub fn is_adc_trig1(&self) -> bool {
        **self == TRGSEL_A::ADC_TRIG1
    }
    #[doc = "Checks if the value of the field is `ADC_TRIG2`"]
    #[inline(always)]
    pub fn is_adc_trig2(&self) -> bool {
        **self == TRGSEL_A::ADC_TRIG2
    }
    #[doc = "Checks if the value of the field is `ADC_TRIG3`"]
    #[inline(always)]
    pub fn is_adc_trig3(&self) -> bool {
        **self == TRGSEL_A::ADC_TRIG3
    }
}
impl core::ops::Deref for TRGSEL_R {
    type Target = crate::FieldReader<u8, TRGSEL_A>;
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
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "External trigger"]
    #[inline(always)]
    pub fn adc_trig0(self) -> &'a mut W {
        self.variant(TRGSEL_A::ADC_TRIG0)
    }
    #[doc = "TIO Output of the Timer Counter Channel 0"]
    #[inline(always)]
    pub fn adc_trig1(self) -> &'a mut W {
        self.variant(TRGSEL_A::ADC_TRIG1)
    }
    #[doc = "TIO Output of the Timer Counter Channel 1"]
    #[inline(always)]
    pub fn adc_trig2(self) -> &'a mut W {
        self.variant(TRGSEL_A::ADC_TRIG2)
    }
    #[doc = "TIO Output of the Timer Counter Channel 2"]
    #[inline(always)]
    pub fn adc_trig3(self) -> &'a mut W {
        self.variant(TRGSEL_A::ADC_TRIG3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOWRES_A {
    #[doc = "0: 10-bit resolution. For higher resolution by interpolation, please refer to \"ADC Extended Mode Register\" on page 81"]
    BITS_10 = 0,
    #[doc = "1: 8-bit resolution"]
    BITS_8 = 1,
}
impl From<LOWRES_A> for bool {
    #[inline(always)]
    fn from(variant: LOWRES_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOWRES` reader - Resolution"]
pub struct LOWRES_R(crate::FieldReader<bool, LOWRES_A>);
impl LOWRES_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOWRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOWRES_A {
        match self.bits {
            false => LOWRES_A::BITS_10,
            true => LOWRES_A::BITS_8,
        }
    }
    #[doc = "Checks if the value of the field is `BITS_10`"]
    #[inline(always)]
    pub fn is_bits_10(&self) -> bool {
        **self == LOWRES_A::BITS_10
    }
    #[doc = "Checks if the value of the field is `BITS_8`"]
    #[inline(always)]
    pub fn is_bits_8(&self) -> bool {
        **self == LOWRES_A::BITS_8
    }
}
impl core::ops::Deref for LOWRES_R {
    type Target = crate::FieldReader<bool, LOWRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOWRES` writer - Resolution"]
pub struct LOWRES_W<'a> {
    w: &'a mut W,
}
impl<'a> LOWRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOWRES_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "10-bit resolution. For higher resolution by interpolation, please refer to \"ADC Extended Mode Register\" on page 81"]
    #[inline(always)]
    pub fn bits_10(self) -> &'a mut W {
        self.variant(LOWRES_A::BITS_10)
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn bits_8(self) -> &'a mut W {
        self.variant(LOWRES_A::BITS_8)
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
#[doc = "Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: Normal Mode: The ADC Core and reference voltage circuitry are kept ON between conversions"]
    NORMAL = 0,
    #[doc = "1: Sleep Mode: The ADC Core and reference voltage circuitry are OFF between conversions"]
    SLEEP = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP` reader - Sleep Mode"]
pub struct SLEEP_R(crate::FieldReader<bool, SLEEP_A>);
impl SLEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::NORMAL,
            true => SLEEP_A::SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == SLEEP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        **self == SLEEP_A::SLEEP
    }
}
impl core::ops::Deref for SLEEP_R {
    type Target = crate::FieldReader<bool, SLEEP_A>;
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
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEEP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Mode: The ADC Core and reference voltage circuitry are kept ON between conversions"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SLEEP_A::NORMAL)
    }
    #[doc = "Sleep Mode: The ADC Core and reference voltage circuitry are OFF between conversions"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut W {
        self.variant(SLEEP_A::SLEEP)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Free Run Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FREERUN_A {
    #[doc = "0: Normal Mode"]
    OFF = 0,
    #[doc = "1: Free Run Mode: Never wait for any trigger."]
    ON = 1,
}
impl From<FREERUN_A> for bool {
    #[inline(always)]
    fn from(variant: FREERUN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FREERUN` reader - Free Run Mode"]
pub struct FREERUN_R(crate::FieldReader<bool, FREERUN_A>);
impl FREERUN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREERUN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FREERUN_A {
        match self.bits {
            false => FREERUN_A::OFF,
            true => FREERUN_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == FREERUN_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == FREERUN_A::ON
    }
}
impl core::ops::Deref for FREERUN_R {
    type Target = crate::FieldReader<bool, FREERUN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREERUN` writer - Free Run Mode"]
pub struct FREERUN_W<'a> {
    w: &'a mut W,
}
impl<'a> FREERUN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREERUN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FREERUN_A::OFF)
    }
    #[doc = "Free Run Mode: Never wait for any trigger."]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(FREERUN_A::ON)
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
#[doc = "Field `PRESCAL` reader - Prescaler Rate Selection"]
pub struct PRESCAL_R(crate::FieldReader<u8, u8>);
impl PRESCAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRESCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESCAL` writer - Prescaler Rate Selection"]
pub struct PRESCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Start Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 0 periods of ADCClock"]
    SUT0 = 0,
    #[doc = "1: 8 periods of ADCClock"]
    SUT8 = 1,
    #[doc = "2: 16 periods of ADCClock"]
    SUT16 = 2,
    #[doc = "3: 24 periods of ADCClock"]
    SUT24 = 3,
    #[doc = "4: 64 periods of ADCClock"]
    SUT64 = 4,
    #[doc = "5: 80 periods of ADCClock"]
    SUT80 = 5,
    #[doc = "6: 96 periods of ADCClock"]
    SUT96 = 6,
    #[doc = "7: 112 periods of ADCClock"]
    SUT112 = 7,
    #[doc = "8: 512 periods of ADCClock"]
    SUT512 = 8,
    #[doc = "9: 576 periods of ADCClock"]
    SUT576 = 9,
    #[doc = "10: 640 periods of ADCClock"]
    SUT640 = 10,
    #[doc = "11: 704 periods of ADCClock"]
    SUT704 = 11,
    #[doc = "12: 768 periods of ADCClock"]
    SUT768 = 12,
    #[doc = "13: 832 periods of ADCClock"]
    SUT832 = 13,
    #[doc = "14: 896 periods of ADCClock"]
    SUT896 = 14,
    #[doc = "15: 960 periods of ADCClock"]
    SUT960 = 15,
}
impl From<STARTUP_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STARTUP` reader - Start Up Time"]
pub struct STARTUP_R(crate::FieldReader<u8, STARTUP_A>);
impl STARTUP_R {
    pub(crate) fn new(bits: u8) -> Self {
        STARTUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTUP_A {
        match self.bits {
            0 => STARTUP_A::SUT0,
            1 => STARTUP_A::SUT8,
            2 => STARTUP_A::SUT16,
            3 => STARTUP_A::SUT24,
            4 => STARTUP_A::SUT64,
            5 => STARTUP_A::SUT80,
            6 => STARTUP_A::SUT96,
            7 => STARTUP_A::SUT112,
            8 => STARTUP_A::SUT512,
            9 => STARTUP_A::SUT576,
            10 => STARTUP_A::SUT640,
            11 => STARTUP_A::SUT704,
            12 => STARTUP_A::SUT768,
            13 => STARTUP_A::SUT832,
            14 => STARTUP_A::SUT896,
            15 => STARTUP_A::SUT960,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SUT0`"]
    #[inline(always)]
    pub fn is_sut0(&self) -> bool {
        **self == STARTUP_A::SUT0
    }
    #[doc = "Checks if the value of the field is `SUT8`"]
    #[inline(always)]
    pub fn is_sut8(&self) -> bool {
        **self == STARTUP_A::SUT8
    }
    #[doc = "Checks if the value of the field is `SUT16`"]
    #[inline(always)]
    pub fn is_sut16(&self) -> bool {
        **self == STARTUP_A::SUT16
    }
    #[doc = "Checks if the value of the field is `SUT24`"]
    #[inline(always)]
    pub fn is_sut24(&self) -> bool {
        **self == STARTUP_A::SUT24
    }
    #[doc = "Checks if the value of the field is `SUT64`"]
    #[inline(always)]
    pub fn is_sut64(&self) -> bool {
        **self == STARTUP_A::SUT64
    }
    #[doc = "Checks if the value of the field is `SUT80`"]
    #[inline(always)]
    pub fn is_sut80(&self) -> bool {
        **self == STARTUP_A::SUT80
    }
    #[doc = "Checks if the value of the field is `SUT96`"]
    #[inline(always)]
    pub fn is_sut96(&self) -> bool {
        **self == STARTUP_A::SUT96
    }
    #[doc = "Checks if the value of the field is `SUT112`"]
    #[inline(always)]
    pub fn is_sut112(&self) -> bool {
        **self == STARTUP_A::SUT112
    }
    #[doc = "Checks if the value of the field is `SUT512`"]
    #[inline(always)]
    pub fn is_sut512(&self) -> bool {
        **self == STARTUP_A::SUT512
    }
    #[doc = "Checks if the value of the field is `SUT576`"]
    #[inline(always)]
    pub fn is_sut576(&self) -> bool {
        **self == STARTUP_A::SUT576
    }
    #[doc = "Checks if the value of the field is `SUT640`"]
    #[inline(always)]
    pub fn is_sut640(&self) -> bool {
        **self == STARTUP_A::SUT640
    }
    #[doc = "Checks if the value of the field is `SUT704`"]
    #[inline(always)]
    pub fn is_sut704(&self) -> bool {
        **self == STARTUP_A::SUT704
    }
    #[doc = "Checks if the value of the field is `SUT768`"]
    #[inline(always)]
    pub fn is_sut768(&self) -> bool {
        **self == STARTUP_A::SUT768
    }
    #[doc = "Checks if the value of the field is `SUT832`"]
    #[inline(always)]
    pub fn is_sut832(&self) -> bool {
        **self == STARTUP_A::SUT832
    }
    #[doc = "Checks if the value of the field is `SUT896`"]
    #[inline(always)]
    pub fn is_sut896(&self) -> bool {
        **self == STARTUP_A::SUT896
    }
    #[doc = "Checks if the value of the field is `SUT960`"]
    #[inline(always)]
    pub fn is_sut960(&self) -> bool {
        **self == STARTUP_A::SUT960
    }
}
impl core::ops::Deref for STARTUP_R {
    type Target = crate::FieldReader<u8, STARTUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTUP` writer - Start Up Time"]
pub struct STARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTUP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "0 periods of ADCClock"]
    #[inline(always)]
    pub fn sut0(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT0)
    }
    #[doc = "8 periods of ADCClock"]
    #[inline(always)]
    pub fn sut8(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT8)
    }
    #[doc = "16 periods of ADCClock"]
    #[inline(always)]
    pub fn sut16(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT16)
    }
    #[doc = "24 periods of ADCClock"]
    #[inline(always)]
    pub fn sut24(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT24)
    }
    #[doc = "64 periods of ADCClock"]
    #[inline(always)]
    pub fn sut64(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT64)
    }
    #[doc = "80 periods of ADCClock"]
    #[inline(always)]
    pub fn sut80(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT80)
    }
    #[doc = "96 periods of ADCClock"]
    #[inline(always)]
    pub fn sut96(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT96)
    }
    #[doc = "112 periods of ADCClock"]
    #[inline(always)]
    pub fn sut112(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT112)
    }
    #[doc = "512 periods of ADCClock"]
    #[inline(always)]
    pub fn sut512(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT512)
    }
    #[doc = "576 periods of ADCClock"]
    #[inline(always)]
    pub fn sut576(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT576)
    }
    #[doc = "640 periods of ADCClock"]
    #[inline(always)]
    pub fn sut640(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT640)
    }
    #[doc = "704 periods of ADCClock"]
    #[inline(always)]
    pub fn sut704(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT704)
    }
    #[doc = "768 periods of ADCClock"]
    #[inline(always)]
    pub fn sut768(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT768)
    }
    #[doc = "832 periods of ADCClock"]
    #[inline(always)]
    pub fn sut832(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT832)
    }
    #[doc = "896 periods of ADCClock"]
    #[inline(always)]
    pub fn sut896(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT896)
    }
    #[doc = "960 periods of ADCClock"]
    #[inline(always)]
    pub fn sut960(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT960)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `DIV1` reader - "]
pub struct DIV1_R(crate::FieldReader<bool, bool>);
impl DIV1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIV1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV1` writer - "]
pub struct DIV1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `TRACKTIM` reader - Tracking Time"]
pub struct TRACKTIM_R(crate::FieldReader<u8, u8>);
impl TRACKTIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRACKTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRACKTIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRACKTIM` writer - Tracking Time"]
pub struct TRACKTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRACKTIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `TRANSFER` reader - Transfer Period"]
pub struct TRANSFER_R(crate::FieldReader<u8, u8>);
impl TRANSFER_R {
    pub(crate) fn new(bits: u8) -> Self {
        TRANSFER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRANSFER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRANSFER` writer - Transfer Period"]
pub struct TRANSFER_W<'a> {
    w: &'a mut W,
}
impl<'a> TRANSFER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "Use Sequence Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USEQ_A {
    #[doc = "0: Normal Mode: The controller converts channels in a simple numeric order depending only on the channel index."]
    NUM_ORDER = 0,
    #[doc = "1: User Sequence Mode: The sequence respects what is defined in ADC_SEQR1 and ADC_SEQR2 registers and can be used to convert several times the same channel."]
    REG_ORDER = 1,
}
impl From<USEQ_A> for bool {
    #[inline(always)]
    fn from(variant: USEQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USEQ` reader - Use Sequence Enable"]
pub struct USEQ_R(crate::FieldReader<bool, USEQ_A>);
impl USEQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        USEQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USEQ_A {
        match self.bits {
            false => USEQ_A::NUM_ORDER,
            true => USEQ_A::REG_ORDER,
        }
    }
    #[doc = "Checks if the value of the field is `NUM_ORDER`"]
    #[inline(always)]
    pub fn is_num_order(&self) -> bool {
        **self == USEQ_A::NUM_ORDER
    }
    #[doc = "Checks if the value of the field is `REG_ORDER`"]
    #[inline(always)]
    pub fn is_reg_order(&self) -> bool {
        **self == USEQ_A::REG_ORDER
    }
}
impl core::ops::Deref for USEQ_R {
    type Target = crate::FieldReader<bool, USEQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USEQ` writer - Use Sequence Enable"]
pub struct USEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> USEQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USEQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal Mode: The controller converts channels in a simple numeric order depending only on the channel index."]
    #[inline(always)]
    pub fn num_order(self) -> &'a mut W {
        self.variant(USEQ_A::NUM_ORDER)
    }
    #[doc = "User Sequence Mode: The sequence respects what is defined in ADC_SEQR1 and ADC_SEQR2 registers and can be used to convert several times the same channel."]
    #[inline(always)]
    pub fn reg_order(self) -> &'a mut W {
        self.variant(USEQ_A::REG_ORDER)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
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
    #[doc = "Bit 4 - Resolution"]
    #[inline(always)]
    pub fn lowres(&self) -> LOWRES_R {
        LOWRES_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FREERUN_R {
        FREERUN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&self) -> PRESCAL_R {
        PRESCAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Start Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn div1(&self) -> DIV1_R {
        DIV1_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    pub fn tracktim(&self) -> TRACKTIM_R {
        TRACKTIM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline(always)]
    pub fn transfer(&self) -> TRANSFER_R {
        TRANSFER_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Use Sequence Enable"]
    #[inline(always)]
    pub fn useq(&self) -> USEQ_R {
        USEQ_R::new(((self.bits >> 31) & 0x01) != 0)
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
    #[doc = "Bit 4 - Resolution"]
    #[inline(always)]
    pub fn lowres(&mut self) -> LOWRES_W {
        LOWRES_W { w: self }
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W {
        SLEEP_W { w: self }
    }
    #[doc = "Bit 7 - Free Run Mode"]
    #[inline(always)]
    pub fn freerun(&mut self) -> FREERUN_W {
        FREERUN_W { w: self }
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&mut self) -> PRESCAL_W {
        PRESCAL_W { w: self }
    }
    #[doc = "Bits 16:19 - Start Up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn div1(&mut self) -> DIV1_W {
        DIV1_W { w: self }
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    pub fn tracktim(&mut self) -> TRACKTIM_W {
        TRACKTIM_W { w: self }
    }
    #[doc = "Bits 28:29 - Transfer Period"]
    #[inline(always)]
    pub fn transfer(&mut self) -> TRANSFER_W {
        TRANSFER_W { w: self }
    }
    #[doc = "Bit 31 - Use Sequence Enable"]
    #[inline(always)]
    pub fn useq(&mut self) -> USEQ_W {
        USEQ_W { w: self }
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
