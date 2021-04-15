#[doc = "Register `PARAMETER` reader"]
pub struct R(crate::R<PARAMETER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAMETER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PARAMETER_SPEC>> for R {
    fn from(reader: crate::R<PARAMETER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Digital Tuner\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DT_A {
    #[doc = "0: Digital tuner off"]
    OFF = 0,
    #[doc = "1: Digital tuner on"]
    ON = 1,
}
impl From<DT_A> for bool {
    #[inline(always)]
    fn from(variant: DT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DT` reader - Digital Tuner"]
pub struct DT_R(crate::FieldReader<bool, DT_A>);
impl DT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DT_A {
        match self.bits {
            false => DT_A::OFF,
            true => DT_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == DT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        **self == DT_A::ON
    }
}
impl core::ops::Deref for DT_R {
    type Target = crate::FieldReader<bool, DT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Digital Tuner Exponent Writeable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DTEXPWA_A {
    #[doc = "0: Digital tuner exponent is a constant value. Writes to EXP bitfield in DTR will be discarded."]
    _0 = 0,
    #[doc = "1: Digital tuner exponent is chosen by writing to EXP bitfield in DTR"]
    _1 = 1,
}
impl From<DTEXPWA_A> for bool {
    #[inline(always)]
    fn from(variant: DTEXPWA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTEXPWA` reader - Digital Tuner Exponent Writeable"]
pub struct DTEXPWA_R(crate::FieldReader<bool, DTEXPWA_A>);
impl DTEXPWA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEXPWA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEXPWA_A {
        match self.bits {
            false => DTEXPWA_A::_0,
            true => DTEXPWA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == DTEXPWA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == DTEXPWA_A::_1
    }
}
impl core::ops::Deref for DTEXPWA_R {
    type Target = crate::FieldReader<bool, DTEXPWA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEXPVALUE` reader - Digital Tuner Exponent Value"]
pub struct DTEXPVALUE_R(crate::FieldReader<u8, u8>);
impl DTEXPVALUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTEXPVALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEXPVALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Number of alarm comparators\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NUMAR_A {
    #[doc = "0: No alarm comparators"]
    ZERO = 0,
    #[doc = "1: One alarm comparator"]
    ONE = 1,
    #[doc = "2: Two alarm comparators"]
    TWO = 2,
}
impl From<NUMAR_A> for u8 {
    #[inline(always)]
    fn from(variant: NUMAR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NUMAR` reader - Number of alarm comparators"]
pub struct NUMAR_R(crate::FieldReader<u8, NUMAR_A>);
impl NUMAR_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUMAR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NUMAR_A> {
        match self.bits {
            0 => Some(NUMAR_A::ZERO),
            1 => Some(NUMAR_A::ONE),
            2 => Some(NUMAR_A::TWO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        **self == NUMAR_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == NUMAR_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        **self == NUMAR_A::TWO
    }
}
impl core::ops::Deref for NUMAR_R {
    type Target = crate::FieldReader<u8, NUMAR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Number of periodic comparators\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NUMPIR_A {
    #[doc = "0: One periodic comparator"]
    ONE = 0,
    #[doc = "1: Two periodic comparators"]
    TWO = 1,
}
impl From<NUMPIR_A> for bool {
    #[inline(always)]
    fn from(variant: NUMPIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NUMPIR` reader - Number of periodic comparators"]
pub struct NUMPIR_R(crate::FieldReader<bool, NUMPIR_A>);
impl NUMPIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NUMPIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUMPIR_A {
        match self.bits {
            false => NUMPIR_A::ONE,
            true => NUMPIR_A::TWO,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == NUMPIR_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        **self == NUMPIR_A::TWO
    }
}
impl core::ops::Deref for NUMPIR_R {
    type Target = crate::FieldReader<bool, NUMPIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Periodic Interval 0 Writeable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIR0WA_A {
    #[doc = "0: Periodic alarm prescaler 0 tapping is a constant value. Writes to INSEL bitfield in PIR0 will be discarded."]
    _0 = 0,
    #[doc = "1: Periodic alarm prescaler 0 tapping is chosen by writing to INSEL bitfield in PIR0"]
    _1 = 1,
}
impl From<PIR0WA_A> for bool {
    #[inline(always)]
    fn from(variant: PIR0WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIR0WA` reader - Periodic Interval 0 Writeable"]
pub struct PIR0WA_R(crate::FieldReader<bool, PIR0WA_A>);
impl PIR0WA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIR0WA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIR0WA_A {
        match self.bits {
            false => PIR0WA_A::_0,
            true => PIR0WA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PIR0WA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PIR0WA_A::_1
    }
}
impl core::ops::Deref for PIR0WA_R {
    type Target = crate::FieldReader<bool, PIR0WA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Periodic Interval 1 Writeable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIR1WA_A {
    #[doc = "0: Writes to PIR1 will be discarded"]
    _0 = 0,
    #[doc = "1: PIR1 can be written"]
    _1 = 1,
}
impl From<PIR1WA_A> for bool {
    #[inline(always)]
    fn from(variant: PIR1WA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIR1WA` reader - Periodic Interval 1 Writeable"]
pub struct PIR1WA_R(crate::FieldReader<bool, PIR1WA_A>);
impl PIR1WA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIR1WA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIR1WA_A {
        match self.bits {
            false => PIR1WA_A::_0,
            true => PIR1WA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == PIR1WA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == PIR1WA_A::_1
    }
}
impl core::ops::Deref for PIR1WA_R {
    type Target = crate::FieldReader<bool, PIR1WA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER0VALUE` reader - Periodic Interval 0 Value"]
pub struct PER0VALUE_R(crate::FieldReader<u8, u8>);
impl PER0VALUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PER0VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER0VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER1VALUE` reader - Periodic Interval 1 Value"]
pub struct PER1VALUE_R(crate::FieldReader<u8, u8>);
impl PER1VALUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PER1VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER1VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Digital Tuner"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Digital Tuner Exponent Writeable"]
    #[inline(always)]
    pub fn dtexpwa(&self) -> DTEXPWA_R {
        DTEXPWA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:6 - Digital Tuner Exponent Value"]
    #[inline(always)]
    pub fn dtexpvalue(&self) -> DTEXPVALUE_R {
        DTEXPVALUE_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Number of alarm comparators"]
    #[inline(always)]
    pub fn numar(&self) -> NUMAR_R {
        NUMAR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 12 - Number of periodic comparators"]
    #[inline(always)]
    pub fn numpir(&self) -> NUMPIR_R {
        NUMPIR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Periodic Interval 0 Writeable"]
    #[inline(always)]
    pub fn pir0wa(&self) -> PIR0WA_R {
        PIR0WA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Periodic Interval 1 Writeable"]
    #[inline(always)]
    pub fn pir1wa(&self) -> PIR1WA_R {
        PIR1WA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Periodic Interval 0 Value"]
    #[inline(always)]
    pub fn per0value(&self) -> PER0VALUE_R {
        PER0VALUE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Periodic Interval 1 Value"]
    #[inline(always)]
    pub fn per1value(&self) -> PER1VALUE_R {
        PER1VALUE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parameter](index.html) module"]
pub struct PARAMETER_SPEC;
impl crate::RegisterSpec for PARAMETER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [parameter::R](R) reader structure"]
impl crate::Readable for PARAMETER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAMETER to value 0"]
impl crate::Resettable for PARAMETER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
