#[doc = "Register `WPSR` reader"]
pub struct R(crate::R<WPSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WPSR_SPEC>> for R {
    fn from(reader: crate::R<WPSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WPROTERR` reader - Write Protection Error"]
pub struct WPROTERR_R(crate::FieldReader<bool, bool>);
impl WPROTERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPROTERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPROTERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPROTADDR` reader - Write Protection Error Address"]
pub struct WPROTADDR_R(crate::FieldReader<u8, u8>);
impl WPROTADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        WPROTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPROTADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Write Protection Error"]
    #[inline(always)]
    pub fn wproterr(&self) -> WPROTERR_R {
        WPROTERR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Write Protection Error Address"]
    #[inline(always)]
    pub fn wprotaddr(&self) -> WPROTADDR_R {
        WPROTADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Write Protect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsr](index.html) module"]
pub struct WPSR_SPEC;
impl crate::RegisterSpec for WPSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpsr::R](R) reader structure"]
impl crate::Readable for WPSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WPSR to value 0"]
impl crate::Resettable for WPSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
