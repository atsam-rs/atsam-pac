#[doc = "Reader of register PARAMETER"]
pub type R = crate::R<u32, super::PARAMETER>;
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
#[doc = "Reader of field `DT`"]
pub type DT_R = crate::R<bool, DT_A>;
impl DT_R {
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
        *self == DT_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == DT_A::ON
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
#[doc = "Reader of field `DTEXPWA`"]
pub type DTEXPWA_R = crate::R<bool, DTEXPWA_A>;
impl DTEXPWA_R {
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
        *self == DTEXPWA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTEXPWA_A::_1
    }
}
#[doc = "Reader of field `DTEXPVALUE`"]
pub type DTEXPVALUE_R = crate::R<u8, u8>;
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
#[doc = "Reader of field `NUMAR`"]
pub type NUMAR_R = crate::R<u8, NUMAR_A>;
impl NUMAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NUMAR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NUMAR_A::ZERO),
            1 => Val(NUMAR_A::ONE),
            2 => Val(NUMAR_A::TWO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == NUMAR_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == NUMAR_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == NUMAR_A::TWO
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
#[doc = "Reader of field `NUMPIR`"]
pub type NUMPIR_R = crate::R<bool, NUMPIR_A>;
impl NUMPIR_R {
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
        *self == NUMPIR_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == NUMPIR_A::TWO
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
#[doc = "Reader of field `PIR0WA`"]
pub type PIR0WA_R = crate::R<bool, PIR0WA_A>;
impl PIR0WA_R {
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
        *self == PIR0WA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIR0WA_A::_1
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
#[doc = "Reader of field `PIR1WA`"]
pub type PIR1WA_R = crate::R<bool, PIR1WA_A>;
impl PIR1WA_R {
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
        *self == PIR1WA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIR1WA_A::_1
    }
}
#[doc = "Reader of field `PER0VALUE`"]
pub type PER0VALUE_R = crate::R<u8, u8>;
#[doc = "Reader of field `PER1VALUE`"]
pub type PER1VALUE_R = crate::R<u8, u8>;
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
