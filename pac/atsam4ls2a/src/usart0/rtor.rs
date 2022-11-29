#[doc = "Register `RTOR` reader"]
pub struct R(crate::R<RTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTOR` writer"]
pub struct W(crate::W<RTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTOR_SPEC>;
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
impl From<crate::W<RTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TO` reader - Time-out Value"]
pub type TO_R = crate::FieldReader<u32, TOSELECT_A>;
#[doc = "Time-out Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum TOSELECT_A {
    #[doc = "0: Disables the RX Time-out function"]
    DISABLE = 0,
}
impl From<TOSELECT_A> for u32 {
    #[inline(always)]
    fn from(variant: TOSELECT_A) -> Self {
        variant as _
    }
}
impl TO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TOSELECT_A> {
        match self.bits {
            0 => Some(TOSELECT_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TOSELECT_A::DISABLE
    }
}
#[doc = "Field `TO` writer - Time-out Value"]
pub type TO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTOR_SPEC, u32, TOSELECT_A, 17, O>;
impl<'a, const O: u8> TO_W<'a, O> {
    #[doc = "Disables the RX Time-out function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TOSELECT_A::DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:16 - Time-out Value"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - Time-out Value"]
    #[inline(always)]
    #[must_use]
    pub fn to(&mut self) -> TO_W<0> {
        TO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receiver Time-out Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtor](index.html) module"]
pub struct RTOR_SPEC;
impl crate::RegisterSpec for RTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtor::R](R) reader structure"]
impl crate::Readable for RTOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtor::W](W) writer structure"]
impl crate::Writable for RTOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RTOR to value 0"]
impl crate::Resettable for RTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
