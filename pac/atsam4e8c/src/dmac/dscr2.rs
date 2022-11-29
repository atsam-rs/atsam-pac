#[doc = "Register `DSCR2` reader"]
pub struct R(crate::R<DSCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSCR2` writer"]
pub struct W(crate::W<DSCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSCR2_SPEC>;
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
impl From<crate::W<DSCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSCR` reader - Buffer Transfer Descriptor Address"]
pub type DSCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSCR` writer - Buffer Transfer Descriptor Address"]
pub type DSCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSCR2_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - Buffer Transfer Descriptor Address"]
    #[inline(always)]
    pub fn dscr(&self) -> DSCR_R {
        DSCR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Buffer Transfer Descriptor Address"]
    #[inline(always)]
    #[must_use]
    pub fn dscr(&mut self) -> DSCR_W<2> {
        DSCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 2)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscr2](index.html) module"]
pub struct DSCR2_SPEC;
impl crate::RegisterSpec for DSCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dscr2::R](R) reader structure"]
impl crate::Readable for DSCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dscr2::W](W) writer structure"]
impl crate::Writable for DSCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSCR2 to value 0"]
impl crate::Resettable for DSCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
