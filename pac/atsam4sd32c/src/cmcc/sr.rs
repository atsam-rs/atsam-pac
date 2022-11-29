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
#[doc = "Field `CSTS` reader - Cache Controller Status"]
pub type CSTS_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Cache Controller Status"]
    #[inline(always)]
    pub fn csts(&self) -> CSTS_R {
        CSTS_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Cache Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
