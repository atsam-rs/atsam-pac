#[doc = "Register `MATRIX_WPMR` reader"]
pub struct R(crate::R<MATRIX_WPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATRIX_WPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATRIX_WPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATRIX_WPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATRIX_WPMR` writer"]
pub struct W(crate::W<MATRIX_WPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATRIX_WPMR_SPEC>;
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
impl From<crate::W<MATRIX_WPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATRIX_WPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WPEN` reader - Write Protect ENable"]
pub struct WPEN_R(crate::FieldReader<bool, bool>);
impl WPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPEN` writer - Write Protect ENable"]
pub struct WPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WPEN_W<'a> {
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
#[doc = "Field `WPKEY` reader - Write Protect KEY (Write-only)"]
pub struct WPKEY_R(crate::FieldReader<u32, u32>);
impl WPKEY_R {
    pub(crate) fn new(bits: u32) -> Self {
        WPKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WPKEY_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WPKEY` writer - Write Protect KEY (Write-only)"]
pub struct WPKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WPKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | ((value as u32 & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write Protect ENable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY (Write-only)"]
    #[inline(always)]
    pub fn wpkey(&self) -> WPKEY_R {
        WPKEY_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protect ENable"]
    #[inline(always)]
    pub fn wpen(&mut self) -> WPEN_W {
        WPEN_W { w: self }
    }
    #[doc = "Bits 8:31 - Write Protect KEY (Write-only)"]
    #[inline(always)]
    pub fn wpkey(&mut self) -> WPKEY_W {
        WPKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Protect Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_wpmr](index.html) module"]
pub struct MATRIX_WPMR_SPEC;
impl crate::RegisterSpec for MATRIX_WPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_wpmr::R](R) reader structure"]
impl crate::Readable for MATRIX_WPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrix_wpmr::W](W) writer structure"]
impl crate::Writable for MATRIX_WPMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MATRIX_WPMR to value 0"]
impl crate::Resettable for MATRIX_WPMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
