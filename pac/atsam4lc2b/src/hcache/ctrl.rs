#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Cache Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CENSELECT_AW {
    #[doc = "0: Disable Cache Controller"]
    NO = 0,
    #[doc = "1: Enable Cache Controller"]
    YES = 1,
}
impl From<CENSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: CENSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEN` writer - Cache Enable"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, CENSELECT_AW, O>;
impl<'a, const O: u8> CEN_W<'a, O> {
    #[doc = "Disable Cache Controller"]
    #[inline(always)]
    pub fn no(self) -> &'a mut W {
        self.variant(CENSELECT_AW::NO)
    }
    #[doc = "Enable Cache Controller"]
    #[inline(always)]
    pub fn yes(self) -> &'a mut W {
        self.variant(CENSELECT_AW::YES)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<0> {
        CEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
