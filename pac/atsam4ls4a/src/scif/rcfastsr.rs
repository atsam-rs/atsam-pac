#[doc = "Register `RCFASTSR` reader"]
pub struct R(crate::R<RCFASTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCFASTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCFASTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCFASTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCFASTSR` writer"]
pub struct W(crate::W<RCFASTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCFASTSR_SPEC>;
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
impl From<crate::W<RCFASTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCFASTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURTRIM` reader - Current Trim Value"]
pub struct CURTRIM_R(crate::FieldReader<u8, u8>);
impl CURTRIM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CURTRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURTRIM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURTRIM` writer - Current Trim Value"]
pub struct CURTRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> CURTRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `CNTERR` reader - Current Count Error"]
pub struct CNTERR_R(crate::FieldReader<u8, u8>);
impl CNTERR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CNTERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTERR` writer - Current Count Error"]
pub struct CNTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTERR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `SIGN` reader - Sign of Current Count Error"]
pub struct SIGN_R(crate::FieldReader<bool, bool>);
impl SIGN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIGN` writer - Sign of Current Count Error"]
pub struct SIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `LOCK` reader - Lock"]
pub struct LOCK_R(crate::FieldReader<bool, bool>);
impl LOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCK` writer - Lock"]
pub struct LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `LOCKLOST` reader - Lock Lost"]
pub struct LOCKLOST_R(crate::FieldReader<bool, bool>);
impl LOCKLOST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKLOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKLOST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKLOST` writer - Lock Lost"]
pub struct LOCKLOST_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKLOST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `UPDATED` reader - Current Trim Value Updated"]
pub struct UPDATED_R(crate::FieldReader<bool, bool>);
impl UPDATED_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPDATED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPDATED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPDATED` writer - Current Trim Value Updated"]
pub struct UPDATED_W<'a> {
    w: &'a mut W,
}
impl<'a> UPDATED_W<'a> {
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
    #[doc = "Bits 0:6 - Current Trim Value"]
    #[inline(always)]
    pub fn curtrim(&self) -> CURTRIM_R {
        CURTRIM_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - Current Count Error"]
    #[inline(always)]
    pub fn cnterr(&self) -> CNTERR_R {
        CNTERR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - Sign of Current Count Error"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Lock Lost"]
    #[inline(always)]
    pub fn locklost(&self) -> LOCKLOST_R {
        LOCKLOST_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Current Trim Value Updated"]
    #[inline(always)]
    pub fn updated(&self) -> UPDATED_R {
        UPDATED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Current Trim Value"]
    #[inline(always)]
    pub fn curtrim(&mut self) -> CURTRIM_W {
        CURTRIM_W { w: self }
    }
    #[doc = "Bits 16:20 - Current Count Error"]
    #[inline(always)]
    pub fn cnterr(&mut self) -> CNTERR_W {
        CNTERR_W { w: self }
    }
    #[doc = "Bit 21 - Sign of Current Count Error"]
    #[inline(always)]
    pub fn sign(&mut self) -> SIGN_W {
        SIGN_W { w: self }
    }
    #[doc = "Bit 24 - Lock"]
    #[inline(always)]
    pub fn lock(&mut self) -> LOCK_W {
        LOCK_W { w: self }
    }
    #[doc = "Bit 25 - Lock Lost"]
    #[inline(always)]
    pub fn locklost(&mut self) -> LOCKLOST_W {
        LOCKLOST_W { w: self }
    }
    #[doc = "Bit 31 - Current Trim Value Updated"]
    #[inline(always)]
    pub fn updated(&mut self) -> UPDATED_W {
        UPDATED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "4/8/12 MHz RC Oscillator Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcfastsr](index.html) module"]
pub struct RCFASTSR_SPEC;
impl crate::RegisterSpec for RCFASTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcfastsr::R](R) reader structure"]
impl crate::Readable for RCFASTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcfastsr::W](W) writer structure"]
impl crate::Writable for RCFASTSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCFASTSR to value 0"]
impl crate::Resettable for RCFASTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
