#[doc = "Register `BOD33SAMPLING` reader"]
pub struct R(crate::R<BOD33SAMPLING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD33SAMPLING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD33SAMPLING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD33SAMPLING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD33SAMPLING` writer"]
pub struct W(crate::W<BOD33SAMPLING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD33SAMPLING_SPEC>;
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
impl From<crate::W<BOD33SAMPLING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD33SAMPLING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CEN` reader - Clock Enable"]
pub struct CEN_R(crate::FieldReader<bool, bool>);
impl CEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEN` writer - Clock Enable"]
pub struct CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CEN_W<'a> {
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
#[doc = "Field `CSSEL` reader - Clock Source Select"]
pub struct CSSEL_R(crate::FieldReader<bool, bool>);
impl CSSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSSEL` writer - Clock Source Select"]
pub struct CSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `PSEL` reader - Prescaler Select"]
pub struct PSEL_R(crate::FieldReader<u8, u8>);
impl PSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSEL` writer - Prescaler Select"]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Clock Source Select"]
    #[inline(always)]
    pub fn cssel(&self) -> CSSEL_R {
        CSSEL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
    #[doc = "Bit 1 - Clock Source Select"]
    #[inline(always)]
    pub fn cssel(&mut self) -> CSSEL_W {
        CSSEL_W { w: self }
    }
    #[doc = "Bits 8:11 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD33 Sampling Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod33sampling](index.html) module"]
pub struct BOD33SAMPLING_SPEC;
impl crate::RegisterSpec for BOD33SAMPLING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod33sampling::R](R) reader structure"]
impl crate::Readable for BOD33SAMPLING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod33sampling::W](W) writer structure"]
impl crate::Writable for BOD33SAMPLING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOD33SAMPLING to value 0"]
impl crate::Resettable for BOD33SAMPLING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
