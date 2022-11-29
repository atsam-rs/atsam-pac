#[doc = "Register `DRH0` reader"]
pub struct R(crate::R<DRH0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRH0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRH0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRH0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRH0` writer"]
pub struct W(crate::W<DRH0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRH0_SPEC>;
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
impl From<crate::W<DRH0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRH0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Segments Value"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - Segments Value"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRH0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Segments Value"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Segments Value"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Register High 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [drh0](index.html) module"]
pub struct DRH0_SPEC;
impl crate::RegisterSpec for DRH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [drh0::R](R) reader structure"]
impl crate::Readable for DRH0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [drh0::W](W) writer structure"]
impl crate::Writable for DRH0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DRH0 to value 0"]
impl crate::Resettable for DRH0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
