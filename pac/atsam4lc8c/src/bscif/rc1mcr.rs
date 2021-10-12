#[doc = "Register `RC1MCR` reader"]
pub struct R(crate::R<RC1MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC1MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RC1MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RC1MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RC1MCR` writer"]
pub struct W(crate::W<RC1MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC1MCR_SPEC>;
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
impl From<crate::W<RC1MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RC1MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKOE` reader - 1MHz RC Osc Clock Output Enable"]
pub struct CLKOE_R(crate::FieldReader<bool, bool>);
impl CLKOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKOE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKOE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKOE` writer - 1MHz RC Osc Clock Output Enable"]
pub struct CLKOE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKOE_W<'a> {
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
#[doc = "Field `FCD` reader - Flash Calibration Done"]
pub struct FCD_R(crate::FieldReader<bool, bool>);
impl FCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        FCD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCD` writer - Flash Calibration Done"]
pub struct FCD_W<'a> {
    w: &'a mut W,
}
impl<'a> FCD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CLKCAL` reader - 1MHz RC Osc Calibration"]
pub struct CLKCAL_R(crate::FieldReader<u8, u8>);
impl CLKCAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLKCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKCAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKCAL` writer - 1MHz RC Osc Calibration"]
pub struct CLKCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - 1MHz RC Osc Clock Output Enable"]
    #[inline(always)]
    pub fn clkoe(&self) -> CLKOE_R {
        CLKOE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 7 - Flash Calibration Done"]
    #[inline(always)]
    pub fn fcd(&self) -> FCD_R {
        FCD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - 1MHz RC Osc Calibration"]
    #[inline(always)]
    pub fn clkcal(&self) -> CLKCAL_R {
        CLKCAL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1MHz RC Osc Clock Output Enable"]
    #[inline(always)]
    pub fn clkoe(&mut self) -> CLKOE_W {
        CLKOE_W { w: self }
    }
    #[doc = "Bit 7 - Flash Calibration Done"]
    #[inline(always)]
    pub fn fcd(&mut self) -> FCD_W {
        FCD_W { w: self }
    }
    #[doc = "Bits 8:12 - 1MHz RC Osc Calibration"]
    #[inline(always)]
    pub fn clkcal(&mut self) -> CLKCAL_W {
        CLKCAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1MHz RC Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc1mcr](index.html) module"]
pub struct RC1MCR_SPEC;
impl crate::RegisterSpec for RC1MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc1mcr::R](R) reader structure"]
impl crate::Readable for RC1MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc1mcr::W](W) writer structure"]
impl crate::Writable for RC1MCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RC1MCR to value 0x0f00"]
impl crate::Resettable for RC1MCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f00
    }
}
