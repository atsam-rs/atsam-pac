#[doc = "Register `UDESC` reader"]
pub struct R(crate::R<UDESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDESC` writer"]
pub struct W(crate::W<UDESC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDESC_SPEC>;
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
impl From<crate::W<UDESC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDESC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDESCA` reader - USB Descriptor Address"]
pub type UDESCA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `UDESCA` writer - USB Descriptor Address"]
pub type UDESCA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UDESC_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - USB Descriptor Address"]
    #[inline(always)]
    pub fn udesca(&self) -> UDESCA_R {
        UDESCA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USB Descriptor Address"]
    #[inline(always)]
    #[must_use]
    pub fn udesca(&mut self) -> UDESCA_W<0> {
        UDESCA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint descriptor table\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udesc](index.html) module"]
pub struct UDESC_SPEC;
impl crate::RegisterSpec for UDESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [udesc::R](R) reader structure"]
impl crate::Readable for UDESC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udesc::W](W) writer structure"]
impl crate::Writable for UDESC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDESC to value 0"]
impl crate::Resettable for UDESC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
