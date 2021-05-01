#[doc = "Reader of register TTGR"]
pub type R = crate::R<u32, super::TTGR>;
#[doc = "Writer for register TTGR"]
pub type W = crate::W<u32, super::TTGR>;
#[doc = "Register TTGR `reset()`'s with value 0"]
impl crate::ResetValue for super::TTGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Timeguard Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TG_A {
    #[doc = "0: Disables the TX Timeguard function."]
    DISABLE = 0,
}
impl From<TG_A> for u8 {
    #[inline(always)]
    fn from(variant: TG_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `TG`"]
pub type TG_R = crate::R<u8, TG_A>;
impl TG_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TG_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TG_A::DISABLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TG_A::DISABLE
    }
}
#[doc = "Write proxy for field `TG`"]
pub struct TG_W<'a> {
    w: &'a mut W,
}
impl<'a> TG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TG_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disables the TX Timeguard function."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TG_A::DISABLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    pub fn tg(&self) -> TG_R {
        TG_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeguard Value"]
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W {
        TG_W { w: self }
    }
}
