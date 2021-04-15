#[doc = "Register `IMR%s` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IMR_SPEC>> for R {
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
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
#[doc = "Field `COVFS` reader - Counter Overflow"]
pub struct COVFS_R(crate::FieldReader<bool, COVFS_A>);
impl COVFS_R {
    pub(crate) fn new(bits: bool) -> Self {
        COVFS_R(crate::FieldReader::new(bits))
    }
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
        **self == COVFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == COVFS_A::_1
    }
}
impl core::ops::Deref for COVFS_R {
    type Target = crate::FieldReader<bool, COVFS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `LOVRS` reader - Load Overrun"]
pub struct LOVRS_R(crate::FieldReader<bool, LOVRS_A>);
impl LOVRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOVRS_R(crate::FieldReader::new(bits))
    }
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
        **self == LOVRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LOVRS_A::_1
    }
}
impl core::ops::Deref for LOVRS_R {
    type Target = crate::FieldReader<bool, LOVRS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CPAS` reader - RA Compare"]
pub struct CPAS_R(crate::FieldReader<bool, CPAS_A>);
impl CPAS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPAS_R(crate::FieldReader::new(bits))
    }
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
        **self == CPAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPAS_A::_1
    }
}
impl core::ops::Deref for CPAS_R {
    type Target = crate::FieldReader<bool, CPAS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CPBS` reader - RB Compare"]
pub struct CPBS_R(crate::FieldReader<bool, CPBS_A>);
impl CPBS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPBS_R(crate::FieldReader::new(bits))
    }
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
        **self == CPBS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPBS_A::_1
    }
}
impl core::ops::Deref for CPBS_R {
    type Target = crate::FieldReader<bool, CPBS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `CPCS` reader - RC Compare"]
pub struct CPCS_R(crate::FieldReader<bool, CPCS_A>);
impl CPCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPCS_R(crate::FieldReader::new(bits))
    }
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
        **self == CPCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CPCS_A::_1
    }
}
impl core::ops::Deref for CPCS_R {
    type Target = crate::FieldReader<bool, CPCS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `LDRAS` reader - RA Loading"]
pub struct LDRAS_R(crate::FieldReader<bool, LDRAS_A>);
impl LDRAS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDRAS_R(crate::FieldReader::new(bits))
    }
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
        **self == LDRAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LDRAS_A::_1
    }
}
impl core::ops::Deref for LDRAS_R {
    type Target = crate::FieldReader<bool, LDRAS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `LDRBS` reader - RB Loading"]
pub struct LDRBS_R(crate::FieldReader<bool, LDRBS_A>);
impl LDRBS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LDRBS_R(crate::FieldReader::new(bits))
    }
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
        **self == LDRBS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == LDRBS_A::_1
    }
}
impl core::ops::Deref for LDRBS_R {
    type Target = crate::FieldReader<bool, LDRBS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Field `ETRGS` reader - External Trigger"]
pub struct ETRGS_R(crate::FieldReader<bool, ETRGS_A>);
impl ETRGS_R {
    pub(crate) fn new(bits: bool) -> Self {
        ETRGS_R(crate::FieldReader::new(bits))
    }
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
        **self == ETRGS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == ETRGS_A::_1
    }
}
impl core::ops::Deref for ETRGS_R {
    type Target = crate::FieldReader<bool, ETRGS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
#[doc = "Interrupt Mask Register Channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR%s to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
