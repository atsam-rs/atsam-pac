#[doc = "Register `UDINTECLR` writer"]
pub struct W(crate::W<UDINTECLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDINTECLR_SPEC>;
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
impl From<crate::W<UDINTECLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDINTECLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPEC` writer - SUSP Interrupt Enable Clear"]
pub type SUSPEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `MSOFEC` writer - MSOF Interrupt Enable Clear"]
pub type MSOFEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `SOFEC` writer - SOF Interrupt Enable Clear"]
pub type SOFEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `EORSTEC` writer - EORST Interrupt Enable Clear"]
pub type EORSTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `WAKEUPEC` writer - WAKEUP Interrupt Enable Clear"]
pub type WAKEUPEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `EORSMEC` writer - EORSM Interrupt Enable Clear"]
pub type EORSMEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `UPRSMEC` writer - UPRSM Interrupt Enable Clear"]
pub type UPRSMEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `EP0INTEC` writer - EP0INT Interrupt Enable Clear"]
pub type EP0INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `EP1INTEC` writer - EP1INT Interrupt Enable Clear"]
pub type EP1INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `EP2INTEC` writer - EP2INT Interrupt Enable Clear"]
pub type EP2INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `EP3INTEC` writer - EP3INT Interrupt Enable Clear"]
pub type EP3INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `EP4INTEC` writer - EP4INT Interrupt Enable Clear"]
pub type EP4INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `EP5INTEC` writer - EP5INT Interrupt Enable Clear"]
pub type EP5INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `EP6INTEC` writer - EP6INT Interrupt Enable Clear"]
pub type EP6INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
#[doc = "Field `EP7INTEC` writer - EP7INT Interrupt Enable Clear"]
pub type EP7INTEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTECLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - SUSP Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn suspec(&mut self) -> SUSPEC_W<0> {
        SUSPEC_W::new(self)
    }
    #[doc = "Bit 1 - MSOF Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn msofec(&mut self) -> MSOFEC_W<1> {
        MSOFEC_W::new(self)
    }
    #[doc = "Bit 2 - SOF Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn sofec(&mut self) -> SOFEC_W<2> {
        SOFEC_W::new(self)
    }
    #[doc = "Bit 3 - EORST Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eorstec(&mut self) -> EORSTEC_W<3> {
        EORSTEC_W::new(self)
    }
    #[doc = "Bit 4 - WAKEUP Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupec(&mut self) -> WAKEUPEC_W<4> {
        WAKEUPEC_W::new(self)
    }
    #[doc = "Bit 5 - EORSM Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn eorsmec(&mut self) -> EORSMEC_W<5> {
        EORSMEC_W::new(self)
    }
    #[doc = "Bit 6 - UPRSM Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn uprsmec(&mut self) -> UPRSMEC_W<6> {
        UPRSMEC_W::new(self)
    }
    #[doc = "Bit 12 - EP0INT Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ep0intec(&mut self) -> EP0INTEC_W<12> {
        EP0INTEC_W::new(self)
    }
    #[doc = "Bit 13 - EP1INT Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ep1intec(&mut self) -> EP1INTEC_W<13> {
        EP1INTEC_W::new(self)
    }
    #[doc = "Bit 14 - EP2INT Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ep2intec(&mut self) -> EP2INTEC_W<14> {
        EP2INTEC_W::new(self)
    }
    #[doc = "Bit 15 - EP3INT Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ep3intec(&mut self) -> EP3INTEC_W<15> {
        EP3INTEC_W::new(self)
    }
    #[doc = "Bit 16 - EP4INT Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ep4intec(&mut self) -> EP4INTEC_W<16> {
        EP4INTEC_W::new(self)
    }
    #[doc = "Bit 17 - EP5INT Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ep5intec(&mut self) -> EP5INTEC_W<17> {
        EP5INTEC_W::new(self)
    }
    #[doc = "Bit 18 - EP6INT Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ep6intec(&mut self) -> EP6INTEC_W<18> {
        EP6INTEC_W::new(self)
    }
    #[doc = "Bit 19 - EP7INT Interrupt Enable Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ep7intec(&mut self) -> EP7INTEC_W<19> {
        EP7INTEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Global Interrupt Enable Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udinteclr](index.html) module"]
pub struct UDINTECLR_SPEC;
impl crate::RegisterSpec for UDINTECLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [udinteclr::W](W) writer structure"]
impl crate::Writable for UDINTECLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDINTECLR to value 0"]
impl crate::Resettable for UDINTECLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
