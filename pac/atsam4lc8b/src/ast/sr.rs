#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVF` reader - Overflow"]
pub struct OVF_R(crate::FieldReader<bool, bool>);
impl OVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARM0` reader - Alarm 0"]
pub struct ALARM0_R(crate::FieldReader<bool, bool>);
impl ALARM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALARM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALARM1` reader - Alarm 1"]
pub struct ALARM1_R(crate::FieldReader<bool, bool>);
impl ALARM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALARM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALARM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER0` reader - Periodic 0"]
pub struct PER0_R(crate::FieldReader<bool, bool>);
impl PER0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PER1` reader - Periodic 1"]
pub struct PER1_R(crate::FieldReader<bool, bool>);
impl PER1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PER1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "AST Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BUSY_A {
    #[doc = "0: The AST accepts writes to CV, WER, DTR, SCR, AR, PIR and CR"]
    _0 = 0,
    #[doc = "1: The AST is busy and will discard writes to CV, WER, DTR, SCR, AR, PIR and CR"]
    _1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - AST Busy"]
pub struct BUSY_R(crate::FieldReader<bool, BUSY_A>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::_0,
            true => BUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == BUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == BUSY_A::_1
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, BUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READY` reader - AST Ready"]
pub struct READY_R(crate::FieldReader<bool, bool>);
impl READY_R {
    pub(crate) fn new(bits: bool) -> Self {
        READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Clock Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKBUSY_A {
    #[doc = "0: The clock is ready and can be changed"]
    _0 = 0,
    #[doc = "1: CEN has been written and the clock is busy"]
    _1 = 1,
}
impl From<CLKBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: CLKBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKBUSY` reader - Clock Busy"]
pub struct CLKBUSY_R(crate::FieldReader<bool, CLKBUSY_A>);
impl CLKBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKBUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKBUSY_A {
        match self.bits {
            false => CLKBUSY_A::_0,
            true => CLKBUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        **self == CLKBUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        **self == CLKBUSY_A::_1
    }
}
impl core::ops::Deref for CLKBUSY_R {
    type Target = crate::FieldReader<bool, CLKBUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKRDY` reader - Clock Ready"]
pub struct CLKRDY_R(crate::FieldReader<bool, bool>);
impl CLKRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
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
    #[doc = "Bit 24 - AST Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - AST Ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Clock Busy"]
    #[inline(always)]
    pub fn clkbusy(&self) -> CLKBUSY_R {
        CLKBUSY_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Clock Ready"]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
