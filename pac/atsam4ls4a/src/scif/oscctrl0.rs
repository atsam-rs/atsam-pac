#[doc = "Register `OSCCTRL0` reader"]
pub struct R(crate::R<OSCCTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCCTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCCTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCCTRL0` writer"]
pub struct W(crate::W<OSCCTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCCTRL0_SPEC>;
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
impl From<crate::W<OSCCTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCCTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Oscillator Mode"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - Oscillator Mode"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCCTRL0_SPEC, bool, O>;
#[doc = "Field `GAIN` reader - Gain"]
pub type GAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN` writer - Gain"]
pub type GAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCCTRL0_SPEC, u8, u8, 2, O>;
#[doc = "Field `AGC` reader - Automatic Gain Control"]
pub type AGC_R = crate::BitReader<bool>;
#[doc = "Field `AGC` writer - Automatic Gain Control"]
pub type AGC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCCTRL0_SPEC, bool, O>;
#[doc = "Field `STARTUP` reader - Oscillator Start-up Time"]
pub type STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTUP` writer - Oscillator Start-up Time"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCCTRL0_SPEC, u8, u8, 4, O>;
#[doc = "Field `OSCEN` reader - Oscillator Enable"]
pub type OSCEN_R = crate::BitReader<bool>;
#[doc = "Field `OSCEN` writer - Oscillator Enable"]
pub type OSCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCCTRL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GAIN_R {
        GAIN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Automatic Gain Control"]
    #[inline(always)]
    pub fn agc(&self) -> AGC_R {
        AGC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Oscillator Start-up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Oscillator Enable"]
    #[inline(always)]
    pub fn oscen(&self) -> OSCEN_R {
        OSCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillator Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    #[doc = "Bits 1:2 - Gain"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GAIN_W<1> {
        GAIN_W::new(self)
    }
    #[doc = "Bit 3 - Automatic Gain Control"]
    #[inline(always)]
    #[must_use]
    pub fn agc(&mut self) -> AGC_W<3> {
        AGC_W::new(self)
    }
    #[doc = "Bits 8:11 - Oscillator Start-up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<8> {
        STARTUP_W::new(self)
    }
    #[doc = "Bit 16 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oscen(&mut self) -> OSCEN_W<16> {
        OSCEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscctrl0](index.html) module"]
pub struct OSCCTRL0_SPEC;
impl crate::RegisterSpec for OSCCTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oscctrl0::R](R) reader structure"]
impl crate::Readable for OSCCTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oscctrl0::W](W) writer structure"]
impl crate::Writable for OSCCTRL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCCTRL0 to value 0"]
impl crate::Resettable for OSCCTRL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
