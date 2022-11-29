#[doc = "Register `UDINTSET` writer"]
pub struct W(crate::W<UDINTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDINTSET_SPEC>;
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
impl From<crate::W<UDINTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDINTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUSPS` writer - SUSP Interrupt Set"]
pub type SUSPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTSET_SPEC, bool, O>;
#[doc = "Field `MSOFS` writer - MSOF Interrupt Set"]
pub type MSOFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTSET_SPEC, bool, O>;
#[doc = "Field `SOFS` writer - SOF Interrupt Set"]
pub type SOFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTSET_SPEC, bool, O>;
#[doc = "Field `EORSTS` writer - EORST Interrupt Set"]
pub type EORSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTSET_SPEC, bool, O>;
#[doc = "Field `WAKEUPS` writer - WAKEUP Interrupt Set"]
pub type WAKEUPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTSET_SPEC, bool, O>;
#[doc = "Field `EORSMS` writer - EORSM Interrupt Set"]
pub type EORSMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTSET_SPEC, bool, O>;
#[doc = "Field `UPRSMS` writer - UPRSM Interrupt Set"]
pub type UPRSMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, UDINTSET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - SUSP Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn susps(&mut self) -> SUSPS_W<0> {
        SUSPS_W::new(self)
    }
    #[doc = "Bit 1 - MSOF Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn msofs(&mut self) -> MSOFS_W<1> {
        MSOFS_W::new(self)
    }
    #[doc = "Bit 2 - SOF Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn sofs(&mut self) -> SOFS_W<2> {
        SOFS_W::new(self)
    }
    #[doc = "Bit 3 - EORST Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn eorsts(&mut self) -> EORSTS_W<3> {
        EORSTS_W::new(self)
    }
    #[doc = "Bit 4 - WAKEUP Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn wakeups(&mut self) -> WAKEUPS_W<4> {
        WAKEUPS_W::new(self)
    }
    #[doc = "Bit 5 - EORSM Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn eorsms(&mut self) -> EORSMS_W<5> {
        EORSMS_W::new(self)
    }
    #[doc = "Bit 6 - UPRSM Interrupt Set"]
    #[inline(always)]
    #[must_use]
    pub fn uprsms(&mut self) -> UPRSMS_W<6> {
        UPRSMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Global Interrupt Set Regsiter\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udintset](index.html) module"]
pub struct UDINTSET_SPEC;
impl crate::RegisterSpec for UDINTSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [udintset::W](W) writer structure"]
impl crate::Writable for UDINTSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UDINTSET to value 0"]
impl crate::Resettable for UDINTSET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
