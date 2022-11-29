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
pub type BKUPPMUX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BKUPPMUX` writer - Backup Pin Muxing"]
pub type BKUPPMUX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BKUPPMUX_SPEC, u16, u16, 9, O>;
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
    #[must_use]
    pub fn bkuppmux(&mut self) -> BKUPPMUX_W<0> {
        BKUPPMUX_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKUPPMUX to value 0"]
impl crate::Resettable for BKUPPMUX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
