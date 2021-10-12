#[doc = "Register `RXERRCNT` reader"]
pub struct R(crate::R<RXERRCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXERRCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXERRCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXERRCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXERRCNT` reader - Receive Error Count"]
pub struct RXERRCNT_R(crate::FieldReader<u8, u8>);
impl RXERRCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXERRCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXERRCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive Error Count"]
    #[inline(always)]
    pub fn rxerrcnt(&self) -> RXERRCNT_R {
        RXERRCNT_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "USART_EXT Receive Error Count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxerrcnt](index.html) module"]
pub struct RXERRCNT_SPEC;
impl crate::RegisterSpec for RXERRCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rxerrcnt::R](R) reader structure"]
impl crate::Readable for RXERRCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXERRCNT to value 0"]
impl crate::Resettable for RXERRCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
