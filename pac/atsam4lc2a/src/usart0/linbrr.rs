#[doc = "Register `LINBRR` reader"]
pub struct R(crate::R<LINBRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINBRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LINBRR_SPEC>> for R {
    fn from(reader: crate::R<LINBRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LINCD` reader - Clock Divider after Synchronization"]
pub struct LINCD_R(crate::FieldReader<u16, u16>);
impl LINCD_R {
    pub(crate) fn new(bits: u16) -> Self {
        LINCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINCD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINFP` reader - Fractional Part after Synchronization"]
pub struct LINFP_R(crate::FieldReader<u8, u8>);
impl LINFP_R {
    pub(crate) fn new(bits: u8) -> Self {
        LINFP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINFP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Clock Divider after Synchronization"]
    #[inline(always)]
    pub fn lincd(&self) -> LINCD_R {
        LINCD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:18 - Fractional Part after Synchronization"]
    #[inline(always)]
    pub fn linfp(&self) -> LINFP_R {
        LINFP_R::new(((self.bits >> 16) & 0x07) as u8)
    }
}
#[doc = "LIN Baud Rate Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linbrr](index.html) module"]
pub struct LINBRR_SPEC;
impl crate::RegisterSpec for LINBRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [linbrr::R](R) reader structure"]
impl crate::Readable for LINBRR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LINBRR to value 0"]
impl crate::Resettable for LINBRR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
