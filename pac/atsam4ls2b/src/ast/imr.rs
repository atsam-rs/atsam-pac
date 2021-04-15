#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVF_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<OVF_A> for bool {
    #[inline(always)]
    fn from(variant: OVF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OVF`"]
pub type OVF_R = crate::R<bool, OVF_A>;
impl OVF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVF_A {
        match self.bits {
            false => OVF_A::_0,
            true => OVF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVF_A::_1
    }
}
#[doc = "Alarm 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM0_A {
    #[doc = "0: Interupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<ALARM0_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALARM0`"]
pub type ALARM0_R = crate::R<bool, ALARM0_A>;
impl ALARM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM0_A {
        match self.bits {
            false => ALARM0_A::_0,
            true => ALARM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALARM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALARM0_A::_1
    }
}
#[doc = "Alarm 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM1_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<ALARM1_A> for bool {
    #[inline(always)]
    fn from(variant: ALARM1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ALARM1`"]
pub type ALARM1_R = crate::R<bool, ALARM1_A>;
impl ALARM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALARM1_A {
        match self.bits {
            false => ALARM1_A::_0,
            true => ALARM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALARM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALARM1_A::_1
    }
}
#[doc = "Periodic 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER0_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<PER0_A> for bool {
    #[inline(always)]
    fn from(variant: PER0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PER0`"]
pub type PER0_R = crate::R<bool, PER0_A>;
impl PER0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER0_A {
        match self.bits {
            false => PER0_A::_0,
            true => PER0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PER0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PER0_A::_1
    }
}
#[doc = "Periodic 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER1_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<PER1_A> for bool {
    #[inline(always)]
    fn from(variant: PER1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PER1`"]
pub type PER1_R = crate::R<bool, PER1_A>;
impl PER1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER1_A {
        match self.bits {
            false => PER1_A::_0,
            true => PER1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PER1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PER1_A::_1
    }
}
#[doc = "AST Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, READY_A>;
impl READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::_0,
            true => READY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == READY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == READY_A::_1
    }
}
#[doc = "Clock Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKRDY_A {
    #[doc = "0: Interrupt is disabled"]
    _0 = 0,
    #[doc = "1: Interrupt is enabled"]
    _1 = 1,
}
impl From<CLKRDY_A> for bool {
    #[inline(always)]
    fn from(variant: CLKRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CLKRDY`"]
pub type CLKRDY_R = crate::R<bool, CLKRDY_A>;
impl CLKRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKRDY_A {
        match self.bits {
            false => CLKRDY_A::_0,
            true => CLKRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKRDY_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Periodic 0"]
    #[inline(always)]
    pub fn per0(&self) -> PER0_R {
        PER0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Periodic 1"]
    #[inline(always)]
    pub fn per1(&self) -> PER1_R {
        PER1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 25 - AST Ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Clock Ready"]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
