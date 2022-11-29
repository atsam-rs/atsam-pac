#[doc = "Register `WPCR` reader"]
pub struct R(crate::R<WPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WPCR` writer"]
pub struct W(crate::W<WPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WPCR_SPEC>;
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
impl From<crate::W<WPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WPEN` reader - Write Protection Enable"]
pub type WPEN_R = crate::BitReader<bool>;
#[doc = "Field `WPEN` writer - Write Protection Enable"]
pub type WPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, WPCR_SPEC, bool, O>;
#[doc = "Field `WPKEY` reader - Write Protection Key Password"]
pub type WPKEY_R = crate::FieldReader<u32, WPKEYSELECT_A>;
#[doc = "Write Protection Key Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WPKEYSELECT_A {
    #[doc = "5460041: SPI Write Protection Key Password"]
    VALUE = 5460041,
}
impl From<WPKEYSELECT_A> for u32 {
    #[inline(always)]
    fn from(variant: WPKEYSELECT_A) -> Self {
        variant as _
    }
}
impl WPKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WPKEYSELECT_A> {
        match self.bits {
            5460041 => Some(WPKEYSELECT_A::VALUE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VALUE`"]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        *self == WPKEYSELECT_A::VALUE
    }
}
#[doc = "Field `WPKEY` writer - Write Protection Key Password"]
pub type WPKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WPCR_SPEC, u32, WPKEYSELECT_A, 24, O>;
impl<'a, const O: u8> WPKEY_W<'a, O> {
    #[doc = "SPI Write Protection Key Password"]
    #[inline(always)]
    pub fn value(self) -> &'a mut W {
        self.variant(WPKEYSELECT_A::VALUE)
    }
}
impl R {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key Password"]
    #[inline(always)]
    pub fn wpkey(&self) -> WPKEY_R {
        WPKEY_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WPEN_W<0> {
        WPEN_W::new(self)
    }
    #[doc = "Bits 8:31 - Write Protection Key Password"]
    #[inline(always)]
    #[must_use]
    pub fn wpkey(&mut self) -> WPKEY_W<8> {
        WPKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Protection control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpcr](index.html) module"]
pub struct WPCR_SPEC;
impl crate::RegisterSpec for WPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpcr::R](R) reader structure"]
impl crate::Readable for WPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wpcr::W](W) writer structure"]
impl crate::Writable for WPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WPCR to value 0"]
impl crate::Resettable for WPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
