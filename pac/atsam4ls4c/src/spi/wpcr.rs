#[doc = "Reader of register WPCR"]
pub type R = crate::R<u32, super::WPCR>;
#[doc = "Writer for register WPCR"]
pub type W = crate::W<u32, super::WPCR>;
#[doc = "Register WPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::WPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WPEN`"]
pub type WPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WPEN`"]
pub struct WPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WPEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write Protection Key Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum WPKEY_A {
    #[doc = "5460041: SPI Write Protection Key Password"]
    VALUE = 5460041,
}
impl From<WPKEY_A> for u32 {
    #[inline(always)]
    fn from(variant: WPKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WPKEY`"]
pub type WPKEY_R = crate::R<u32, WPKEY_A>;
impl WPKEY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, WPKEY_A> {
        use crate::Variant::*;
        match self.bits {
            5460041 => Val(WPKEY_A::VALUE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `VALUE`"]
    #[inline(always)]
    pub fn is_value(&self) -> bool {
        *self == WPKEY_A::VALUE
    }
}
#[doc = "Write proxy for field `WPKEY`"]
pub struct WPKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WPKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPKEY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI Write Protection Key Password"]
    #[inline(always)]
    pub fn value(self) -> &'a mut W {
        self.variant(WPKEY_A::VALUE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key Password"]
    #[inline(always)]
    pub fn wpkey(&self) -> WPKEY_R {
        WPKEY_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&mut self) -> WPEN_W {
        WPEN_W { w: self }
    }
    #[doc = "Bits 8:31 - Write Protection Key Password"]
    #[inline(always)]
    pub fn wpkey(&mut self) -> WPKEY_W {
        WPKEY_W { w: self }
    }
}