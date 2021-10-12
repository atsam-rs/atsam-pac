#[doc = "Register `R2Y_SET1` reader"]
pub struct R(crate::R<R2Y_SET1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R2Y_SET1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R2Y_SET1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R2Y_SET1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `R2Y_SET1` writer"]
pub struct W(crate::W<R2Y_SET1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R2Y_SET1_SPEC>;
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
impl From<crate::W<R2Y_SET1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R2Y_SET1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `C3` reader - Color Space Conversion Matrix Coefficient C3"]
pub struct C3_R(crate::FieldReader<u8, u8>);
impl C3_R {
    pub(crate) fn new(bits: u8) -> Self {
        C3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C3` writer - Color Space Conversion Matrix Coefficient C3"]
pub struct C3_W<'a> {
    w: &'a mut W,
}
impl<'a> C3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `C4` reader - Color Space Conversion Matrix Coefficient C4"]
pub struct C4_R(crate::FieldReader<u8, u8>);
impl C4_R {
    pub(crate) fn new(bits: u8) -> Self {
        C4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C4` writer - Color Space Conversion Matrix Coefficient C4"]
pub struct C4_W<'a> {
    w: &'a mut W,
}
impl<'a> C4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | ((value as u32 & 0x7f) << 8);
        self.w
    }
}
#[doc = "Field `C5` reader - Color Space Conversion Matrix Coefficient C5"]
pub struct C5_R(crate::FieldReader<u8, u8>);
impl C5_R {
    pub(crate) fn new(bits: u8) -> Self {
        C5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for C5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `C5` writer - Color Space Conversion Matrix Coefficient C5"]
pub struct C5_W<'a> {
    w: &'a mut W,
}
impl<'a> C5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `Goff` reader - Color Space Conversion Green Component Offset"]
pub struct GOFF_R(crate::FieldReader<bool, bool>);
impl GOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        GOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GOFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Goff` writer - Color Space Conversion Green Component Offset"]
pub struct GOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> GOFF_W<'a> {
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
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C3"]
    #[inline(always)]
    pub fn c3(&self) -> C3_R {
        C3_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    pub fn c4(&self) -> C4_R {
        C4_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C5"]
    #[inline(always)]
    pub fn c5(&self) -> C5_R {
        C5_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Color Space Conversion Green Component Offset"]
    #[inline(always)]
    pub fn goff(&self) -> GOFF_R {
        GOFF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Color Space Conversion Matrix Coefficient C3"]
    #[inline(always)]
    pub fn c3(&mut self) -> C3_W {
        C3_W { w: self }
    }
    #[doc = "Bits 8:14 - Color Space Conversion Matrix Coefficient C4"]
    #[inline(always)]
    pub fn c4(&mut self) -> C4_W {
        C4_W { w: self }
    }
    #[doc = "Bits 16:22 - Color Space Conversion Matrix Coefficient C5"]
    #[inline(always)]
    pub fn c5(&mut self) -> C5_W {
        C5_W { w: self }
    }
    #[doc = "Bit 24 - Color Space Conversion Green Component Offset"]
    #[inline(always)]
    pub fn goff(&mut self) -> GOFF_W {
        GOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ISI Color Space Conversion RGB To YCrCb Set 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r2y_set1](index.html) module"]
pub struct R2Y_SET1_SPEC;
impl crate::RegisterSpec for R2Y_SET1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r2y_set1::R](R) reader structure"]
impl crate::Readable for R2Y_SET1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [r2y_set1::W](W) writer structure"]
impl crate::Writable for R2Y_SET1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R2Y_SET1 to value 0"]
impl crate::Resettable for R2Y_SET1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
