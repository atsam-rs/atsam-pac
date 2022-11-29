#[doc = "Register `PTSR1` reader"]
pub struct R(crate::R<PTSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXTEN` reader - Receiver Transfer Enable"]
pub type RXTEN_R = crate::BitReader<bool>;
#[doc = "Field `TXTEN` reader - Transmitter Transfer Enable"]
pub type TXTEN_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Receiver Transfer Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RXTEN_R {
        RXTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Transmitter Transfer Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TXTEN_R {
        TXTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Transfer Status Register (pdc = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr1](index.html) module"]
pub struct PTSR1_SPEC;
impl crate::RegisterSpec for PTSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr1::R](R) reader structure"]
impl crate::Readable for PTSR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTSR1 to value 0"]
impl crate::Resettable for PTSR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
