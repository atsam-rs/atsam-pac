#[doc = "Register `ACMDR` writer"]
pub struct W(crate::W<ACMDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMDR_SPEC>;
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
impl From<crate::W<ACMDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACMDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ASCII` writer - ASCII Code"]
pub struct ASCII_W<'a> {
    w: &'a mut W,
}
impl<'a> ASCII_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:6 - ASCII Code"]
    #[inline(always)]
    pub fn ascii(&mut self) -> ASCII_W {
        ASCII_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automated Character Mapping Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmdr](index.html) module"]
pub struct ACMDR_SPEC;
impl crate::RegisterSpec for ACMDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [acmdr::W](W) writer structure"]
impl crate::Writable for ACMDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACMDR to value 0"]
impl crate::Resettable for ACMDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
