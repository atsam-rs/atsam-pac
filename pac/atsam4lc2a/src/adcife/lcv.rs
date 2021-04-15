#[doc = "Register `LCV` reader"]
pub struct R(crate::R<LCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<LCV_SPEC>> for R {
    fn from(reader: crate::R<LCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LCV` reader - Last converted value"]
pub struct LCV_R(crate::FieldReader<u16, u16>);
impl LCV_R {
    pub(crate) fn new(bits: u16) -> Self {
        LCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCPC` reader - Last converted positive channel"]
pub struct LCPC_R(crate::FieldReader<u8, u8>);
impl LCPC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LCPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCPC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCNC` reader - Last converted negative channel"]
pub struct LCNC_R(crate::FieldReader<u8, u8>);
impl LCNC_R {
    pub(crate) fn new(bits: u8) -> Self {
        LCNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCNC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Last converted value"]
    #[inline(always)]
    pub fn lcv(&self) -> LCV_R {
        LCV_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Last converted positive channel"]
    #[inline(always)]
    pub fn lcpc(&self) -> LCPC_R {
        LCPC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Last converted negative channel"]
    #[inline(always)]
    pub fn lcnc(&self) -> LCNC_R {
        LCNC_R::new(((self.bits >> 20) & 0x07) as u8)
    }
}
#[doc = "Sequencer Last Converted Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcv](index.html) module"]
pub struct LCV_SPEC;
impl crate::RegisterSpec for LCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcv::R](R) reader structure"]
impl crate::Readable for LCV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LCV to value 0"]
impl crate::Resettable for LCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
