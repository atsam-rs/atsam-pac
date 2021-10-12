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
pub struct ACIMPL0_R(crate::FieldReader<bool, bool>);
impl ACIMPL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACIMPL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACIMPL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACIMPL1` reader - Analog Comparator 1 Implemented"]
pub struct ACIMPL1_R(crate::FieldReader<bool, bool>);
impl ACIMPL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACIMPL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACIMPL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACIMPL2` reader - Analog Comparator 2 Implemented"]
pub struct ACIMPL2_R(crate::FieldReader<bool, bool>);
impl ACIMPL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACIMPL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACIMPL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACIMPL3` reader - Analog Comparator 3 Implemented"]
pub struct ACIMPL3_R(crate::FieldReader<bool, bool>);
impl ACIMPL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACIMPL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACIMPL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACIMPL4` reader - Analog Comparator 4 Implemented"]
pub struct ACIMPL4_R(crate::FieldReader<bool, bool>);
impl ACIMPL4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACIMPL4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACIMPL4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACIMPL5` reader - Analog Comparator 5 Implemented"]
pub struct ACIMPL5_R(crate::FieldReader<bool, bool>);
impl ACIMPL5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACIMPL5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACIMPL5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACIMPL6` reader - Analog Comparator 6 Implemented"]
pub struct ACIMPL6_R(crate::FieldReader<bool, bool>);
impl ACIMPL6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACIMPL6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACIMPL6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACIMPL7` reader - Analog Comparator 7 Implemented"]
pub struct ACIMPL7_R(crate::FieldReader<bool, bool>);
impl ACIMPL7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACIMPL7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACIMPL7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIMPL0` reader - Window0 Mode Implemented"]
pub struct WIMPL0_R(crate::FieldReader<bool, bool>);
impl WIMPL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIMPL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIMPL0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIMPL1` reader - Window1 Mode Implemented"]
pub struct WIMPL1_R(crate::FieldReader<bool, bool>);
impl WIMPL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIMPL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIMPL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIMPL2` reader - Window2 Mode Implemented"]
pub struct WIMPL2_R(crate::FieldReader<bool, bool>);
impl WIMPL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIMPL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIMPL2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIMPL3` reader - Window3 Mode Implemented"]
pub struct WIMPL3_R(crate::FieldReader<bool, bool>);
impl WIMPL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIMPL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIMPL3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Analog Comparator 0 Implemented"]
    #[inline(always)]
    pub fn acimpl0(&self) -> ACIMPL0_R {
        ACIMPL0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator 1 Implemented"]
    #[inline(always)]
    pub fn acimpl1(&self) -> ACIMPL1_R {
        ACIMPL1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Analog Comparator 2 Implemented"]
    #[inline(always)]
    pub fn acimpl2(&self) -> ACIMPL2_R {
        ACIMPL2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Analog Comparator 3 Implemented"]
    #[inline(always)]
    pub fn acimpl3(&self) -> ACIMPL3_R {
        ACIMPL3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Analog Comparator 4 Implemented"]
    #[inline(always)]
    pub fn acimpl4(&self) -> ACIMPL4_R {
        ACIMPL4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Analog Comparator 5 Implemented"]
    #[inline(always)]
    pub fn acimpl5(&self) -> ACIMPL5_R {
        ACIMPL5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Analog Comparator 6 Implemented"]
    #[inline(always)]
    pub fn acimpl6(&self) -> ACIMPL6_R {
        ACIMPL6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator 7 Implemented"]
    #[inline(always)]
    pub fn acimpl7(&self) -> ACIMPL7_R {
        ACIMPL7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Window0 Mode Implemented"]
    #[inline(always)]
    pub fn wimpl0(&self) -> WIMPL0_R {
        WIMPL0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Window1 Mode Implemented"]
    #[inline(always)]
    pub fn wimpl1(&self) -> WIMPL1_R {
        WIMPL1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Window2 Mode Implemented"]
    #[inline(always)]
    pub fn wimpl2(&self) -> WIMPL2_R {
        WIMPL2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Window3 Mode Implemented"]
    #[inline(always)]
    pub fn wimpl3(&self) -> WIMPL3_R {
        WIMPL3_R::new(((self.bits >> 19) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
