#[doc = "Reader of register CCFG_SYSIO"]
pub type R = crate::R<u32, super::CCFG_SYSIO>;
#[doc = "Writer for register CCFG_SYSIO"]
pub type W = crate::W<u32, super::CCFG_SYSIO>;
#[doc = "Register CCFG_SYSIO `reset()`'s with value 0"]
impl crate::ResetValue for super::CCFG_SYSIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSIO4`"]
pub type SYSIO4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSIO4`"]
pub struct SYSIO4_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO4_W<'a> {
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
#[doc = "Reader of field `SYSIO5`"]
pub type SYSIO5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSIO5`"]
pub struct SYSIO5_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO5_W<'a> {
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
#[doc = "Reader of field `SYSIO6`"]
pub type SYSIO6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSIO6`"]
pub struct SYSIO6_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO6_W<'a> {
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
#[doc = "Reader of field `SYSIO7`"]
pub type SYSIO7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSIO7`"]
pub struct SYSIO7_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO7_W<'a> {
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
#[doc = "Reader of field `SYSIO10`"]
pub type SYSIO10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSIO10`"]
pub struct SYSIO10_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO10_W<'a> {
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
#[doc = "Reader of field `SYSIO11`"]
pub type SYSIO11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSIO11`"]
pub struct SYSIO11_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO11_W<'a> {
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
#[doc = "Reader of field `SYSIO12`"]
pub type SYSIO12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SYSIO12`"]
pub struct SYSIO12_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSIO12_W<'a> {
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
impl R {
    #[doc = "Bit 4 - PB4 or TDI Assignment"]
    #[inline(always)]
    pub fn sysio4(&self) -> SYSIO4_R {
        SYSIO4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PB5 or TDO/TRACESWO Assignment"]
    #[inline(always)]
    pub fn sysio5(&self) -> SYSIO5_R {
        SYSIO5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PB6 or TMS/SWDIO Assignment"]
    #[inline(always)]
    pub fn sysio6(&self) -> SYSIO6_R {
        SYSIO6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PB7 or TCK/SWCLK Assignment"]
    #[inline(always)]
    pub fn sysio7(&self) -> SYSIO7_R {
        SYSIO7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PB10 or DDM Assignment"]
    #[inline(always)]
    pub fn sysio10(&self) -> SYSIO10_R {
        SYSIO10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - PB11 or DDP Assignment"]
    #[inline(always)]
    pub fn sysio11(&self) -> SYSIO11_R {
        SYSIO11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - PB12 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&self) -> SYSIO12_R {
        SYSIO12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - PB4 or TDI Assignment"]
    #[inline(always)]
    pub fn sysio4(&mut self) -> SYSIO4_W {
        SYSIO4_W { w: self }
    }
    #[doc = "Bit 5 - PB5 or TDO/TRACESWO Assignment"]
    #[inline(always)]
    pub fn sysio5(&mut self) -> SYSIO5_W {
        SYSIO5_W { w: self }
    }
    #[doc = "Bit 6 - PB6 or TMS/SWDIO Assignment"]
    #[inline(always)]
    pub fn sysio6(&mut self) -> SYSIO6_W {
        SYSIO6_W { w: self }
    }
    #[doc = "Bit 7 - PB7 or TCK/SWCLK Assignment"]
    #[inline(always)]
    pub fn sysio7(&mut self) -> SYSIO7_W {
        SYSIO7_W { w: self }
    }
    #[doc = "Bit 10 - PB10 or DDM Assignment"]
    #[inline(always)]
    pub fn sysio10(&mut self) -> SYSIO10_W {
        SYSIO10_W { w: self }
    }
    #[doc = "Bit 11 - PB11 or DDP Assignment"]
    #[inline(always)]
    pub fn sysio11(&mut self) -> SYSIO11_W {
        SYSIO11_W { w: self }
    }
    #[doc = "Bit 12 - PB12 or ERASE Assignment"]
    #[inline(always)]
    pub fn sysio12(&mut self) -> SYSIO12_W {
        SYSIO12_W { w: self }
    }
}
