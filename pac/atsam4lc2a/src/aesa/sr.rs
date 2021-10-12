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
#[doc = "Field `ODATARDY` reader - Output Data Ready"]
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
#[doc = "Field `IBUFRDY` reader - Input Buffer Ready"]
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
    #[doc = "Bit 0 - Output Data Ready"]
    #[inline(always)]
    pub fn odatardy(&self) -> ODATARDY_R {
        ODATARDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input Buffer Ready"]
    #[inline(always)]
    pub fn ibufrdy(&self) -> IBUFRDY_R {
        IBUFRDY_R::new(((self.bits >> 16) & 0x01) != 0)
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
#[doc = "`reset()` method sets SR to value 0x0001_0000"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
