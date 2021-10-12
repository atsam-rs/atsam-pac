#[doc = "Register `HSBMASK` reader"]
pub struct R(crate::R<HSBMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSBMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSBMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSBMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSBMASK` writer"]
pub struct W(crate::W<HSBMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSBMASK_SPEC>;
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
impl From<crate::W<HSBMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSBMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDCA_` reader - PDCA HSB Clock Mask"]
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
#[doc = "Field `PDCA_` writer - PDCA HSB Clock Mask"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `HFLASHC_` reader - HFLASHC HSB Clock Mask"]
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
#[doc = "Field `HFLASHC_` writer - HFLASHC HSB Clock Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `HRAMC1_` reader - HRAMC1 HSB Clock Mask"]
pub struct HRAMC1__R(crate::FieldReader<bool, bool>);
impl HRAMC1__R {
    pub(crate) fn new(bits: bool) -> Self {
        HRAMC1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRAMC1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRAMC1_` writer - HRAMC1 HSB Clock Mask"]
pub struct HRAMC1__W<'a> {
    w: &'a mut W,
}
impl<'a> HRAMC1__W<'a> {
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
#[doc = "Field `USBC_` reader - USBC HSB Clock Mask"]
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
#[doc = "Field `USBC_` writer - USBC HSB Clock Mask"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CRCCU_` reader - CRCCU HSB Clock Mask"]
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
#[doc = "Field `CRCCU_` writer - CRCCU HSB Clock Mask"]
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
#[doc = "Field `HTOP0_` reader - HTOP0 HSB Clock Mask"]
pub struct HTOP0__R(crate::FieldReader<bool, bool>);
impl HTOP0__R {
    pub(crate) fn new(bits: bool) -> Self {
        HTOP0__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTOP0__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTOP0_` writer - HTOP0 HSB Clock Mask"]
pub struct HTOP0__W<'a> {
    w: &'a mut W,
}
impl<'a> HTOP0__W<'a> {
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
#[doc = "Field `HTOP1_` reader - HTOP1 HSB Clock Mask"]
pub struct HTOP1__R(crate::FieldReader<bool, bool>);
impl HTOP1__R {
    pub(crate) fn new(bits: bool) -> Self {
        HTOP1__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTOP1__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTOP1_` writer - HTOP1 HSB Clock Mask"]
pub struct HTOP1__W<'a> {
    w: &'a mut W,
}
impl<'a> HTOP1__W<'a> {
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
#[doc = "Field `HTOP2_` reader - HTOP2 HSB Clock Mask"]
pub struct HTOP2__R(crate::FieldReader<bool, bool>);
impl HTOP2__R {
    pub(crate) fn new(bits: bool) -> Self {
        HTOP2__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTOP2__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTOP2_` writer - HTOP2 HSB Clock Mask"]
pub struct HTOP2__W<'a> {
    w: &'a mut W,
}
impl<'a> HTOP2__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `HTOP3_` reader - HTOP3 HSB Clock Mask"]
pub struct HTOP3__R(crate::FieldReader<bool, bool>);
impl HTOP3__R {
    pub(crate) fn new(bits: bool) -> Self {
        HTOP3__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HTOP3__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HTOP3_` writer - HTOP3 HSB Clock Mask"]
pub struct HTOP3__W<'a> {
    w: &'a mut W,
}
impl<'a> HTOP3__W<'a> {
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
impl R {
    #[doc = "Bit 0 - PDCA HSB Clock Mask"]
    #[inline(always)]
    pub fn pdca_(&self) -> PDCA__R {
        PDCA__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFLASHC HSB Clock Mask"]
    #[inline(always)]
    pub fn hflashc_(&self) -> HFLASHC__R {
        HFLASHC__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HRAMC1 HSB Clock Mask"]
    #[inline(always)]
    pub fn hramc1_(&self) -> HRAMC1__R {
        HRAMC1__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USBC HSB Clock Mask"]
    #[inline(always)]
    pub fn usbc_(&self) -> USBC__R {
        USBC__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRCCU HSB Clock Mask"]
    #[inline(always)]
    pub fn crccu_(&self) -> CRCCU__R {
        CRCCU__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HTOP0 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop0_(&self) -> HTOP0__R {
        HTOP0__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HTOP1 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop1_(&self) -> HTOP1__R {
        HTOP1__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HTOP2 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop2_(&self) -> HTOP2__R {
        HTOP2__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - HTOP3 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop3_(&self) -> HTOP3__R {
        HTOP3__R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDCA HSB Clock Mask"]
    #[inline(always)]
    pub fn pdca_(&mut self) -> PDCA__W {
        PDCA__W { w: self }
    }
    #[doc = "Bit 1 - HFLASHC HSB Clock Mask"]
    #[inline(always)]
    pub fn hflashc_(&mut self) -> HFLASHC__W {
        HFLASHC__W { w: self }
    }
    #[doc = "Bit 2 - HRAMC1 HSB Clock Mask"]
    #[inline(always)]
    pub fn hramc1_(&mut self) -> HRAMC1__W {
        HRAMC1__W { w: self }
    }
    #[doc = "Bit 3 - USBC HSB Clock Mask"]
    #[inline(always)]
    pub fn usbc_(&mut self) -> USBC__W {
        USBC__W { w: self }
    }
    #[doc = "Bit 4 - CRCCU HSB Clock Mask"]
    #[inline(always)]
    pub fn crccu_(&mut self) -> CRCCU__W {
        CRCCU__W { w: self }
    }
    #[doc = "Bit 5 - HTOP0 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop0_(&mut self) -> HTOP0__W {
        HTOP0__W { w: self }
    }
    #[doc = "Bit 6 - HTOP1 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop1_(&mut self) -> HTOP1__W {
        HTOP1__W { w: self }
    }
    #[doc = "Bit 7 - HTOP2 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop2_(&mut self) -> HTOP2__W {
        HTOP2__W { w: self }
    }
    #[doc = "Bit 8 - HTOP3 HSB Clock Mask"]
    #[inline(always)]
    pub fn htop3_(&mut self) -> HTOP3__W {
        HTOP3__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSB Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsbmask](index.html) module"]
pub struct HSBMASK_SPEC;
impl crate::RegisterSpec for HSBMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsbmask::R](R) reader structure"]
impl crate::Readable for HSBMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsbmask::W](W) writer structure"]
impl crate::Writable for HSBMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSBMASK to value 0x01e2"]
impl crate::Resettable for HSBMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01e2
    }
}
