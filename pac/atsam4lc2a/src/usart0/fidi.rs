#[doc = "Register `FIDI` reader"]
pub struct R(crate::R<FIDI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIDI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<FIDI_SPEC>> for R {
    fn from(reader: crate::R<FIDI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIDI` writer"]
pub struct W(crate::W<FIDI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIDI_SPEC>;
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
impl core::convert::From<crate::W<FIDI_SPEC>> for W {
    fn from(writer: crate::W<FIDI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "FI Over DI Ratio Value\n\nValue on reset: 372"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum FI_DI_RATIO_A {
    #[doc = "0: Baud Rate = 0"]
    DISABLE = 0,
}
impl From<FI_DI_RATIO_A> for u16 {
    #[inline(always)]
    fn from(variant: FI_DI_RATIO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FI_DI_RATIO` reader - FI Over DI Ratio Value"]
pub struct FI_DI_RATIO_R(crate::FieldReader<u16, FI_DI_RATIO_A>);
impl FI_DI_RATIO_R {
    pub(crate) fn new(bits: u16) -> Self {
        FI_DI_RATIO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FI_DI_RATIO_A> {
        match self.bits {
            0 => Some(FI_DI_RATIO_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FI_DI_RATIO_A::DISABLE
    }
}
impl core::ops::Deref for FI_DI_RATIO_R {
    type Target = crate::FieldReader<u16, FI_DI_RATIO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FI_DI_RATIO` writer - FI Over DI Ratio Value"]
pub struct FI_DI_RATIO_W<'a> {
    w: &'a mut W,
}
impl<'a> FI_DI_RATIO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FI_DI_RATIO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Baud Rate = 0"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FI_DI_RATIO_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - FI Over DI Ratio Value"]
    #[inline(always)]
    pub fn fi_di_ratio(&self) -> FI_DI_RATIO_R {
        FI_DI_RATIO_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - FI Over DI Ratio Value"]
    #[inline(always)]
    pub fn fi_di_ratio(&mut self) -> FI_DI_RATIO_W {
        FI_DI_RATIO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FI DI Ratio Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fidi](index.html) module"]
pub struct FIDI_SPEC;
impl crate::RegisterSpec for FIDI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fidi::R](R) reader structure"]
impl crate::Readable for FIDI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fidi::W](W) writer structure"]
impl crate::Writable for FIDI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIDI to value 0x0174"]
impl crate::Resettable for FIDI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0174
    }
}
