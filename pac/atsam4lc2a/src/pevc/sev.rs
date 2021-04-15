#[doc = "Register `SEV` writer"]
pub struct W(crate::W<SEV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEV_SPEC>;
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
impl core::convert::From<crate::W<SEV_SPEC>> for W {
    fn from(writer: crate::W<SEV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEV` writer - Software Event"]
pub struct SEV_W<'a> {
    w: &'a mut W,
}
impl<'a> SEV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Software Event"]
    #[inline(always)]
    pub fn sev(&mut self) -> SEV_W {
        SEV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Event\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sev](index.html) module"]
pub struct SEV_SPEC;
impl crate::RegisterSpec for SEV_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sev::W](W) writer structure"]
impl crate::Writable for SEV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEV to value 0"]
impl crate::Resettable for SEV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
