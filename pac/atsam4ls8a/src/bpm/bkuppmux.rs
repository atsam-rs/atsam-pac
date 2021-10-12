#[doc = "Register `BKUPPMUX` reader"]
pub struct R(crate::R<BKUPPMUX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKUPPMUX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKUPPMUX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKUPPMUX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKUPPMUX` writer"]
pub struct W(crate::W<BKUPPMUX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKUPPMUX_SPEC>;
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
impl From<crate::W<BKUPPMUX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKUPPMUX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKUPPMUX` reader - Backup Pin Muxing"]
pub struct BKUPPMUX_R(crate::FieldReader<u16, u16>);
impl BKUPPMUX_R {
    pub(crate) fn new(bits: u16) -> Self {
        BKUPPMUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BKUPPMUX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BKUPPMUX` writer - Backup Pin Muxing"]
pub struct BKUPPMUX_W<'a> {
    w: &'a mut W,
}
impl<'a> BKUPPMUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Backup Pin Muxing"]
    #[inline(always)]
    pub fn bkuppmux(&self) -> BKUPPMUX_R {
        BKUPPMUX_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Backup Pin Muxing"]
    #[inline(always)]
    pub fn bkuppmux(&mut self) -> BKUPPMUX_W {
        BKUPPMUX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Pin Muxing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkuppmux](index.html) module"]
pub struct BKUPPMUX_SPEC;
impl crate::RegisterSpec for BKUPPMUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkuppmux::R](R) reader structure"]
impl crate::Readable for BKUPPMUX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkuppmux::W](W) writer structure"]
impl crate::Writable for BKUPPMUX_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BKUPPMUX to value 0"]
impl crate::Resettable for BKUPPMUX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
