#[doc = "Register `CMCFG` reader"]
pub struct R(crate::R<CMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMCFG` writer"]
pub struct W(crate::W<CMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMCFG_SPEC>;
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
impl From<crate::W<CMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DREV` reader - Digit Reverse Mode"]
pub type DREV_R = crate::BitReader<bool>;
#[doc = "Field `DREV` writer - Digit Reverse Mode"]
pub type DREV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMCFG_SPEC, bool, O>;
#[doc = "Field `TDG` reader - Type of Digit"]
pub type TDG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDG` writer - Type of Digit"]
pub type TDG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMCFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `STSEG` reader - Start Segment"]
pub type STSEG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STSEG` writer - Start Segment"]
pub type STSEG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CMCFG_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - Digit Reverse Mode"]
    #[inline(always)]
    pub fn drev(&self) -> DREV_R {
        DREV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Type of Digit"]
    #[inline(always)]
    pub fn tdg(&self) -> TDG_R {
        TDG_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 8:13 - Start Segment"]
    #[inline(always)]
    pub fn stseg(&self) -> STSEG_R {
        STSEG_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Digit Reverse Mode"]
    #[inline(always)]
    #[must_use]
    pub fn drev(&mut self) -> DREV_W<0> {
        DREV_W::new(self)
    }
    #[doc = "Bits 1:2 - Type of Digit"]
    #[inline(always)]
    #[must_use]
    pub fn tdg(&mut self) -> TDG_W<1> {
        TDG_W::new(self)
    }
    #[doc = "Bits 8:13 - Start Segment"]
    #[inline(always)]
    #[must_use]
    pub fn stseg(&mut self) -> STSEG_W<8> {
        STSEG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Character Mapping Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmcfg](index.html) module"]
pub struct CMCFG_SPEC;
impl crate::RegisterSpec for CMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmcfg::R](R) reader structure"]
impl crate::Readable for CMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmcfg::W](W) writer structure"]
impl crate::Writable for CMCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMCFG to value 0"]
impl crate::Resettable for CMCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
