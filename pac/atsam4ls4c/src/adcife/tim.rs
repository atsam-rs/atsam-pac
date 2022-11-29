#[doc = "Register `TIM` reader"]
pub struct R(crate::R<TIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM` writer"]
pub struct W(crate::W<TIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM_SPEC>;
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
impl From<crate::W<TIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTUP` reader - Startup time"]
pub type STARTUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STARTUP` writer - Startup time"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `ENSTUP` reader - Enable Startup"]
pub type ENSTUP_R = crate::BitReader<bool>;
#[doc = "Field `ENSTUP` writer - Enable Startup"]
pub type ENSTUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Startup time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Enable Startup"]
    #[inline(always)]
    pub fn enstup(&self) -> ENSTUP_R {
        ENSTUP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Startup time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> STARTUP_W<0> {
        STARTUP_W::new(self)
    }
    #[doc = "Bit 8 - Enable Startup"]
    #[inline(always)]
    #[must_use]
    pub fn enstup(&mut self) -> ENSTUP_W<8> {
        ENSTUP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim](index.html) module"]
pub struct TIM_SPEC;
impl crate::RegisterSpec for TIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim::R](R) reader structure"]
impl crate::Readable for TIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim::W](W) writer structure"]
impl crate::Writable for TIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIM to value 0"]
impl crate::Resettable for TIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
