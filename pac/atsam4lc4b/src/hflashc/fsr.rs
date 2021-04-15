#[doc = "Reader of register FSR"]
pub type R = crate::R<u32, super::FSR>;
#[doc = "Reader of field `FRDY`"]
pub type FRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCKE`"]
pub type LOCKE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROGE`"]
pub type PROGE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SECURITY`"]
pub type SECURITY_R = crate::R<bool, bool>;
#[doc = "Reader of field `QPRR`"]
pub type QPRR_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSMODE`"]
pub type HSMODE_R = crate::R<bool, bool>;
#[doc = "ECC Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ECCERR_A {
    #[doc = "0: no error"]
    NOERROR = 0,
    #[doc = "1: one ECC error detected"]
    ONEECCERR = 1,
    #[doc = "2: two ECC errors detected"]
    TWOECCERR = 2,
}
impl From<ECCERR_A> for u8 {
    #[inline(always)]
    fn from(variant: ECCERR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ECCERR`"]
pub type ECCERR_R = crate::R<u8, ECCERR_A>;
impl ECCERR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ECCERR_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ECCERR_A::NOERROR),
            1 => Val(ECCERR_A::ONEECCERR),
            2 => Val(ECCERR_A::TWOECCERR),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == ECCERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ONEECCERR`"]
    #[inline(always)]
    pub fn is_oneeccerr(&self) -> bool {
        *self == ECCERR_A::ONEECCERR
    }
    #[doc = "Checks if the value of the field is `TWOECCERR`"]
    #[inline(always)]
    pub fn is_twoeccerr(&self) -> bool {
        *self == ECCERR_A::TWOECCERR
    }
}
#[doc = "Reader of field `LOCK0`"]
pub type LOCK0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK1`"]
pub type LOCK1_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK2`"]
pub type LOCK2_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK3`"]
pub type LOCK3_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK4`"]
pub type LOCK4_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK5`"]
pub type LOCK5_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK6`"]
pub type LOCK6_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK7`"]
pub type LOCK7_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK8`"]
pub type LOCK8_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK9`"]
pub type LOCK9_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK10`"]
pub type LOCK10_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK11`"]
pub type LOCK11_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK12`"]
pub type LOCK12_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK13`"]
pub type LOCK13_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK14`"]
pub type LOCK14_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK15`"]
pub type LOCK15_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Flash Ready Status"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Error Status"]
    #[inline(always)]
    pub fn locke(&self) -> LOCKE_R {
        LOCKE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Programming Error Status"]
    #[inline(always)]
    pub fn proge(&self) -> PROGE_R {
        PROGE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Security Bit Status"]
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Quick Page Read Result"]
    #[inline(always)]
    pub fn qprr(&self) -> QPRR_R {
        QPRR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - High Speed Mode"]
    #[inline(always)]
    pub fn hsmode(&self) -> HSMODE_R {
        HSMODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - ECC Error Status"]
    #[inline(always)]
    pub fn eccerr(&self) -> ECCERR_R {
        ECCERR_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Lock Region 0 Lock Status"]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Lock Region 1 Lock Status"]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Lock Region 2 Lock Status"]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Lock Region 3 Lock Status"]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Lock Region 4 Lock Status"]
    #[inline(always)]
    pub fn lock4(&self) -> LOCK4_R {
        LOCK4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Lock Region 5 Lock Status"]
    #[inline(always)]
    pub fn lock5(&self) -> LOCK5_R {
        LOCK5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Lock Region 6 Lock Status"]
    #[inline(always)]
    pub fn lock6(&self) -> LOCK6_R {
        LOCK6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Region 7 Lock Status"]
    #[inline(always)]
    pub fn lock7(&self) -> LOCK7_R {
        LOCK7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Lock Region 8 Lock Status"]
    #[inline(always)]
    pub fn lock8(&self) -> LOCK8_R {
        LOCK8_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Lock Region 9 Lock Status"]
    #[inline(always)]
    pub fn lock9(&self) -> LOCK9_R {
        LOCK9_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Lock Region 10 Lock Status"]
    #[inline(always)]
    pub fn lock10(&self) -> LOCK10_R {
        LOCK10_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Lock Region 11 Lock Status"]
    #[inline(always)]
    pub fn lock11(&self) -> LOCK11_R {
        LOCK11_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Lock Region 12 Lock Status"]
    #[inline(always)]
    pub fn lock12(&self) -> LOCK12_R {
        LOCK12_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Lock Region 13 Lock Status"]
    #[inline(always)]
    pub fn lock13(&self) -> LOCK13_R {
        LOCK13_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Lock Region 14 Lock Status"]
    #[inline(always)]
    pub fn lock14(&self) -> LOCK14_R {
        LOCK14_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock Region 15 Lock Status"]
    #[inline(always)]
    pub fn lock15(&self) -> LOCK15_R {
        LOCK15_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
