#[doc = "Register `UERST` reader"]
pub struct R(crate::R<UERST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UERST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UERST_SPEC>> for R {
    fn from(reader: crate::R<UERST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UERST` writer"]
pub struct W(crate::W<UERST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UERST_SPEC>;
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
impl core::convert::From<crate::W<UERST_SPEC>> for W {
    fn from(writer: crate::W<UERST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPEN0` reader - Endpoint0 Enable"]
pub struct EPEN0_R(crate::FieldReader<bool, bool>);
impl EPEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN0` writer - Endpoint0 Enable"]
pub struct EPEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN0_W<'a> {
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
#[doc = "Field `EPEN1` reader - Endpoint1 Enable"]
pub struct EPEN1_R(crate::FieldReader<bool, bool>);
impl EPEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN1` writer - Endpoint1 Enable"]
pub struct EPEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN1_W<'a> {
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
#[doc = "Field `EPEN2` reader - Endpoint2 Enable"]
pub struct EPEN2_R(crate::FieldReader<bool, bool>);
impl EPEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN2` writer - Endpoint2 Enable"]
pub struct EPEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN2_W<'a> {
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
#[doc = "Field `EPEN3` reader - Endpoint3 Enable"]
pub struct EPEN3_R(crate::FieldReader<bool, bool>);
impl EPEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN3` writer - Endpoint3 Enable"]
pub struct EPEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN3_W<'a> {
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
#[doc = "Field `EPEN4` reader - Endpoint4 Enable"]
pub struct EPEN4_R(crate::FieldReader<bool, bool>);
impl EPEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN4` writer - Endpoint4 Enable"]
pub struct EPEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN4_W<'a> {
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
#[doc = "Field `EPEN5` reader - Endpoint5 Enable"]
pub struct EPEN5_R(crate::FieldReader<bool, bool>);
impl EPEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN5` writer - Endpoint5 Enable"]
pub struct EPEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN5_W<'a> {
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
#[doc = "Field `EPEN6` reader - Endpoint6 Enable"]
pub struct EPEN6_R(crate::FieldReader<bool, bool>);
impl EPEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN6` writer - Endpoint6 Enable"]
pub struct EPEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN6_W<'a> {
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
#[doc = "Field `EPEN7` reader - Endpoint7 Enable"]
pub struct EPEN7_R(crate::FieldReader<bool, bool>);
impl EPEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPEN7` writer - Endpoint7 Enable"]
pub struct EPEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> EPEN7_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Endpoint0 Enable"]
    #[inline(always)]
    pub fn epen0(&self) -> EPEN0_R {
        EPEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint1 Enable"]
    #[inline(always)]
    pub fn epen1(&self) -> EPEN1_R {
        EPEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint2 Enable"]
    #[inline(always)]
    pub fn epen2(&self) -> EPEN2_R {
        EPEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint3 Enable"]
    #[inline(always)]
    pub fn epen3(&self) -> EPEN3_R {
        EPEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint4 Enable"]
    #[inline(always)]
    pub fn epen4(&self) -> EPEN4_R {
        EPEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint5 Enable"]
    #[inline(always)]
    pub fn epen5(&self) -> EPEN5_R {
        EPEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint6 Enable"]
    #[inline(always)]
    pub fn epen6(&self) -> EPEN6_R {
        EPEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint7 Enable"]
    #[inline(always)]
    pub fn epen7(&self) -> EPEN7_R {
        EPEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint0 Enable"]
    #[inline(always)]
    pub fn epen0(&mut self) -> EPEN0_W {
        EPEN0_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint1 Enable"]
    #[inline(always)]
    pub fn epen1(&mut self) -> EPEN1_W {
        EPEN1_W { w: self }
    }
    #[doc = "Bit 2 - Endpoint2 Enable"]
    #[inline(always)]
    pub fn epen2(&mut self) -> EPEN2_W {
        EPEN2_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint3 Enable"]
    #[inline(always)]
    pub fn epen3(&mut self) -> EPEN3_W {
        EPEN3_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint4 Enable"]
    #[inline(always)]
    pub fn epen4(&mut self) -> EPEN4_W {
        EPEN4_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint5 Enable"]
    #[inline(always)]
    pub fn epen5(&mut self) -> EPEN5_W {
        EPEN5_W { w: self }
    }
    #[doc = "Bit 6 - Endpoint6 Enable"]
    #[inline(always)]
    pub fn epen6(&mut self) -> EPEN6_W {
        EPEN6_W { w: self }
    }
    #[doc = "Bit 7 - Endpoint7 Enable"]
    #[inline(always)]
    pub fn epen7(&mut self) -> EPEN7_W {
        EPEN7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Enable/Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uerst](index.html) module"]
pub struct UERST_SPEC;
impl crate::RegisterSpec for UERST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uerst::R](R) reader structure"]
impl crate::Readable for UERST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uerst::W](W) writer structure"]
impl crate::Writable for UERST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UERST to value 0"]
impl crate::Resettable for UERST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
