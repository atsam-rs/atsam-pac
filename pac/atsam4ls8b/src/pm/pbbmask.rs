#[doc = "Register `PBBMASK` reader"]
pub struct R(crate::R<PBBMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBBMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBBMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBBMASK` writer"]
pub struct W(crate::W<PBBMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBBMASK_SPEC>;
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
impl From<crate::W<PBBMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBBMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFLASHC_` reader - HFLASHC APB Clock Enable"]
pub struct HFLASHC__R(crate::FieldReader<bool, bool>);
impl HFLASHC__R {
    pub(crate) fn new(bits: bool) -> Self {
        HFLASHC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFLASHC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFLASHC_` writer - HFLASHC APB Clock Enable"]
pub struct HFLASHC__W<'a> {
    w: &'a mut W,
}
impl<'a> HFLASHC__W<'a> {
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
#[doc = "Field `HCACHE_` reader - HCACHE APB Clock Enable"]
pub struct HCACHE__R(crate::FieldReader<bool, bool>);
impl HCACHE__R {
    pub(crate) fn new(bits: bool) -> Self {
        HCACHE__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCACHE__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCACHE_` writer - HCACHE APB Clock Enable"]
pub struct HCACHE__W<'a> {
    w: &'a mut W,
}
impl<'a> HCACHE__W<'a> {
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
#[doc = "Field `HMATRIX_` reader - HMATRIX APB Clock Enable"]
pub struct HMATRIX__R(crate::FieldReader<bool, bool>);
impl HMATRIX__R {
    pub(crate) fn new(bits: bool) -> Self {
        HMATRIX__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HMATRIX__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HMATRIX_` writer - HMATRIX APB Clock Enable"]
pub struct HMATRIX__W<'a> {
    w: &'a mut W,
}
impl<'a> HMATRIX__W<'a> {
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
#[doc = "Field `PDCA_` reader - PDCA APB Clock Enable"]
pub struct PDCA__R(crate::FieldReader<bool, bool>);
impl PDCA__R {
    pub(crate) fn new(bits: bool) -> Self {
        PDCA__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDCA__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDCA_` writer - PDCA APB Clock Enable"]
pub struct PDCA__W<'a> {
    w: &'a mut W,
}
impl<'a> PDCA__W<'a> {
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
#[doc = "Field `CRCCU_` reader - CRCCU APB Clock Enable"]
pub struct CRCCU__R(crate::FieldReader<bool, bool>);
impl CRCCU__R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCCU__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCCU__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCCU_` writer - CRCCU APB Clock Enable"]
pub struct CRCCU__W<'a> {
    w: &'a mut W,
}
impl<'a> CRCCU__W<'a> {
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
#[doc = "Field `USBC_` reader - USBC APB Clock Enable"]
pub struct USBC__R(crate::FieldReader<bool, bool>);
impl USBC__R {
    pub(crate) fn new(bits: bool) -> Self {
        USBC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBC_` writer - USBC APB Clock Enable"]
pub struct USBC__W<'a> {
    w: &'a mut W,
}
impl<'a> USBC__W<'a> {
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
#[doc = "Field `PEVC_` reader - PEVC APB Clock Enable"]
pub struct PEVC__R(crate::FieldReader<bool, bool>);
impl PEVC__R {
    pub(crate) fn new(bits: bool) -> Self {
        PEVC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEVC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEVC_` writer - PEVC APB Clock Enable"]
pub struct PEVC__W<'a> {
    w: &'a mut W,
}
impl<'a> PEVC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HFLASHC APB Clock Enable"]
    #[inline(always)]
    pub fn hflashc_(&self) -> HFLASHC__R {
        HFLASHC__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HCACHE APB Clock Enable"]
    #[inline(always)]
    pub fn hcache_(&self) -> HCACHE__R {
        HCACHE__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    pub fn hmatrix_(&self) -> HMATRIX__R {
        HMATRIX__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PDCA APB Clock Enable"]
    #[inline(always)]
    pub fn pdca_(&self) -> PDCA__R {
        PDCA__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRCCU APB Clock Enable"]
    #[inline(always)]
    pub fn crccu_(&self) -> CRCCU__R {
        CRCCU__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - USBC APB Clock Enable"]
    #[inline(always)]
    pub fn usbc_(&self) -> USBC__R {
        USBC__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PEVC APB Clock Enable"]
    #[inline(always)]
    pub fn pevc_(&self) -> PEVC__R {
        PEVC__R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFLASHC APB Clock Enable"]
    #[inline(always)]
    pub fn hflashc_(&mut self) -> HFLASHC__W {
        HFLASHC__W { w: self }
    }
    #[doc = "Bit 1 - HCACHE APB Clock Enable"]
    #[inline(always)]
    pub fn hcache_(&mut self) -> HCACHE__W {
        HCACHE__W { w: self }
    }
    #[doc = "Bit 2 - HMATRIX APB Clock Enable"]
    #[inline(always)]
    pub fn hmatrix_(&mut self) -> HMATRIX__W {
        HMATRIX__W { w: self }
    }
    #[doc = "Bit 3 - PDCA APB Clock Enable"]
    #[inline(always)]
    pub fn pdca_(&mut self) -> PDCA__W {
        PDCA__W { w: self }
    }
    #[doc = "Bit 4 - CRCCU APB Clock Enable"]
    #[inline(always)]
    pub fn crccu_(&mut self) -> CRCCU__W {
        CRCCU__W { w: self }
    }
    #[doc = "Bit 5 - USBC APB Clock Enable"]
    #[inline(always)]
    pub fn usbc_(&mut self) -> USBC__W {
        USBC__W { w: self }
    }
    #[doc = "Bit 6 - PEVC APB Clock Enable"]
    #[inline(always)]
    pub fn pevc_(&mut self) -> PEVC__W {
        PEVC__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbbmask](index.html) module"]
pub struct PBBMASK_SPEC;
impl crate::RegisterSpec for PBBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbbmask::R](R) reader structure"]
impl crate::Readable for PBBMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbbmask::W](W) writer structure"]
impl crate::Writable for PBBMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBBMASK to value 0x01"]
impl crate::Resettable for PBBMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
