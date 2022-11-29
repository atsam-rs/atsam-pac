#[doc = "Register `UDINTESET` writer"]
pub struct W(crate::W<UDINTESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDINTESET_SPEC>;
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
impl From<crate::W<UDINTESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDINTESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPES` writer - SUSP Interrupt Enable Set"]
pub type SUSPES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `MSOFES` writer - MSOF Interrupt Enable Set"]
pub type MSOFES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `SOFES` writer - SOF Interrupt Enable Set"]
pub type SOFES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `EORSTES` writer - EORST Interrupt Enable Set"]
pub type EORSTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `WAKEUPES` writer - WAKEUP Interrupt Enable Set"]
pub type WAKEUPES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `EORSMES` writer - EORSM Interrupt Enable Set"]
pub type EORSMES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `UPRSMES` writer - UPRSM Interrupt Enable Set"]
pub type UPRSMES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `EP0INTES` writer - EP0INT Interrupt Enable Set"]
pub type EP0INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `EP1INTES` writer - EP1INT Interrupt Enable Set"]
pub type EP1INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `EP2INTES` writer - EP2INT Interrupt Enable Set"]
pub type EP2INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `EP3INTES` writer - EP3INT Interrupt Enable Set"]
pub type EP3INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `EP4INTES` writer - EP4INT Interrupt Enable Set"]
pub type EP4INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `EP5INTES` writer - EP5INT Interrupt Enable Set"]
pub type EP5INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `EP6INTES` writer - EP6INT Interrupt Enable Set"]
pub type EP6INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
#[doc = "Field `EP7INTES` writer - EP7INT Interrupt Enable Set"]
pub type EP7INTES_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTESET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - SUSP Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn suspes(&mut self) -> SUSPES_W<0> {
        SUSPES_W::new(self)
    }
    #[doc = "Bit 1 - MSOF Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn msofes(&mut self) -> MSOFES_W<1> {
        MSOFES_W::new(self)
    }
    #[doc = "Bit 2 - SOF Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn sofes(&mut self) -> SOFES_W<2> {
        SOFES_W::new(self)
    }
    #[doc = "Bit 3 - EORST Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn eorstes(&mut self) -> EORSTES_W<3> {
        EORSTES_W::new(self)
    }
    #[doc = "Bit 4 - WAKEUP Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupes(&mut self) -> WAKEUPES_W<4> {
        WAKEUPES_W::new(self)
    }
    #[doc = "Bit 5 - EORSM Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn eorsmes(&mut self) -> EORSMES_W<5> {
        EORSMES_W::new(self)
    }
    #[doc = "Bit 6 - UPRSM Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn uprsmes(&mut self) -> UPRSMES_W<6> {
        UPRSMES_W::new(self)
    }
    #[doc = "Bit 12 - EP0INT Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ep0intes(&mut self) -> EP0INTES_W<12> {
        EP0INTES_W::new(self)
    }
    #[doc = "Bit 13 - EP1INT Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ep1intes(&mut self) -> EP1INTES_W<13> {
        EP1INTES_W::new(self)
    }
    #[doc = "Bit 14 - EP2INT Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ep2intes(&mut self) -> EP2INTES_W<14> {
        EP2INTES_W::new(self)
    }
    #[doc = "Bit 15 - EP3INT Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ep3intes(&mut self) -> EP3INTES_W<15> {
        EP3INTES_W::new(self)
    }
    #[doc = "Bit 16 - EP4INT Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ep4intes(&mut self) -> EP4INTES_W<16> {
        EP4INTES_W::new(self)
    }
    #[doc = "Bit 17 - EP5INT Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ep5intes(&mut self) -> EP5INTES_W<17> {
        EP5INTES_W::new(self)
    }
    #[doc = "Bit 18 - EP6INT Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ep6intes(&mut self) -> EP6INTES_W<18> {
        EP6INTES_W::new(self)
    }
    #[doc = "Bit 19 - EP7INT Interrupt Enable Set"]
    #[inline(always)]
    #[must_use]
    pub fn ep7intes(&mut self) -> EP7INTES_W<19> {
        EP7INTES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Global Interrupt Enable Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udinteset](index.html) module"]
pub struct UDINTESET_SPEC;
impl crate::RegisterSpec for UDINTESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [udinteset::W](W) writer structure"]
impl crate::Writable for UDINTESET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDINTESET to value 0"]
impl crate::Resettable for UDINTESET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
