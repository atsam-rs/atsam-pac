#[doc = "Register `SAT1` reader"]
pub struct R(crate::R<SAT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAT1` writer"]
pub struct W(crate::W<SAT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAT1_SPEC>;
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
impl From<crate::W<SAT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - Specific Address 1"]
pub type ADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR` writer - Specific Address 1"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAT1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Specific Address 1"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Specific Address 1"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Specific Address 1 Top \\[47:32\\]
Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sat1](index.html) module"]
pub struct SAT1_SPEC;
impl crate::RegisterSpec for SAT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sat1::R](R) reader structure"]
impl crate::Readable for SAT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sat1::W](W) writer structure"]
impl crate::Writable for SAT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAT1 to value 0"]
impl crate::Resettable for SAT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
