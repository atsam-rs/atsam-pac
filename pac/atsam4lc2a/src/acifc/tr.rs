#[doc = "Register `TR` reader"]
pub struct R(crate::R<TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TR` writer"]
pub struct W(crate::W<TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_SPEC>;
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
impl From<crate::W<TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACTEST0` reader - AC0 Output Override Value"]
pub struct ACTEST0_R(crate::FieldReader<bool, bool>);
impl ACTEST0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTEST0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTEST0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTEST0` writer - AC0 Output Override Value"]
pub struct ACTEST0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST0_W<'a> {
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
#[doc = "Field `ACTEST1` reader - AC1 Output Override Value"]
pub struct ACTEST1_R(crate::FieldReader<bool, bool>);
impl ACTEST1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTEST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTEST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTEST1` writer - AC1 Output Override Value"]
pub struct ACTEST1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST1_W<'a> {
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
#[doc = "Field `ACTEST2` reader - AC2 Output Override Value"]
pub struct ACTEST2_R(crate::FieldReader<bool, bool>);
impl ACTEST2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTEST2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTEST2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTEST2` writer - AC2 Output Override Value"]
pub struct ACTEST2_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST2_W<'a> {
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
#[doc = "Field `ACTEST3` reader - AC3 Output Override Value"]
pub struct ACTEST3_R(crate::FieldReader<bool, bool>);
impl ACTEST3_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTEST3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTEST3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTEST3` writer - AC3 Output Override Value"]
pub struct ACTEST3_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST3_W<'a> {
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
#[doc = "Field `ACTEST4` reader - AC4 Output Override Value"]
pub struct ACTEST4_R(crate::FieldReader<bool, bool>);
impl ACTEST4_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTEST4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTEST4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTEST4` writer - AC4 Output Override Value"]
pub struct ACTEST4_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST4_W<'a> {
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
#[doc = "Field `ACTEST5` reader - AC5 Output Override Value"]
pub struct ACTEST5_R(crate::FieldReader<bool, bool>);
impl ACTEST5_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTEST5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTEST5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTEST5` writer - AC5 Output Override Value"]
pub struct ACTEST5_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST5_W<'a> {
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
#[doc = "Field `ACTEST6` reader - AC6 Output Override Value"]
pub struct ACTEST6_R(crate::FieldReader<bool, bool>);
impl ACTEST6_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTEST6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTEST6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTEST6` writer - AC6 Output Override Value"]
pub struct ACTEST6_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST6_W<'a> {
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
#[doc = "Field `ACTEST7` reader - AC7 Output Override Value"]
pub struct ACTEST7_R(crate::FieldReader<bool, bool>);
impl ACTEST7_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACTEST7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTEST7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACTEST7` writer - AC7 Output Override Value"]
pub struct ACTEST7_W<'a> {
    w: &'a mut W,
}
impl<'a> ACTEST7_W<'a> {
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
    #[doc = "Bit 0 - AC0 Output Override Value"]
    #[inline(always)]
    pub fn actest0(&self) -> ACTEST0_R {
        ACTEST0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - AC1 Output Override Value"]
    #[inline(always)]
    pub fn actest1(&self) -> ACTEST1_R {
        ACTEST1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AC2 Output Override Value"]
    #[inline(always)]
    pub fn actest2(&self) -> ACTEST2_R {
        ACTEST2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - AC3 Output Override Value"]
    #[inline(always)]
    pub fn actest3(&self) -> ACTEST3_R {
        ACTEST3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AC4 Output Override Value"]
    #[inline(always)]
    pub fn actest4(&self) -> ACTEST4_R {
        ACTEST4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AC5 Output Override Value"]
    #[inline(always)]
    pub fn actest5(&self) -> ACTEST5_R {
        ACTEST5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - AC6 Output Override Value"]
    #[inline(always)]
    pub fn actest6(&self) -> ACTEST6_R {
        ACTEST6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - AC7 Output Override Value"]
    #[inline(always)]
    pub fn actest7(&self) -> ACTEST7_R {
        ACTEST7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AC0 Output Override Value"]
    #[inline(always)]
    pub fn actest0(&mut self) -> ACTEST0_W {
        ACTEST0_W { w: self }
    }
    #[doc = "Bit 1 - AC1 Output Override Value"]
    #[inline(always)]
    pub fn actest1(&mut self) -> ACTEST1_W {
        ACTEST1_W { w: self }
    }
    #[doc = "Bit 2 - AC2 Output Override Value"]
    #[inline(always)]
    pub fn actest2(&mut self) -> ACTEST2_W {
        ACTEST2_W { w: self }
    }
    #[doc = "Bit 3 - AC3 Output Override Value"]
    #[inline(always)]
    pub fn actest3(&mut self) -> ACTEST3_W {
        ACTEST3_W { w: self }
    }
    #[doc = "Bit 4 - AC4 Output Override Value"]
    #[inline(always)]
    pub fn actest4(&mut self) -> ACTEST4_W {
        ACTEST4_W { w: self }
    }
    #[doc = "Bit 5 - AC5 Output Override Value"]
    #[inline(always)]
    pub fn actest5(&mut self) -> ACTEST5_W {
        ACTEST5_W { w: self }
    }
    #[doc = "Bit 6 - AC6 Output Override Value"]
    #[inline(always)]
    pub fn actest6(&mut self) -> ACTEST6_W {
        ACTEST6_W { w: self }
    }
    #[doc = "Bit 7 - AC7 Output Override Value"]
    #[inline(always)]
    pub fn actest7(&mut self) -> ACTEST7_W {
        ACTEST7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](index.html) module"]
pub struct TR_SPEC;
impl crate::RegisterSpec for TR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tr::R](R) reader structure"]
impl crate::Readable for TR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tr::W](W) writer structure"]
impl crate::Writable for TR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TR to value 0"]
impl crate::Resettable for TR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
