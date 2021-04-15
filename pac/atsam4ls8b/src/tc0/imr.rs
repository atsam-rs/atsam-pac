#[doc = "Reader of register IMR%s"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Counter Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COVFS_A {
    #[doc = "0: The Counter Overflow Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The Counter Overflow Interrupt is enabled."]
    _1 = 1,
}
impl From<COVFS_A> for bool {
    #[inline(always)]
    fn from(variant: COVFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COVFS`"]
pub type COVFS_R = crate::R<bool, COVFS_A>;
impl COVFS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COVFS_A {
        match self.bits {
            false => COVFS_A::_0,
            true => COVFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COVFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COVFS_A::_1
    }
}
#[doc = "Load Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOVRS_A {
    #[doc = "0: The Load Overrun Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The Load Overrun Interrupt is enabled."]
    _1 = 1,
}
impl From<LOVRS_A> for bool {
    #[inline(always)]
    fn from(variant: LOVRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOVRS`"]
pub type LOVRS_R = crate::R<bool, LOVRS_A>;
impl LOVRS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOVRS_A {
        match self.bits {
            false => LOVRS_A::_0,
            true => LOVRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOVRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOVRS_A::_1
    }
}
#[doc = "RA Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPAS_A {
    #[doc = "0: The RA Compare Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The RA Compare Interrupt is enabled."]
    _1 = 1,
}
impl From<CPAS_A> for bool {
    #[inline(always)]
    fn from(variant: CPAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPAS`"]
pub type CPAS_R = crate::R<bool, CPAS_A>;
impl CPAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPAS_A {
        match self.bits {
            false => CPAS_A::_0,
            true => CPAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPAS_A::_1
    }
}
#[doc = "RB Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPBS_A {
    #[doc = "0: The RB Compare Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The RB Compare Interrupt is enabled."]
    _1 = 1,
}
impl From<CPBS_A> for bool {
    #[inline(always)]
    fn from(variant: CPBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPBS`"]
pub type CPBS_R = crate::R<bool, CPBS_A>;
impl CPBS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPBS_A {
        match self.bits {
            false => CPBS_A::_0,
            true => CPBS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPBS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPBS_A::_1
    }
}
#[doc = "RC Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPCS_A {
    #[doc = "0: The RC Compare Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The RC Compare Interrupt is enabled."]
    _1 = 1,
}
impl From<CPCS_A> for bool {
    #[inline(always)]
    fn from(variant: CPCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CPCS`"]
pub type CPCS_R = crate::R<bool, CPCS_A>;
impl CPCS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPCS_A {
        match self.bits {
            false => CPCS_A::_0,
            true => CPCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPCS_A::_1
    }
}
#[doc = "RA Loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRAS_A {
    #[doc = "0: The Load RA Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The Load RA Interrupt is enabled."]
    _1 = 1,
}
impl From<LDRAS_A> for bool {
    #[inline(always)]
    fn from(variant: LDRAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LDRAS`"]
pub type LDRAS_R = crate::R<bool, LDRAS_A>;
impl LDRAS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRAS_A {
        match self.bits {
            false => LDRAS_A::_0,
            true => LDRAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDRAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDRAS_A::_1
    }
}
#[doc = "RB Loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRBS_A {
    #[doc = "0: The Load RB Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The Load RB Interrupt is enabled."]
    _1 = 1,
}
impl From<LDRBS_A> for bool {
    #[inline(always)]
    fn from(variant: LDRBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LDRBS`"]
pub type LDRBS_R = crate::R<bool, LDRBS_A>;
impl LDRBS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDRBS_A {
        match self.bits {
            false => LDRBS_A::_0,
            true => LDRBS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LDRBS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LDRBS_A::_1
    }
}
#[doc = "External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETRGS_A {
    #[doc = "0: The External Trigger Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The External Trigger Interrupt is enabled."]
    _1 = 1,
}
impl From<ETRGS_A> for bool {
    #[inline(always)]
    fn from(variant: ETRGS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ETRGS`"]
pub type ETRGS_R = crate::R<bool, ETRGS_A>;
impl ETRGS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETRGS_A {
        match self.bits {
            false => ETRGS_A::_0,
            true => ETRGS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ETRGS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ETRGS_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    pub fn covfs(&self) -> COVFS_R {
        COVFS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    pub fn lovrs(&self) -> LOVRS_R {
        LOVRS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    pub fn cpas(&self) -> CPAS_R {
        CPAS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    pub fn cpbs(&self) -> CPBS_R {
        CPBS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    pub fn cpcs(&self) -> CPCS_R {
        CPCS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    pub fn ldras(&self) -> LDRAS_R {
        LDRAS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LDRBS_R {
        LDRBS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn etrgs(&self) -> ETRGS_R {
        ETRGS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
