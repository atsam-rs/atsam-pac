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
#[doc = "Field `PRESC` reader - LCD Prescaler Select"]
pub type PRESC_R = crate::BitReader<bool>;
#[doc = "Field `PRESC` writer - LCD Prescaler Select"]
pub type PRESC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM_SPEC, bool, O>;
#[doc = "Field `CLKDIV` reader - LCD Clock Division"]
pub type CLKDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKDIV` writer - LCD Clock Division"]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_SPEC, u8, u8, 3, O>;
#[doc = "Field `FC0` reader - Frame Counter 0"]
pub type FC0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FC0` writer - Frame Counter 0"]
pub type FC0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `FC0PB` reader - Frame Counter 0 Prescaler Bypass"]
pub type FC0PB_R = crate::BitReader<bool>;
#[doc = "Field `FC0PB` writer - Frame Counter 0 Prescaler Bypass"]
pub type FC0PB_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIM_SPEC, bool, O>;
#[doc = "Field `FC1` reader - Frame Counter 1"]
pub type FC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FC1` writer - Frame Counter 1"]
pub type FC1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `FC2` reader - Frame Counter 2"]
pub type FC2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FC2` writer - Frame Counter 2"]
pub type FC2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - LCD Prescaler Select"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - LCD Clock Division"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 8:12 - Frame Counter 0"]
    #[inline(always)]
    pub fn fc0(&self) -> FC0_R {
        FC0_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Frame Counter 0 Prescaler Bypass"]
    #[inline(always)]
    pub fn fc0pb(&self) -> FC0PB_R {
        FC0PB_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Frame Counter 1"]
    #[inline(always)]
    pub fn fc1(&self) -> FC1_R {
        FC1_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Frame Counter 2"]
    #[inline(always)]
    pub fn fc2(&self) -> FC2_R {
        FC2_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PRESC_W<0> {
        PRESC_W::new(self)
    }
    #[doc = "Bits 1:3 - LCD Clock Division"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<1> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bits 8:12 - Frame Counter 0"]
    #[inline(always)]
    #[must_use]
    pub fn fc0(&mut self) -> FC0_W<8> {
        FC0_W::new(self)
    }
    #[doc = "Bit 13 - Frame Counter 0 Prescaler Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn fc0pb(&mut self) -> FC0PB_W<13> {
        FC0PB_W::new(self)
    }
    #[doc = "Bits 16:20 - Frame Counter 1"]
    #[inline(always)]
    #[must_use]
    pub fn fc1(&mut self) -> FC1_W<16> {
        FC1_W::new(self)
    }
    #[doc = "Bits 24:28 - Frame Counter 2"]
    #[inline(always)]
    #[must_use]
    pub fn fc2(&mut self) -> FC2_W<24> {
        FC2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim](index.html) module"]
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
