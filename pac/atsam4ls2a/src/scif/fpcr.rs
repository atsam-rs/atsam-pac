#[doc = "Register `FPCR` reader"]
pub struct R(crate::R<FPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPCR` writer"]
pub struct W(crate::W<FPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPCR_SPEC>;
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
impl From<crate::W<FPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FPEN` reader - High Resolution Prescaler Enable"]
pub struct FPEN_R(crate::FieldReader<bool, bool>);
impl FPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FPEN` writer - High Resolution Prescaler Enable"]
pub struct FPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FPEN_W<'a> {
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
#[doc = "Field `CKSEL` reader - Clock Input Selection"]
pub struct CKSEL_R(crate::FieldReader<u8, u8>);
impl CKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKSEL` writer - Clock Input Selection"]
pub struct CKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - High Resolution Prescaler Enable"]
    #[inline(always)]
    pub fn fpen(&self) -> FPEN_R {
        FPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Clock Input Selection"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - High Resolution Prescaler Enable"]
    #[inline(always)]
    pub fn fpen(&mut self) -> FPEN_W {
        FPEN_W { w: self }
    }
    #[doc = "Bits 1:3 - Clock Input Selection"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W {
        CKSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional Prescaler Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpcr](index.html) module"]
pub struct FPCR_SPEC;
impl crate::RegisterSpec for FPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpcr::R](R) reader structure"]
impl crate::Readable for FPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpcr::W](W) writer structure"]
impl crate::Writable for FPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FPCR to value 0"]
impl crate::Resettable for FPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
