#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - ACIFC Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - ACIFC Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EVENTEN` reader - Peripheral Event Trigger Enable"]
pub type EVENTEN_R = crate::BitReader<bool>;
#[doc = "Field `EVENTEN` writer - Peripheral Event Trigger Enable"]
pub type EVENTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `USTART` reader - User Start Single Comparison"]
pub type USTART_R = crate::BitReader<bool>;
#[doc = "Field `USTART` writer - User Start Single Comparison"]
pub type USTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ESTART` reader - Peripheral Event Start Single Comparison"]
pub type ESTART_R = crate::BitReader<bool>;
#[doc = "Field `ESTART` writer - Peripheral Event Start Single Comparison"]
pub type ESTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `ACTEST` reader - Analog Comparator Test Mode"]
pub type ACTEST_R = crate::BitReader<bool>;
#[doc = "Field `ACTEST` writer - Analog Comparator Test Mode"]
pub type ACTEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - ACIFC Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Peripheral Event Trigger Enable"]
    #[inline(always)]
    pub fn eventen(&self) -> EVENTEN_R {
        EVENTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - User Start Single Comparison"]
    #[inline(always)]
    pub fn ustart(&self) -> USTART_R {
        USTART_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral Event Start Single Comparison"]
    #[inline(always)]
    pub fn estart(&self) -> ESTART_R {
        ESTART_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Analog Comparator Test Mode"]
    #[inline(always)]
    pub fn actest(&self) -> ACTEST_R {
        ACTEST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACIFC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Peripheral Event Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eventen(&mut self) -> EVENTEN_W<1> {
        EVENTEN_W::new(self)
    }
    #[doc = "Bit 4 - User Start Single Comparison"]
    #[inline(always)]
    #[must_use]
    pub fn ustart(&mut self) -> USTART_W<4> {
        USTART_W::new(self)
    }
    #[doc = "Bit 5 - Peripheral Event Start Single Comparison"]
    #[inline(always)]
    #[must_use]
    pub fn estart(&mut self) -> ESTART_W<5> {
        ESTART_W::new(self)
    }
    #[doc = "Bit 7 - Analog Comparator Test Mode"]
    #[inline(always)]
    #[must_use]
    pub fn actest(&mut self) -> ACTEST_W<7> {
        ACTEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
