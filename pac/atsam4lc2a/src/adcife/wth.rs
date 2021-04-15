#[doc = "Register `WTH` reader"]
pub struct R(crate::R<WTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WTH_SPEC>> for R {
    fn from(reader: crate::R<WTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WTH` writer"]
pub struct W(crate::W<WTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WTH_SPEC>;
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
impl core::convert::From<crate::W<WTH_SPEC>> for W {
    fn from(writer: crate::W<WTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LT` reader - Low threshold"]
pub struct LT_R(crate::FieldReader<u16, u16>);
impl LT_R {
    pub(crate) fn new(bits: u16) -> Self {
        LT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LT` writer - Low threshold"]
pub struct LT_W<'a> {
    w: &'a mut W,
}
impl<'a> LT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `HT` reader - High Threshold"]
pub struct HT_R(crate::FieldReader<u16, u16>);
impl HT_R {
    pub(crate) fn new(bits: u16) -> Self {
        HT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HT` writer - High Threshold"]
pub struct HT_W<'a> {
    w: &'a mut W,
}
impl<'a> HT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - Low threshold"]
    #[inline(always)]
    pub fn lt(&self) -> LT_R {
        LT_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Low threshold"]
    #[inline(always)]
    pub fn lt(&mut self) -> LT_W {
        LT_W { w: self }
    }
    #[doc = "Bits 16:27 - High Threshold"]
    #[inline(always)]
    pub fn ht(&mut self) -> HT_W {
        HT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Window Monitor Threshold Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wth](index.html) module"]
pub struct WTH_SPEC;
impl crate::RegisterSpec for WTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wth::R](R) reader structure"]
impl crate::Readable for WTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wth::W](W) writer structure"]
impl crate::Writable for WTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WTH to value 0"]
impl crate::Resettable for WTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
