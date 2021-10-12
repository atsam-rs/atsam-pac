#[doc = "Register `RA0` reader"]
pub struct R(crate::R<RA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RA0` writer"]
pub struct W(crate::W<RA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RA0_SPEC>;
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
impl From<crate::W<RA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RA` reader - Register A"]
pub struct RA_R(crate::FieldReader<u32, u32>);
impl RA_R {
    pub(crate) fn new(bits: u32) -> Self {
        RA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RA` writer - Register A"]
pub struct RA_W<'a> {
    w: &'a mut W,
}
impl<'a> RA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Register A"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Register A"]
    #[inline(always)]
    pub fn ra(&mut self) -> RA_W {
        RA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register A (channel = 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ra0](index.html) module"]
pub struct RA0_SPEC;
impl crate::RegisterSpec for RA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ra0::R](R) reader structure"]
impl crate::Readable for RA0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ra0::W](W) writer structure"]
impl crate::Writable for RA0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RA0 to value 0"]
impl crate::Resettable for RA0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
