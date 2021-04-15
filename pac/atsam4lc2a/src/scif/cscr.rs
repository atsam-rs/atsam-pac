#[doc = "Register `CSCR` reader"]
pub struct R(crate::R<CSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CSCR_SPEC>> for R {
    fn from(reader: crate::R<CSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCR` writer"]
pub struct W(crate::W<CSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCR_SPEC>;
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
impl core::convert::From<crate::W<CSCR_SPEC>> for W {
    fn from(writer: crate::W<CSCR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Specific Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cscr](index.html) module"]
pub struct CSCR_SPEC;
impl crate::RegisterSpec for CSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cscr::R](R) reader structure"]
impl crate::Readable for CSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cscr::W](W) writer structure"]
impl crate::Writable for CSCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCR to value 0"]
impl crate::Resettable for CSCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
