#[doc = "Reader of register MR"]
pub type R = crate::R<u32, super::MR>;
#[doc = "Writer for register MR"]
pub type W = crate::W<u32, super::MR>;
#[doc = "Register MR `reset()`'s with value 0"]
impl crate::ResetValue for super::MR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Reader of field `COMPARE`"]
pub type COMPARE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMPARE`"]
pub struct COMPARE_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Primitive Polynomial\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PTYPE_A {
    #[doc = "0: Polynom 0x04C11DB7"]
    CCITT8023 = 0,
    #[doc = "1: Polynom 0x1EDC6F41"]
    CASTAGNOLI = 1,
    #[doc = "2: Polynom 0x1021"]
    CCITT16 = 2,
}
impl From<PTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: PTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PTYPE`"]
pub type PTYPE_R = crate::R<u8, PTYPE_A>;
impl PTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PTYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PTYPE_A::CCITT8023),
            1 => Val(PTYPE_A::CASTAGNOLI),
            2 => Val(PTYPE_A::CCITT16),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `CCITT8023`"]
    #[inline(always)]
    pub fn is_ccitt8023(&self) -> bool {
        *self == PTYPE_A::CCITT8023
    }
    #[doc = "Checks if the value of the field is `CASTAGNOLI`"]
    #[inline(always)]
    pub fn is_castagnoli(&self) -> bool {
        *self == PTYPE_A::CASTAGNOLI
    }
    #[doc = "Checks if the value of the field is `CCITT16`"]
    #[inline(always)]
    pub fn is_ccitt16(&self) -> bool {
        *self == PTYPE_A::CCITT16
    }
}
#[doc = "Write proxy for field `PTYPE`"]
pub struct PTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> PTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PTYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Polynom 0x04C11DB7"]
    #[inline(always)]
    pub fn ccitt8023(self) -> &'a mut W {
        self.variant(PTYPE_A::CCITT8023)
    }
    #[doc = "Polynom 0x1EDC6F41"]
    #[inline(always)]
    pub fn castagnoli(self) -> &'a mut W {
        self.variant(PTYPE_A::CASTAGNOLI)
    }
    #[doc = "Polynom 0x1021"]
    #[inline(always)]
    pub fn ccitt16(self) -> &'a mut W {
        self.variant(PTYPE_A::CCITT16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `DIVIDER`"]
pub type DIVIDER_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVIDER`"]
pub struct DIVIDER_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CRC Compare"]
    #[inline(always)]
    pub fn compare(&self) -> COMPARE_R {
        COMPARE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Primitive Polynomial"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Request Divider"]
    #[inline(always)]
    pub fn divider(&self) -> DIVIDER_R {
        DIVIDER_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CRC Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - CRC Compare"]
    #[inline(always)]
    pub fn compare(&mut self) -> COMPARE_W {
        COMPARE_W { w: self }
    }
    #[doc = "Bits 2:3 - Primitive Polynomial"]
    #[inline(always)]
    pub fn ptype(&mut self) -> PTYPE_W {
        PTYPE_W { w: self }
    }
    #[doc = "Bits 4:7 - Request Divider"]
    #[inline(always)]
    pub fn divider(&mut self) -> DIVIDER_W {
        DIVIDER_W { w: self }
    }
}
