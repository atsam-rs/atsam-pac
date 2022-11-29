#[doc = "Register `IER` writer"]
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
#[doc = "Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVFSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable Interrupt."]
    _1 = 1,
}
impl From<OVFSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: OVFSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVF` writer - Overflow"]
pub type OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, OVFSELECT_AW, O>;
impl<'a, const O: u8> OVF_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVFSELECT_AW::_0)
    }
    #[doc = "Enable Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVFSELECT_AW::_1)
    }
}
#[doc = "Alarm 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM0SELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<ALARM0SELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ALARM0SELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALARM0` writer - Alarm 0"]
pub type ALARM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ALARM0SELECT_AW, O>;
impl<'a, const O: u8> ALARM0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALARM0SELECT_AW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALARM0SELECT_AW::_1)
    }
}
#[doc = "Alarm 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALARM1SELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<ALARM1SELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: ALARM1SELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALARM1` writer - Alarm 1"]
pub type ALARM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, ALARM1SELECT_AW, O>;
impl<'a, const O: u8> ALARM1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALARM1SELECT_AW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALARM1SELECT_AW::_1)
    }
}
#[doc = "Periodic 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER0SELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<PER0SELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: PER0SELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER0` writer - Periodic 0"]
pub type PER0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, PER0SELECT_AW, O>;
impl<'a, const O: u8> PER0_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER0SELECT_AW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PER0SELECT_AW::_1)
    }
}
#[doc = "Periodic 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER1SELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<PER1SELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: PER1SELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PER1` writer - Periodic 1"]
pub type PER1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, PER1SELECT_AW, O>;
impl<'a, const O: u8> PER1_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER1SELECT_AW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PER1SELECT_AW::_1)
    }
}
#[doc = "AST Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum READYSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<READYSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: READYSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` writer - AST Ready"]
pub type READY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, READYSELECT_AW, O>;
impl<'a, const O: u8> READY_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(READYSELECT_AW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(READYSELECT_AW::_1)
    }
}
#[doc = "Clock Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKRDYSELECT_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<CLKRDYSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CLKRDYSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKRDY` writer - Clock Ready"]
pub type CLKRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, CLKRDYSELECT_AW, O>;
impl<'a, const O: u8> CLKRDY_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKRDYSELECT_AW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKRDYSELECT_AW::_1)
    }
}
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
    #[doc = "Bit 16 - Periodic 0"]
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
    #[doc = "Bit 25 - AST Ready"]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> READY_W<25> {
        READY_W::new(self)
    }
    #[doc = "Bit 29 - Clock Ready"]
    #[inline(always)]
    #[must_use]
    pub fn clkrdy(&mut self) -> CLKRDY_W<29> {
        CLKRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](index.html) module"]
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
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
