#[doc = "Register `FSR` reader"]
pub struct R(crate::R<FSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FSR_SPEC>> for R {
    fn from(reader: crate::R<FSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FRDY` reader - Flash Ready Status"]
pub struct FRDY_R(crate::FieldReader<bool, bool>);
impl FRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKE` reader - Lock Error Status"]
pub struct LOCKE_R(crate::FieldReader<bool, bool>);
impl LOCKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROGE` reader - Programming Error Status"]
pub struct PROGE_R(crate::FieldReader<bool, bool>);
impl PROGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURITY` reader - Security Bit Status"]
pub struct SECURITY_R(crate::FieldReader<bool, bool>);
impl SECURITY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECURITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QPRR` reader - Quick Page Read Result"]
pub struct QPRR_R(crate::FieldReader<bool, bool>);
impl QPRR_R {
    pub(crate) fn new(bits: bool) -> Self {
        QPRR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QPRR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSMODE` reader - High Speed Mode"]
pub struct HSMODE_R(crate::FieldReader<bool, bool>);
impl HSMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Field `ECCERR` reader - ECC Error Status"]
pub struct ECCERR_R(crate::FieldReader<u8, ECCERR_A>);
impl ECCERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ECCERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ECCERR_A> {
        match self.bits {
            0 => Some(ECCERR_A::NOERROR),
            1 => Some(ECCERR_A::ONEECCERR),
            2 => Some(ECCERR_A::TWOECCERR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOERROR`"]
    #[inline(always)]
    pub fn is_noerror(&self) -> bool {
        **self == ECCERR_A::NOERROR
    }
    #[doc = "Checks if the value of the field is `ONEECCERR`"]
    #[inline(always)]
    pub fn is_oneeccerr(&self) -> bool {
        **self == ECCERR_A::ONEECCERR
    }
    #[doc = "Checks if the value of the field is `TWOECCERR`"]
    #[inline(always)]
    pub fn is_twoeccerr(&self) -> bool {
        **self == ECCERR_A::TWOECCERR
    }
}
impl core::ops::Deref for ECCERR_R {
    type Target = crate::FieldReader<u8, ECCERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK0` reader - Lock Region 0 Lock Status"]
pub struct LOCK0_R(crate::FieldReader<bool, bool>);
impl LOCK0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK1` reader - Lock Region 1 Lock Status"]
pub struct LOCK1_R(crate::FieldReader<bool, bool>);
impl LOCK1_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK2` reader - Lock Region 2 Lock Status"]
pub struct LOCK2_R(crate::FieldReader<bool, bool>);
impl LOCK2_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK3` reader - Lock Region 3 Lock Status"]
pub struct LOCK3_R(crate::FieldReader<bool, bool>);
impl LOCK3_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK4` reader - Lock Region 4 Lock Status"]
pub struct LOCK4_R(crate::FieldReader<bool, bool>);
impl LOCK4_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK5` reader - Lock Region 5 Lock Status"]
pub struct LOCK5_R(crate::FieldReader<bool, bool>);
impl LOCK5_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK6` reader - Lock Region 6 Lock Status"]
pub struct LOCK6_R(crate::FieldReader<bool, bool>);
impl LOCK6_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK7` reader - Lock Region 7 Lock Status"]
pub struct LOCK7_R(crate::FieldReader<bool, bool>);
impl LOCK7_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK8` reader - Lock Region 8 Lock Status"]
pub struct LOCK8_R(crate::FieldReader<bool, bool>);
impl LOCK8_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK9` reader - Lock Region 9 Lock Status"]
pub struct LOCK9_R(crate::FieldReader<bool, bool>);
impl LOCK9_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK10` reader - Lock Region 10 Lock Status"]
pub struct LOCK10_R(crate::FieldReader<bool, bool>);
impl LOCK10_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK11` reader - Lock Region 11 Lock Status"]
pub struct LOCK11_R(crate::FieldReader<bool, bool>);
impl LOCK11_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK12` reader - Lock Region 12 Lock Status"]
pub struct LOCK12_R(crate::FieldReader<bool, bool>);
impl LOCK12_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK13` reader - Lock Region 13 Lock Status"]
pub struct LOCK13_R(crate::FieldReader<bool, bool>);
impl LOCK13_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK14` reader - Lock Region 14 Lock Status"]
pub struct LOCK14_R(crate::FieldReader<bool, bool>);
impl LOCK14_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK15` reader - Lock Region 15 Lock Status"]
pub struct LOCK15_R(crate::FieldReader<bool, bool>);
impl LOCK15_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Flash Controller Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsr](index.html) module"]
pub struct FSR_SPEC;
impl crate::RegisterSpec for FSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fsr::R](R) reader structure"]
impl crate::Readable for FSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSR to value 0"]
impl crate::Resettable for FSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
