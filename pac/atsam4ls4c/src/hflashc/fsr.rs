#[doc = "Register `FSR` reader"]
pub struct R(crate::R<FSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSR` writer"]
pub struct W(crate::W<FSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSR_SPEC>;
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
impl From<crate::W<FSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRDY` reader - Flash Ready Status"]
pub type FRDY_R = crate::BitReader<bool>;
#[doc = "Field `LOCKE` reader - Lock Error Status"]
pub type LOCKE_R = crate::BitReader<bool>;
#[doc = "Field `PROGE` reader - Programming Error Status"]
pub type PROGE_R = crate::BitReader<bool>;
#[doc = "Field `SECURITY` reader - Security Bit Status"]
pub type SECURITY_R = crate::BitReader<bool>;
#[doc = "Field `QPRR` reader - Quick Page Read Result"]
pub type QPRR_R = crate::BitReader<bool>;
#[doc = "Field `HSMODE` reader - High Speed Mode"]
pub type HSMODE_R = crate::BitReader<bool>;
#[doc = "Field `ECCERR` reader - ECC Error Status"]
pub type ECCERR_R = crate::FieldReader<u8, ECCERRSELECT_A>;
#[doc = "ECC Error Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ECCERRSELECT_A {
    #[doc = "0: no error"]
    NOERROR = 0,
    #[doc = "1: one ECC error detected"]
    ONEECCERR = 1,
    #[doc = "2: two ECC errors detected"]
    TWOECCERR = 2,
}
impl From<ECCERRSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: ECCERRSELECT_A) -> Self {
        variant as _
    }
}
impl ECCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ECCERRSELECT_A> {
        match self.bits {
            0 => Some(ECCERRSELECT_A::NOERROR),
            1 => Some(ECCERRSELECT_A::ONEECCERR),
            2 => Some(ECCERRSELECT_A::TWOECCERR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        *self == ECCERRSELECT_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ONEECCERR`"]
    #[inline(always)]
    pub fn is_oneeccerr(&self) -> bool {
        *self == ECCERRSELECT_A::ONEECCERR
    }
    #[doc = "Checks if the value of the field is `TWOECCERR`"]
    #[inline(always)]
    pub fn is_twoeccerr(&self) -> bool {
        *self == ECCERRSELECT_A::TWOECCERR
    }
}
#[doc = "Field `LOCK0` reader - Lock Region 0 Lock Status"]
pub type LOCK0_R = crate::BitReader<bool>;
#[doc = "Field `LOCK1` reader - Lock Region 1 Lock Status"]
pub type LOCK1_R = crate::BitReader<bool>;
#[doc = "Field `LOCK2` reader - Lock Region 2 Lock Status"]
pub type LOCK2_R = crate::BitReader<bool>;
#[doc = "Field `LOCK3` reader - Lock Region 3 Lock Status"]
pub type LOCK3_R = crate::BitReader<bool>;
#[doc = "Field `LOCK4` reader - Lock Region 4 Lock Status"]
pub type LOCK4_R = crate::BitReader<bool>;
#[doc = "Field `LOCK5` reader - Lock Region 5 Lock Status"]
pub type LOCK5_R = crate::BitReader<bool>;
#[doc = "Field `LOCK6` reader - Lock Region 6 Lock Status"]
pub type LOCK6_R = crate::BitReader<bool>;
#[doc = "Field `LOCK7` reader - Lock Region 7 Lock Status"]
pub type LOCK7_R = crate::BitReader<bool>;
#[doc = "Field `LOCK8` reader - Lock Region 8 Lock Status"]
pub type LOCK8_R = crate::BitReader<bool>;
#[doc = "Field `LOCK9` reader - Lock Region 9 Lock Status"]
pub type LOCK9_R = crate::BitReader<bool>;
#[doc = "Field `LOCK10` reader - Lock Region 10 Lock Status"]
pub type LOCK10_R = crate::BitReader<bool>;
#[doc = "Field `LOCK11` reader - Lock Region 11 Lock Status"]
pub type LOCK11_R = crate::BitReader<bool>;
#[doc = "Field `LOCK12` reader - Lock Region 12 Lock Status"]
pub type LOCK12_R = crate::BitReader<bool>;
#[doc = "Field `LOCK13` reader - Lock Region 13 Lock Status"]
pub type LOCK13_R = crate::BitReader<bool>;
#[doc = "Field `LOCK14` reader - Lock Region 14 Lock Status"]
pub type LOCK14_R = crate::BitReader<bool>;
#[doc = "Field `LOCK15` reader - Lock Region 15 Lock Status"]
pub type LOCK15_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Flash Ready Status"]
    #[inline(always)]
    pub fn frdy(&self) -> FRDY_R {
        FRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Lock Error Status"]
    #[inline(always)]
    pub fn locke(&self) -> LOCKE_R {
        LOCKE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Programming Error Status"]
    #[inline(always)]
    pub fn proge(&self) -> PROGE_R {
        PROGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Security Bit Status"]
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quick Page Read Result"]
    #[inline(always)]
    pub fn qprr(&self) -> QPRR_R {
        QPRR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - High Speed Mode"]
    #[inline(always)]
    pub fn hsmode(&self) -> HSMODE_R {
        HSMODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - ECC Error Status"]
    #[inline(always)]
    pub fn eccerr(&self) -> ECCERR_R {
        ECCERR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Lock Region 0 Lock Status"]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Lock Region 1 Lock Status"]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Lock Region 2 Lock Status"]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Lock Region 3 Lock Status"]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Lock Region 4 Lock Status"]
    #[inline(always)]
    pub fn lock4(&self) -> LOCK4_R {
        LOCK4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Lock Region 5 Lock Status"]
    #[inline(always)]
    pub fn lock5(&self) -> LOCK5_R {
        LOCK5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Lock Region 6 Lock Status"]
    #[inline(always)]
    pub fn lock6(&self) -> LOCK6_R {
        LOCK6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Lock Region 7 Lock Status"]
    #[inline(always)]
    pub fn lock7(&self) -> LOCK7_R {
        LOCK7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Lock Region 8 Lock Status"]
    #[inline(always)]
    pub fn lock8(&self) -> LOCK8_R {
        LOCK8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Lock Region 9 Lock Status"]
    #[inline(always)]
    pub fn lock9(&self) -> LOCK9_R {
        LOCK9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Lock Region 10 Lock Status"]
    #[inline(always)]
    pub fn lock10(&self) -> LOCK10_R {
        LOCK10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Lock Region 11 Lock Status"]
    #[inline(always)]
    pub fn lock11(&self) -> LOCK11_R {
        LOCK11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Lock Region 12 Lock Status"]
    #[inline(always)]
    pub fn lock12(&self) -> LOCK12_R {
        LOCK12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Lock Region 13 Lock Status"]
    #[inline(always)]
    pub fn lock13(&self) -> LOCK13_R {
        LOCK13_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Lock Region 14 Lock Status"]
    #[inline(always)]
    pub fn lock14(&self) -> LOCK14_R {
        LOCK14_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Lock Region 15 Lock Status"]
    #[inline(always)]
    pub fn lock15(&self) -> LOCK15_R {
        LOCK15_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsr](index.html) module"]
pub struct FSR_SPEC;
impl crate::RegisterSpec for FSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsr::R](R) reader structure"]
impl crate::Readable for FSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fsr::W](W) writer structure"]
impl crate::Writable for FSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSR to value 0"]
impl crate::Resettable for FSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
