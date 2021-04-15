#[doc = "Writer for register CCR%s"]
pub type W = crate::W<u32, super::CCR>;
#[doc = "Register CCR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter Clock Enable Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKEN_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the clock if CLKDIS is not 1."]
    _1 = 1,
}
impl From<CLKEN_AW> for bool {
    #[inline(always)]
    fn from(variant: CLKEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLKEN`"]
pub struct CLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKEN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKEN_AW::_0)
    }
    #[doc = "Enables the clock if CLKDIS is not 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKEN_AW::_1)
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
#[doc = "Counter Clock Disable Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKDIS_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Disables the clock."]
    _1 = 1,
}
impl From<CLKDIS_AW> for bool {
    #[inline(always)]
    fn from(variant: CLKDIS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLKDIS`"]
pub struct CLKDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKDIS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKDIS_AW::_0)
    }
    #[doc = "Disables the clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKDIS_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Software Trigger Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWTRG_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: A software trigger is performed:the counter is reset and clock is started."]
    _1 = 1,
}
impl From<SWTRG_AW> for bool {
    #[inline(always)]
    fn from(variant: SWTRG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SWTRG`"]
pub struct SWTRG_W<'a> {
    w: &'a mut W,
}
impl<'a> SWTRG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWTRG_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWTRG_AW::_0)
    }
    #[doc = "A software trigger is performed:the counter is reset and clock is started."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWTRG_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Counter Clock Enable Command"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W {
        CLKEN_W { w: self }
    }
    #[doc = "Bit 1 - Counter Clock Disable Command"]
    #[inline(always)]
    pub fn clkdis(&mut self) -> CLKDIS_W {
        CLKDIS_W { w: self }
    }
    #[doc = "Bit 2 - Software Trigger Command"]
    #[inline(always)]
    pub fn swtrg(&mut self) -> SWTRG_W {
        SWTRG_W { w: self }
    }
}
