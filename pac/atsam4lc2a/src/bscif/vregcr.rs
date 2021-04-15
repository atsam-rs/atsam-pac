#[doc = "Register `VREGCR` reader"]
pub struct R(crate::R<VREGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<VREGCR_SPEC>> for R {
    fn from(reader: crate::R<VREGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREGCR` writer"]
pub struct W(crate::W<VREGCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREGCR_SPEC>;
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
impl core::convert::From<crate::W<VREGCR_SPEC>> for W {
    fn from(writer: crate::W<VREGCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS` reader - Voltage Regulator disable"]
pub struct DIS_R(crate::FieldReader<bool, bool>);
impl DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS` writer - Voltage Regulator disable"]
pub struct DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_W<'a> {
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
#[doc = "Field `SSG` reader - Spread Spectrum Generator Enable"]
pub struct SSG_R(crate::FieldReader<bool, bool>);
impl SSG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSG` writer - Spread Spectrum Generator Enable"]
pub struct SSG_W<'a> {
    w: &'a mut W,
}
impl<'a> SSG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SSW` reader - Stop Switching"]
pub struct SSW_R(crate::FieldReader<bool, bool>);
impl SSW_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSW` writer - Stop Switching"]
pub struct SSW_W<'a> {
    w: &'a mut W,
}
impl<'a> SSW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `SSWEVT` reader - Stop Switching On Event Enable"]
pub struct SSWEVT_R(crate::FieldReader<bool, bool>);
impl SSWEVT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SSWEVT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSWEVT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SSWEVT` writer - Stop Switching On Event Enable"]
pub struct SSWEVT_W<'a> {
    w: &'a mut W,
}
impl<'a> SSWEVT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `SFV` reader - Store Final Value"]
pub struct SFV_R(crate::FieldReader<bool, bool>);
impl SFV_R {
    pub(crate) fn new(bits: bool) -> Self {
        SFV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFV` writer - Store Final Value"]
pub struct SFV_W<'a> {
    w: &'a mut W,
}
impl<'a> SFV_W<'a> {
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
    #[doc = "Bit 0 - Voltage Regulator disable"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - Spread Spectrum Generator Enable"]
    #[inline(always)]
    pub fn ssg(&self) -> SSG_R {
        SSG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Stop Switching"]
    #[inline(always)]
    pub fn ssw(&self) -> SSW_R {
        SSW_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Stop Switching On Event Enable"]
    #[inline(always)]
    pub fn sswevt(&self) -> SSWEVT_R {
        SSWEVT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Store Final Value"]
    #[inline(always)]
    pub fn sfv(&self) -> SFV_R {
        SFV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Regulator disable"]
    #[inline(always)]
    pub fn dis(&mut self) -> DIS_W {
        DIS_W { w: self }
    }
    #[doc = "Bit 8 - Spread Spectrum Generator Enable"]
    #[inline(always)]
    pub fn ssg(&mut self) -> SSG_W {
        SSG_W { w: self }
    }
    #[doc = "Bit 9 - Stop Switching"]
    #[inline(always)]
    pub fn ssw(&mut self) -> SSW_W {
        SSW_W { w: self }
    }
    #[doc = "Bit 10 - Stop Switching On Event Enable"]
    #[inline(always)]
    pub fn sswevt(&mut self) -> SSWEVT_W {
        SSWEVT_W { w: self }
    }
    #[doc = "Bit 31 - Store Final Value"]
    #[inline(always)]
    pub fn sfv(&mut self) -> SFV_W {
        SFV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Voltage Regulator Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vregcr](index.html) module"]
pub struct VREGCR_SPEC;
impl crate::RegisterSpec for VREGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vregcr::R](R) reader structure"]
impl crate::Readable for VREGCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vregcr::W](W) writer structure"]
impl crate::Writable for VREGCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREGCR to value 0"]
impl crate::Resettable for VREGCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
