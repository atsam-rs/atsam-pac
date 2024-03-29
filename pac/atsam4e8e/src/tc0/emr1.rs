#[doc = "Register `EMR1` reader"]
pub struct R(crate::R<EMR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EMR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EMR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EMR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EMR1` writer"]
pub struct W(crate::W<EMR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EMR1_SPEC>;
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
impl From<crate::W<EMR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EMR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIGSRCA` reader - Trigger Source for Input A"]
pub type TRIGSRCA_R = crate::FieldReader<u8, TRIGSRCA_A>;
#[doc = "Trigger Source for Input A\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGSRCA_A {
    #[doc = "0: The trigger/capture input A is driven by external pin TIOAx"]
    EXTERNAL_TIOAX = 0,
    #[doc = "1: The trigger/capture input A is driven internally by PWMx"]
    PWMX = 1,
}
impl From<TRIGSRCA_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRCA_A) -> Self {
        variant as _
    }
}
impl TRIGSRCA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGSRCA_A> {
        match self.bits {
            0 => Some(TRIGSRCA_A::EXTERNAL_TIOAX),
            1 => Some(TRIGSRCA_A::PWMX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_TIOAX`"]
    #[inline(always)]
    pub fn is_external_tioax(&self) -> bool {
        *self == TRIGSRCA_A::EXTERNAL_TIOAX
    }
    #[doc = "Checks if the value of the field is `PWMX`"]
    #[inline(always)]
    pub fn is_pwmx(&self) -> bool {
        *self == TRIGSRCA_A::PWMX
    }
}
#[doc = "Field `TRIGSRCA` writer - Trigger Source for Input A"]
pub type TRIGSRCA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMR1_SPEC, u8, TRIGSRCA_A, 2, O>;
impl<'a, const O: u8> TRIGSRCA_W<'a, O> {
    #[doc = "The trigger/capture input A is driven by external pin TIOAx"]
    #[inline(always)]
    pub fn external_tioax(self) -> &'a mut W {
        self.variant(TRIGSRCA_A::EXTERNAL_TIOAX)
    }
    #[doc = "The trigger/capture input A is driven internally by PWMx"]
    #[inline(always)]
    pub fn pwmx(self) -> &'a mut W {
        self.variant(TRIGSRCA_A::PWMX)
    }
}
#[doc = "Field `TRIGSRCB` reader - Trigger Source for Input B"]
pub type TRIGSRCB_R = crate::FieldReader<u8, TRIGSRCB_A>;
#[doc = "Trigger Source for Input B\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIGSRCB_A {
    #[doc = "0: The trigger/capture input B is driven by external pin TIOBx"]
    EXTERNAL_TIOBX = 0,
    #[doc = "1: The trigger/capture input B is driven internally by PWMx"]
    PWMX = 1,
}
impl From<TRIGSRCB_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIGSRCB_A) -> Self {
        variant as _
    }
}
impl TRIGSRCB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIGSRCB_A> {
        match self.bits {
            0 => Some(TRIGSRCB_A::EXTERNAL_TIOBX),
            1 => Some(TRIGSRCB_A::PWMX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_TIOBX`"]
    #[inline(always)]
    pub fn is_external_tiobx(&self) -> bool {
        *self == TRIGSRCB_A::EXTERNAL_TIOBX
    }
    #[doc = "Checks if the value of the field is `PWMX`"]
    #[inline(always)]
    pub fn is_pwmx(&self) -> bool {
        *self == TRIGSRCB_A::PWMX
    }
}
#[doc = "Field `TRIGSRCB` writer - Trigger Source for Input B"]
pub type TRIGSRCB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EMR1_SPEC, u8, TRIGSRCB_A, 2, O>;
impl<'a, const O: u8> TRIGSRCB_W<'a, O> {
    #[doc = "The trigger/capture input B is driven by external pin TIOBx"]
    #[inline(always)]
    pub fn external_tiobx(self) -> &'a mut W {
        self.variant(TRIGSRCB_A::EXTERNAL_TIOBX)
    }
    #[doc = "The trigger/capture input B is driven internally by PWMx"]
    #[inline(always)]
    pub fn pwmx(self) -> &'a mut W {
        self.variant(TRIGSRCB_A::PWMX)
    }
}
#[doc = "Field `NODIVCLK` reader - No Divided Clock"]
pub type NODIVCLK_R = crate::BitReader<bool>;
#[doc = "Field `NODIVCLK` writer - No Divided Clock"]
pub type NODIVCLK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EMR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline(always)]
    pub fn trigsrca(&self) -> TRIGSRCA_R {
        TRIGSRCA_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline(always)]
    pub fn trigsrcb(&self) -> TRIGSRCB_R {
        TRIGSRCB_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline(always)]
    pub fn nodivclk(&self) -> NODIVCLK_R {
        NODIVCLK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trigger Source for Input A"]
    #[inline(always)]
    #[must_use]
    pub fn trigsrca(&mut self) -> TRIGSRCA_W<0> {
        TRIGSRCA_W::new(self)
    }
    #[doc = "Bits 4:5 - Trigger Source for Input B"]
    #[inline(always)]
    #[must_use]
    pub fn trigsrcb(&mut self) -> TRIGSRCB_W<4> {
        TRIGSRCB_W::new(self)
    }
    #[doc = "Bit 8 - No Divided Clock"]
    #[inline(always)]
    #[must_use]
    pub fn nodivclk(&mut self) -> NODIVCLK_W<8> {
        NODIVCLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Extended Mode Register (channel = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emr1](index.html) module"]
pub struct EMR1_SPEC;
impl crate::RegisterSpec for EMR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [emr1::R](R) reader structure"]
impl crate::Readable for EMR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [emr1::W](W) writer structure"]
impl crate::Writable for EMR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EMR1 to value 0"]
impl crate::Resettable for EMR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
