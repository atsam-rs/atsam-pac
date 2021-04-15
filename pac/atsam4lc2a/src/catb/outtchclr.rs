#[doc = "Register `OUTTCHCLR%s` writer"]
pub struct W(crate::W<OUTTCHCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTTCHCLR_SPEC>;
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
impl core::convert::From<crate::W<OUTTCHCLR_SPEC>> for W {
    fn from(writer: crate::W<OUTTCHCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUTTCHCLR` writer - Out of Touch"]
pub struct OUTTCHCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTCHCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Out of Touch"]
    #[inline(always)]
    pub fn outtchclr(&mut self) -> OUTTCHCLR_W {
        OUTTCHCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Out-of-Touch Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outtchclr](index.html) module"]
pub struct OUTTCHCLR_SPEC;
impl crate::RegisterSpec for OUTTCHCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [outtchclr::W](W) writer structure"]
impl crate::Writable for OUTTCHCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTTCHCLR%s to value 0"]
impl crate::Resettable for OUTTCHCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
