#[doc = "Register `BCFG` reader"]
pub struct R(crate::R<BCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<BCFG_SPEC>> for R {
    fn from(reader: crate::R<BCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BCFG` writer"]
pub struct W(crate::W<BCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BCFG_SPEC>;
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
impl core::convert::From<crate::W<BCFG_SPEC>> for W {
    fn from(writer: crate::W<BCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODE` reader - Blinking Mode"]
pub struct MODE_R(crate::FieldReader<bool, bool>);
impl MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Blinking Mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
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
#[doc = "Field `FCS` reader - Frame Counter Selection"]
pub struct FCS_R(crate::FieldReader<u8, u8>);
impl FCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FCS` writer - Frame Counter Selection"]
pub struct FCS_W<'a> {
    w: &'a mut W,
}
impl<'a> FCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `BSS0` reader - Blink Segment Selection 0"]
pub struct BSS0_R(crate::FieldReader<u8, u8>);
impl BSS0_R {
    pub(crate) fn new(bits: u8) -> Self {
        BSS0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSS0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSS0` writer - Blink Segment Selection 0"]
pub struct BSS0_W<'a> {
    w: &'a mut W,
}
impl<'a> BSS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `BSS1` reader - Blink Segment Selection 1"]
pub struct BSS1_R(crate::FieldReader<u8, u8>);
impl BSS1_R {
    pub(crate) fn new(bits: u8) -> Self {
        BSS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BSS1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BSS1` writer - Blink Segment Selection 1"]
pub struct BSS1_W<'a> {
    w: &'a mut W,
}
impl<'a> BSS1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Blinking Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&self) -> FCS_R {
        FCS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Blink Segment Selection 0"]
    #[inline(always)]
    pub fn bss0(&self) -> BSS0_R {
        BSS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Blink Segment Selection 1"]
    #[inline(always)]
    pub fn bss1(&self) -> BSS1_R {
        BSS1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Blinking Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 1:2 - Frame Counter Selection"]
    #[inline(always)]
    pub fn fcs(&mut self) -> FCS_W {
        FCS_W { w: self }
    }
    #[doc = "Bits 8:11 - Blink Segment Selection 0"]
    #[inline(always)]
    pub fn bss0(&mut self) -> BSS0_W {
        BSS0_W { w: self }
    }
    #[doc = "Bits 12:15 - Blink Segment Selection 1"]
    #[inline(always)]
    pub fn bss1(&mut self) -> BSS1_W {
        BSS1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Blink Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcfg](index.html) module"]
pub struct BCFG_SPEC;
impl crate::RegisterSpec for BCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bcfg::R](R) reader structure"]
impl crate::Readable for BCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bcfg::W](W) writer structure"]
impl crate::Writable for BCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BCFG to value 0"]
impl crate::Resettable for BCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
