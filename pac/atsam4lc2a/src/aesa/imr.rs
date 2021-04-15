#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IMR_SPEC>> for R {
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ODATARDY` reader - Output Data Ready Interrupt Mask"]
pub struct ODATARDY_R(crate::FieldReader<bool, bool>);
impl ODATARDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        ODATARDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODATARDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBUFRDY` reader - Input Buffer Ready Interrupt Mask"]
pub struct IBUFRDY_R(crate::FieldReader<bool, bool>);
impl IBUFRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        IBUFRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBUFRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Output Data Ready Interrupt Mask"]
    #[inline(always)]
    pub fn odatardy(&self) -> ODATARDY_R {
        ODATARDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input Buffer Ready Interrupt Mask"]
    #[inline(always)]
    pub fn ibufrdy(&self) -> IBUFRDY_R {
        IBUFRDY_R::new(((self.bits >> 16) & 0x01) != 0)
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
