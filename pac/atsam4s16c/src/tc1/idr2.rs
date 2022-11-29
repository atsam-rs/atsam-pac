#[doc = "Register `IDR2` writer"]
pub struct W(crate::W<IDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR2_SPEC>;
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
impl From<crate::W<IDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COVFS` writer - Counter Overflow"]
pub type COVFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR2_SPEC, bool, O>;
#[doc = "Field `LOVRS` writer - Load Overrun"]
pub type LOVRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR2_SPEC, bool, O>;
#[doc = "Field `CPAS` writer - RA Compare"]
pub type CPAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR2_SPEC, bool, O>;
#[doc = "Field `CPBS` writer - RB Compare"]
pub type CPBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR2_SPEC, bool, O>;
#[doc = "Field `CPCS` writer - RC Compare"]
pub type CPCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR2_SPEC, bool, O>;
#[doc = "Field `LDRAS` writer - RA Loading"]
pub type LDRAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR2_SPEC, bool, O>;
#[doc = "Field `LDRBS` writer - RB Loading"]
pub type LDRBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR2_SPEC, bool, O>;
#[doc = "Field `ETRGS` writer - External Trigger"]
pub type ETRGS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDR2_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn covfs(&mut self) -> COVFS_W<0> {
        COVFS_W::new(self)
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    #[must_use]
    pub fn lovrs(&mut self) -> LOVRS_W<1> {
        LOVRS_W::new(self)
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpas(&mut self) -> CPAS_W<2> {
        CPAS_W::new(self)
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpbs(&mut self) -> CPBS_W<3> {
        CPBS_W::new(self)
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    #[must_use]
    pub fn cpcs(&mut self) -> CPCS_W<4> {
        CPCS_W::new(self)
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldras(&mut self) -> LDRAS_W<5> {
        LDRAS_W::new(self)
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    #[must_use]
    pub fn ldrbs(&mut self) -> LDRBS_W<6> {
        LDRBS_W::new(self)
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn etrgs(&mut self) -> ETRGS_W<7> {
        ETRGS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Register (channel = 2)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr2](index.html) module"]
pub struct IDR2_SPEC;
impl crate::RegisterSpec for IDR2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idr2::W](W) writer structure"]
impl crate::Writable for IDR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
