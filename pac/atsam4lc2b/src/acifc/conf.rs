#[doc = "Register `CONF%s` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF%s` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IS` reader - Interupt Settings"]
pub type IS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IS` writer - Interupt Settings"]
pub type IS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `MODE` reader - Analog Comparator Mode"]
pub type MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MODE` writer - Analog Comparator Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `INSELN` reader - Negative Input Select"]
pub type INSELN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSELN` writer - Negative Input Select"]
pub type INSELN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `EVENN` reader - Peripheral Event Enable Negative"]
pub type EVENN_R = crate::BitReader<bool>;
#[doc = "Field `EVENN` writer - Peripheral Event Enable Negative"]
pub type EVENN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF_SPEC, bool, O>;
#[doc = "Field `EVENP` reader - Peripheral Event Enable Positive"]
pub type EVENP_R = crate::BitReader<bool>;
#[doc = "Field `EVENP` writer - Peripheral Event Enable Positive"]
pub type EVENP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF_SPEC, bool, O>;
#[doc = "Field `HYS` reader - Hysteresis Voltage Value"]
pub type HYS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HYS` writer - Hysteresis Voltage Value"]
pub type HYS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `FAST` reader - Fast Mode Enable"]
pub type FAST_R = crate::BitReader<bool>;
#[doc = "Field `FAST` writer - Fast Mode Enable"]
pub type FAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF_SPEC, bool, O>;
#[doc = "Field `ALWAYSON` reader - Always On"]
pub type ALWAYSON_R = crate::BitReader<bool>;
#[doc = "Field `ALWAYSON` writer - Always On"]
pub type ALWAYSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Interupt Settings"]
    #[inline(always)]
    pub fn is(&self) -> IS_R {
        IS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Analog Comparator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Negative Input Select"]
    #[inline(always)]
    pub fn inseln(&self) -> INSELN_R {
        INSELN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Peripheral Event Enable Negative"]
    #[inline(always)]
    pub fn evenn(&self) -> EVENN_R {
        EVENN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Peripheral Event Enable Positive"]
    #[inline(always)]
    pub fn evenp(&self) -> EVENP_R {
        EVENP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Hysteresis Voltage Value"]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Fast Mode Enable"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Always On"]
    #[inline(always)]
    pub fn alwayson(&self) -> ALWAYSON_R {
        ALWAYSON_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interupt Settings"]
    #[inline(always)]
    #[must_use]
    pub fn is(&mut self) -> IS_W<0> {
        IS_W::new(self)
    }
    #[doc = "Bits 4:5 - Analog Comparator Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<4> {
        MODE_W::new(self)
    }
    #[doc = "Bits 8:9 - Negative Input Select"]
    #[inline(always)]
    #[must_use]
    pub fn inseln(&mut self) -> INSELN_W<8> {
        INSELN_W::new(self)
    }
    #[doc = "Bit 16 - Peripheral Event Enable Negative"]
    #[inline(always)]
    #[must_use]
    pub fn evenn(&mut self) -> EVENN_W<16> {
        EVENN_W::new(self)
    }
    #[doc = "Bit 17 - Peripheral Event Enable Positive"]
    #[inline(always)]
    #[must_use]
    pub fn evenp(&mut self) -> EVENP_W<17> {
        EVENP_W::new(self)
    }
    #[doc = "Bits 24:25 - Hysteresis Voltage Value"]
    #[inline(always)]
    #[must_use]
    pub fn hys(&mut self) -> HYS_W<24> {
        HYS_W::new(self)
    }
    #[doc = "Bit 26 - Fast Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fast(&mut self) -> FAST_W<26> {
        FAST_W::new(self)
    }
    #[doc = "Bit 27 - Always On"]
    #[inline(always)]
    #[must_use]
    pub fn alwayson(&mut self) -> ALWAYSON_W<27> {
        ALWAYSON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF%s to value 0"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
