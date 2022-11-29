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
pub type OVF_R = crate::BitReader<bool>;
#[doc = "Field `ALARM0` reader - Alarm 0"]
pub type ALARM0_R = crate::BitReader<bool>;
#[doc = "Field `ALARM1` reader - Alarm 1"]
pub type ALARM1_R = crate::BitReader<bool>;
#[doc = "Field `PER0` reader - Periodic 0"]
pub type PER0_R = crate::BitReader<bool>;
#[doc = "Field `PER1` reader - Periodic 1"]
pub type PER1_R = crate::BitReader<bool>;
#[doc = "Field `BUSY` reader - AST Busy"]
pub type BUSY_R = crate::BitReader<BUSYSELECT_A>;
#[doc = "AST Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSYSELECT_A {
    #[doc = "0: The AST accepts writes to CV, WER, DTR, SCR, AR, PIR and CR"]
    _0 = 0,
    #[doc = "1: The AST is busy and will discard writes to CV, WER, DTR, SCR, AR, PIR and CR"]
    _1 = 1,
}
impl From<BUSYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: BUSYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSYSELECT_A {
        match self.bits {
            false => BUSYSELECT_A::_0,
            true => BUSYSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSYSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSYSELECT_A::_1
    }
}
#[doc = "Field `READY` reader - AST Ready"]
pub type READY_R = crate::BitReader<bool>;
#[doc = "Field `CLKBUSY` reader - Clock Busy"]
pub type CLKBUSY_R = crate::BitReader<CLKBUSYSELECT_A>;
#[doc = "Clock Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKBUSYSELECT_A {
    #[doc = "0: The clock is ready and can be changed"]
    _0 = 0,
    #[doc = "1: CEN has been written and the clock is busy"]
    _1 = 1,
}
impl From<CLKBUSYSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CLKBUSYSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKBUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKBUSYSELECT_A {
        match self.bits {
            false => CLKBUSYSELECT_A::_0,
            true => CLKBUSYSELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKBUSYSELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKBUSYSELECT_A::_1
    }
}
#[doc = "Field `CLKRDY` reader - Clock Ready"]
pub type CLKRDY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&self) -> OVF_R {
        OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    pub fn alarm0(&self) -> ALARM0_R {
        ALARM0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    pub fn alarm1(&self) -> ALARM1_R {
        ALARM1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Periodic 0"]
    #[inline(always)]
    pub fn per0(&self) -> PER0_R {
        PER0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Periodic 1"]
    #[inline(always)]
    pub fn per1(&self) -> PER1_R {
        PER1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - AST Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AST Ready"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Clock Busy"]
    #[inline(always)]
    pub fn clkbusy(&self) -> CLKBUSY_R {
        CLKBUSY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Clock Ready"]
    #[inline(always)]
    pub fn clkrdy(&self) -> CLKRDY_R {
        CLKRDY_R::new(((self.bits >> 29) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
