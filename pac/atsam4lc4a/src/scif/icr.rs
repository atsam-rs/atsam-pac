#[doc = "Writer for register ICR"]
pub type W = crate::W<u32, super::ICR>;
#[doc = "Register ICR `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `OSC0RDY`"]
pub struct OSC0RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC0RDY_W<'a> {
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
#[doc = "Write proxy for field `DFLL0LOCKC`"]
pub struct DFLL0LOCKC_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLL0LOCKC_W<'a> {
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
#[doc = "Write proxy for field `DFLL0LOCKF`"]
pub struct DFLL0LOCKF_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLL0LOCKF_W<'a> {
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
#[doc = "Write proxy for field `DFLL0RDY`"]
pub struct DFLL0RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLL0RDY_W<'a> {
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
#[doc = "Write proxy for field `DFLL0RCS`"]
pub struct DFLL0RCS_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLL0RCS_W<'a> {
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
#[doc = "Write proxy for field `DFLL0OOB`"]
pub struct DFLL0OOB_W<'a> {
    w: &'a mut W,
}
impl<'a> DFLL0OOB_W<'a> {
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
#[doc = "Write proxy for field `PLL0LOCK`"]
pub struct PLL0LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL0LOCK_W<'a> {
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
#[doc = "Write proxy for field `PLL0LOCKLOST`"]
pub struct PLL0LOCKLOST_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL0LOCKLOST_W<'a> {
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
#[doc = "Write proxy for field `RCFASTLOCK`"]
pub struct RCFASTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> RCFASTLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Write proxy for field `RCFASTLOCKLOST`"]
pub struct RCFASTLOCKLOST_W<'a> {
    w: &'a mut W,
}
impl<'a> RCFASTLOCKLOST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `AE`"]
pub struct AE_W<'a> {
    w: &'a mut W,
}
impl<'a> AE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - OSC0 Ready"]
    #[inline(always)]
    pub fn osc0rdy(&mut self) -> OSC0RDY_W {
        OSC0RDY_W { w: self }
    }
    #[doc = "Bit 1 - DFLL0 Lock Coarse"]
    #[inline(always)]
    pub fn dfll0lockc(&mut self) -> DFLL0LOCKC_W {
        DFLL0LOCKC_W { w: self }
    }
    #[doc = "Bit 2 - DFLL0 Lock Fine"]
    #[inline(always)]
    pub fn dfll0lockf(&mut self) -> DFLL0LOCKF_W {
        DFLL0LOCKF_W { w: self }
    }
    #[doc = "Bit 3 - DFLL0 Ready"]
    #[inline(always)]
    pub fn dfll0rdy(&mut self) -> DFLL0RDY_W {
        DFLL0RDY_W { w: self }
    }
    #[doc = "Bit 4 - DFLL0 Reference Clock Stopped"]
    #[inline(always)]
    pub fn dfll0rcs(&mut self) -> DFLL0RCS_W {
        DFLL0RCS_W { w: self }
    }
    #[doc = "Bit 5 - DFLL0 Out Of Bounds"]
    #[inline(always)]
    pub fn dfll0oob(&mut self) -> DFLL0OOB_W {
        DFLL0OOB_W { w: self }
    }
    #[doc = "Bit 6 - PLL0 Lock"]
    #[inline(always)]
    pub fn pll0lock(&mut self) -> PLL0LOCK_W {
        PLL0LOCK_W { w: self }
    }
    #[doc = "Bit 7 - PLL0 Lock Lost"]
    #[inline(always)]
    pub fn pll0locklost(&mut self) -> PLL0LOCKLOST_W {
        PLL0LOCKLOST_W { w: self }
    }
    #[doc = "Bit 13 - RCFAST Lock"]
    #[inline(always)]
    pub fn rcfastlock(&mut self) -> RCFASTLOCK_W {
        RCFASTLOCK_W { w: self }
    }
    #[doc = "Bit 14 - RCFAST Lock Lost"]
    #[inline(always)]
    pub fn rcfastlocklost(&mut self) -> RCFASTLOCKLOST_W {
        RCFASTLOCKLOST_W { w: self }
    }
    #[doc = "Bit 31 - Access Error"]
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W {
        AE_W { w: self }
    }
}
