#[doc = "Register `MEN` reader"]
pub struct R(crate::R<MEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MEN` writer"]
pub struct W(crate::W<MEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MEN_SPEC>;
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
impl From<crate::W<MEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MENABLESELECT_AW {
    #[doc = "0: Disable Monitor Counter"]
    DIS = 0,
    #[doc = "1: Enable Monitor Counter"]
    EN = 1,
}
impl From<MENABLESELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: MENABLESELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MENABLE` writer - Monitor Enable"]
pub type MENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MEN_SPEC, MENABLESELECT_AW, O>;
impl<'a, const O: u8> MENABLE_W<'a, O> {
    #[doc = "Disable Monitor Counter"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(MENABLESELECT_AW::DIS)
    }
    #[doc = "Enable Monitor Counter"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(MENABLESELECT_AW::EN)
    }
}
impl W {
    #[doc = "Bit 0 - Monitor Enable"]
    #[inline(always)]
    #[must_use]
    pub fn menable(&mut self) -> MENABLE_W<0> {
        MENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Monitor Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [men](index.html) module"]
pub struct MEN_SPEC;
impl crate::RegisterSpec for MEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [men::R](R) reader structure"]
impl crate::Readable for MEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [men::W](W) writer structure"]
impl crate::Writable for MEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEN to value 0"]
impl crate::Resettable for MEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
