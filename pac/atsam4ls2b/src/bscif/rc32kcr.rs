#[doc = "Register `RC32KCR` reader"]
pub struct R(crate::R<RC32KCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC32KCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC32KCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC32KCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RC32KCR` writer"]
pub struct W(crate::W<RC32KCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC32KCR_SPEC>;
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
impl From<crate::W<RC32KCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC32KCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable as Generic clock source"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable as Generic clock source"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32KCR_SPEC, bool, O>;
#[doc = "Field `TCEN` reader - Temperature Compensation Enable"]
pub type TCEN_R = crate::BitReader<bool>;
#[doc = "Field `TCEN` writer - Temperature Compensation Enable"]
pub type TCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32KCR_SPEC, bool, O>;
#[doc = "Field `EN32K` reader - Enable 32 KHz output"]
pub type EN32K_R = crate::BitReader<bool>;
#[doc = "Field `EN32K` writer - Enable 32 KHz output"]
pub type EN32K_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32KCR_SPEC, bool, O>;
#[doc = "Field `EN1K` reader - Enable 1 kHz output"]
pub type EN1K_R = crate::BitReader<bool>;
#[doc = "Field `EN1K` writer - Enable 1 kHz output"]
pub type EN1K_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32KCR_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Mode Selection"]
pub type MODE_R = crate::BitReader<bool>;
#[doc = "Field `MODE` writer - Mode Selection"]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32KCR_SPEC, bool, O>;
#[doc = "Field `REF` reader - Reference select"]
pub type REF_R = crate::BitReader<bool>;
#[doc = "Field `REF` writer - Reference select"]
pub type REF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32KCR_SPEC, bool, O>;
#[doc = "Field `FCD` reader - Flash calibration done"]
pub type FCD_R = crate::BitReader<bool>;
#[doc = "Field `FCD` writer - Flash calibration done"]
pub type FCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RC32KCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable as Generic clock source"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Temperature Compensation Enable"]
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable 32 KHz output"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable 1 kHz output"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mode Selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reference select"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Flash calibration done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable as Generic clock source"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Temperature Compensation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcen(&mut self) -> TCEN_W<1> {
        TCEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable 32 KHz output"]
    #[inline(always)]
    #[must_use]
    pub fn en32k(&mut self) -> EN32K_W<2> {
        EN32K_W::new(self)
    }
    #[doc = "Bit 3 - Enable 1 kHz output"]
    #[inline(always)]
    #[must_use]
    pub fn en1k(&mut self) -> EN1K_W<3> {
        EN1K_W::new(self)
    }
    #[doc = "Bit 4 - Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<4> {
        MODE_W::new(self)
    }
    #[doc = "Bit 5 - Reference select"]
    #[inline(always)]
    #[must_use]
    pub fn ref_(&mut self) -> REF_W<5> {
        REF_W::new(self)
    }
    #[doc = "Bit 7 - Flash calibration done"]
    #[inline(always)]
    #[must_use]
    pub fn fcd(&mut self) -> FCD_W<7> {
        FCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32 kHz RC Oscillator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc32kcr](index.html) module"]
pub struct RC32KCR_SPEC;
impl crate::RegisterSpec for RC32KCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc32kcr::R](R) reader structure"]
impl crate::Readable for RC32KCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc32kcr::W](W) writer structure"]
impl crate::Writable for RC32KCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RC32KCR to value 0"]
impl crate::Resettable for RC32KCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
