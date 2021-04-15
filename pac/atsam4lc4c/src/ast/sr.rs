#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Reader of field `OVF`"]
pub type OVF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALARM0`"]
pub type ALARM0_R = crate::R<bool, bool>;
#[doc = "Reader of field `ALARM1`"]
pub type ALARM1_R = crate::R<bool, bool>;
#[doc = "Reader of field `PER0`"]
pub type PER0_R = crate::R<bool, bool>;
#[doc = "Reader of field `PER1`"]
pub type PER1_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `BUSY`"]
pub type BUSY_R = crate::R<bool, BUSY_A>;
impl BUSY_R {
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
        *self == BUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSY_A::_1
    }
}
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, bool>;
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
#[doc = "Reader of field `CLKBUSY`"]
pub type CLKBUSY_R = crate::R<bool, CLKBUSY_A>;
impl CLKBUSY_R {
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
        *self == CLKBUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKBUSY_A::_1
    }
}
#[doc = "Reader of field `CLKRDY`"]
pub type CLKRDY_R = crate::R<bool, bool>;
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
