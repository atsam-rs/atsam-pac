#[doc = "Register `VCR0` reader"]
pub struct R(crate::R<VCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VCR0` writer"]
pub struct W(crate::W<VCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VCR0_SPEC>;
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
impl From<crate::W<VCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VOLUME` reader - Volume Control"]
pub struct VOLUME_R(crate::FieldReader<u16, u16>);
impl VOLUME_R {
    pub(crate) fn new(bits: u16) -> Self {
        VOLUME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VOLUME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VOLUME` writer - Volume Control"]
pub struct VOLUME_W<'a> {
    w: &'a mut W,
}
impl<'a> VOLUME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | (value as u32 & 0x7fff);
        self.w
    }
}
#[doc = "Field `MUTE` reader - Mute"]
pub struct MUTE_R(crate::FieldReader<bool, bool>);
impl MUTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUTE` writer - Mute"]
pub struct MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_W<'a> {
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
    #[doc = "Bits 0:14 - Volume Control"]
    #[inline(always)]
    pub fn volume(&self) -> VOLUME_R {
        VOLUME_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Mute"]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - Volume Control"]
    #[inline(always)]
    pub fn volume(&mut self) -> VOLUME_W {
        VOLUME_W { w: self }
    }
    #[doc = "Bit 31 - Mute"]
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W {
        MUTE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Volume Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vcr0](index.html) module"]
pub struct VCR0_SPEC;
impl crate::RegisterSpec for VCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vcr0::R](R) reader structure"]
impl crate::Readable for VCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vcr0::W](W) writer structure"]
impl crate::Writable for VCR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VCR0 to value 0"]
impl crate::Resettable for VCR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
