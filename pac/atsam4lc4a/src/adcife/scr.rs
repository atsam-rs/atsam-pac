#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEOC` writer - Sequencer end of conversion"]
pub type SEOC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `LOVR` writer - Sequencer last converted value overrun"]
pub type LOVR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `WM` writer - Window monitor"]
pub type WM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SMTRG` writer - Sequencer missed trigger event"]
pub type SMTRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `SUTD` writer - Start-up time done"]
pub type SUTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
#[doc = "Field `TTO` writer - Timer time-out"]
pub type TTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Sequencer end of conversion"]
    #[inline(always)]
    #[must_use]
    pub fn seoc(&mut self) -> SEOC_W<0> {
        SEOC_W::new(self)
    }
    #[doc = "Bit 1 - Sequencer last converted value overrun"]
    #[inline(always)]
    #[must_use]
    pub fn lovr(&mut self) -> LOVR_W<1> {
        LOVR_W::new(self)
    }
    #[doc = "Bit 2 - Window monitor"]
    #[inline(always)]
    #[must_use]
    pub fn wm(&mut self) -> WM_W<2> {
        WM_W::new(self)
    }
    #[doc = "Bit 3 - Sequencer missed trigger event"]
    #[inline(always)]
    #[must_use]
    pub fn smtrg(&mut self) -> SMTRG_W<3> {
        SMTRG_W::new(self)
    }
    #[doc = "Bit 4 - Start-up time done"]
    #[inline(always)]
    #[must_use]
    pub fn sutd(&mut self) -> SUTD_W<4> {
        SUTD_W::new(self)
    }
    #[doc = "Bit 5 - Timer time-out"]
    #[inline(always)]
    #[must_use]
    pub fn tto(&mut self) -> TTO_W<5> {
        TTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
