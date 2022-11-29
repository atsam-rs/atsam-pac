#[doc = "Register `LCV` reader"]
pub struct R(crate::R<LCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LCV` reader - Last converted value"]
pub type LCV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LCPC` reader - Last converted positive channel"]
pub type LCPC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LCNC` reader - Last converted negative channel"]
pub type LCNC_R = crate::FieldReader<u8, u8>;
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
        LCNC_R::new(((self.bits >> 20) & 7) as u8)
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
    const RESET_VALUE: Self::Ux = 0;
}
