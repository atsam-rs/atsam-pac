#[doc = "Register `DFLL0MUL` reader"]
pub struct R(crate::R<DFLL0MUL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFLL0MUL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<DFLL0MUL_SPEC>> for R {
    fn from(reader: crate::R<DFLL0MUL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFLL0MUL` writer"]
pub struct W(crate::W<DFLL0MUL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFLL0MUL_SPEC>;
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
impl core::convert::From<crate::W<DFLL0MUL_SPEC>> for W {
    fn from(writer: crate::W<DFLL0MUL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MUL` reader - DFLL Multiply Factor"]
pub struct MUL_R(crate::FieldReader<u16, u16>);
impl MUL_R {
    pub(crate) fn new(bits: u16) -> Self {
        MUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUL` writer - DFLL Multiply Factor"]
pub struct MUL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    pub fn mul(&self) -> MUL_R {
        MUL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DFLL Multiply Factor"]
    #[inline(always)]
    pub fn mul(&mut self) -> MUL_W {
        MUL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFLL0 Multiplier Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfll0mul](index.html) module"]
pub struct DFLL0MUL_SPEC;
impl crate::RegisterSpec for DFLL0MUL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfll0mul::R](R) reader structure"]
impl crate::Readable for DFLL0MUL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfll0mul::W](W) writer structure"]
impl crate::Writable for DFLL0MUL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFLL0MUL to value 0"]
impl crate::Resettable for DFLL0MUL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
