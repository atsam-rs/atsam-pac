#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XBIAS` reader - External Bias Generation"]
pub type XBIAS_R = crate::BitReader<bool>;
#[doc = "Field `XBIAS` writer - External Bias Generation"]
pub type XBIAS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `WMOD` reader - Waveform Mode"]
pub type WMOD_R = crate::BitReader<bool>;
#[doc = "Field `WMOD` writer - Waveform Mode"]
pub type WMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `BLANK` reader - Blank LCD"]
pub type BLANK_R = crate::BitReader<bool>;
#[doc = "Field `BLANK` writer - Blank LCD"]
pub type BLANK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `LOCK` reader - Lock"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `DUTY` reader - Duty Select"]
pub type DUTY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DUTY` writer - Duty Select"]
pub type DUTY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 2, O>;
#[doc = "Field `FCST` reader - Fine Contrast"]
pub type FCST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FCST` writer - Fine Contrast"]
pub type FCST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `NSU` reader - Number of Segment Terminals in Use"]
pub type NSU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NSU` writer - Number of Segment Terminals in Use"]
pub type NSU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - External Bias Generation"]
    #[inline(always)]
    pub fn xbias(&self) -> XBIAS_R {
        XBIAS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Waveform Mode"]
    #[inline(always)]
    pub fn wmod(&self) -> WMOD_R {
        WMOD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Blank LCD"]
    #[inline(always)]
    pub fn blank(&self) -> BLANK_R {
        BLANK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Duty Select"]
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:21 - Fine Contrast"]
    #[inline(always)]
    pub fn fcst(&self) -> FCST_R {
        FCST_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Number of Segment Terminals in Use"]
    #[inline(always)]
    pub fn nsu(&self) -> NSU_R {
        NSU_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Bias Generation"]
    #[inline(always)]
    #[must_use]
    pub fn xbias(&mut self) -> XBIAS_W<0> {
        XBIAS_W::new(self)
    }
    #[doc = "Bit 1 - Waveform Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wmod(&mut self) -> WMOD_W<1> {
        WMOD_W::new(self)
    }
    #[doc = "Bit 2 - Blank LCD"]
    #[inline(always)]
    #[must_use]
    pub fn blank(&mut self) -> BLANK_W<2> {
        BLANK_W::new(self)
    }
    #[doc = "Bit 3 - Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<3> {
        LOCK_W::new(self)
    }
    #[doc = "Bits 8:9 - Duty Select"]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<8> {
        DUTY_W::new(self)
    }
    #[doc = "Bits 16:21 - Fine Contrast"]
    #[inline(always)]
    #[must_use]
    pub fn fcst(&mut self) -> FCST_W<16> {
        FCST_W::new(self)
    }
    #[doc = "Bits 24:29 - Number of Segment Terminals in Use"]
    #[inline(always)]
    #[must_use]
    pub fn nsu(&mut self) -> NSU_W<24> {
        NSU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
