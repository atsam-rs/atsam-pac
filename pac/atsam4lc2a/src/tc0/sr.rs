#[doc = "Register `SR%s` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<SR_SPEC>> for R {
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Counter Overflow Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COVFS_A {
    #[doc = "0: No counter overflow has occurred since the last read of the Status Register."]
    _0 = 0,
    #[doc = "1: A counter overflow has occurred since the last read of the Status Register."]
    _1 = 1,
}
impl From<COVFS_A> for bool {
    #[inline(always)]
    fn from(variant: COVFS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COVFS` reader - Counter Overflow Status"]
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
#[doc = "Load Overrun Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOVRS_A {
    #[doc = "0: Load overrun has not occurred since the last read of the Status Register or WAVE:1."]
    _0 = 0,
    #[doc = "1: RA or RB have been loaded at least twice without any read of the corresponding register since the last read of the StatusRegister, if WAVE:0."]
    _1 = 1,
}
impl From<LOVRS_A> for bool {
    #[inline(always)]
    fn from(variant: LOVRS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOVRS` reader - Load Overrun Status"]
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
#[doc = "RA Compare Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPAS_A {
    #[doc = "0: RA Compare has not occurred since the last read of the Status Register or WAVE:0."]
    _0 = 0,
    #[doc = "1: RA Compare has occurred since the last read of the Status Register, if WAVE:1."]
    _1 = 1,
}
impl From<CPAS_A> for bool {
    #[inline(always)]
    fn from(variant: CPAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPAS` reader - RA Compare Status"]
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
#[doc = "RB Compare Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPBS_A {
    #[doc = "0: RB Compare has not occurred since the last read of the Status Register or WAVE:0."]
    _0 = 0,
    #[doc = "1: RB Compare has occurred since the last read of the Status Register, if WAVE:1."]
    _1 = 1,
}
impl From<CPBS_A> for bool {
    #[inline(always)]
    fn from(variant: CPBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPBS` reader - RB Compare Status"]
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
#[doc = "RC Compare Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPCS_A {
    #[doc = "0: RC Compare has not occurred since the last read of the Status Register."]
    _0 = 0,
    #[doc = "1: RC Compare has occurred since the last read of the Status Register."]
    _1 = 1,
}
impl From<CPCS_A> for bool {
    #[inline(always)]
    fn from(variant: CPCS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPCS` reader - RC Compare Status"]
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
#[doc = "RA Loading Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRAS_A {
    #[doc = "0: RA Load has not occurred since the last read of the Status Register or WAVE:1."]
    _0 = 0,
    #[doc = "1: RA Load has occurred since the last read of the Status Register, if WAVE:0."]
    _1 = 1,
}
impl From<LDRAS_A> for bool {
    #[inline(always)]
    fn from(variant: LDRAS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDRAS` reader - RA Loading Status"]
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
#[doc = "RB Loading Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRBS_A {
    #[doc = "0: RB Load has not occurred since the last read of the Status Register or WAVE:1."]
    _0 = 0,
    #[doc = "1: RB Load has occurred since the last read of the Status Register, if WAVE:0."]
    _1 = 1,
}
impl From<LDRBS_A> for bool {
    #[inline(always)]
    fn from(variant: LDRBS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDRBS` reader - RB Loading Status"]
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
#[doc = "External Trigger Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETRGS_A {
    #[doc = "0: External trigger has not occurred since the last read of the Status Register."]
    _0 = 0,
    #[doc = "1: External trigger has occurred since the last read of the Status Register."]
    _1 = 1,
}
impl From<ETRGS_A> for bool {
    #[inline(always)]
    fn from(variant: ETRGS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETRGS` reader - External Trigger Status"]
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
#[doc = "Clock Enabling Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKSTA_A {
    #[doc = "0: Clock is disabled."]
    _0 = 0,
    #[doc = "1: Clock is enabled."]
    _1 = 1,
}
impl From<CLKSTA_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSTA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSTA` reader - Clock Enabling Status"]
pub struct CLKSTA_R(crate::FieldReader<bool, CLKSTA_A>);
impl CLKSTA_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKSTA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSTA_A {
        match self.bits {
            false => CLKSTA_A::_0,
            true => CLKSTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKSTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKSTA_A::_1
    }
}
impl core::ops::Deref for CLKSTA_R {
    type Target = crate::FieldReader<bool, CLKSTA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "TIOA Mirror\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTIOA_A {
    #[doc = "0: TIOA is low. If WAVE:0, this means that TIOA pin is low. If WAVE:1, this means that TIOA is driven low."]
    _0 = 0,
    #[doc = "1: TIOA is high. If WAVE:0, this means that TIOA pin is high. If WAVE:1, this means that TIOA is driven high."]
    _1 = 1,
}
impl From<MTIOA_A> for bool {
    #[inline(always)]
    fn from(variant: MTIOA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTIOA` reader - TIOA Mirror"]
pub struct MTIOA_R(crate::FieldReader<bool, MTIOA_A>);
impl MTIOA_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTIOA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTIOA_A {
        match self.bits {
            false => MTIOA_A::_0,
            true => MTIOA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MTIOA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MTIOA_A::_1
    }
}
impl core::ops::Deref for MTIOA_R {
    type Target = crate::FieldReader<bool, MTIOA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "TIOB Mirror\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MTIOB_A {
    #[doc = "0: TIOB is low. If WAVE:0, this means that TIOB pin is low. If WAVE:1, this means that TIOB is driven low."]
    _0 = 0,
    #[doc = "1: TIOB is high. If WAVE:0, this means that TIOB pin is high. If WAVE:1, this means that TIOB is driven high."]
    _1 = 1,
}
impl From<MTIOB_A> for bool {
    #[inline(always)]
    fn from(variant: MTIOB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MTIOB` reader - TIOB Mirror"]
pub struct MTIOB_R(crate::FieldReader<bool, MTIOB_A>);
impl MTIOB_R {
    pub(crate) fn new(bits: bool) -> Self {
        MTIOB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MTIOB_A {
        match self.bits {
            false => MTIOB_A::_0,
            true => MTIOB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == MTIOB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == MTIOB_A::_1
    }
}
impl core::ops::Deref for MTIOB_R {
    type Target = crate::FieldReader<bool, MTIOB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Counter Overflow Status"]
    #[inline(always)]
    pub fn covfs(&self) -> COVFS_R {
        COVFS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Load Overrun Status"]
    #[inline(always)]
    pub fn lovrs(&self) -> LOVRS_R {
        LOVRS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RA Compare Status"]
    #[inline(always)]
    pub fn cpas(&self) -> CPAS_R {
        CPAS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RB Compare Status"]
    #[inline(always)]
    pub fn cpbs(&self) -> CPBS_R {
        CPBS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RC Compare Status"]
    #[inline(always)]
    pub fn cpcs(&self) -> CPCS_R {
        CPCS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RA Loading Status"]
    #[inline(always)]
    pub fn ldras(&self) -> LDRAS_R {
        LDRAS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RB Loading Status"]
    #[inline(always)]
    pub fn ldrbs(&self) -> LDRBS_R {
        LDRBS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - External Trigger Status"]
    #[inline(always)]
    pub fn etrgs(&self) -> ETRGS_R {
        ETRGS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Clock Enabling Status"]
    #[inline(always)]
    pub fn clksta(&self) -> CLKSTA_R {
        CLKSTA_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - TIOA Mirror"]
    #[inline(always)]
    pub fn mtioa(&self) -> MTIOA_R {
        MTIOA_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - TIOB Mirror"]
    #[inline(always)]
    pub fn mtiob(&self) -> MTIOB_R {
        MTIOB_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
#[doc = "Status Register Channel\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR%s to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
