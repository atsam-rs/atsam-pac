#[doc = "Register `PBDMASK` reader"]
pub struct R(crate::R<PBDMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBDMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBDMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBDMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBDMASK` writer"]
pub struct W(crate::W<PBDMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBDMASK_SPEC>;
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
impl From<crate::W<PBDMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBDMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BPM_` reader - BPM APB Clock Enable"]
pub struct BPM__R(crate::FieldReader<bool, bool>);
impl BPM__R {
    pub(crate) fn new(bits: bool) -> Self {
        BPM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BPM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BPM_` writer - BPM APB Clock Enable"]
pub struct BPM__W<'a> {
    w: &'a mut W,
}
impl<'a> BPM__W<'a> {
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
#[doc = "Field `BSCIF_` reader - BSCIF APB Clock Enable"]
pub struct BSCIF__R(crate::FieldReader<bool, bool>);
impl BSCIF__R {
    pub(crate) fn new(bits: bool) -> Self {
        BSCIF__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSCIF__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSCIF_` writer - BSCIF APB Clock Enable"]
pub struct BSCIF__W<'a> {
    w: &'a mut W,
}
impl<'a> BSCIF__W<'a> {
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
#[doc = "Field `AST_` reader - AST APB Clock Enable"]
pub struct AST__R(crate::FieldReader<bool, bool>);
impl AST__R {
    pub(crate) fn new(bits: bool) -> Self {
        AST__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AST__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AST_` writer - AST APB Clock Enable"]
pub struct AST__W<'a> {
    w: &'a mut W,
}
impl<'a> AST__W<'a> {
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
#[doc = "Field `WDT_` reader - WDT APB Clock Enable"]
pub struct WDT__R(crate::FieldReader<bool, bool>);
impl WDT__R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_` writer - WDT APB Clock Enable"]
pub struct WDT__W<'a> {
    w: &'a mut W,
}
impl<'a> WDT__W<'a> {
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
#[doc = "Field `EIC_` reader - EIC APB Clock Enable"]
pub struct EIC__R(crate::FieldReader<bool, bool>);
impl EIC__R {
    pub(crate) fn new(bits: bool) -> Self {
        EIC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIC_` writer - EIC APB Clock Enable"]
pub struct EIC__W<'a> {
    w: &'a mut W,
}
impl<'a> EIC__W<'a> {
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
#[doc = "Field `PICOUART_` reader - PICOUART APB Clock Enable"]
pub struct PICOUART__R(crate::FieldReader<bool, bool>);
impl PICOUART__R {
    pub(crate) fn new(bits: bool) -> Self {
        PICOUART__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PICOUART__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PICOUART_` writer - PICOUART APB Clock Enable"]
pub struct PICOUART__W<'a> {
    w: &'a mut W,
}
impl<'a> PICOUART__W<'a> {
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
impl R {
    #[doc = "Bit 0 - BPM APB Clock Enable"]
    #[inline(always)]
    pub fn bpm_(&self) -> BPM__R {
        BPM__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BSCIF APB Clock Enable"]
    #[inline(always)]
    pub fn bscif_(&self) -> BSCIF__R {
        BSCIF__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AST APB Clock Enable"]
    #[inline(always)]
    pub fn ast_(&self) -> AST__R {
        AST__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&self) -> WDT__R {
        WDT__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&self) -> EIC__R {
        EIC__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PICOUART APB Clock Enable"]
    #[inline(always)]
    pub fn picouart_(&self) -> PICOUART__R {
        PICOUART__R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BPM APB Clock Enable"]
    #[inline(always)]
    pub fn bpm_(&mut self) -> BPM__W {
        BPM__W { w: self }
    }
    #[doc = "Bit 1 - BSCIF APB Clock Enable"]
    #[inline(always)]
    pub fn bscif_(&mut self) -> BSCIF__W {
        BSCIF__W { w: self }
    }
    #[doc = "Bit 2 - AST APB Clock Enable"]
    #[inline(always)]
    pub fn ast_(&mut self) -> AST__W {
        AST__W { w: self }
    }
    #[doc = "Bit 3 - WDT APB Clock Enable"]
    #[inline(always)]
    pub fn wdt_(&mut self) -> WDT__W {
        WDT__W { w: self }
    }
    #[doc = "Bit 4 - EIC APB Clock Enable"]
    #[inline(always)]
    pub fn eic_(&mut self) -> EIC__W {
        EIC__W { w: self }
    }
    #[doc = "Bit 5 - PICOUART APB Clock Enable"]
    #[inline(always)]
    pub fn picouart_(&mut self) -> PICOUART__W {
        PICOUART__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PBD Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbdmask](index.html) module"]
pub struct PBDMASK_SPEC;
impl crate::RegisterSpec for PBDMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pbdmask::R](R) reader structure"]
impl crate::Readable for PBDMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbdmask::W](W) writer structure"]
impl crate::Writable for PBDMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBDMASK to value 0x3f"]
impl crate::Resettable for PBDMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f
    }
}
