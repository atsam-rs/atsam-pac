#[doc = "Register `DTR` reader"]
pub struct R(crate::R<DTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DTR_SPEC>> for R {
    fn from(reader: crate::R<DTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTR` writer"]
pub struct W(crate::W<DTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTR_SPEC>;
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
impl core::convert::From<crate::W<DTR_SPEC>> for W {
    fn from(writer: crate::W<DTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXP` reader - EXP"]
pub struct EXP_R(crate::FieldReader<u8, u8>);
impl EXP_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXP` writer - EXP"]
pub struct EXP_W<'a> {
    w: &'a mut W,
}
impl<'a> EXP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `ADD` reader - ADD"]
pub struct ADD_R(crate::FieldReader<bool, bool>);
impl ADD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADD` writer - ADD"]
pub struct ADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `VALUE` reader - VALUE"]
pub struct VALUE_R(crate::FieldReader<u8, u8>);
impl VALUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VALUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VALUE` writer - VALUE"]
pub struct VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - EXP"]
    #[inline(always)]
    pub fn exp(&self) -> EXP_R {
        EXP_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - ADD"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - VALUE"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - EXP"]
    #[inline(always)]
    pub fn exp(&mut self) -> EXP_W {
        EXP_W { w: self }
    }
    #[doc = "Bit 5 - ADD"]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W {
        ADD_W { w: self }
    }
    #[doc = "Bits 8:15 - VALUE"]
    #[inline(always)]
    pub fn value(&mut self) -> VALUE_W {
        VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Tuner Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtr](index.html) module"]
pub struct DTR_SPEC;
impl crate::RegisterSpec for DTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtr::R](R) reader structure"]
impl crate::Readable for DTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtr::W](W) writer structure"]
impl crate::Writable for DTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTR to value 0"]
impl crate::Resettable for DTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
