#[doc = "Writer for register IDR%s"]
pub type W = crate::W<u32, super::IDR>;
#[doc = "Register IDR%s `reset()`'s with value 0"]
impl crate::ResetValue for super::IDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Counter Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COVFS_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Disables the Counter Overflow Interrupt."]
    _1 = 1,
}
impl From<COVFS_AW> for bool {
    #[inline(always)]
    fn from(variant: COVFS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `COVFS`"]
pub struct COVFS_W<'a> {
    w: &'a mut W,
}
impl<'a> COVFS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COVFS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COVFS_AW::_0)
    }
    #[doc = "Disables the Counter Overflow Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COVFS_AW::_1)
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
#[doc = "Load Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOVRS_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Disables the Load Overrun Interrupt (if WAVE:0)."]
    _1 = 1,
}
impl From<LOVRS_AW> for bool {
    #[inline(always)]
    fn from(variant: LOVRS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LOVRS`"]
pub struct LOVRS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOVRS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOVRS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOVRS_AW::_0)
    }
    #[doc = "Disables the Load Overrun Interrupt (if WAVE:0)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOVRS_AW::_1)
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
#[doc = "RA Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPAS_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Disables the RA Compare Interrupt (if WAVE:1)."]
    _1 = 1,
}
impl From<CPAS_AW> for bool {
    #[inline(always)]
    fn from(variant: CPAS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CPAS`"]
pub struct CPAS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPAS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPAS_AW::_0)
    }
    #[doc = "Disables the RA Compare Interrupt (if WAVE:1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPAS_AW::_1)
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
#[doc = "RB Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPBS_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Disables the RB Compare Interrupt (if WAVE:1)."]
    _1 = 1,
}
impl From<CPBS_AW> for bool {
    #[inline(always)]
    fn from(variant: CPBS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CPBS`"]
pub struct CPBS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPBS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPBS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPBS_AW::_0)
    }
    #[doc = "Disables the RB Compare Interrupt (if WAVE:1)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPBS_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "RC Compare\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CPCS_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Disables the RC Compare Interrupt."]
    _1 = 1,
}
impl From<CPCS_AW> for bool {
    #[inline(always)]
    fn from(variant: CPCS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CPCS`"]
pub struct CPCS_W<'a> {
    w: &'a mut W,
}
impl<'a> CPCS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CPCS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPCS_AW::_0)
    }
    #[doc = "Disables the RC Compare Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPCS_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "RA Loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRAS_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Disables the RA Load Interrupt (if WAVE:0)."]
    _1 = 1,
}
impl From<LDRAS_AW> for bool {
    #[inline(always)]
    fn from(variant: LDRAS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LDRAS`"]
pub struct LDRAS_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRAS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDRAS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDRAS_AW::_0)
    }
    #[doc = "Disables the RA Load Interrupt (if WAVE:0)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDRAS_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "RB Loading\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LDRBS_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Disables the RB Load Interrupt (if WAVE:0)."]
    _1 = 1,
}
impl From<LDRBS_AW> for bool {
    #[inline(always)]
    fn from(variant: LDRBS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `LDRBS`"]
pub struct LDRBS_W<'a> {
    w: &'a mut W,
}
impl<'a> LDRBS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LDRBS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LDRBS_AW::_0)
    }
    #[doc = "Disables the RB Load Interrupt (if WAVE:0)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LDRBS_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "External Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETRGS_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Disables the External Trigger Interrupt."]
    _1 = 1,
}
impl From<ETRGS_AW> for bool {
    #[inline(always)]
    fn from(variant: ETRGS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ETRGS`"]
pub struct ETRGS_W<'a> {
    w: &'a mut W,
}
impl<'a> ETRGS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETRGS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETRGS_AW::_0)
    }
    #[doc = "Disables the External Trigger Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETRGS_AW::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Counter Overflow"]
    #[inline(always)]
    pub fn covfs(&mut self) -> COVFS_W {
        COVFS_W { w: self }
    }
    #[doc = "Bit 1 - Load Overrun"]
    #[inline(always)]
    pub fn lovrs(&mut self) -> LOVRS_W {
        LOVRS_W { w: self }
    }
    #[doc = "Bit 2 - RA Compare"]
    #[inline(always)]
    pub fn cpas(&mut self) -> CPAS_W {
        CPAS_W { w: self }
    }
    #[doc = "Bit 3 - RB Compare"]
    #[inline(always)]
    pub fn cpbs(&mut self) -> CPBS_W {
        CPBS_W { w: self }
    }
    #[doc = "Bit 4 - RC Compare"]
    #[inline(always)]
    pub fn cpcs(&mut self) -> CPCS_W {
        CPCS_W { w: self }
    }
    #[doc = "Bit 5 - RA Loading"]
    #[inline(always)]
    pub fn ldras(&mut self) -> LDRAS_W {
        LDRAS_W { w: self }
    }
    #[doc = "Bit 6 - RB Loading"]
    #[inline(always)]
    pub fn ldrbs(&mut self) -> LDRBS_W {
        LDRBS_W { w: self }
    }
    #[doc = "Bit 7 - External Trigger"]
    #[inline(always)]
    pub fn etrgs(&mut self) -> ETRGS_W {
        ETRGS_W { w: self }
    }
}
