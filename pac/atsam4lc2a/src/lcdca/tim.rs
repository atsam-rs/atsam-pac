#[doc = "Register `TIM` reader"]
pub struct R(crate::R<TIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<TIM_SPEC>> for R {
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
impl core::convert::From<crate::W<TIM_SPEC>> for W {
    fn from(writer: crate::W<TIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESC` reader - LCD Prescaler Select"]
pub struct PRESC_R(crate::FieldReader<bool, bool>);
impl PRESC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRESC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESC` writer - LCD Prescaler Select"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CLKDIV` reader - LCD Clock Division"]
pub struct CLKDIV_R(crate::FieldReader<u8, u8>);
impl CLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKDIV` writer - LCD Clock Division"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `FC0` reader - Frame Counter 0"]
pub struct FC0_R(crate::FieldReader<u8, u8>);
impl FC0_R {
    pub(crate) fn new(bits: u8) -> Self {
        FC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC0` writer - Frame Counter 0"]
pub struct FC0_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `FC0PB` reader - Frame Counter 0 Prescaler Bypass"]
pub struct FC0PB_R(crate::FieldReader<bool, bool>);
impl FC0PB_R {
    pub(crate) fn new(bits: bool) -> Self {
        FC0PB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC0PB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC0PB` writer - Frame Counter 0 Prescaler Bypass"]
pub struct FC0PB_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0PB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `FC1` reader - Frame Counter 1"]
pub struct FC1_R(crate::FieldReader<u8, u8>);
impl FC1_R {
    pub(crate) fn new(bits: u8) -> Self {
        FC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC1` writer - Frame Counter 1"]
pub struct FC1_W<'a> {
    w: &'a mut W,
}
impl<'a> FC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `FC2` reader - Frame Counter 2"]
pub struct FC2_R(crate::FieldReader<u8, u8>);
impl FC2_R {
    pub(crate) fn new(bits: u8) -> Self {
        FC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FC2` writer - Frame Counter 2"]
pub struct FC2_W<'a> {
    w: &'a mut W,
}
impl<'a> FC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LCD Prescaler Select"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - LCD Clock Division"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - Frame Counter 0"]
    #[inline(always)]
    pub fn fc0(&self) -> FC0_R {
        FC0_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Frame Counter 0 Prescaler Bypass"]
    #[inline(always)]
    pub fn fc0pb(&self) -> FC0PB_R {
        FC0PB_R::new(((self.bits >> 13) & 0x01) != 0)
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
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bits 1:3 - LCD Clock Division"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bits 8:12 - Frame Counter 0"]
    #[inline(always)]
    pub fn fc0(&mut self) -> FC0_W {
        FC0_W { w: self }
    }
    #[doc = "Bit 13 - Frame Counter 0 Prescaler Bypass"]
    #[inline(always)]
    pub fn fc0pb(&mut self) -> FC0PB_W {
        FC0PB_W { w: self }
    }
    #[doc = "Bits 16:20 - Frame Counter 1"]
    #[inline(always)]
    pub fn fc1(&mut self) -> FC1_W {
        FC1_W { w: self }
    }
    #[doc = "Bits 24:28 - Frame Counter 2"]
    #[inline(always)]
    pub fn fc2(&mut self) -> FC2_W {
        FC2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
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
}
#[doc = "`reset()` method sets TIM to value 0"]
impl crate::Resettable for TIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
