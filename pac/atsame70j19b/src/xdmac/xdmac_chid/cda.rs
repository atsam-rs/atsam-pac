#[doc = "Register `CDA` reader"]
pub struct R(crate::R<CDA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDA` writer"]
pub struct W(crate::W<CDA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDA_SPEC>;
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
impl From<crate::W<CDA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DA` reader - Channel x Destination Address"]
pub struct DA_R(crate::FieldReader<u32, u32>);
impl DA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DA` writer - Channel x Destination Address"]
pub struct DA_W<'a> {
    w: &'a mut W,
}
impl<'a> DA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Destination Address"]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W {
        DA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Destination Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cda](index.html) module"]
pub struct CDA_SPEC;
impl crate::RegisterSpec for CDA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cda::R](R) reader structure"]
impl crate::Readable for CDA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cda::W](W) writer structure"]
impl crate::Writable for CDA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDA to value 0"]
impl crate::Resettable for CDA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
