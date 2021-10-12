#[doc = "Register `SADDR1` reader"]
pub struct R(crate::R<SADDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SADDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SADDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SADDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SADDR1` writer"]
pub struct W(crate::W<SADDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SADDR1_SPEC>;
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
impl From<crate::W<SADDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SADDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SADDR` reader - Channel x Source Address"]
pub struct SADDR_R(crate::FieldReader<u32, u32>);
impl SADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        SADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SADDR` writer - Channel x Source Address"]
pub struct SADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn saddr(&self) -> SADDR_R {
        SADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel x Source Address"]
    #[inline(always)]
    pub fn saddr(&mut self) -> SADDR_W {
        SADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Source Address Register (ch_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddr1](index.html) module"]
pub struct SADDR1_SPEC;
impl crate::RegisterSpec for SADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saddr1::R](R) reader structure"]
impl crate::Readable for SADDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saddr1::W](W) writer structure"]
impl crate::Writable for SADDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SADDR1 to value 0"]
impl crate::Resettable for SADDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
