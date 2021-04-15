#[doc = "Register `UPRST` reader"]
pub struct R(crate::R<UPRST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPRST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<UPRST_SPEC>> for R {
    fn from(reader: crate::R<UPRST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPRST` writer"]
pub struct W(crate::W<UPRST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPRST_SPEC>;
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
impl core::convert::From<crate::W<UPRST_SPEC>> for W {
    fn from(writer: crate::W<UPRST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PEN0` reader - Pipe0 Enable"]
pub struct PEN0_R(crate::FieldReader<bool, bool>);
impl PEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN0` writer - Pipe0 Enable"]
pub struct PEN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN0_W<'a> {
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
#[doc = "Field `PEN1` reader - Pipe1 Enable"]
pub struct PEN1_R(crate::FieldReader<bool, bool>);
impl PEN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN1` writer - Pipe1 Enable"]
pub struct PEN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN1_W<'a> {
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
#[doc = "Field `PEN2` reader - Pipe2 Enable"]
pub struct PEN2_R(crate::FieldReader<bool, bool>);
impl PEN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN2` writer - Pipe2 Enable"]
pub struct PEN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN2_W<'a> {
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
#[doc = "Field `PEN3` reader - Pipe3 Enable"]
pub struct PEN3_R(crate::FieldReader<bool, bool>);
impl PEN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN3` writer - Pipe3 Enable"]
pub struct PEN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN3_W<'a> {
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
#[doc = "Field `PEN4` reader - Pipe4 Enable"]
pub struct PEN4_R(crate::FieldReader<bool, bool>);
impl PEN4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN4` writer - Pipe4 Enable"]
pub struct PEN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN4_W<'a> {
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
#[doc = "Field `PEN5` reader - Pipe5 Enable"]
pub struct PEN5_R(crate::FieldReader<bool, bool>);
impl PEN5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN5` writer - Pipe5 Enable"]
pub struct PEN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN5_W<'a> {
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
#[doc = "Field `PEN6` reader - Pipe6 Enable"]
pub struct PEN6_R(crate::FieldReader<bool, bool>);
impl PEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN6` writer - Pipe6 Enable"]
pub struct PEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN6_W<'a> {
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
#[doc = "Field `PEN7` reader - Pipe7 Enable"]
pub struct PEN7_R(crate::FieldReader<bool, bool>);
impl PEN7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEN7` writer - Pipe7 Enable"]
pub struct PEN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PEN7_W<'a> {
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
    #[doc = "Bit 0 - Pipe0 Enable"]
    #[inline(always)]
    pub fn pen0(&self) -> PEN0_R {
        PEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pipe1 Enable"]
    #[inline(always)]
    pub fn pen1(&self) -> PEN1_R {
        PEN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pipe2 Enable"]
    #[inline(always)]
    pub fn pen2(&self) -> PEN2_R {
        PEN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pipe3 Enable"]
    #[inline(always)]
    pub fn pen3(&self) -> PEN3_R {
        PEN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pipe4 Enable"]
    #[inline(always)]
    pub fn pen4(&self) -> PEN4_R {
        PEN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pipe5 Enable"]
    #[inline(always)]
    pub fn pen5(&self) -> PEN5_R {
        PEN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Pipe6 Enable"]
    #[inline(always)]
    pub fn pen6(&self) -> PEN6_R {
        PEN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Pipe7 Enable"]
    #[inline(always)]
    pub fn pen7(&self) -> PEN7_R {
        PEN7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pipe0 Enable"]
    #[inline(always)]
    pub fn pen0(&mut self) -> PEN0_W {
        PEN0_W { w: self }
    }
    #[doc = "Bit 1 - Pipe1 Enable"]
    #[inline(always)]
    pub fn pen1(&mut self) -> PEN1_W {
        PEN1_W { w: self }
    }
    #[doc = "Bit 2 - Pipe2 Enable"]
    #[inline(always)]
    pub fn pen2(&mut self) -> PEN2_W {
        PEN2_W { w: self }
    }
    #[doc = "Bit 3 - Pipe3 Enable"]
    #[inline(always)]
    pub fn pen3(&mut self) -> PEN3_W {
        PEN3_W { w: self }
    }
    #[doc = "Bit 4 - Pipe4 Enable"]
    #[inline(always)]
    pub fn pen4(&mut self) -> PEN4_W {
        PEN4_W { w: self }
    }
    #[doc = "Bit 5 - Pipe5 Enable"]
    #[inline(always)]
    pub fn pen5(&mut self) -> PEN5_W {
        PEN5_W { w: self }
    }
    #[doc = "Bit 6 - Pipe6 Enable"]
    #[inline(always)]
    pub fn pen6(&mut self) -> PEN6_W {
        PEN6_W { w: self }
    }
    #[doc = "Bit 7 - Pipe7 Enable"]
    #[inline(always)]
    pub fn pen7(&mut self) -> PEN7_W {
        PEN7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pipe Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uprst](index.html) module"]
pub struct UPRST_SPEC;
impl crate::RegisterSpec for UPRST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uprst::R](R) reader structure"]
impl crate::Readable for UPRST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uprst::W](W) writer structure"]
impl crate::Writable for UPRST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UPRST to value 0"]
impl crate::Resettable for UPRST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
