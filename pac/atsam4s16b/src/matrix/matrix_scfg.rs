#[doc = "Register `MATRIX_SCFG[%s]` reader"]
pub struct R(crate::R<MATRIX_SCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATRIX_SCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATRIX_SCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATRIX_SCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATRIX_SCFG[%s]` writer"]
pub struct W(crate::W<MATRIX_SCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATRIX_SCFG_SPEC>;
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
impl From<crate::W<MATRIX_SCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATRIX_SCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOT_CYCLE` reader - Maximum Number of Allowed Cycles for a Burst"]
pub type SLOT_CYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLOT_CYCLE` writer - Maximum Number of Allowed Cycles for a Burst"]
pub type SLOT_CYCLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MATRIX_SCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `DEFMSTR_TYPE` reader - Default Master Type"]
pub type DEFMSTR_TYPE_R = crate::FieldReader<u8, DEFMSTR_TYPE_A>;
#[doc = "Default Master Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEFMSTR_TYPE_A {
    #[doc = "0: At the end of current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in having a one cycle latency for the first access of a burst transfer or for a single access."]
    NO_DEFAULT = 0,
    #[doc = "1: At the end of current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having the one cycle latency when the last master tries to access the slave again."]
    LAST = 1,
    #[doc = "2: At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having the one cycle latency when the fixed master tries to access the slave again."]
    FIXED = 2,
}
impl From<DEFMSTR_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: DEFMSTR_TYPE_A) -> Self {
        variant as _
    }
}
impl DEFMSTR_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEFMSTR_TYPE_A> {
        match self.bits {
            0 => Some(DEFMSTR_TYPE_A::NO_DEFAULT),
            1 => Some(DEFMSTR_TYPE_A::LAST),
            2 => Some(DEFMSTR_TYPE_A::FIXED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DEFAULT`"]
    #[inline(always)]
    pub fn is_no_default(&self) -> bool {
        *self == DEFMSTR_TYPE_A::NO_DEFAULT
    }
    #[doc = "Checks if the value of the field is `LAST`"]
    #[inline(always)]
    pub fn is_last(&self) -> bool {
        *self == DEFMSTR_TYPE_A::LAST
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == DEFMSTR_TYPE_A::FIXED
    }
}
#[doc = "Field `DEFMSTR_TYPE` writer - Default Master Type"]
pub type DEFMSTR_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MATRIX_SCFG_SPEC, u8, DEFMSTR_TYPE_A, 2, O>;
impl<'a, const O: u8> DEFMSTR_TYPE_W<'a, O> {
    #[doc = "At the end of current slave access, if no other master request is pending, the slave is disconnected from all masters.This results in having a one cycle latency for the first access of a burst transfer or for a single access."]
    #[inline(always)]
    pub fn no_default(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPE_A::NO_DEFAULT)
    }
    #[doc = "At the end of current slave access, if no other master request is pending, the slave stays connected to the last master having accessed it.This results in not having the one cycle latency when the last master tries to access the slave again."]
    #[inline(always)]
    pub fn last(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPE_A::LAST)
    }
    #[doc = "At the end of the current slave access, if no other master request is pending, the slave connects to the fixed master the number that has been written in the FIXED_DEFMSTR field.This results in not having the one cycle latency when the fixed master tries to access the slave again."]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPE_A::FIXED)
    }
}
#[doc = "Field `FIXED_DEFMSTR` reader - Fixed Default Master"]
pub type FIXED_DEFMSTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIXED_DEFMSTR` writer - Fixed Default Master"]
pub type FIXED_DEFMSTR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MATRIX_SCFG_SPEC, u8, u8, 3, O>;
#[doc = "Field `ARBT` reader - Arbitration Type"]
pub type ARBT_R = crate::FieldReader<u8, ARBT_A>;
#[doc = "Arbitration Type"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBT_A {
    #[doc = "0: Round-robin arbitration"]
    ROUND_ROBIN = 0,
    #[doc = "1: Fixed priority arbitration"]
    FIXED_PRIORITY = 1,
}
impl From<ARBT_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBT_A) -> Self {
        variant as _
    }
}
impl ARBT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARBT_A> {
        match self.bits {
            0 => Some(ARBT_A::ROUND_ROBIN),
            1 => Some(ARBT_A::FIXED_PRIORITY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ROUND_ROBIN`"]
    #[inline(always)]
    pub fn is_round_robin(&self) -> bool {
        *self == ARBT_A::ROUND_ROBIN
    }
    #[doc = "Checks if the value of the field is `FIXED_PRIORITY`"]
    #[inline(always)]
    pub fn is_fixed_priority(&self) -> bool {
        *self == ARBT_A::FIXED_PRIORITY
    }
}
#[doc = "Field `ARBT` writer - Arbitration Type"]
pub type ARBT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MATRIX_SCFG_SPEC, u8, ARBT_A, 2, O>;
impl<'a, const O: u8> ARBT_W<'a, O> {
    #[doc = "Round-robin arbitration"]
    #[inline(always)]
    pub fn round_robin(self) -> &'a mut W {
        self.variant(ARBT_A::ROUND_ROBIN)
    }
    #[doc = "Fixed priority arbitration"]
    #[inline(always)]
    pub fn fixed_priority(self) -> &'a mut W {
        self.variant(ARBT_A::FIXED_PRIORITY)
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    pub fn slot_cycle(&self) -> SLOT_CYCLE_R {
        SLOT_CYCLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&self) -> DEFMSTR_TYPE_R {
        DEFMSTR_TYPE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&self) -> FIXED_DEFMSTR_R {
        FIXED_DEFMSTR_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Arbitration Type"]
    #[inline(always)]
    pub fn arbt(&self) -> ARBT_R {
        ARBT_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    #[must_use]
    pub fn slot_cycle(&mut self) -> SLOT_CYCLE_W<0> {
        SLOT_CYCLE_W::new(self)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    #[must_use]
    pub fn defmstr_type(&mut self) -> DEFMSTR_TYPE_W<16> {
        DEFMSTR_TYPE_W::new(self)
    }
    #[doc = "Bits 18:20 - Fixed Default Master"]
    #[inline(always)]
    #[must_use]
    pub fn fixed_defmstr(&mut self) -> FIXED_DEFMSTR_W<18> {
        FIXED_DEFMSTR_W::new(self)
    }
    #[doc = "Bits 24:25 - Arbitration Type"]
    #[inline(always)]
    #[must_use]
    pub fn arbt(&mut self) -> ARBT_W<24> {
        ARBT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_scfg](index.html) module"]
pub struct MATRIX_SCFG_SPEC;
impl crate::RegisterSpec for MATRIX_SCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_scfg::R](R) reader structure"]
impl crate::Readable for MATRIX_SCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrix_scfg::W](W) writer structure"]
impl crate::Writable for MATRIX_SCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
