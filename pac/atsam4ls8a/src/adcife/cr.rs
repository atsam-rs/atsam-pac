#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` writer - Software reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TSTOP` writer - Internal timer stop bit"]
pub type TSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TSTART` writer - Internal timer start bit"]
pub type TSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `STRIG` writer - Sequencer trigger"]
pub type STRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `REFBUFEN` writer - Reference buffer enable"]
pub type REFBUFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `REFBUFDIS` writer - Reference buffer disable"]
pub type REFBUFDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EN` writer - ADCIFD enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DIS` writer - ADCIFD disable"]
pub type DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BGREQEN` writer - Bandgap buffer request enable"]
pub type BGREQEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BGREQDIS` writer - Bandgap buffer request disable"]
pub type BGREQDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Internal timer stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn tstop(&mut self) -> TSTOP_W<1> {
        TSTOP_W::new(self)
    }
    #[doc = "Bit 2 - Internal timer start bit"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TSTART_W<2> {
        TSTART_W::new(self)
    }
    #[doc = "Bit 3 - Sequencer trigger"]
    #[inline(always)]
    #[must_use]
    pub fn strig(&mut self) -> STRIG_W<3> {
        STRIG_W::new(self)
    }
    #[doc = "Bit 4 - Reference buffer enable"]
    #[inline(always)]
    #[must_use]
    pub fn refbufen(&mut self) -> REFBUFEN_W<4> {
        REFBUFEN_W::new(self)
    }
    #[doc = "Bit 5 - Reference buffer disable"]
    #[inline(always)]
    #[must_use]
    pub fn refbufdis(&mut self) -> REFBUFDIS_W<5> {
        REFBUFDIS_W::new(self)
    }
    #[doc = "Bit 8 - ADCIFD enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<8> {
        EN_W::new(self)
    }
    #[doc = "Bit 9 - ADCIFD disable"]
    #[inline(always)]
    #[must_use]
    pub fn dis(&mut self) -> DIS_W<9> {
        DIS_W::new(self)
    }
    #[doc = "Bit 10 - Bandgap buffer request enable"]
    #[inline(always)]
    #[must_use]
    pub fn bgreqen(&mut self) -> BGREQEN_W<10> {
        BGREQEN_W::new(self)
    }
    #[doc = "Bit 11 - Bandgap buffer request disable"]
    #[inline(always)]
    #[must_use]
    pub fn bgreqdis(&mut self) -> BGREQDIS_W<11> {
        BGREQDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
