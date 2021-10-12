#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACINT0` reader - AC0 Interrupt Mask"]
pub struct ACINT0_R(crate::FieldReader<bool, bool>);
impl ACINT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACINT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACINT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUTINT0` reader - AC0 Startup Time Interrupt Mask"]
pub struct SUTINT0_R(crate::FieldReader<bool, bool>);
impl SUTINT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUTINT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUTINT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACINT1` reader - AC1 Interrupt Mask"]
pub struct ACINT1_R(crate::FieldReader<bool, bool>);
impl ACINT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACINT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACINT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUTINT1` reader - AC1 Startup Time Interrupt Mask"]
pub struct SUTINT1_R(crate::FieldReader<bool, bool>);
impl SUTINT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUTINT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUTINT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACINT2` reader - AC2 Interrupt Mask"]
pub struct ACINT2_R(crate::FieldReader<bool, bool>);
impl ACINT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACINT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACINT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUTINT2` reader - AC2 Startup Time Interrupt Mask"]
pub struct SUTINT2_R(crate::FieldReader<bool, bool>);
impl SUTINT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUTINT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUTINT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACINT3` reader - AC3 Interrupt Mask"]
pub struct ACINT3_R(crate::FieldReader<bool, bool>);
impl ACINT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACINT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACINT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUTINT3` reader - AC3 Startup Time Interrupt Mask"]
pub struct SUTINT3_R(crate::FieldReader<bool, bool>);
impl SUTINT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUTINT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUTINT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACINT4` reader - AC4 Interrupt Mask"]
pub struct ACINT4_R(crate::FieldReader<bool, bool>);
impl ACINT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACINT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACINT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUTINT4` reader - AC4 Startup Time Interrupt Mask"]
pub struct SUTINT4_R(crate::FieldReader<bool, bool>);
impl SUTINT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUTINT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUTINT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACINT5` reader - AC5 Interrupt Mask"]
pub struct ACINT5_R(crate::FieldReader<bool, bool>);
impl ACINT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACINT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACINT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUTINT5` reader - AC5 Startup Time Interrupt Mask"]
pub struct SUTINT5_R(crate::FieldReader<bool, bool>);
impl SUTINT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUTINT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUTINT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACINT6` reader - AC6 Interrupt Mask"]
pub struct ACINT6_R(crate::FieldReader<bool, bool>);
impl ACINT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACINT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACINT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUTINT6` reader - AC6 Startup Time Interrupt Mask"]
pub struct SUTINT6_R(crate::FieldReader<bool, bool>);
impl SUTINT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUTINT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUTINT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACINT7` reader - AC7 Interrupt Mask"]
pub struct ACINT7_R(crate::FieldReader<bool, bool>);
impl ACINT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACINT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACINT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUTINT7` reader - AC7 Startup Time Interrupt Mask"]
pub struct SUTINT7_R(crate::FieldReader<bool, bool>);
impl SUTINT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUTINT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUTINT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFINT0` reader - Window0 Mode Interrupt Mask"]
pub struct WFINT0_R(crate::FieldReader<bool, bool>);
impl WFINT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFINT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFINT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFINT1` reader - Window1 Mode Interrupt Mask"]
pub struct WFINT1_R(crate::FieldReader<bool, bool>);
impl WFINT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFINT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFINT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFINT2` reader - Window2 Mode Interrupt Mask"]
pub struct WFINT2_R(crate::FieldReader<bool, bool>);
impl WFINT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFINT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFINT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WFINT3` reader - Window3 Mode Interrupt Mask"]
pub struct WFINT3_R(crate::FieldReader<bool, bool>);
impl WFINT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WFINT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WFINT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - AC0 Interrupt Mask"]
    #[inline(always)]
    pub fn acint0(&self) -> ACINT0_R {
        ACINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AC0 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint0(&self) -> SUTINT0_R {
        SUTINT0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AC1 Interrupt Mask"]
    #[inline(always)]
    pub fn acint1(&self) -> ACINT1_R {
        ACINT1_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AC1 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint1(&self) -> SUTINT1_R {
        SUTINT1_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AC2 Interrupt Mask"]
    #[inline(always)]
    pub fn acint2(&self) -> ACINT2_R {
        ACINT2_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AC2 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint2(&self) -> SUTINT2_R {
        SUTINT2_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AC3 Interrupt Mask"]
    #[inline(always)]
    pub fn acint3(&self) -> ACINT3_R {
        ACINT3_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AC3 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint3(&self) -> SUTINT3_R {
        SUTINT3_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - AC4 Interrupt Mask"]
    #[inline(always)]
    pub fn acint4(&self) -> ACINT4_R {
        ACINT4_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AC4 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint4(&self) -> SUTINT4_R {
        SUTINT4_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AC5 Interrupt Mask"]
    #[inline(always)]
    pub fn acint5(&self) -> ACINT5_R {
        ACINT5_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - AC5 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint5(&self) -> SUTINT5_R {
        SUTINT5_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - AC6 Interrupt Mask"]
    #[inline(always)]
    pub fn acint6(&self) -> ACINT6_R {
        ACINT6_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - AC6 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint6(&self) -> SUTINT6_R {
        SUTINT6_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - AC7 Interrupt Mask"]
    #[inline(always)]
    pub fn acint7(&self) -> ACINT7_R {
        ACINT7_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - AC7 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint7(&self) -> SUTINT7_R {
        SUTINT7_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Window0 Mode Interrupt Mask"]
    #[inline(always)]
    pub fn wfint0(&self) -> WFINT0_R {
        WFINT0_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Window1 Mode Interrupt Mask"]
    #[inline(always)]
    pub fn wfint1(&self) -> WFINT1_R {
        WFINT1_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Window2 Mode Interrupt Mask"]
    #[inline(always)]
    pub fn wfint2(&self) -> WFINT2_R {
        WFINT2_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Window3 Mode Interrupt Mask"]
    #[inline(always)]
    pub fn wfint3(&self) -> WFINT3_R {
        WFINT3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
