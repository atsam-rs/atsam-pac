#[doc = "Register `HSTR` reader"]
pub struct R(crate::R<HSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<HSTR_SPEC>> for R {
    fn from(reader: crate::R<HSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTR` writer"]
pub struct W(crate::W<HSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTR_SPEC>;
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
impl core::convert::From<crate::W<HSTR_SPEC>> for W {
    fn from(writer: crate::W<HSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDDAT` reader - Data Hold Cycles"]
pub struct HDDAT_R(crate::FieldReader<u8, u8>);
impl HDDAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        HDDAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HDDAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HDDAT` writer - Data Hold Cycles"]
pub struct HDDAT_W<'a> {
    w: &'a mut W,
}
impl<'a> HDDAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:23 - Data Hold Cycles"]
    #[inline(always)]
    pub fn hddat(&self) -> HDDAT_R {
        HDDAT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 16:23 - Data Hold Cycles"]
    #[inline(always)]
    pub fn hddat(&mut self) -> HDDAT_W {
        HDDAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HS-mode Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstr](index.html) module"]
pub struct HSTR_SPEC;
impl crate::RegisterSpec for HSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstr::R](R) reader structure"]
impl crate::Readable for HSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstr::W](W) writer structure"]
impl crate::Writable for HSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSTR to value 0"]
impl crate::Resettable for HSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
