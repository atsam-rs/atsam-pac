#[doc = "Register `VER` reader"]
pub struct R(crate::R<VER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NVTIM` reader - Non-valid Time"]
pub struct NVTIM_R(crate::FieldReader<bool, bool>);
impl NVTIM_R {
    pub(crate) fn new(bits: bool) -> Self {
        NVTIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVTIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVCAL` reader - Non-valid Calendar"]
pub struct NVCAL_R(crate::FieldReader<bool, bool>);
impl NVCAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        NVCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVCAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVTIMALR` reader - Non-valid Time Alarm"]
pub struct NVTIMALR_R(crate::FieldReader<bool, bool>);
impl NVTIMALR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NVTIMALR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVTIMALR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NVCALALR` reader - Non-valid Calendar Alarm"]
pub struct NVCALALR_R(crate::FieldReader<bool, bool>);
impl NVCALALR_R {
    pub(crate) fn new(bits: bool) -> Self {
        NVCALALR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NVCALALR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Non-valid Time"]
    #[inline(always)]
    pub fn nvtim(&self) -> NVTIM_R {
        NVTIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Non-valid Calendar"]
    #[inline(always)]
    pub fn nvcal(&self) -> NVCAL_R {
        NVCAL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Non-valid Time Alarm"]
    #[inline(always)]
    pub fn nvtimalr(&self) -> NVTIMALR_R {
        NVTIMALR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Non-valid Calendar Alarm"]
    #[inline(always)]
    pub fn nvcalalr(&self) -> NVCALALR_R {
        NVCALALR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "Valid Entry Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ver](index.html) module"]
pub struct VER_SPEC;
impl crate::RegisterSpec for VER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ver::R](R) reader structure"]
impl crate::Readable for VER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets VER to value 0"]
impl crate::Resettable for VER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
