#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRVCE` reader - Internal Reference Voltage Change Enable"]
pub type IRVCE_R = crate::BitReader<IRVCE_A>;
#[doc = "Internal Reference Voltage Change Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRVCE_A {
    #[doc = "0: The internal reference voltage is stuck at the default value (see the product electrical charac-teristics for further details)."]
    STUCK_AT_DEFAULT = 0,
    #[doc = "1: The internal reference voltage is defined by field IRVS."]
    SELECTION = 1,
}
impl From<IRVCE_A> for bool {
    #[inline(always)]
    fn from(variant: IRVCE_A) -> Self {
        variant as u8 != 0
    }
}
impl IRVCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRVCE_A {
        match self.bits {
            false => IRVCE_A::STUCK_AT_DEFAULT,
            true => IRVCE_A::SELECTION,
        }
    }
    #[doc = "Checks if the value of the field is `STUCK_AT_DEFAULT`"]
    #[inline(always)]
    pub fn is_stuck_at_default(&self) -> bool {
        *self == IRVCE_A::STUCK_AT_DEFAULT
    }
    #[doc = "Checks if the value of the field is `SELECTION`"]
    #[inline(always)]
    pub fn is_selection(&self) -> bool {
        *self == IRVCE_A::SELECTION
    }
}
#[doc = "Field `IRVCE` writer - Internal Reference Voltage Change Enable"]
pub type IRVCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, IRVCE_A, O>;
impl<'a, const O: u8> IRVCE_W<'a, O> {
    #[doc = "The internal reference voltage is stuck at the default value (see the product electrical charac-teristics for further details)."]
    #[inline(always)]
    pub fn stuck_at_default(self) -> &'a mut W {
        self.variant(IRVCE_A::STUCK_AT_DEFAULT)
    }
    #[doc = "The internal reference voltage is defined by field IRVS."]
    #[inline(always)]
    pub fn selection(self) -> &'a mut W {
        self.variant(IRVCE_A::SELECTION)
    }
}
#[doc = "Field `IRVS` reader - Internal Reference Voltage Selection"]
pub type IRVS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRVS` writer - Internal Reference Voltage Selection"]
pub type IRVS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACR_SPEC, u8, u8, 4, O>;
#[doc = "Field `FORCEREF` reader - Force Internal Reference Voltage"]
pub type FORCEREF_R = crate::BitReader<bool>;
#[doc = "Field `FORCEREF` writer - Force Internal Reference Voltage"]
pub type FORCEREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
#[doc = "Field `ONREF` reader - Internal Voltage Reference ON"]
pub type ONREF_R = crate::BitReader<bool>;
#[doc = "Field `ONREF` writer - Internal Voltage Reference ON"]
pub type ONREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Internal Reference Voltage Change Enable"]
    #[inline(always)]
    pub fn irvce(&self) -> IRVCE_R {
        IRVCE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - Internal Reference Voltage Selection"]
    #[inline(always)]
    pub fn irvs(&self) -> IRVS_R {
        IRVS_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - Force Internal Reference Voltage"]
    #[inline(always)]
    pub fn forceref(&self) -> FORCEREF_R {
        FORCEREF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Internal Voltage Reference ON"]
    #[inline(always)]
    pub fn onref(&self) -> ONREF_R {
        ONREF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Internal Reference Voltage Change Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irvce(&mut self) -> IRVCE_W<2> {
        IRVCE_W::new(self)
    }
    #[doc = "Bits 3:6 - Internal Reference Voltage Selection"]
    #[inline(always)]
    #[must_use]
    pub fn irvs(&mut self) -> IRVS_W<3> {
        IRVS_W::new(self)
    }
    #[doc = "Bit 19 - Force Internal Reference Voltage"]
    #[inline(always)]
    #[must_use]
    pub fn forceref(&mut self) -> FORCEREF_W<19> {
        FORCEREF_W::new(self)
    }
    #[doc = "Bit 20 - Internal Voltage Reference ON"]
    #[inline(always)]
    #[must_use]
    pub fn onref(&mut self) -> ONREF_W<20> {
        ONREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR to value 0x0008_0000"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0008_0000;
}
