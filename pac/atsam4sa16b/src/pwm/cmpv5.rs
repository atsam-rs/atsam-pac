#[doc = "Register `CMPV5` reader"]
pub struct R(crate::R<CMPV5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPV5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPV5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPV5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPV5` writer"]
pub struct W(crate::W<CMPV5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPV5_SPEC>;
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
impl From<crate::W<CMPV5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPV5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CV` reader - Comparison x Value"]
pub struct CV_R(crate::FieldReader<u32, u32>);
impl CV_R {
    pub(crate) fn new(bits: u32) -> Self {
        CV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CV` writer - Comparison x Value"]
pub struct CV_W<'a> {
    w: &'a mut W,
}
impl<'a> CV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Field `CVM` reader - Comparison x Value Mode"]
pub struct CVM_R(crate::FieldReader<bool, bool>);
impl CVM_R {
    pub(crate) fn new(bits: bool) -> Self {
        CVM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CVM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CVM` writer - Comparison x Value Mode"]
pub struct CVM_W<'a> {
    w: &'a mut W,
}
impl<'a> CVM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&self) -> CVM_R {
        CVM_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Comparison x Value"]
    #[inline(always)]
    pub fn cv(&mut self) -> CV_W {
        CV_W { w: self }
    }
    #[doc = "Bit 24 - Comparison x Value Mode"]
    #[inline(always)]
    pub fn cvm(&mut self) -> CVM_W {
        CVM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PWM Comparison 5 Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpv5](index.html) module"]
pub struct CMPV5_SPEC;
impl crate::RegisterSpec for CMPV5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpv5::R](R) reader structure"]
impl crate::Readable for CMPV5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpv5::W](W) writer structure"]
impl crate::Writable for CMPV5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMPV5 to value 0"]
impl crate::Resettable for CMPV5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
