#[doc = "Register `IDR` reader"]
pub struct R(crate::R<IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<IDR_SPEC>> for R {
    fn from(reader: crate::R<IDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APIDV` reader - AP Identification Variant"]
pub struct APIDV_R(crate::FieldReader<u8, u8>);
impl APIDV_R {
    pub(crate) fn new(bits: u8) -> Self {
        APIDV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APIDV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `APID` reader - AP Identification"]
pub struct APID_R(crate::FieldReader<u8, u8>);
impl APID_R {
    pub(crate) fn new(bits: u8) -> Self {
        APID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for APID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLSS` reader - Class"]
pub struct CLSS_R(crate::FieldReader<bool, bool>);
impl CLSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IC` reader - JEP-106 Identity Code"]
pub struct IC_R(crate::FieldReader<u8, u8>);
impl IC_R {
    pub(crate) fn new(bits: u8) -> Self {
        IC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC` reader - JEP-106 Continuation Code"]
pub struct CC_R(crate::FieldReader<u8, u8>);
impl CC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVISION` reader - Revision"]
pub struct REVISION_R(crate::FieldReader<u8, u8>);
impl REVISION_R {
    pub(crate) fn new(bits: u8) -> Self {
        REVISION_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVISION_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - AP Identification Variant"]
    #[inline(always)]
    pub fn apidv(&self) -> APIDV_R {
        APIDV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AP Identification"]
    #[inline(always)]
    pub fn apid(&self) -> APID_R {
        APID_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Class"]
    #[inline(always)]
    pub fn clss(&self) -> CLSS_R {
        CLSS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:23 - JEP-106 Identity Code"]
    #[inline(always)]
    pub fn ic(&self) -> IC_R {
        IC_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - JEP-106 Continuation Code"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Revision"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "AP Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idr::R](R) reader structure"]
impl crate::Readable for IDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDR to value 0x003e_0000"]
impl crate::Resettable for IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x003e_0000
    }
}
