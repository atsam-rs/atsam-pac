#[doc = "Reader of register RTOR"]
pub type R = crate::R<u32, super::RTOR>;
#[doc = "Writer for register RTOR"]
pub type W = crate::W<u32, super::RTOR>;
#[doc = "Register RTOR `reset()`'s with value 0"]
impl crate::ResetValue for super::RTOR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Time-out Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum TO_A {
    #[doc = "0: Disables the RX Time-out function"]
    DISABLE = 0,
}
impl From<TO_A> for u32 {
    #[inline(always)]
    fn from(variant: TO_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TO`"]
pub type TO_R = crate::R<u32, TO_A>;
impl TO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, TO_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TO_A::DISABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TO_A::DISABLE
    }
}
#[doc = "Write proxy for field `TO`"]
pub struct TO_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disables the RX Time-out function"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TO_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0001_ffff) | ((value as u32) & 0x0001_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:16 - Time-out Value"]
    #[inline(always)]
    pub fn to(&self) -> TO_R {
        TO_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Time-out Value"]
    #[inline(always)]
    pub fn to(&mut self) -> TO_W {
        TO_W { w: self }
    }
}
