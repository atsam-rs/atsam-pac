#[doc = "Register `ABMDR` writer"]
pub struct W(crate::W<ABMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ABMDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<ABMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ABMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Segments Value"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ABMDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMASK` writer - Data Mask"]
pub type DMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ABMDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `OFF` writer - Byte Offset"]
pub type OFF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ABMDR_SPEC, u8, u8, 5, O>;
impl W {
    #[doc = "Bits 0:7 - Segments Value"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Bits 8:15 - Data Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dmask(&mut self) -> DMASK_W<8> {
        DMASK_W::new(self)
    }
    #[doc = "Bits 16:20 - Byte Offset"]
    #[inline(always)]
    #[must_use]
    pub fn off(&mut self) -> OFF_W<16> {
        OFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automated Bit Mapping Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abmdr](index.html) module"]
pub struct ABMDR_SPEC;
impl crate::RegisterSpec for ABMDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [abmdr::W](W) writer structure"]
impl crate::Writable for ABMDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ABMDR to value 0"]
impl crate::Resettable for ABMDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
