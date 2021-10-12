#[doc = "Register `SMR` reader"]
pub struct R(crate::R<SMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMR` writer"]
pub struct W(crate::W<SMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMR_SPEC>;
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
impl From<crate::W<SMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADR` reader - Slave Address"]
pub struct SADR_R(crate::FieldReader<u8, u8>);
impl SADR_R {
    pub(crate) fn new(bits: u8) -> Self {
        SADR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADR` writer - Slave Address"]
pub struct SADR_W<'a> {
    w: &'a mut W,
}
impl<'a> SADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&self) -> SADR_R {
        SADR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:22 - Slave Address"]
    #[inline(always)]
    pub fn sadr(&mut self) -> SADR_W {
        SADR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smr](index.html) module"]
pub struct SMR_SPEC;
impl crate::RegisterSpec for SMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smr::R](R) reader structure"]
impl crate::Readable for SMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smr::W](W) writer structure"]
impl crate::Writable for SMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMR to value 0"]
impl crate::Resettable for SMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
