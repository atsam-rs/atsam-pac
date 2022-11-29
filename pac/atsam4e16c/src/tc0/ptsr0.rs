#[doc = "Register `PTSR0` reader"]
pub struct R(crate::R<PTSR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PTSR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PTSR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PTSR0_SPEC>) -> Self {
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
#[doc = "Transfer Status Register (pdc = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptsr0](index.html) module"]
pub struct PTSR0_SPEC;
impl crate::RegisterSpec for PTSR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ptsr0::R](R) reader structure"]
impl crate::Readable for PTSR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PTSR0 to value 0"]
impl crate::Resettable for PTSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
