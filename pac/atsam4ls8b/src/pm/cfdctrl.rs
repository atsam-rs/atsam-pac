#[doc = "Register `CFDCTRL` reader"]
pub struct R(crate::R<CFDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFDCTRL` writer"]
pub struct W(crate::W<CFDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFDCTRL_SPEC>;
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
impl From<crate::W<CFDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFDEN` reader - Clock Failure Detection Enable"]
pub type CFDEN_R = crate::BitReader<bool>;
#[doc = "Field `CFDEN` writer - Clock Failure Detection Enable"]
pub type CFDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCTRL_SPEC, bool, O>;
#[doc = "Field `SFV` reader - Store Final Value"]
pub type SFV_R = crate::BitReader<bool>;
#[doc = "Field `SFV` writer - Store Final Value"]
pub type SFV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFDCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clock Failure Detection Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 31 - Store Final Value"]
    #[inline(always)]
    pub fn sfv(&self) -> SFV_R {
        SFV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Failure Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CFDEN_W<0> {
        CFDEN_W::new(self)
    }
    #[doc = "Bit 31 - Store Final Value"]
    #[inline(always)]
    #[must_use]
    pub fn sfv(&mut self) -> SFV_W<31> {
        SFV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Failure Detector Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfdctrl](index.html) module"]
pub struct CFDCTRL_SPEC;
impl crate::RegisterSpec for CFDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfdctrl::R](R) reader structure"]
impl crate::Readable for CFDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfdctrl::W](W) writer structure"]
impl crate::Writable for CFDCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFDCTRL to value 0"]
impl crate::Resettable for CFDCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
