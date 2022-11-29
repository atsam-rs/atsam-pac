#[doc = "Register `MATRIX_WPMR` reader"]
pub struct R(crate::R<MATRIX_WPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MATRIX_WPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MATRIX_WPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MATRIX_WPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MATRIX_WPMR` writer"]
pub struct W(crate::W<MATRIX_WPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MATRIX_WPMR_SPEC>;
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
impl From<crate::W<MATRIX_WPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MATRIX_WPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WPEN` reader - Write Protect Enable"]
pub type WPEN_R = crate::BitReader<bool>;
#[doc = "Field `WPEN` writer - Write Protect Enable"]
pub type WPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MATRIX_WPMR_SPEC, bool, O>;
#[doc = "Field `WPKEY` reader - Write Protect KEY (Write-only)"]
pub type WPKEY_R = crate::FieldReader<u32, WPKEY_A>;
#[doc = "Write Protect KEY (Write-only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum WPKEY_A {
    #[doc = "5062996: Writing any other value in this field aborts the write operation of the WPEN bit.Always reads as 0."]
    PASSWD = 5062996,
}
impl From<WPKEY_A> for u32 {
    #[inline(always)]
    fn from(variant: WPKEY_A) -> Self {
        variant as _
    }
}
impl WPKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WPKEY_A> {
        match self.bits {
            5062996 => Some(WPKEY_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == WPKEY_A::PASSWD
    }
}
#[doc = "Field `WPKEY` writer - Write Protect KEY (Write-only)"]
pub type WPKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MATRIX_WPMR_SPEC, u32, WPKEY_A, 24, O>;
impl<'a, const O: u8> WPKEY_W<'a, O> {
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(WPKEY_A::PASSWD)
    }
}
impl R {
    #[doc = "Bit 0 - Write Protect Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protect KEY (Write-only)"]
    #[inline(always)]
    pub fn wpkey(&self) -> WPKEY_R {
        WPKEY_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protect Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WPEN_W<0> {
        WPEN_W::new(self)
    }
    #[doc = "Bits 8:31 - Write Protect KEY (Write-only)"]
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
#[doc = "Write Protect Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [matrix_wpmr](index.html) module"]
pub struct MATRIX_WPMR_SPEC;
impl crate::RegisterSpec for MATRIX_WPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [matrix_wpmr::R](R) reader structure"]
impl crate::Readable for MATRIX_WPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [matrix_wpmr::W](W) writer structure"]
impl crate::Writable for MATRIX_WPMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MATRIX_WPMR to value 0"]
impl crate::Resettable for MATRIX_WPMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
