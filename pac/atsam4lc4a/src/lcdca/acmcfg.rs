#[doc = "Register `ACMCFG` reader"]
pub struct R(crate::R<ACMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACMCFG` writer"]
pub struct W(crate::W<ACMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACMCFG_SPEC>;
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
impl From<crate::W<ACMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMCFG_SPEC, bool, O>;
#[doc = "Field `FCS` reader - Frame Counter Selection"]
pub type FCS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCS` writer - Frame Counter Selection"]
pub type FCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACMCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE` reader - Mode (sequential or scrolling)"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - Mode (sequential or scrolling)"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMCFG_SPEC, bool, O>;
#[doc = "Field `DREV` reader - Digit Reverse"]
pub type DREV_R = crate::BitReader<bool>;
#[doc = "Field `DREV` writer - Digit Reverse"]
pub type DREV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACMCFG_SPEC, bool, O>;
#[doc = "Field `TDG` reader - Type of Digit"]
pub type TDG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDG` writer - Type of Digit"]
pub type TDG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACMCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `STSEG` reader - Start Segment"]
pub type STSEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STSEG` writer - Start Segment"]
pub type STSEG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACMCFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `STEPS` reader - Scrolling Steps"]
pub type STEPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STEPS` writer - Scrolling Steps"]
pub type STEPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACMCFG_SPEC, u8, u8, 8, O>;
#[doc = "Field `DIGN` reader - Digit Number"]
pub type DIGN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIGN` writer - Digit Number"]
pub type DIGN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACMCFG_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Mode (sequential or scrolling)"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Digit Reverse"]
    #[inline(always)]
    pub fn drev(&self) -> DREV_R {
        DREV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Type of Digit"]
    #[inline(always)]
    pub fn tdg(&self) -> TDG_R {
        TDG_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Start Segment"]
    #[inline(always)]
    pub fn stseg(&self) -> STSEG_R {
        STSEG_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:23 - Scrolling Steps"]
    #[inline(always)]
    pub fn steps(&self) -> STEPS_R {
        STEPS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Digit Number"]
    #[inline(always)]
    pub fn dign(&self) -> DIGN_R {
        DIGN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn fcs(&mut self) -> FCS_W<1> {
        FCS_W::new(self)
    }
    #[doc = "Bit 3 - Mode (sequential or scrolling)"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<3> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - Digit Reverse"]
    #[inline(always)]
    #[must_use]
    pub fn drev(&mut self) -> DREV_W<4> {
        DREV_W::new(self)
    }
    #[doc = "Bits 5:6 - Type of Digit"]
    #[inline(always)]
    #[must_use]
    pub fn tdg(&mut self) -> TDG_W<5> {
        TDG_W::new(self)
    }
    #[doc = "Bits 8:13 - Start Segment"]
    #[inline(always)]
    #[must_use]
    pub fn stseg(&mut self) -> STSEG_W<8> {
        STSEG_W::new(self)
    }
    #[doc = "Bits 16:23 - Scrolling Steps"]
    #[inline(always)]
    #[must_use]
    pub fn steps(&mut self) -> STEPS_W<16> {
        STEPS_W::new(self)
    }
    #[doc = "Bits 24:27 - Digit Number"]
    #[inline(always)]
    #[must_use]
    pub fn dign(&mut self) -> DIGN_W<24> {
        DIGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Automated Character Mapping Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acmcfg](index.html) module"]
pub struct ACMCFG_SPEC;
impl crate::RegisterSpec for ACMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acmcfg::R](R) reader structure"]
impl crate::Readable for ACMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acmcfg::W](W) writer structure"]
impl crate::Writable for ACMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACMCFG to value 0"]
impl crate::Resettable for ACMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
