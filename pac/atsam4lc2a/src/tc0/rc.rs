#[doc = "Register `RC%s` reader"]
pub struct R(crate::R<RC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<RC_SPEC>> for R {
    fn from(reader: crate::R<RC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RC%s` writer"]
pub struct W(crate::W<RC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RC_SPEC>;
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
impl core::convert::From<crate::W<RC_SPEC>> for W {
    fn from(writer: crate::W<RC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RC` reader - Register C"]
pub struct RC_R(crate::FieldReader<u16, u16>);
impl RC_R {
    pub(crate) fn new(bits: u16) -> Self {
        RC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RC` writer - Register C"]
pub struct RC_W<'a> {
    w: &'a mut W,
}
impl<'a> RC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Register C"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Register C"]
    #[inline(always)]
    pub fn rc(&mut self) -> RC_W {
        RC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Register C Channel\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rc](index.html) module"]
pub struct RC_SPEC;
impl crate::RegisterSpec for RC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rc::R](R) reader structure"]
impl crate::Readable for RC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rc::W](W) writer structure"]
impl crate::Writable for RC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RC%s to value 0"]
impl crate::Resettable for RC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
