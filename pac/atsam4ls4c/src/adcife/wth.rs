#[doc = "Register `WTH` reader"]
pub struct R(crate::R<WTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WTH` writer"]
pub struct W(crate::W<WTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WTH_SPEC>;
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
impl From<crate::W<WTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LT` reader - Low threshold"]
pub type LT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LT` writer - Low threshold"]
pub type LT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WTH_SPEC, u16, u16, 12, O>;
#[doc = "Field `HT` reader - High Threshold"]
pub type HT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HT` writer - High Threshold"]
pub type HT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WTH_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Low threshold"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Low threshold"]
    #[inline(always)]
    #[must_use]
    pub fn lt(&mut self) -> LT_W<0> {
        LT_W::new(self)
    }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<16> {
        HT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window Monitor Threshold Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wth](index.html) module"]
pub struct WTH_SPEC;
impl crate::RegisterSpec for WTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wth::R](R) reader structure"]
impl crate::Readable for WTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wth::W](W) writer structure"]
impl crate::Writable for WTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WTH to value 0"]
impl crate::Resettable for WTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
