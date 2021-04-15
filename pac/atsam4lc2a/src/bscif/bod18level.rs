#[doc = "Register `BOD18LEVEL` reader"]
pub struct R(crate::R<BOD18LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD18LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BOD18LEVEL_SPEC>> for R {
    fn from(reader: crate::R<BOD18LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD18LEVEL` writer"]
pub struct W(crate::W<BOD18LEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD18LEVEL_SPEC>;
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
impl core::convert::From<crate::W<BOD18LEVEL_SPEC>> for W {
    fn from(writer: crate::W<BOD18LEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - BOD Value"]
pub struct VAL_R(crate::FieldReader<u8, u8>);
impl VAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL` writer - BOD Value"]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `RANGE` reader - BOD Threshold Range"]
pub struct RANGE_R(crate::FieldReader<bool, bool>);
impl RANGE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RANGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RANGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RANGE` writer - BOD Threshold Range"]
pub struct RANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - BOD Value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 31 - BOD Threshold Range"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - BOD Value"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W {
        VAL_W { w: self }
    }
    #[doc = "Bit 31 - BOD Threshold Range"]
    #[inline(always)]
    pub fn range(&mut self) -> RANGE_W {
        RANGE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD18 Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod18level](index.html) module"]
pub struct BOD18LEVEL_SPEC;
impl crate::RegisterSpec for BOD18LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod18level::R](R) reader structure"]
impl crate::Readable for BOD18LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod18level::W](W) writer structure"]
impl crate::Writable for BOD18LEVEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOD18LEVEL to value 0"]
impl crate::Resettable for BOD18LEVEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
