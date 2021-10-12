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
#[doc = "Field `NPINS` reader - Number of Pins"]
pub struct NPINS_R(crate::FieldReader<u8, u8>);
impl NPINS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NPINS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPINS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSTATUS` reader - Number of Status bits"]
pub struct NSTATUS_R(crate::FieldReader<u8, u8>);
impl NSTATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSTATUS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRACTIONAL` reader - Number of Fractional bits"]
pub struct FRACTIONAL_R(crate::FieldReader<u8, u8>);
impl FRACTIONAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRACTIONAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRACTIONAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of Pins"]
    #[inline(always)]
    pub fn npins(&self) -> NPINS_R {
        NPINS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of Status bits"]
    #[inline(always)]
    pub fn nstatus(&self) -> NSTATUS_R {
        NSTATUS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Number of Fractional bits"]
    #[inline(always)]
    pub fn fractional(&self) -> FRACTIONAL_R {
        FRACTIONAL_R::new(((self.bits >> 16) & 0x0f) as u8)
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
