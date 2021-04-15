#[doc = "Register `RB%s` reader"]
pub struct R(crate::R<RB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RB_SPEC>> for R {
    fn from(reader: crate::R<RB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RB%s` writer"]
pub struct W(crate::W<RB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RB_SPEC>;
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
impl core::convert::From<crate::W<RB_SPEC>> for W {
    fn from(writer: crate::W<RB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RB` reader - Register B"]
pub struct RB_R(crate::FieldReader<u16, u16>);
impl RB_R {
    pub(crate) fn new(bits: u16) -> Self {
        RB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RB_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RB` writer - Register B"]
pub struct RB_W<'a> {
    w: &'a mut W,
}
impl<'a> RB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Register B"]
    #[inline(always)]
    pub fn rb(&self) -> RB_R {
        RB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Register B"]
    #[inline(always)]
    pub fn rb(&mut self) -> RB_W {
        RB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register B Channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rb](index.html) module"]
pub struct RB_SPEC;
impl crate::RegisterSpec for RB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rb::R](R) reader structure"]
impl crate::Readable for RB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rb::W](W) writer structure"]
impl crate::Writable for RB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RB%s to value 0"]
impl crate::Resettable for RB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
