#[doc = "Register `BOD33LEVEL` reader"]
pub struct R(crate::R<BOD33LEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOD33LEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOD33LEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOD33LEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOD33LEVEL` writer"]
pub struct W(crate::W<BOD33LEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOD33LEVEL_SPEC>;
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
impl From<crate::W<BOD33LEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOD33LEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL` reader - BOD Value"]
pub struct VAL_R(crate::FieldReader<u8, u8>);
impl VAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VAL` writer - BOD Value"]
pub struct VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> VAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - BOD Value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - BOD Value"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W {
        VAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BOD33 Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bod33level](index.html) module"]
pub struct BOD33LEVEL_SPEC;
impl crate::RegisterSpec for BOD33LEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bod33level::R](R) reader structure"]
impl crate::Readable for BOD33LEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bod33level::W](W) writer structure"]
impl crate::Writable for BOD33LEVEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BOD33LEVEL to value 0"]
impl crate::Resettable for BOD33LEVEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
