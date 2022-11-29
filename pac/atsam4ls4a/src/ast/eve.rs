#[doc = "Register `EVE` writer"]
pub struct W(crate::W<EVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVE_SPEC>;
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
impl From<crate::W<EVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF` writer - Overflow"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVE_SPEC, bool, O>;
#[doc = "Field `ALARM0` writer - Alarm 0"]
pub type ALARM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVE_SPEC, bool, O>;
#[doc = "Field `ALARM1` writer - Alarm 1"]
pub type ALARM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVE_SPEC, bool, O>;
#[doc = "Field `PER0` writer - Perioidc 0"]
pub type PER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVE_SPEC, bool, O>;
#[doc = "Field `PER1` writer - Periodic 1"]
pub type PER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EVE_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn ovf(&mut self) -> OVF_W<0> {
        OVF_W::new(self)
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    #[must_use]
    pub fn alarm0(&mut self) -> ALARM0_W<8> {
        ALARM0_W::new(self)
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    #[must_use]
    pub fn alarm1(&mut self) -> ALARM1_W<9> {
        ALARM1_W::new(self)
    }
    #[doc = "Bit 16 - Perioidc 0"]
    #[inline(always)]
    #[must_use]
    pub fn per0(&mut self) -> PER0_W<16> {
        PER0_W::new(self)
    }
    #[doc = "Bit 17 - Periodic 1"]
    #[inline(always)]
    #[must_use]
    pub fn per1(&mut self) -> PER1_W<17> {
        PER1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Event Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eve](index.html) module"]
pub struct EVE_SPEC;
impl crate::RegisterSpec for EVE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [eve::W](W) writer structure"]
impl crate::Writable for EVE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVE to value 0"]
impl crate::Resettable for EVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
