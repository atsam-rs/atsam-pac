#[doc = "Register `PLL` reader"]
pub struct R(crate::R<PLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLL` writer"]
pub struct W(crate::W<PLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLL_SPEC>;
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
impl From<crate::W<PLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLEN` reader - PLL Enable"]
pub type PLLEN_R = crate::BitReader<bool>;
#[doc = "Field `PLLEN` writer - PLL Enable"]
pub type PLLEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLL_SPEC, bool, O>;
#[doc = "Field `PLLOSC` reader - PLL Oscillator Select"]
pub type PLLOSC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLOSC` writer - PLL Oscillator Select"]
pub type PLLOSC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_SPEC, u8, u8, 2, O>;
#[doc = "Field `PLLOPT` reader - PLL Option"]
pub type PLLOPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLOPT` writer - PLL Option"]
pub type PLLOPT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_SPEC, u8, u8, 3, O>;
#[doc = "Field `PLLDIV` reader - PLL Division Factor"]
pub type PLLDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLDIV` writer - PLL Division Factor"]
pub type PLLDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_SPEC, u8, u8, 4, O>;
#[doc = "Field `PLLMUL` reader - PLL Multiply Factor"]
pub type PLLMUL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLMUL` writer - PLL Multiply Factor"]
pub type PLLMUL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_SPEC, u8, u8, 4, O>;
#[doc = "Field `PLLCOUNT` reader - PLL Count"]
pub type PLLCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLCOUNT` writer - PLL Count"]
pub type PLLCOUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - PLL Enable"]
    #[inline(always)]
    pub fn pllen(&self) -> PLLEN_R {
        PLLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - PLL Oscillator Select"]
    #[inline(always)]
    pub fn pllosc(&self) -> PLLOSC_R {
        PLLOSC_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:5 - PLL Option"]
    #[inline(always)]
    pub fn pllopt(&self) -> PLLOPT_R {
        PLLOPT_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 8:11 - PLL Division Factor"]
    #[inline(always)]
    pub fn plldiv(&self) -> PLLDIV_R {
        PLLDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PLL Multiply Factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:29 - PLL Count"]
    #[inline(always)]
    pub fn pllcount(&self) -> PLLCOUNT_R {
        PLLCOUNT_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pllen(&mut self) -> PLLEN_W<0> {
        PLLEN_W::new(self)
    }
    #[doc = "Bits 1:2 - PLL Oscillator Select"]
    #[inline(always)]
    #[must_use]
    pub fn pllosc(&mut self) -> PLLOSC_W<1> {
        PLLOSC_W::new(self)
    }
    #[doc = "Bits 3:5 - PLL Option"]
    #[inline(always)]
    #[must_use]
    pub fn pllopt(&mut self) -> PLLOPT_W<3> {
        PLLOPT_W::new(self)
    }
    #[doc = "Bits 8:11 - PLL Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn plldiv(&mut self) -> PLLDIV_W<8> {
        PLLDIV_W::new(self)
    }
    #[doc = "Bits 16:19 - PLL Multiply Factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllmul(&mut self) -> PLLMUL_W<16> {
        PLLMUL_W::new(self)
    }
    #[doc = "Bits 24:29 - PLL Count"]
    #[inline(always)]
    #[must_use]
    pub fn pllcount(&mut self) -> PLLCOUNT_W<24> {
        PLLCOUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pll](index.html) module"]
pub struct PLL_SPEC;
impl crate::RegisterSpec for PLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pll::R](R) reader structure"]
impl crate::Readable for PLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pll::W](W) writer structure"]
impl crate::Writable for PLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLL to value 0"]
impl crate::Resettable for PLL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
