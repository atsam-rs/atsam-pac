#[doc = "Register `PARAMETER` reader"]
pub struct R(crate::R<PARAMETER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAMETER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAMETER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAMETER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACIMPL0` reader - Analog Comparator 0 Implemented"]
pub type ACIMPL0_R = crate::BitReader<bool>;
#[doc = "Field `ACIMPL1` reader - Analog Comparator 1 Implemented"]
pub type ACIMPL1_R = crate::BitReader<bool>;
#[doc = "Field `ACIMPL2` reader - Analog Comparator 2 Implemented"]
pub type ACIMPL2_R = crate::BitReader<bool>;
#[doc = "Field `ACIMPL3` reader - Analog Comparator 3 Implemented"]
pub type ACIMPL3_R = crate::BitReader<bool>;
#[doc = "Field `ACIMPL4` reader - Analog Comparator 4 Implemented"]
pub type ACIMPL4_R = crate::BitReader<bool>;
#[doc = "Field `ACIMPL5` reader - Analog Comparator 5 Implemented"]
pub type ACIMPL5_R = crate::BitReader<bool>;
#[doc = "Field `ACIMPL6` reader - Analog Comparator 6 Implemented"]
pub type ACIMPL6_R = crate::BitReader<bool>;
#[doc = "Field `ACIMPL7` reader - Analog Comparator 7 Implemented"]
pub type ACIMPL7_R = crate::BitReader<bool>;
#[doc = "Field `WIMPL0` reader - Window0 Mode Implemented"]
pub type WIMPL0_R = crate::BitReader<bool>;
#[doc = "Field `WIMPL1` reader - Window1 Mode Implemented"]
pub type WIMPL1_R = crate::BitReader<bool>;
#[doc = "Field `WIMPL2` reader - Window2 Mode Implemented"]
pub type WIMPL2_R = crate::BitReader<bool>;
#[doc = "Field `WIMPL3` reader - Window3 Mode Implemented"]
pub type WIMPL3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Analog Comparator 0 Implemented"]
    #[inline(always)]
    pub fn acimpl0(&self) -> ACIMPL0_R {
        ACIMPL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator 1 Implemented"]
    #[inline(always)]
    pub fn acimpl1(&self) -> ACIMPL1_R {
        ACIMPL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Comparator 2 Implemented"]
    #[inline(always)]
    pub fn acimpl2(&self) -> ACIMPL2_R {
        ACIMPL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator 3 Implemented"]
    #[inline(always)]
    pub fn acimpl3(&self) -> ACIMPL3_R {
        ACIMPL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog Comparator 4 Implemented"]
    #[inline(always)]
    pub fn acimpl4(&self) -> ACIMPL4_R {
        ACIMPL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Analog Comparator 5 Implemented"]
    #[inline(always)]
    pub fn acimpl5(&self) -> ACIMPL5_R {
        ACIMPL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Analog Comparator 6 Implemented"]
    #[inline(always)]
    pub fn acimpl6(&self) -> ACIMPL6_R {
        ACIMPL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator 7 Implemented"]
    #[inline(always)]
    pub fn acimpl7(&self) -> ACIMPL7_R {
        ACIMPL7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Window0 Mode Implemented"]
    #[inline(always)]
    pub fn wimpl0(&self) -> WIMPL0_R {
        WIMPL0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Window1 Mode Implemented"]
    #[inline(always)]
    pub fn wimpl1(&self) -> WIMPL1_R {
        WIMPL1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Window2 Mode Implemented"]
    #[inline(always)]
    pub fn wimpl2(&self) -> WIMPL2_R {
        WIMPL2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Window3 Mode Implemented"]
    #[inline(always)]
    pub fn wimpl3(&self) -> WIMPL3_R {
        WIMPL3_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [parameter](index.html) module"]
pub struct PARAMETER_SPEC;
impl crate::RegisterSpec for PARAMETER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [parameter::R](R) reader structure"]
impl crate::Readable for PARAMETER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAMETER to value 0"]
impl crate::Resettable for PARAMETER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
