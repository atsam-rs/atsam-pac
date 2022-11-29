#[doc = "Register `RB%s` reader"]
pub struct R(crate::R<RB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RB%s` writer"]
pub struct W(crate::W<RB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RB_SPEC>;
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
impl From<crate::W<RB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB` reader - Register B"]
pub type RB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RB` writer - Register B"]
pub type RB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RB_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Register B"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Register B"]
    #[inline(always)]
    #[must_use]
    pub fn rb(&mut self) -> RB_W<0> {
        RB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register B Channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rb](index.html) module"]
pub struct RB_SPEC;
impl crate::RegisterSpec for RB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rb::R](R) reader structure"]
impl crate::Readable for RB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rb::W](W) writer structure"]
impl crate::Writable for RB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RB%s to value 0"]
impl crate::Resettable for RB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
