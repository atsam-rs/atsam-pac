#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CANEN` reader - CAN Controller Enable"]
pub type CANEN_R = crate::BitReader<bool>;
#[doc = "Field `CANEN` writer - CAN Controller Enable"]
pub type CANEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `LPM` reader - Disable/Enable Low Power Mode"]
pub type LPM_R = crate::BitReader<bool>;
#[doc = "Field `LPM` writer - Disable/Enable Low Power Mode"]
pub type LPM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `ABM` reader - Disable/Enable Autobaud/Listen mode"]
pub type ABM_R = crate::BitReader<bool>;
#[doc = "Field `ABM` writer - Disable/Enable Autobaud/Listen mode"]
pub type ABM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `OVL` reader - Disable/Enable Overload Frame"]
pub type OVL_R = crate::BitReader<bool>;
#[doc = "Field `OVL` writer - Disable/Enable Overload Frame"]
pub type OVL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `TEOF` reader - Timestamp messages at each end of Frame"]
pub type TEOF_R = crate::BitReader<bool>;
#[doc = "Field `TEOF` writer - Timestamp messages at each end of Frame"]
pub type TEOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `TTM` reader - Disable/Enable Time Triggered Mode"]
pub type TTM_R = crate::BitReader<bool>;
#[doc = "Field `TTM` writer - Disable/Enable Time Triggered Mode"]
pub type TTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `TIMFRZ` reader - Enable Timer Freeze"]
pub type TIMFRZ_R = crate::BitReader<bool>;
#[doc = "Field `TIMFRZ` writer - Enable Timer Freeze"]
pub type TIMFRZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `DRPT` reader - Disable Repeat"]
pub type DRPT_R = crate::BitReader<bool>;
#[doc = "Field `DRPT` writer - Disable Repeat"]
pub type DRPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline(always)]
    pub fn canen(&self) -> CANEN_R {
        CANEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline(always)]
    pub fn abm(&self) -> ABM_R {
        ABM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline(always)]
    pub fn ovl(&self) -> OVL_R {
        OVL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline(always)]
    pub fn teof(&self) -> TEOF_R {
        TEOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline(always)]
    pub fn ttm(&self) -> TTM_R {
        TTM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline(always)]
    pub fn timfrz(&self) -> TIMFRZ_R {
        TIMFRZ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline(always)]
    pub fn drpt(&self) -> DRPT_R {
        DRPT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAN Controller Enable"]
    #[inline(always)]
    #[must_use]
    pub fn canen(&mut self) -> CANEN_W<0> {
        CANEN_W::new(self)
    }
    #[doc = "Bit 1 - Disable/Enable Low Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpm(&mut self) -> LPM_W<1> {
        LPM_W::new(self)
    }
    #[doc = "Bit 2 - Disable/Enable Autobaud/Listen mode"]
    #[inline(always)]
    #[must_use]
    pub fn abm(&mut self) -> ABM_W<2> {
        ABM_W::new(self)
    }
    #[doc = "Bit 3 - Disable/Enable Overload Frame"]
    #[inline(always)]
    #[must_use]
    pub fn ovl(&mut self) -> OVL_W<3> {
        OVL_W::new(self)
    }
    #[doc = "Bit 4 - Timestamp messages at each end of Frame"]
    #[inline(always)]
    #[must_use]
    pub fn teof(&mut self) -> TEOF_W<4> {
        TEOF_W::new(self)
    }
    #[doc = "Bit 5 - Disable/Enable Time Triggered Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ttm(&mut self) -> TTM_W<5> {
        TTM_W::new(self)
    }
    #[doc = "Bit 6 - Enable Timer Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn timfrz(&mut self) -> TIMFRZ_W<6> {
        TIMFRZ_W::new(self)
    }
    #[doc = "Bit 7 - Disable Repeat"]
    #[inline(always)]
    #[must_use]
    pub fn drpt(&mut self) -> DRPT_W<7> {
        DRPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
