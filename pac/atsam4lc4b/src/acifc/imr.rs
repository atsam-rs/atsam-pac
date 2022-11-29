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
pub type ACINT0_R = crate::BitReader<bool>;
#[doc = "Field `SUTINT0` reader - AC0 Startup Time Interrupt Mask"]
pub type SUTINT0_R = crate::BitReader<bool>;
#[doc = "Field `ACINT1` reader - AC1 Interrupt Mask"]
pub type ACINT1_R = crate::BitReader<bool>;
#[doc = "Field `SUTINT1` reader - AC1 Startup Time Interrupt Mask"]
pub type SUTINT1_R = crate::BitReader<bool>;
#[doc = "Field `ACINT2` reader - AC2 Interrupt Mask"]
pub type ACINT2_R = crate::BitReader<bool>;
#[doc = "Field `SUTINT2` reader - AC2 Startup Time Interrupt Mask"]
pub type SUTINT2_R = crate::BitReader<bool>;
#[doc = "Field `ACINT3` reader - AC3 Interrupt Mask"]
pub type ACINT3_R = crate::BitReader<bool>;
#[doc = "Field `SUTINT3` reader - AC3 Startup Time Interrupt Mask"]
pub type SUTINT3_R = crate::BitReader<bool>;
#[doc = "Field `ACINT4` reader - AC4 Interrupt Mask"]
pub type ACINT4_R = crate::BitReader<bool>;
#[doc = "Field `SUTINT4` reader - AC4 Startup Time Interrupt Mask"]
pub type SUTINT4_R = crate::BitReader<bool>;
#[doc = "Field `ACINT5` reader - AC5 Interrupt Mask"]
pub type ACINT5_R = crate::BitReader<bool>;
#[doc = "Field `SUTINT5` reader - AC5 Startup Time Interrupt Mask"]
pub type SUTINT5_R = crate::BitReader<bool>;
#[doc = "Field `ACINT6` reader - AC6 Interrupt Mask"]
pub type ACINT6_R = crate::BitReader<bool>;
#[doc = "Field `SUTINT6` reader - AC6 Startup Time Interrupt Mask"]
pub type SUTINT6_R = crate::BitReader<bool>;
#[doc = "Field `ACINT7` reader - AC7 Interrupt Mask"]
pub type ACINT7_R = crate::BitReader<bool>;
#[doc = "Field `SUTINT7` reader - AC7 Startup Time Interrupt Mask"]
pub type SUTINT7_R = crate::BitReader<bool>;
#[doc = "Field `WFINT0` reader - Window0 Mode Interrupt Mask"]
pub type WFINT0_R = crate::BitReader<bool>;
#[doc = "Field `WFINT1` reader - Window1 Mode Interrupt Mask"]
pub type WFINT1_R = crate::BitReader<bool>;
#[doc = "Field `WFINT2` reader - Window2 Mode Interrupt Mask"]
pub type WFINT2_R = crate::BitReader<bool>;
#[doc = "Field `WFINT3` reader - Window3 Mode Interrupt Mask"]
pub type WFINT3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - AC0 Interrupt Mask"]
    #[inline(always)]
    pub fn acint0(&self) -> ACINT0_R {
        ACINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AC0 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint0(&self) -> SUTINT0_R {
        SUTINT0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AC1 Interrupt Mask"]
    #[inline(always)]
    pub fn acint1(&self) -> ACINT1_R {
        ACINT1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AC1 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint1(&self) -> SUTINT1_R {
        SUTINT1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AC2 Interrupt Mask"]
    #[inline(always)]
    pub fn acint2(&self) -> ACINT2_R {
        ACINT2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AC2 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint2(&self) -> SUTINT2_R {
        SUTINT2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AC3 Interrupt Mask"]
    #[inline(always)]
    pub fn acint3(&self) -> ACINT3_R {
        ACINT3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AC3 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint3(&self) -> SUTINT3_R {
        SUTINT3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AC4 Interrupt Mask"]
    #[inline(always)]
    pub fn acint4(&self) -> ACINT4_R {
        ACINT4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AC4 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint4(&self) -> SUTINT4_R {
        SUTINT4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AC5 Interrupt Mask"]
    #[inline(always)]
    pub fn acint5(&self) -> ACINT5_R {
        ACINT5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AC5 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint5(&self) -> SUTINT5_R {
        SUTINT5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AC6 Interrupt Mask"]
    #[inline(always)]
    pub fn acint6(&self) -> ACINT6_R {
        ACINT6_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AC6 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint6(&self) -> SUTINT6_R {
        SUTINT6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AC7 Interrupt Mask"]
    #[inline(always)]
    pub fn acint7(&self) -> ACINT7_R {
        ACINT7_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AC7 Startup Time Interrupt Mask"]
    #[inline(always)]
    pub fn sutint7(&self) -> SUTINT7_R {
        SUTINT7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Window0 Mode Interrupt Mask"]
    #[inline(always)]
    pub fn wfint0(&self) -> WFINT0_R {
        WFINT0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Window1 Mode Interrupt Mask"]
    #[inline(always)]
    pub fn wfint1(&self) -> WFINT1_R {
        WFINT1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Window2 Mode Interrupt Mask"]
    #[inline(always)]
    pub fn wfint2(&self) -> WFINT2_R {
        WFINT2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Window3 Mode Interrupt Mask"]
    #[inline(always)]
    pub fn wfint3(&self) -> WFINT3_R {
        WFINT3_R::new(((self.bits >> 27) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
