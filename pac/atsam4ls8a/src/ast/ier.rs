#[doc = "Writer for register IER"]
pub type W = crate::W<u32, super::IER>;
#[doc = "Register IER `reset()`'s with value 0"]
impl crate::ResetValue for super::IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVF_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable Interrupt."]
    _1 = 1,
}
impl From<OVF_AW> for bool {
    #[inline(always)]
    fn from(variant: OVF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `OVF`"]
pub struct OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVF_AW::_0)
    }
    #[doc = "Enable Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVF_AW::_1)
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
#[doc = "Alarm 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<ALARM0_AW> for bool {
    #[inline(always)]
    fn from(variant: ALARM0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ALARM0`"]
pub struct ALARM0_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALARM0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALARM0_AW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALARM0_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Alarm 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALARM1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<ALARM1_AW> for bool {
    #[inline(always)]
    fn from(variant: ALARM1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ALARM1`"]
pub struct ALARM1_W<'a> {
    w: &'a mut W,
}
impl<'a> ALARM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALARM1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALARM1_AW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALARM1_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Periodic 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER0_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<PER0_AW> for bool {
    #[inline(always)]
    fn from(variant: PER0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PER0`"]
pub struct PER0_W<'a> {
    w: &'a mut W,
}
impl<'a> PER0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PER0_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER0_AW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PER0_AW::_1)
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
#[doc = "Periodic 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PER1_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<PER1_AW> for bool {
    #[inline(always)]
    fn from(variant: PER1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `PER1`"]
pub struct PER1_W<'a> {
    w: &'a mut W,
}
impl<'a> PER1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PER1_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PER1_AW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PER1_AW::_1)
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
#[doc = "AST Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<READY_AW> for bool {
    #[inline(always)]
    fn from(variant: READY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `READY`"]
pub struct READY_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(READY_AW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(READY_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Clock Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKRDY_AW {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<CLKRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: CLKRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CLKRDY`"]
pub struct CLKRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKRDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKRDY_AW::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKRDY_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Overflow"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W {
        OVF_W { w: self }
    }
    #[doc = "Bit 8 - Alarm 0"]
    #[inline(always)]
    pub fn alarm0(&mut self) -> ALARM0_W {
        ALARM0_W { w: self }
    }
    #[doc = "Bit 9 - Alarm 1"]
    #[inline(always)]
    pub fn alarm1(&mut self) -> ALARM1_W {
        ALARM1_W { w: self }
    }
    #[doc = "Bit 16 - Periodic 0"]
    #[inline(always)]
    pub fn per0(&mut self) -> PER0_W {
        PER0_W { w: self }
    }
    #[doc = "Bit 17 - Periodic 1"]
    #[inline(always)]
    pub fn per1(&mut self) -> PER1_W {
        PER1_W { w: self }
    }
    #[doc = "Bit 25 - AST Ready"]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W {
        READY_W { w: self }
    }
    #[doc = "Bit 29 - Clock Ready"]
    #[inline(always)]
    pub fn clkrdy(&mut self) -> CLKRDY_W {
        CLKRDY_W { w: self }
    }
}
