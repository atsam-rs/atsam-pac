#[doc = "Reader of register FIDI"]
pub type R = crate::R<u32, super::FIDI>;
#[doc = "Writer for register FIDI"]
pub type W = crate::W<u32, super::FIDI>;
#[doc = "Register FIDI `reset()`'s with value 0x0174"]
impl crate::ResetValue for super::FIDI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0174
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
#[doc = "Reader of field `FI_DI_RATIO`"]
pub type FI_DI_RATIO_R = crate::R<u16, FI_DI_RATIO_A>;
impl FI_DI_RATIO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u16, FI_DI_RATIO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FI_DI_RATIO_A::DISABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FI_DI_RATIO_A::DISABLE
    }
}
#[doc = "Write proxy for field `FI_DI_RATIO`"]
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
        self.w.bits = (self.w.bits & !0x07ff) | ((value as u32) & 0x07ff);
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
}
