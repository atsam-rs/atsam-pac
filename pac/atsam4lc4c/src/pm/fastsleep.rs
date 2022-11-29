#[doc = "Register `FASTSLEEP` reader"]
pub struct R(crate::R<FASTSLEEP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FASTSLEEP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FASTSLEEP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FASTSLEEP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FASTSLEEP` writer"]
pub struct W(crate::W<FASTSLEEP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FASTSLEEP_SPEC>;
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
impl From<crate::W<FASTSLEEP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FASTSLEEP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC` reader - Oscillator"]
pub type OSC_R = crate::BitReader<bool>;
#[doc = "Field `OSC` writer - Oscillator"]
pub type OSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FASTSLEEP_SPEC, bool, O>;
#[doc = "Field `PLL` reader - PLL"]
pub type PLL_R = crate::BitReader<bool>;
#[doc = "Field `PLL` writer - PLL"]
pub type PLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FASTSLEEP_SPEC, bool, O>;
#[doc = "Field `FASTRCOSC` reader - RC80 or FLO"]
pub type FASTRCOSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FASTRCOSC` writer - RC80 or FLO"]
pub type FASTRCOSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FASTSLEEP_SPEC, u8, u8, 5, O>;
#[doc = "Field `DFLL` reader - DFLL"]
pub type DFLL_R = crate::BitReader<bool>;
#[doc = "Field `DFLL` writer - DFLL"]
pub type DFLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, FASTSLEEP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Oscillator"]
    #[inline(always)]
    pub fn osc(&self) -> OSC_R {
        OSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - PLL"]
    #[inline(always)]
    pub fn pll(&self) -> PLL_R {
        PLL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:20 - RC80 or FLO"]
    #[inline(always)]
    pub fn fastrcosc(&self) -> FASTRCOSC_R {
        FASTRCOSC_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - DFLL"]
    #[inline(always)]
    pub fn dfll(&self) -> DFLL_R {
        DFLL_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillator"]
    #[inline(always)]
    #[must_use]
    pub fn osc(&mut self) -> OSC_W<0> {
        OSC_W::new(self)
    }
    #[doc = "Bit 8 - PLL"]
    #[inline(always)]
    #[must_use]
    pub fn pll(&mut self) -> PLL_W<8> {
        PLL_W::new(self)
    }
    #[doc = "Bits 16:20 - RC80 or FLO"]
    #[inline(always)]
    #[must_use]
    pub fn fastrcosc(&mut self) -> FASTRCOSC_W<16> {
        FASTRCOSC_W::new(self)
    }
    #[doc = "Bit 24 - DFLL"]
    #[inline(always)]
    #[must_use]
    pub fn dfll(&mut self) -> DFLL_W<24> {
        DFLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fast Sleep Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fastsleep](index.html) module"]
pub struct FASTSLEEP_SPEC;
impl crate::RegisterSpec for FASTSLEEP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fastsleep::R](R) reader structure"]
impl crate::Readable for FASTSLEEP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fastsleep::W](W) writer structure"]
impl crate::Writable for FASTSLEEP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FASTSLEEP to value 0"]
impl crate::Resettable for FASTSLEEP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
