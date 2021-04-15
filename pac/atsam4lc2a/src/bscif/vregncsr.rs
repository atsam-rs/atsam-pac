#[doc = "Register `VREGNCSR` reader"]
pub struct R(crate::R<VREGNCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREGNCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<VREGNCSR_SPEC>> for R {
    fn from(reader: crate::R<VREGNCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREGNCSR` writer"]
pub struct W(crate::W<VREGNCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREGNCSR_SPEC>;
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
impl core::convert::From<crate::W<VREGNCSR_SPEC>> for W {
    fn from(writer: crate::W<VREGNCSR_SPEC>) -> Self {
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
#[doc = "Normal Mode Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vregncsr](index.html) module"]
pub struct VREGNCSR_SPEC;
impl crate::RegisterSpec for VREGNCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vregncsr::R](R) reader structure"]
impl crate::Readable for VREGNCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vregncsr::W](W) writer structure"]
impl crate::Writable for VREGNCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREGNCSR to value 0"]
impl crate::Resettable for VREGNCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
