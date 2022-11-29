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
#[doc = "Field `DIS` writer - Disable"]
pub type DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FC0DIS` writer - Frame Counter 0 Disable"]
pub type FC0DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FC0EN` writer - Frame Counter 0 Enable"]
pub type FC0EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FC1DIS` writer - Frame Counter 1 Disable"]
pub type FC1DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FC1EN` writer - Frame Counter 1 Enable"]
pub type FC1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FC2DIS` writer - Frame Counter 2 Disable"]
pub type FC2DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FC2EN` writer - Frame Counter 2 Enable"]
pub type FC2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CDM` writer - Clear Display Memory"]
pub type CDM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WDIS` writer - Wake up Disable"]
pub type WDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WEN` writer - Wake up Enable"]
pub type WEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BSTART` writer - Blinking Start"]
pub type BSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BSTOP` writer - Blinking Stop"]
pub type BSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CSTART` writer - Circular Shift Start"]
pub type CSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `CSTOP` writer - Circular Shift Stop"]
pub type CSTOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dis(&mut self) -> DIS_W<0> {
        DIS_W::new(self)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<1> {
        EN_W::new(self)
    }
    #[doc = "Bit 2 - Frame Counter 0 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fc0dis(&mut self) -> FC0DIS_W<2> {
        FC0DIS_W::new(self)
    }
    #[doc = "Bit 3 - Frame Counter 0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc0en(&mut self) -> FC0EN_W<3> {
        FC0EN_W::new(self)
    }
    #[doc = "Bit 4 - Frame Counter 1 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fc1dis(&mut self) -> FC1DIS_W<4> {
        FC1DIS_W::new(self)
    }
    #[doc = "Bit 5 - Frame Counter 1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc1en(&mut self) -> FC1EN_W<5> {
        FC1EN_W::new(self)
    }
    #[doc = "Bit 6 - Frame Counter 2 Disable"]
    #[inline(always)]
    #[must_use]
    pub fn fc2dis(&mut self) -> FC2DIS_W<6> {
        FC2DIS_W::new(self)
    }
    #[doc = "Bit 7 - Frame Counter 2 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fc2en(&mut self) -> FC2EN_W<7> {
        FC2EN_W::new(self)
    }
    #[doc = "Bit 8 - Clear Display Memory"]
    #[inline(always)]
    #[must_use]
    pub fn cdm(&mut self) -> CDM_W<8> {
        CDM_W::new(self)
    }
    #[doc = "Bit 9 - Wake up Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wdis(&mut self) -> WDIS_W<9> {
        WDIS_W::new(self)
    }
    #[doc = "Bit 10 - Wake up Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wen(&mut self) -> WEN_W<10> {
        WEN_W::new(self)
    }
    #[doc = "Bit 11 - Blinking Start"]
    #[inline(always)]
    #[must_use]
    pub fn bstart(&mut self) -> BSTART_W<11> {
        BSTART_W::new(self)
    }
    #[doc = "Bit 12 - Blinking Stop"]
    #[inline(always)]
    #[must_use]
    pub fn bstop(&mut self) -> BSTOP_W<12> {
        BSTOP_W::new(self)
    }
    #[doc = "Bit 13 - Circular Shift Start"]
    #[inline(always)]
    #[must_use]
    pub fn cstart(&mut self) -> CSTART_W<13> {
        CSTART_W::new(self)
    }
    #[doc = "Bit 14 - Circular Shift Stop"]
    #[inline(always)]
    #[must_use]
    pub fn cstop(&mut self) -> CSTOP_W<14> {
        CSTOP_W::new(self)
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
