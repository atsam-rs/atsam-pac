#[doc = "Register `DELAYR` reader"]
pub struct R(crate::R<DELAYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DELAYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DELAYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DELAYR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DELAYR` writer"]
pub struct W(crate::W<DELAYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DELAYR_SPEC>;
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
impl From<crate::W<DELAYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DELAYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Delay0` reader - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY0_R(crate::FieldReader<u8, u8>);
impl DELAY0_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELAY0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Delay0` writer - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY0_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `Delay1` reader - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY1_R(crate::FieldReader<u8, u8>);
impl DELAY1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELAY1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Delay1` writer - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY1_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `Delay2` reader - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY2_R(crate::FieldReader<u8, u8>);
impl DELAY2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELAY2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Delay2` writer - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY2_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `Delay3` reader - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY3_R(crate::FieldReader<u8, u8>);
impl DELAY3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELAY3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Delay3` writer - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY3_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `Delay4` reader - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY4_R(crate::FieldReader<u8, u8>);
impl DELAY4_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELAY4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Delay4` writer - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY4_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `Delay5` reader - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY5_R(crate::FieldReader<u8, u8>);
impl DELAY5_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELAY5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Delay5` writer - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY5_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `Delay6` reader - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY6_R(crate::FieldReader<u8, u8>);
impl DELAY6_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELAY6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Delay6` writer - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY6_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `Delay7` reader - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY7_R(crate::FieldReader<u8, u8>);
impl DELAY7_R {
    pub(crate) fn new(bits: u8) -> Self {
        DELAY7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Delay7` writer - Delay Control for Simultaneous Switch Reduction"]
pub struct DELAY7_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay0(&self) -> DELAY0_R {
        DELAY0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay1(&self) -> DELAY1_R {
        DELAY1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay2(&self) -> DELAY2_R {
        DELAY2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay3(&self) -> DELAY3_R {
        DELAY3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay4(&self) -> DELAY4_R {
        DELAY4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay5(&self) -> DELAY5_R {
        DELAY5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay6(&self) -> DELAY6_R {
        DELAY6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay7(&self) -> DELAY7_R {
        DELAY7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay0(&mut self) -> DELAY0_W {
        DELAY0_W { w: self }
    }
    #[doc = "Bits 4:7 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay1(&mut self) -> DELAY1_W {
        DELAY1_W { w: self }
    }
    #[doc = "Bits 8:11 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay2(&mut self) -> DELAY2_W {
        DELAY2_W { w: self }
    }
    #[doc = "Bits 12:15 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay3(&mut self) -> DELAY3_W {
        DELAY3_W { w: self }
    }
    #[doc = "Bits 16:19 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay4(&mut self) -> DELAY4_W {
        DELAY4_W { w: self }
    }
    #[doc = "Bits 20:23 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay5(&mut self) -> DELAY5_W {
        DELAY5_W { w: self }
    }
    #[doc = "Bits 24:27 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay6(&mut self) -> DELAY6_W {
        DELAY6_W { w: self }
    }
    #[doc = "Bits 28:31 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay7(&mut self) -> DELAY7_W {
        DELAY7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [delayr](index.html) module"]
pub struct DELAYR_SPEC;
impl crate::RegisterSpec for DELAYR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [delayr::R](R) reader structure"]
impl crate::Readable for DELAYR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [delayr::W](W) writer structure"]
impl crate::Writable for DELAYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DELAYR to value 0"]
impl crate::Resettable for DELAYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
