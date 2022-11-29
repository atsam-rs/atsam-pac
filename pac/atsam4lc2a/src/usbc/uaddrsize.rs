#[doc = "Register `UADDRSIZE` reader"]
pub struct R(crate::R<UADDRSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UADDRSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UADDRSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UADDRSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UADDRSIZE` reader - IP PB Address Size"]
pub type UADDRSIZE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - IP PB Address Size"]
    #[inline(always)]
    pub fn uaddrsize(&self) -> UADDRSIZE_R {
        UADDRSIZE_R::new(self.bits)
    }
}
#[doc = "IP PB address size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uaddrsize](index.html) module"]
pub struct UADDRSIZE_SPEC;
impl crate::RegisterSpec for UADDRSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uaddrsize::R](R) reader structure"]
impl crate::Readable for UADDRSIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UADDRSIZE to value 0x1000"]
impl crate::Resettable for UADDRSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
