#[doc = "Register `SCFG%s` reader"]
pub struct R(crate::R<SCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCFG%s` writer"]
pub struct W(crate::W<SCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCFG_SPEC>;
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
impl From<crate::W<SCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLOT_CYCLE` reader - Maximum Number of Allowed Cycles for a Burst"]
pub type SLOT_CYCLE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLOT_CYCLE` writer - Maximum Number of Allowed Cycles for a Burst"]
pub type SLOT_CYCLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `DEFMSTR_TYPE` reader - Default Master Type"]
pub type DEFMSTR_TYPE_R = crate::FieldReader<u8, DEFMSTR_TYPESELECT_A>;
#[doc = "Default Master Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEFMSTR_TYPESELECT_A {
    #[doc = "0: No Default Master. At the end of current slave access, if no other master request is pending, the slave is deconnected from all masters. This resusts in having a one cycle latency for the first transfer of a burst."]
    NO_DEFAULT = 0,
    #[doc = "1: Last Default Master At the end of current slave access, if no other master request is pending, the slave stay connected with the last master havingaccessed it.This resusts in not having the one cycle latency when the last master re-trying access on the slave."]
    LAST_DEFAULT = 1,
    #[doc = "2: Fixed Default Master At the end of current slave access, if no other master request is pending, the slave connects with fixed master which numberis in FIXED_DEFMSTR register.This resusts in not having the one cycle latency when the fixed master re-trying access on the slave."]
    FIXED_DEFAULT = 2,
}
impl From<DEFMSTR_TYPESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: DEFMSTR_TYPESELECT_A) -> Self {
        variant as _
    }
}
impl DEFMSTR_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEFMSTR_TYPESELECT_A> {
        match self.bits {
            0 => Some(DEFMSTR_TYPESELECT_A::NO_DEFAULT),
            1 => Some(DEFMSTR_TYPESELECT_A::LAST_DEFAULT),
            2 => Some(DEFMSTR_TYPESELECT_A::FIXED_DEFAULT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DEFAULT`"]
    #[inline(always)]
    pub fn is_no_default(&self) -> bool {
        *self == DEFMSTR_TYPESELECT_A::NO_DEFAULT
    }
    #[doc = "Checks if the value of the field is `LAST_DEFAULT`"]
    #[inline(always)]
    pub fn is_last_default(&self) -> bool {
        *self == DEFMSTR_TYPESELECT_A::LAST_DEFAULT
    }
    #[doc = "Checks if the value of the field is `FIXED_DEFAULT`"]
    #[inline(always)]
    pub fn is_fixed_default(&self) -> bool {
        *self == DEFMSTR_TYPESELECT_A::FIXED_DEFAULT
    }
}
#[doc = "Field `DEFMSTR_TYPE` writer - Default Master Type"]
pub type DEFMSTR_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCFG_SPEC, u8, DEFMSTR_TYPESELECT_A, 2, O>;
impl<'a, const O: u8> DEFMSTR_TYPE_W<'a, O> {
    #[doc = "No Default Master. At the end of current slave access, if no other master request is pending, the slave is deconnected from all masters. This resusts in having a one cycle latency for the first transfer of a burst."]
    #[inline(always)]
    pub fn no_default(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPESELECT_A::NO_DEFAULT)
    }
    #[doc = "Last Default Master At the end of current slave access, if no other master request is pending, the slave stay connected with the last master havingaccessed it.This resusts in not having the one cycle latency when the last master re-trying access on the slave."]
    #[inline(always)]
    pub fn last_default(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPESELECT_A::LAST_DEFAULT)
    }
    #[doc = "Fixed Default Master At the end of current slave access, if no other master request is pending, the slave connects with fixed master which numberis in FIXED_DEFMSTR register.This resusts in not having the one cycle latency when the fixed master re-trying access on the slave."]
    #[inline(always)]
    pub fn fixed_default(self) -> &'a mut W {
        self.variant(DEFMSTR_TYPESELECT_A::FIXED_DEFAULT)
    }
}
#[doc = "Field `FIXED_DEFMSTR` reader - Fixed Index of Default Master"]
pub type FIXED_DEFMSTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FIXED_DEFMSTR` writer - Fixed Index of Default Master"]
pub type FIXED_DEFMSTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `ARBT` reader - Arbitration Type"]
pub type ARBT_R = crate::BitReader<ARBTSELECT_A>;
#[doc = "Arbitration Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARBTSELECT_A {
    #[doc = "0: Round-Robin Arbitration"]
    ROUND_ROBIN = 0,
    #[doc = "1: Fixed Priority Arbitration"]
    FIXED_PRIORITY = 1,
}
impl From<ARBTSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ARBTSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ARBT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBTSELECT_A {
        match self.bits {
            false => ARBTSELECT_A::ROUND_ROBIN,
            true => ARBTSELECT_A::FIXED_PRIORITY,
        }
    }
    #[doc = "Checks if the value of the field is `ROUND_ROBIN`"]
    #[inline(always)]
    pub fn is_round_robin(&self) -> bool {
        *self == ARBTSELECT_A::ROUND_ROBIN
    }
    #[doc = "Checks if the value of the field is `FIXED_PRIORITY`"]
    #[inline(always)]
    pub fn is_fixed_priority(&self) -> bool {
        *self == ARBTSELECT_A::FIXED_PRIORITY
    }
}
#[doc = "Field `ARBT` writer - Arbitration Type"]
pub type ARBT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCFG_SPEC, ARBTSELECT_A, O>;
impl<'a, const O: u8> ARBT_W<'a, O> {
    #[doc = "Round-Robin Arbitration"]
    #[inline(always)]
    pub fn round_robin(self) -> &'a mut W {
        self.variant(ARBTSELECT_A::ROUND_ROBIN)
    }
    #[doc = "Fixed Priority Arbitration"]
    #[inline(always)]
    pub fn fixed_priority(self) -> &'a mut W {
        self.variant(ARBTSELECT_A::FIXED_PRIORITY)
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
    #[doc = "Bits 18:21 - Fixed Index of Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&self) -> FIXED_DEFMSTR_R {
        FIXED_DEFMSTR_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Arbitration Type"]
    #[inline(always)]
    pub fn arbt(&self) -> ARBT_R {
        ARBT_R::new(((self.bits >> 24) & 1) != 0)
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
    #[doc = "Bits 18:21 - Fixed Index of Default Master"]
    #[inline(always)]
    #[must_use]
    pub fn fixed_defmstr(&mut self) -> FIXED_DEFMSTR_W<18> {
        FIXED_DEFMSTR_W::new(self)
    }
    #[doc = "Bit 24 - Arbitration Type"]
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
#[doc = "Slave Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfg](index.html) module"]
pub struct SCFG_SPEC;
impl crate::RegisterSpec for SCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scfg::R](R) reader structure"]
impl crate::Readable for SCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scfg::W](W) writer structure"]
impl crate::Writable for SCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCFG%s to value 0x10"]
impl crate::Resettable for SCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
