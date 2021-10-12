#[doc = "Register `DFLL0VAL` reader"]
pub struct R(crate::R<DFLL0VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLL0VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFLL0VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFLL0VAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLL0VAL` writer"]
pub struct W(crate::W<DFLL0VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLL0VAL_SPEC>;
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
impl From<crate::W<DFLL0VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFLL0VAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINE` reader - Fine Value"]
pub struct FINE_R(crate::FieldReader<u8, u8>);
impl FINE_R {
    pub(crate) fn new(bits: u8) -> Self {
        FINE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINE` writer - Fine Value"]
pub struct FINE_W<'a> {
    w: &'a mut W,
}
impl<'a> FINE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `COARSE` reader - Coarse Value"]
pub struct COARSE_R(crate::FieldReader<u8, u8>);
impl COARSE_R {
    pub(crate) fn new(bits: u8) -> Self {
        COARSE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COARSE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COARSE` writer - Coarse Value"]
pub struct COARSE_W<'a> {
    w: &'a mut W,
}
impl<'a> COARSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Fine Value"]
    #[inline(always)]
    pub fn fine(&self) -> FINE_R {
        FINE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - Coarse Value"]
    #[inline(always)]
    pub fn coarse(&self) -> COARSE_R {
        COARSE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fine Value"]
    #[inline(always)]
    pub fn fine(&mut self) -> FINE_W {
        FINE_W { w: self }
    }
    #[doc = "Bits 16:20 - Coarse Value"]
    #[inline(always)]
    pub fn coarse(&mut self) -> COARSE_W {
        COARSE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0val](index.html) module"]
pub struct DFLL0VAL_SPEC;
impl crate::RegisterSpec for DFLL0VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfll0val::R](R) reader structure"]
impl crate::Readable for DFLL0VAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfll0val::W](W) writer structure"]
impl crate::Writable for DFLL0VAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFLL0VAL to value 0"]
impl crate::Resettable for DFLL0VAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
