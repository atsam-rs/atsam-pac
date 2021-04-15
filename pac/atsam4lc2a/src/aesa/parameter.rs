#[doc = "Register `PARAMETER` reader"]
pub struct R(crate::R<PARAMETER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAMETER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PARAMETER_SPEC>> for R {
    fn from(reader: crate::R<PARAMETER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEYSIZE` reader - Maximum Key Size"]
pub struct KEYSIZE_R(crate::FieldReader<u8, u8>);
impl KEYSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        KEYSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OPMODE` reader - Maximum Number of Confidentiality Modes of Operation"]
pub struct OPMODE_R(crate::FieldReader<u8, u8>);
impl OPMODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        OPMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OPMODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRMEAS` reader - Countermeasures"]
pub struct CTRMEAS_R(crate::FieldReader<bool, bool>);
impl CTRMEAS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRMEAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRMEAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - Maximum Key Size"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:4 - Maximum Number of Confidentiality Modes of Operation"]
    #[inline(always)]
    pub fn opmode(&self) -> OPMODE_R {
        OPMODE_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Countermeasures"]
    #[inline(always)]
    pub fn ctrmeas(&self) -> CTRMEAS_R {
        CTRMEAS_R::new(((self.bits >> 8) & 0x01) != 0)
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
#[doc = "`reset()` method sets PARAMETER to value 0x0112"]
impl crate::Resettable for PARAMETER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0112
    }
}
