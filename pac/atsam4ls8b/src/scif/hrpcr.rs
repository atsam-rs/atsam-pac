#[doc = "Register `HRPCR` reader"]
pub struct R(crate::R<HRPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HRPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HRPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HRPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HRPCR` writer"]
pub struct W(crate::W<HRPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HRPCR_SPEC>;
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
impl From<crate::W<HRPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HRPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HRPEN` reader - High Resolution Prescaler Enable"]
pub struct HRPEN_R(crate::FieldReader<bool, bool>);
impl HRPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRPEN` writer - High Resolution Prescaler Enable"]
pub struct HRPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HRPEN_W<'a> {
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
#[doc = "Field `HRCOUNT` reader - High Resolution Counter"]
pub struct HRCOUNT_R(crate::FieldReader<u32, u32>);
impl HRCOUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        HRCOUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRCOUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRCOUNT` writer - High Resolution Counter"]
pub struct HRCOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> HRCOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - High Resolution Prescaler Enable"]
    #[inline(always)]
    pub fn hrpen(&self) -> HRPEN_R {
        HRPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Clock Input Selection"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 8:31 - High Resolution Counter"]
    #[inline(always)]
    pub fn hrcount(&self) -> HRCOUNT_R {
        HRCOUNT_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - High Resolution Prescaler Enable"]
    #[inline(always)]
    pub fn hrpen(&mut self) -> HRPEN_W {
        HRPEN_W { w: self }
    }
    #[doc = "Bits 1:3 - Clock Input Selection"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W {
        CKSEL_W { w: self }
    }
    #[doc = "Bits 8:31 - High Resolution Counter"]
    #[inline(always)]
    pub fn hrcount(&mut self) -> HRCOUNT_W {
        HRCOUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Resolution Prescaler Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hrpcr](index.html) module"]
pub struct HRPCR_SPEC;
impl crate::RegisterSpec for HRPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hrpcr::R](R) reader structure"]
impl crate::Readable for HRPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hrpcr::W](W) writer structure"]
impl crate::Writable for HRPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HRPCR to value 0"]
impl crate::Resettable for HRPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
