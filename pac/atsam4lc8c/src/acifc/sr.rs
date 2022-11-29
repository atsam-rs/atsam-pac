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
#[doc = "Field `ACCS0` reader - AC0 Current Comparison Status"]
pub type ACCS0_R = crate::BitReader<bool>;
#[doc = "Field `ACRDY0` reader - AC0 Ready"]
pub type ACRDY0_R = crate::BitReader<bool>;
#[doc = "Field `ACCS1` reader - AC1 Current Comparison Status"]
pub type ACCS1_R = crate::BitReader<bool>;
#[doc = "Field `ACRDY1` reader - AC1 Ready"]
pub type ACRDY1_R = crate::BitReader<bool>;
#[doc = "Field `ACCS2` reader - AC2 Current Comparison Status"]
pub type ACCS2_R = crate::BitReader<bool>;
#[doc = "Field `ACRDY2` reader - AC2 Ready"]
pub type ACRDY2_R = crate::BitReader<bool>;
#[doc = "Field `ACCS3` reader - AC3 Current Comparison Status"]
pub type ACCS3_R = crate::BitReader<bool>;
#[doc = "Field `ACRDY3` reader - AC3 Ready"]
pub type ACRDY3_R = crate::BitReader<bool>;
#[doc = "Field `ACCS4` reader - AC4 Current Comparison Status"]
pub type ACCS4_R = crate::BitReader<bool>;
#[doc = "Field `ACRDY4` reader - AC4 Ready"]
pub type ACRDY4_R = crate::BitReader<bool>;
#[doc = "Field `ACCS5` reader - AC5 Current Comparison Status"]
pub type ACCS5_R = crate::BitReader<bool>;
#[doc = "Field `ACRDY5` reader - AC5 Ready"]
pub type ACRDY5_R = crate::BitReader<bool>;
#[doc = "Field `ACCS6` reader - AC6 Current Comparison Status"]
pub type ACCS6_R = crate::BitReader<bool>;
#[doc = "Field `ACRDY6` reader - AC6 Ready"]
pub type ACRDY6_R = crate::BitReader<bool>;
#[doc = "Field `ACCS7` reader - AC7 Current Comparison Status"]
pub type ACCS7_R = crate::BitReader<bool>;
#[doc = "Field `ACRDY7` reader - AC7 Ready"]
pub type ACRDY7_R = crate::BitReader<bool>;
#[doc = "Field `WFCS0` reader - Window0 Mode Current Status"]
pub type WFCS0_R = crate::BitReader<bool>;
#[doc = "Field `WFCS1` reader - Window1 Mode Current Status"]
pub type WFCS1_R = crate::BitReader<bool>;
#[doc = "Field `WFCS2` reader - Window2 Mode Current Status"]
pub type WFCS2_R = crate::BitReader<bool>;
#[doc = "Field `WFCS3` reader - Window3 Mode Current Status"]
pub type WFCS3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - AC0 Current Comparison Status"]
    #[inline(always)]
    pub fn accs0(&self) -> ACCS0_R {
        ACCS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AC0 Ready"]
    #[inline(always)]
    pub fn acrdy0(&self) -> ACRDY0_R {
        ACRDY0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AC1 Current Comparison Status"]
    #[inline(always)]
    pub fn accs1(&self) -> ACCS1_R {
        ACCS1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AC1 Ready"]
    #[inline(always)]
    pub fn acrdy1(&self) -> ACRDY1_R {
        ACRDY1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AC2 Current Comparison Status"]
    #[inline(always)]
    pub fn accs2(&self) -> ACCS2_R {
        ACCS2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AC2 Ready"]
    #[inline(always)]
    pub fn acrdy2(&self) -> ACRDY2_R {
        ACRDY2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AC3 Current Comparison Status"]
    #[inline(always)]
    pub fn accs3(&self) -> ACCS3_R {
        ACCS3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AC3 Ready"]
    #[inline(always)]
    pub fn acrdy3(&self) -> ACRDY3_R {
        ACRDY3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AC4 Current Comparison Status"]
    #[inline(always)]
    pub fn accs4(&self) -> ACCS4_R {
        ACCS4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AC4 Ready"]
    #[inline(always)]
    pub fn acrdy4(&self) -> ACRDY4_R {
        ACRDY4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AC5 Current Comparison Status"]
    #[inline(always)]
    pub fn accs5(&self) -> ACCS5_R {
        ACCS5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AC5 Ready"]
    #[inline(always)]
    pub fn acrdy5(&self) -> ACRDY5_R {
        ACRDY5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AC6 Current Comparison Status"]
    #[inline(always)]
    pub fn accs6(&self) -> ACCS6_R {
        ACCS6_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AC6 Ready"]
    #[inline(always)]
    pub fn acrdy6(&self) -> ACRDY6_R {
        ACRDY6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AC7 Current Comparison Status"]
    #[inline(always)]
    pub fn accs7(&self) -> ACCS7_R {
        ACCS7_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AC7 Ready"]
    #[inline(always)]
    pub fn acrdy7(&self) -> ACRDY7_R {
        ACRDY7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Window0 Mode Current Status"]
    #[inline(always)]
    pub fn wfcs0(&self) -> WFCS0_R {
        WFCS0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Window1 Mode Current Status"]
    #[inline(always)]
    pub fn wfcs1(&self) -> WFCS1_R {
        WFCS1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Window2 Mode Current Status"]
    #[inline(always)]
    pub fn wfcs2(&self) -> WFCS2_R {
        WFCS2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Window3 Mode Current Status"]
    #[inline(always)]
    pub fn wfcs3(&self) -> WFCS3_R {
        WFCS3_R::new(((self.bits >> 27) & 1) != 0)
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
