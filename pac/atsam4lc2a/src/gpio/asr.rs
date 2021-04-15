#[doc = "Register `ASR%s` reader"]
pub struct R(crate::R<ASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<ASR_SPEC>> for R {
    fn from(reader: crate::R<ASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ASR%s` writer"]
pub struct W(crate::W<ASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ASR_SPEC>;
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
impl core::convert::From<crate::W<ASR_SPEC>> for W {
    fn from(writer: crate::W<ASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AR` reader - Access Error"]
pub struct AR_R(crate::FieldReader<bool, bool>);
impl AR_R {
    pub(crate) fn new(bits: bool) -> Self {
        AR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AR` writer - Access Error"]
pub struct AR_W<'a> {
    w: &'a mut W,
}
impl<'a> AR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Access Error"]
    #[inline(always)]
    pub fn ar(&self) -> AR_R {
        AR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Access Error"]
    #[inline(always)]
    pub fn ar(&mut self) -> AR_W {
        AR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Access Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asr](index.html) module"]
pub struct ASR_SPEC;
impl crate::RegisterSpec for ASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [asr::R](R) reader structure"]
impl crate::Readable for ASR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [asr::W](W) writer structure"]
impl crate::Writable for ASR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ASR%s to value 0"]
impl crate::Resettable for ASR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
