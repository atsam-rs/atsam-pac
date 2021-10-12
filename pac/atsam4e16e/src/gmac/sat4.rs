#[doc = "Register `SAT4` reader"]
pub struct R(crate::R<SAT4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAT4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAT4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAT4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAT4` writer"]
pub struct W(crate::W<SAT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAT4_SPEC>;
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
impl From<crate::W<SAT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Specific Address 4"]
pub struct ADDR_R(crate::FieldReader<u16, u16>);
impl ADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDR` writer - Specific Address 4"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Specific Address 4"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific Address 4"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Specific Address 4 Top \\[47:32\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sat4](index.html) module"]
pub struct SAT4_SPEC;
impl crate::RegisterSpec for SAT4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sat4::R](R) reader structure"]
impl crate::Readable for SAT4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sat4::W](W) writer structure"]
impl crate::Writable for SAT4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAT4 to value 0"]
impl crate::Resettable for SAT4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
