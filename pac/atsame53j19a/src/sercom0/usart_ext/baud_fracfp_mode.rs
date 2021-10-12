#[doc = "Register `BAUD_FRACFP_MODE` reader"]
pub struct R(crate::R<BAUD_FRACFP_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAUD_FRACFP_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAUD_FRACFP_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAUD_FRACFP_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAUD_FRACFP_MODE` writer"]
pub struct W(crate::W<BAUD_FRACFP_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAUD_FRACFP_MODE_SPEC>;
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
impl From<crate::W<BAUD_FRACFP_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAUD_FRACFP_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BAUD` reader - Baud Rate Value"]
pub struct BAUD_R(crate::FieldReader<u16, u16>);
impl BAUD_R {
    pub(crate) fn new(bits: u16) -> Self {
        BAUD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAUD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAUD` writer - Baud Rate Value"]
pub struct BAUD_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1fff) | (value as u16 & 0x1fff);
        self.w
    }
}
#[doc = "Field `FP` reader - Fractional Part"]
pub struct FP_R(crate::FieldReader<u8, u8>);
impl FP_R {
    pub(crate) fn new(bits: u8) -> Self {
        FP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FP` writer - Fractional Part"]
pub struct FP_W<'a> {
    w: &'a mut W,
}
impl<'a> FP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u16 & 0x07) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&self) -> BAUD_R {
        BAUD_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&self) -> FP_R {
        FP_R::new(((self.bits >> 13) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - Baud Rate Value"]
    #[inline(always)]
    pub fn baud(&mut self) -> BAUD_W {
        BAUD_W { w: self }
    }
    #[doc = "Bits 13:15 - Fractional Part"]
    #[inline(always)]
    pub fn fp(&mut self) -> FP_W {
        FP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART_EXT Baud Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [baud_fracfp_mode](index.html) module"]
pub struct BAUD_FRACFP_MODE_SPEC;
impl crate::RegisterSpec for BAUD_FRACFP_MODE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [baud_fracfp_mode::R](R) reader structure"]
impl crate::Readable for BAUD_FRACFP_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [baud_fracfp_mode::W](W) writer structure"]
impl crate::Writable for BAUD_FRACFP_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BAUD_FRACFP_MODE to value 0"]
impl crate::Resettable for BAUD_FRACFP_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
