#[doc = "Register `DATABUFPTR` reader"]
pub struct R(crate::R<DATABUFPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATABUFPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATABUFPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATABUFPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATABUFPTR` writer"]
pub struct W(crate::W<DATABUFPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATABUFPTR_SPEC>;
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
impl From<crate::W<DATABUFPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATABUFPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDATAW` reader - Input Data Word"]
pub struct IDATAW_R(crate::FieldReader<u8, u8>);
impl IDATAW_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDATAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDATAW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDATAW` writer - Input Data Word"]
pub struct IDATAW_W<'a> {
    w: &'a mut W,
}
impl<'a> IDATAW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `ODATAW` reader - Output Data Word"]
pub struct ODATAW_R(crate::FieldReader<u8, u8>);
impl ODATAW_R {
    pub(crate) fn new(bits: u8) -> Self {
        ODATAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODATAW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODATAW` writer - Output Data Word"]
pub struct ODATAW_W<'a> {
    w: &'a mut W,
}
impl<'a> ODATAW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Input Data Word"]
    #[inline(always)]
    pub fn idataw(&self) -> IDATAW_R {
        IDATAW_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Output Data Word"]
    #[inline(always)]
    pub fn odataw(&self) -> ODATAW_R {
        ODATAW_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Data Word"]
    #[inline(always)]
    pub fn idataw(&mut self) -> IDATAW_W {
        IDATAW_W { w: self }
    }
    #[doc = "Bits 4:5 - Output Data Word"]
    #[inline(always)]
    pub fn odataw(&mut self) -> ODATAW_W {
        ODATAW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Buffer Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [databufptr](index.html) module"]
pub struct DATABUFPTR_SPEC;
impl crate::RegisterSpec for DATABUFPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [databufptr::R](R) reader structure"]
impl crate::Readable for DATABUFPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [databufptr::W](W) writer structure"]
impl crate::Writable for DATABUFPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATABUFPTR to value 0"]
impl crate::Resettable for DATABUFPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
