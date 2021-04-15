#[doc = "Reader of register EVS%s"]
pub type R = crate::R<u32, super::EVS>;
#[doc = "Writer for register EVS%s"]
pub type W = crate::W<u32, super::EVS>;
#[doc = "Register EVS%s `reset()`'s with value 0"]
impl crate::ResetValue for super::EVS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Event Shaper Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EN_A {
    #[doc = "0: No Action"]
    _0 = 0,
    #[doc = "1: Event Shaper enable"]
    _1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, EN_A>;
impl EN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::_0,
            true => EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN_A::_1
    }
}
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_A::_0)
    }
    #[doc = "Event Shaper enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_A::_1)
    }
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
#[doc = "Input Glitch Filter Rise\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGFR_A {
    #[doc = "0: No Action"]
    _0 = 0,
    #[doc = "1: Input Glitch Filter : a rising edge on event input will raise trigger output"]
    _1 = 1,
}
impl From<IGFR_A> for bool {
    #[inline(always)]
    fn from(variant: IGFR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IGFR`"]
pub type IGFR_R = crate::R<bool, IGFR_A>;
impl IGFR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGFR_A {
        match self.bits {
            false => IGFR_A::_0,
            true => IGFR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IGFR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IGFR_A::_1
    }
}
#[doc = "Write proxy for field `IGFR`"]
pub struct IGFR_W<'a> {
    w: &'a mut W,
}
impl<'a> IGFR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IGFR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IGFR_A::_0)
    }
    #[doc = "Input Glitch Filter : a rising edge on event input will raise trigger output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IGFR_A::_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Input Glitch Filter Fall\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IGFF_A {
    #[doc = "0: No Action"]
    _0 = 0,
    #[doc = "1: Input Glitch Filter : a falling edge on event input will raise trigger output"]
    _1 = 1,
}
impl From<IGFF_A> for bool {
    #[inline(always)]
    fn from(variant: IGFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `IGFF`"]
pub type IGFF_R = crate::R<bool, IGFF_A>;
impl IGFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IGFF_A {
        match self.bits {
            false => IGFF_A::_0,
            true => IGFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IGFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IGFF_A::_1
    }
}
#[doc = "Write proxy for field `IGFF`"]
pub struct IGFF_W<'a> {
    w: &'a mut W,
}
impl<'a> IGFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IGFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IGFF_A::_0)
    }
    #[doc = "Input Glitch Filter : a falling edge on event input will raise trigger output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IGFF_A::_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `IGFON`"]
pub type IGFON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IGFON`"]
pub struct IGFON_W<'a> {
    w: &'a mut W,
}
impl<'a> IGFON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Event Shaper Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Input Glitch Filter Rise"]
    #[inline(always)]
    pub fn igfr(&self) -> IGFR_R {
        IGFR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Input Glitch Filter Fall"]
    #[inline(always)]
    pub fn igff(&self) -> IGFF_R {
        IGFF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Input Glitch Filter Status"]
    #[inline(always)]
    pub fn igfon(&self) -> IGFON_R {
        IGFON_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event Shaper Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 16 - Input Glitch Filter Rise"]
    #[inline(always)]
    pub fn igfr(&mut self) -> IGFR_W {
        IGFR_W { w: self }
    }
    #[doc = "Bit 17 - Input Glitch Filter Fall"]
    #[inline(always)]
    pub fn igff(&mut self) -> IGFF_W {
        IGFF_W { w: self }
    }
    #[doc = "Bit 18 - Input Glitch Filter Status"]
    #[inline(always)]
    pub fn igfon(&mut self) -> IGFON_W {
        IGFON_W { w: self }
    }
}
