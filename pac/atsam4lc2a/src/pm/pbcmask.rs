#[doc = "Register `PBCMASK` reader"]
pub struct R(crate::R<PBCMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBCMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<PBCMASK_SPEC>> for R {
    fn from(reader: crate::R<PBCMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBCMASK` writer"]
pub struct W(crate::W<PBCMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBCMASK_SPEC>;
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
impl core::convert::From<crate::W<PBCMASK_SPEC>> for W {
    fn from(writer: crate::W<PBCMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PM_` reader - PM APB Clock Enable"]
pub struct PM__R(crate::FieldReader<bool, bool>);
impl PM__R {
    pub(crate) fn new(bits: bool) -> Self {
        PM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM_` writer - PM APB Clock Enable"]
pub struct PM__W<'a> {
    w: &'a mut W,
}
impl<'a> PM__W<'a> {
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
#[doc = "Field `CHIPID_` reader - CHIPID APB Clock Enable"]
pub struct CHIPID__R(crate::FieldReader<bool, bool>);
impl CHIPID__R {
    pub(crate) fn new(bits: bool) -> Self {
        CHIPID__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CHIPID__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHIPID_` writer - CHIPID APB Clock Enable"]
pub struct CHIPID__W<'a> {
    w: &'a mut W,
}
impl<'a> CHIPID__W<'a> {
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
#[doc = "Field `SCIF_` reader - SCIF APB Clock Enable"]
pub struct SCIF__R(crate::FieldReader<bool, bool>);
impl SCIF__R {
    pub(crate) fn new(bits: bool) -> Self {
        SCIF__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCIF__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCIF_` writer - SCIF APB Clock Enable"]
pub struct SCIF__W<'a> {
    w: &'a mut W,
}
impl<'a> SCIF__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `FREQM_` reader - FREQM APB Clock Enable"]
pub struct FREQM__R(crate::FieldReader<bool, bool>);
impl FREQM__R {
    pub(crate) fn new(bits: bool) -> Self {
        FREQM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQM_` writer - FREQM APB Clock Enable"]
pub struct FREQM__W<'a> {
    w: &'a mut W,
}
impl<'a> FREQM__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `GPIO_` reader - GPIO APB Clock Enable"]
pub struct GPIO__R(crate::FieldReader<bool, bool>);
impl GPIO__R {
    pub(crate) fn new(bits: bool) -> Self {
        GPIO__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPIO_` writer - GPIO APB Clock Enable"]
pub struct GPIO__W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&self) -> PM__R {
        PM__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CHIPID APB Clock Enable"]
    #[inline(always)]
    pub fn chipid_(&self) -> CHIPID__R {
        CHIPID__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SCIF APB Clock Enable"]
    #[inline(always)]
    pub fn scif_(&self) -> SCIF__R {
        SCIF__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FREQM APB Clock Enable"]
    #[inline(always)]
    pub fn freqm_(&self) -> FREQM__R {
        FREQM__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - GPIO APB Clock Enable"]
    #[inline(always)]
    pub fn gpio_(&self) -> GPIO__R {
        GPIO__R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PM APB Clock Enable"]
    #[inline(always)]
    pub fn pm_(&mut self) -> PM__W {
        PM__W { w: self }
    }
    #[doc = "Bit 1 - CHIPID APB Clock Enable"]
    #[inline(always)]
    pub fn chipid_(&mut self) -> CHIPID__W {
        CHIPID__W { w: self }
    }
    #[doc = "Bit 2 - SCIF APB Clock Enable"]
    #[inline(always)]
    pub fn scif_(&mut self) -> SCIF__W {
        SCIF__W { w: self }
    }
    #[doc = "Bit 3 - FREQM APB Clock Enable"]
    #[inline(always)]
    pub fn freqm_(&mut self) -> FREQM__W {
        FREQM__W { w: self }
    }
    #[doc = "Bit 4 - GPIO APB Clock Enable"]
    #[inline(always)]
    pub fn gpio_(&mut self) -> GPIO__W {
        GPIO__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBC Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbcmask](index.html) module"]
pub struct PBCMASK_SPEC;
impl crate::RegisterSpec for PBCMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbcmask::R](R) reader structure"]
impl crate::Readable for PBCMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbcmask::W](W) writer structure"]
impl crate::Writable for PBCMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBCMASK to value 0x1f"]
impl crate::Resettable for PBCMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
