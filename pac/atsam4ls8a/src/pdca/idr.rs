#[doc = "Register `IDR%s` writer"]
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
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
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCZ` writer - Reload Counter Zero"]
pub type RCZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `TRC` writer - Transfer Complete"]
pub type TRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
#[doc = "Field `TERR` writer - Transfer Error"]
pub type TERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Reload Counter Zero"]
    #[inline(always)]
    #[must_use]
    pub fn rcz(&mut self) -> RCZ_W<0> {
        RCZ_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn trc(&mut self) -> TRC_W<1> {
        TRC_W::new(self)
    }
    #[doc = "Bit 2 - Transfer Error"]
    #[inline(always)]
    #[must_use]
    pub fn terr(&mut self) -> TERR_W<2> {
        TERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](index.html) module"]
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr::W](W) writer structure"]
impl crate::Writable for IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDR%s to value 0"]
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
