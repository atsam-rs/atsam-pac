#[doc = "Register `IER%s` writer"]
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COVFSSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the Counter Overflow Interrupt."]
    _1 = 1,
}
impl From<COVFSSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: COVFSSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COVFS` writer - Counter Overflow"]
pub type COVFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, COVFSSELECT_AW, O>;
impl<'a, const O: u8> COVFS_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COVFSSELECT_AW::_0)
    }
    #[doc = "Enables the Counter Overflow Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COVFSSELECT_AW::_1)
    }
}
#[doc = "Load Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOVRSSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the Load Overrun Interrupt."]
    _1 = 1,
}
impl From<LOVRSSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: LOVRSSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOVRS` writer - Load Overrun"]
pub type LOVRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, LOVRSSELECT_AW, O>;
impl<'a, const O: u8> LOVRS_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOVRSSELECT_AW::_0)
    }
    #[doc = "Enables the Load Overrun Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOVRSSELECT_AW::_1)
    }
}
#[doc = "RA Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPASSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the RA Compare Interrupt."]
    _1 = 1,
}
impl From<CPASSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CPASSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPAS` writer - RA Compare"]
pub type CPAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, CPASSELECT_AW, O>;
impl<'a, const O: u8> CPAS_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPASSELECT_AW::_0)
    }
    #[doc = "Enables the RA Compare Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPASSELECT_AW::_1)
    }
}
#[doc = "RB Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPBSSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the RB Compare Interrupt."]
    _1 = 1,
}
impl From<CPBSSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CPBSSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPBS` writer - RB Compare"]
pub type CPBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, CPBSSELECT_AW, O>;
impl<'a, const O: u8> CPBS_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPBSSELECT_AW::_0)
    }
    #[doc = "Enables the RB Compare Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPBSSELECT_AW::_1)
    }
}
#[doc = "RC Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPCSSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the RC Compare Interrupt."]
    _1 = 1,
}
impl From<CPCSSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CPCSSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPCS` writer - RC Compare"]
pub type CPCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, CPCSSELECT_AW, O>;
impl<'a, const O: u8> CPCS_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPCSSELECT_AW::_0)
    }
    #[doc = "Enables the RC Compare Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPCSSELECT_AW::_1)
    }
}
#[doc = "RA Loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDRASSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the RA Load Interrupt."]
    _1 = 1,
}
impl From<LDRASSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: LDRASSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDRAS` writer - RA Loading"]
pub type LDRAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, LDRASSELECT_AW, O>;
impl<'a, const O: u8> LDRAS_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDRASSELECT_AW::_0)
    }
    #[doc = "Enables the RA Load Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDRASSELECT_AW::_1)
    }
}
#[doc = "RB Loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDRBSSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the RB Load Interrupt."]
    _1 = 1,
}
impl From<LDRBSSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: LDRBSSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDRBS` writer - RB Loading"]
pub type LDRBS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, LDRBSSELECT_AW, O>;
impl<'a, const O: u8> LDRBS_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDRBSSELECT_AW::_0)
    }
    #[doc = "Enables the RB Load Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDRBSSELECT_AW::_1)
    }
}
#[doc = "External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETRGSSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the External Trigger Interrupt."]
    _1 = 1,
}
impl From<ETRGSSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ETRGSSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ETRGS` writer - External Trigger"]
pub type ETRGS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ETRGSSELECT_AW, O>;
impl<'a, const O: u8> ETRGS_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETRGSSELECT_AW::_0)
    }
    #[doc = "Enables the External Trigger Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETRGSSELECT_AW::_1)
    }
}
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
#[doc = "Interrupt Enable Register Channel\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ier::W](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IER%s to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
