#[doc = "Register `UNLOCK` writer"]
pub struct W(crate::W<UNLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UNLOCK_SPEC>;
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
impl From<crate::W<UNLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UNLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` writer - Unlock Address"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UNLOCK_SPEC, u16, u16, 10, O>;
#[doc = "Unlock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEYSELECT_AW {
    #[doc = "170: Valid Key to Unlock register"]
    VALID = 170,
}
impl From<KEYSELECT_AW> for u8 {
    #[inline(always)]
    fn from(variant: KEYSELECT_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `KEY` writer - Unlock Key"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, UNLOCK_SPEC, u8, KEYSELECT_AW, 8, O>;
impl<'a, const O: u8> KEY_W<'a, O> {
    #[doc = "Valid Key to Unlock register"]
    #[inline(always)]
    pub fn valid(self) -> &'a mut W {
        self.variant(KEYSELECT_AW::VALID)
    }
}
impl W {
    #[doc = "Bits 0:9 - Unlock Address"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bits 24:31 - Unlock Key"]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<24> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Unlock Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unlock](index.html) module"]
pub struct UNLOCK_SPEC;
impl crate::RegisterSpec for UNLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [unlock::W](W) writer structure"]
impl crate::Writable for UNLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UNLOCK to value 0"]
impl crate::Resettable for UNLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
