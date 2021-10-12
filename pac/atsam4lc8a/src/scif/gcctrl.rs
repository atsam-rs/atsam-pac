#[doc = "Register `GCCTRL%s` reader"]
pub struct R(crate::R<GCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCCTRL%s` writer"]
pub struct W(crate::W<GCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCCTRL_SPEC>;
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
impl From<crate::W<GCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCCTRL_SPEC>) -> Self {
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
#[doc = "Field `DIVEN` reader - Divide Enable"]
pub struct DIVEN_R(crate::FieldReader<bool, bool>);
impl DIVEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIVEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVEN` writer - Divide Enable"]
pub struct DIVEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVEN_W<'a> {
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
#[doc = "Field `OSCSEL` reader - Clock Select"]
pub struct OSCSEL_R(crate::FieldReader<u8, u8>);
impl OSCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        OSCSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSCSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCSEL` writer - Clock Select"]
pub struct OSCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `DIV` reader - Division Factor"]
pub struct DIV_R(crate::FieldReader<u16, u16>);
impl DIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV` writer - Division Factor"]
pub struct DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Divide Enable"]
    #[inline(always)]
    pub fn diven(&self) -> DIVEN_R {
        DIVEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Clock Select"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:31 - Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    pub fn cen(&mut self) -> CEN_W {
        CEN_W { w: self }
    }
    #[doc = "Bit 1 - Divide Enable"]
    #[inline(always)]
    pub fn diven(&mut self) -> DIVEN_W {
        DIVEN_W { w: self }
    }
    #[doc = "Bits 8:12 - Clock Select"]
    #[inline(always)]
    pub fn oscsel(&mut self) -> OSCSEL_W {
        OSCSEL_W { w: self }
    }
    #[doc = "Bits 16:31 - Division Factor"]
    #[inline(always)]
    pub fn div(&mut self) -> DIV_W {
        DIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Generic Clock Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcctrl](index.html) module"]
pub struct GCCTRL_SPEC;
impl crate::RegisterSpec for GCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcctrl::R](R) reader structure"]
impl crate::Readable for GCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcctrl::W](W) writer structure"]
impl crate::Writable for GCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCCTRL%s to value 0"]
impl crate::Resettable for GCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
