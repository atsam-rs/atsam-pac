#[doc = "Register `IFR` reader"]
pub struct R(crate::R<IFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFR` writer"]
pub struct W(crate::W<IFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFR_SPEC>;
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
impl From<crate::W<IFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRDA_FILTER` reader - Irda filter"]
pub type IRDA_FILTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRDA_FILTER` writer - Irda filter"]
pub type IRDA_FILTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IFR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Irda filter"]
    #[inline(always)]
    pub fn irda_filter(&self) -> IRDA_FILTER_R {
        IRDA_FILTER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Irda filter"]
    #[inline(always)]
    #[must_use]
    pub fn irda_filter(&mut self) -> IRDA_FILTER_W<0> {
        IRDA_FILTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IrDA Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifr](index.html) module"]
pub struct IFR_SPEC;
impl crate::RegisterSpec for IFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifr::R](R) reader structure"]
impl crate::Readable for IFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifr::W](W) writer structure"]
impl crate::Writable for IFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFR to value 0"]
impl crate::Resettable for IFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
