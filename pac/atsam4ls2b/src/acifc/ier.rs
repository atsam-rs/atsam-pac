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
#[doc = "Write proxy for field `ACINT0`"]
pub struct ACINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT0_W<'a> {
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
#[doc = "Write proxy for field `SUTINT0`"]
pub struct SUTINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT0_W<'a> {
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
#[doc = "Write proxy for field `ACINT1`"]
pub struct ACINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT1_W<'a> {
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
#[doc = "Write proxy for field `SUTINT1`"]
pub struct SUTINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT1_W<'a> {
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
#[doc = "Write proxy for field `ACINT2`"]
pub struct ACINT2_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT2_W<'a> {
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
#[doc = "Write proxy for field `SUTINT2`"]
pub struct SUTINT2_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT2_W<'a> {
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
#[doc = "Write proxy for field `ACINT3`"]
pub struct ACINT3_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT3_W<'a> {
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
#[doc = "Write proxy for field `SUTINT3`"]
pub struct SUTINT3_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT3_W<'a> {
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
#[doc = "Write proxy for field `ACINT4`"]
pub struct ACINT4_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT4_W<'a> {
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
#[doc = "Write proxy for field `SUTINT4`"]
pub struct SUTINT4_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT4_W<'a> {
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
#[doc = "Write proxy for field `ACINT5`"]
pub struct ACINT5_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Write proxy for field `SUTINT5`"]
pub struct SUTINT5_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Write proxy for field `ACINT6`"]
pub struct ACINT6_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Write proxy for field `SUTINT6`"]
pub struct SUTINT6_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT6_W<'a> {
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
#[doc = "Write proxy for field `ACINT7`"]
pub struct ACINT7_W<'a> {
    w: &'a mut W,
}
impl<'a> ACINT7_W<'a> {
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
#[doc = "Write proxy for field `SUTINT7`"]
pub struct SUTINT7_W<'a> {
    w: &'a mut W,
}
impl<'a> SUTINT7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Write proxy for field `WFINT0`"]
pub struct WFINT0_W<'a> {
    w: &'a mut W,
}
impl<'a> WFINT0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Write proxy for field `WFINT1`"]
pub struct WFINT1_W<'a> {
    w: &'a mut W,
}
impl<'a> WFINT1_W<'a> {
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
#[doc = "Write proxy for field `WFINT2`"]
pub struct WFINT2_W<'a> {
    w: &'a mut W,
}
impl<'a> WFINT2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Write proxy for field `WFINT3`"]
pub struct WFINT3_W<'a> {
    w: &'a mut W,
}
impl<'a> WFINT3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - AC0 Interrupt Enable"]
    #[inline(always)]
    pub fn acint0(&mut self) -> ACINT0_W {
        ACINT0_W { w: self }
    }
    #[doc = "Bit 1 - AC0 Startup Time Interrupt Enable"]
    #[inline(always)]
    pub fn sutint0(&mut self) -> SUTINT0_W {
        SUTINT0_W { w: self }
    }
    #[doc = "Bit 2 - AC1 Interrupt Enable"]
    #[inline(always)]
    pub fn acint1(&mut self) -> ACINT1_W {
        ACINT1_W { w: self }
    }
    #[doc = "Bit 3 - AC1 Startup Time Interrupt Enable"]
    #[inline(always)]
    pub fn sutint1(&mut self) -> SUTINT1_W {
        SUTINT1_W { w: self }
    }
    #[doc = "Bit 4 - AC2 Interrupt Enable"]
    #[inline(always)]
    pub fn acint2(&mut self) -> ACINT2_W {
        ACINT2_W { w: self }
    }
    #[doc = "Bit 5 - AC2 Startup Time Interrupt Enable"]
    #[inline(always)]
    pub fn sutint2(&mut self) -> SUTINT2_W {
        SUTINT2_W { w: self }
    }
    #[doc = "Bit 6 - AC3 Interrupt Enable"]
    #[inline(always)]
    pub fn acint3(&mut self) -> ACINT3_W {
        ACINT3_W { w: self }
    }
    #[doc = "Bit 7 - AC3 Startup Time Interrupt Enable"]
    #[inline(always)]
    pub fn sutint3(&mut self) -> SUTINT3_W {
        SUTINT3_W { w: self }
    }
    #[doc = "Bit 8 - AC4 Interrupt Enable"]
    #[inline(always)]
    pub fn acint4(&mut self) -> ACINT4_W {
        ACINT4_W { w: self }
    }
    #[doc = "Bit 9 - AC4 Startup Time Interrupt Enable"]
    #[inline(always)]
    pub fn sutint4(&mut self) -> SUTINT4_W {
        SUTINT4_W { w: self }
    }
    #[doc = "Bit 10 - AC5 Interrupt Enable"]
    #[inline(always)]
    pub fn acint5(&mut self) -> ACINT5_W {
        ACINT5_W { w: self }
    }
    #[doc = "Bit 11 - AC5 Startup Time Interrupt Enable"]
    #[inline(always)]
    pub fn sutint5(&mut self) -> SUTINT5_W {
        SUTINT5_W { w: self }
    }
    #[doc = "Bit 12 - AC6 Interrupt Enable"]
    #[inline(always)]
    pub fn acint6(&mut self) -> ACINT6_W {
        ACINT6_W { w: self }
    }
    #[doc = "Bit 13 - AC6 Startup Time Interrupt Enable"]
    #[inline(always)]
    pub fn sutint6(&mut self) -> SUTINT6_W {
        SUTINT6_W { w: self }
    }
    #[doc = "Bit 14 - AC7 Interrupt Enable"]
    #[inline(always)]
    pub fn acint7(&mut self) -> ACINT7_W {
        ACINT7_W { w: self }
    }
    #[doc = "Bit 15 - AC7 Startup Time Interrupt Enable"]
    #[inline(always)]
    pub fn sutint7(&mut self) -> SUTINT7_W {
        SUTINT7_W { w: self }
    }
    #[doc = "Bit 24 - Window0 Mode Interrupt Enable"]
    #[inline(always)]
    pub fn wfint0(&mut self) -> WFINT0_W {
        WFINT0_W { w: self }
    }
    #[doc = "Bit 25 - Window1 Mode Interrupt Enable"]
    #[inline(always)]
    pub fn wfint1(&mut self) -> WFINT1_W {
        WFINT1_W { w: self }
    }
    #[doc = "Bit 26 - Window2 Mode Interrupt Enable"]
    #[inline(always)]
    pub fn wfint2(&mut self) -> WFINT2_W {
        WFINT2_W { w: self }
    }
    #[doc = "Bit 27 - Window3 Mode Interrupt Enable"]
    #[inline(always)]
    pub fn wfint3(&mut self) -> WFINT3_W {
        WFINT3_W { w: self }
    }
}
