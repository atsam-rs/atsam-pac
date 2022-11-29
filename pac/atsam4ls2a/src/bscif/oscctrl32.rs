#[doc = "Register `OSCCTRL32` reader"]
pub struct R(crate::R<OSCCTRL32_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSCCTRL32_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSCCTRL32_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSCCTRL32_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSCCTRL32` writer"]
pub struct W(crate::W<OSCCTRL32_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCCTRL32_SPEC>;
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
impl From<crate::W<OSCCTRL32_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCCTRL32_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OSC32EN` reader - 32 KHz Oscillator Enable"]
pub type OSC32EN_R = crate::BitReader<bool>;
#[doc = "Field `OSC32EN` writer - 32 KHz Oscillator Enable"]
pub type OSC32EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCCTRL32_SPEC, bool, O>;
#[doc = "Field `PINSEL` reader - Pins Select"]
pub type PINSEL_R = crate::BitReader<bool>;
#[doc = "Field `PINSEL` writer - Pins Select"]
pub type PINSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCCTRL32_SPEC, bool, O>;
#[doc = "Field `EN32K` reader - 32 KHz output Enable"]
pub type EN32K_R = crate::BitReader<bool>;
#[doc = "Field `EN32K` writer - 32 KHz output Enable"]
pub type EN32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCCTRL32_SPEC, bool, O>;
#[doc = "Field `EN1K` reader - 1 KHz output Enable"]
pub type EN1K_R = crate::BitReader<bool>;
#[doc = "Field `EN1K` writer - 1 KHz output Enable"]
pub type EN1K_W<'a, const O: u8> = crate::BitWriter<'a, u32, OSCCTRL32_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Oscillator Mode"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - Oscillator Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCCTRL32_SPEC, u8, u8, 3, O>;
#[doc = "Field `SELCURR` reader - Current selection"]
pub type SELCURR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SELCURR` writer - Current selection"]
pub type SELCURR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCCTRL32_SPEC, u8, u8, 4, O>;
#[doc = "Field `STARTUP` reader - Oscillator Start-up Time"]
pub type STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTUP` writer - Oscillator Start-up Time"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSCCTRL32_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - 32 KHz Oscillator Enable"]
    #[inline(always)]
    pub fn osc32en(&self) -> OSC32EN_R {
        OSC32EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pins Select"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 32 KHz output Enable"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 KHz output Enable"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Oscillator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - Current selection"]
    #[inline(always)]
    pub fn selcurr(&self) -> SELCURR_R {
        SELCURR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Oscillator Start-up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 32 KHz Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osc32en(&mut self) -> OSC32EN_W<0> {
        OSC32EN_W::new(self)
    }
    #[doc = "Bit 1 - Pins Select"]
    #[inline(always)]
    #[must_use]
    pub fn pinsel(&mut self) -> PINSEL_W<1> {
        PINSEL_W::new(self)
    }
    #[doc = "Bit 2 - 32 KHz output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en32k(&mut self) -> EN32K_W<2> {
        EN32K_W::new(self)
    }
    #[doc = "Bit 3 - 1 KHz output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1k(&mut self) -> EN1K_W<3> {
        EN1K_W::new(self)
    }
    #[doc = "Bits 8:10 - Oscillator Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<8> {
        MODE_W::new(self)
    }
    #[doc = "Bits 12:15 - Current selection"]
    #[inline(always)]
    #[must_use]
    pub fn selcurr(&mut self) -> SELCURR_W<12> {
        SELCURR_W::new(self)
    }
    #[doc = "Bits 16:18 - Oscillator Start-up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<16> {
        STARTUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator 32 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscctrl32](index.html) module"]
pub struct OSCCTRL32_SPEC;
impl crate::RegisterSpec for OSCCTRL32_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [oscctrl32::R](R) reader structure"]
impl crate::Readable for OSCCTRL32_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oscctrl32::W](W) writer structure"]
impl crate::Writable for OSCCTRL32_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSCCTRL32 to value 0x04"]
impl crate::Resettable for OSCCTRL32_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
