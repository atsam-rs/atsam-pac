#[doc = "Register `PARAMETER` reader"]
pub struct R(crate::R<PARAMETER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAMETER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAMETER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAMETER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DT` reader - Digital Tuner"]
pub type DT_R = crate::BitReader<DTSELECT_A>;
#[doc = "Digital Tuner\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTSELECT_A {
    #[doc = "0: Digital tuner off"]
    OFF = 0,
    #[doc = "1: Digital tuner on"]
    ON = 1,
}
impl From<DTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTSELECT_A {
        match self.bits {
            false => DTSELECT_A::OFF,
            true => DTSELECT_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DTSELECT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == DTSELECT_A::ON
    }
}
#[doc = "Field `DTEXPWA` reader - Digital Tuner Exponent Writeable"]
pub type DTEXPWA_R = crate::BitReader<DTEXPWASELECT_A>;
#[doc = "Digital Tuner Exponent Writeable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTEXPWASELECT_A {
    #[doc = "0: Digital tuner exponent is a constant value. Writes to EXP bitfield in DTR will be discarded."]
    _0 = 0,
    #[doc = "1: Digital tuner exponent is chosen by writing to EXP bitfield in DTR"]
    _1 = 1,
}
impl From<DTEXPWASELECT_A> for bool {
    #[inline(always)]
    fn from(variant: DTEXPWASELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl DTEXPWA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTEXPWASELECT_A {
        match self.bits {
            false => DTEXPWASELECT_A::_0,
            true => DTEXPWASELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTEXPWASELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTEXPWASELECT_A::_1
    }
}
#[doc = "Field `DTEXPVALUE` reader - Digital Tuner Exponent Value"]
pub type DTEXPVALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMAR` reader - Number of alarm comparators"]
pub type NUMAR_R = crate::FieldReader<u8, NUMARSELECT_A>;
#[doc = "Number of alarm comparators\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NUMARSELECT_A {
    #[doc = "0: No alarm comparators"]
    ZERO = 0,
    #[doc = "1: One alarm comparator"]
    ONE = 1,
    #[doc = "2: Two alarm comparators"]
    TWO = 2,
}
impl From<NUMARSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: NUMARSELECT_A) -> Self {
        variant as _
    }
}
impl NUMAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NUMARSELECT_A> {
        match self.bits {
            0 => Some(NUMARSELECT_A::ZERO),
            1 => Some(NUMARSELECT_A::ONE),
            2 => Some(NUMARSELECT_A::TWO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == NUMARSELECT_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == NUMARSELECT_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == NUMARSELECT_A::TWO
    }
}
#[doc = "Field `NUMPIR` reader - Number of periodic comparators"]
pub type NUMPIR_R = crate::BitReader<NUMPIRSELECT_A>;
#[doc = "Number of periodic comparators\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NUMPIRSELECT_A {
    #[doc = "0: One periodic comparator"]
    ONE = 0,
    #[doc = "1: Two periodic comparators"]
    TWO = 1,
}
impl From<NUMPIRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: NUMPIRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl NUMPIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NUMPIRSELECT_A {
        match self.bits {
            false => NUMPIRSELECT_A::ONE,
            true => NUMPIRSELECT_A::TWO,
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == NUMPIRSELECT_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == NUMPIRSELECT_A::TWO
    }
}
#[doc = "Field `PIR0WA` reader - Periodic Interval 0 Writeable"]
pub type PIR0WA_R = crate::BitReader<PIR0WASELECT_A>;
#[doc = "Periodic Interval 0 Writeable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIR0WASELECT_A {
    #[doc = "0: Periodic alarm prescaler 0 tapping is a constant value. Writes to INSEL bitfield in PIR0 will be discarded."]
    _0 = 0,
    #[doc = "1: Periodic alarm prescaler 0 tapping is chosen by writing to INSEL bitfield in PIR0"]
    _1 = 1,
}
impl From<PIR0WASELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PIR0WASELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PIR0WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIR0WASELECT_A {
        match self.bits {
            false => PIR0WASELECT_A::_0,
            true => PIR0WASELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIR0WASELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIR0WASELECT_A::_1
    }
}
#[doc = "Field `PIR1WA` reader - Periodic Interval 1 Writeable"]
pub type PIR1WA_R = crate::BitReader<PIR1WASELECT_A>;
#[doc = "Periodic Interval 1 Writeable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIR1WASELECT_A {
    #[doc = "0: Writes to PIR1 will be discarded"]
    _0 = 0,
    #[doc = "1: PIR1 can be written"]
    _1 = 1,
}
impl From<PIR1WASELECT_A> for bool {
    #[inline(always)]
    fn from(variant: PIR1WASELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl PIR1WA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIR1WASELECT_A {
        match self.bits {
            false => PIR1WASELECT_A::_0,
            true => PIR1WASELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIR1WASELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIR1WASELECT_A::_1
    }
}
#[doc = "Field `PER0VALUE` reader - Periodic Interval 0 Value"]
pub type PER0VALUE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PER1VALUE` reader - Periodic Interval 1 Value"]
pub type PER1VALUE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Digital Tuner"]
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Digital Tuner Exponent Writeable"]
    #[inline(always)]
    pub fn dtexpwa(&self) -> DTEXPWA_R {
        DTEXPWA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Digital Tuner Exponent Value"]
    #[inline(always)]
    pub fn dtexpvalue(&self) -> DTEXPVALUE_R {
        DTEXPVALUE_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Number of alarm comparators"]
    #[inline(always)]
    pub fn numar(&self) -> NUMAR_R {
        NUMAR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Number of periodic comparators"]
    #[inline(always)]
    pub fn numpir(&self) -> NUMPIR_R {
        NUMPIR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Periodic Interval 0 Writeable"]
    #[inline(always)]
    pub fn pir0wa(&self) -> PIR0WA_R {
        PIR0WA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Periodic Interval 1 Writeable"]
    #[inline(always)]
    pub fn pir1wa(&self) -> PIR1WA_R {
        PIR1WA_R::new(((self.bits >> 15) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
