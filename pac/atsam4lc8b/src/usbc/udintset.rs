#[doc = "Writer for register UDINTSET"]
pub type W = crate::W<u32, super::UDINTSET>;
#[doc = "Register UDINTSET `reset()`'s with value 0"]
impl crate::ResetValue for super::UDINTSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SUSPS`"]
pub struct SUSPS_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPS_W<'a> {
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
#[doc = "Write proxy for field `MSOFS`"]
pub struct MSOFS_W<'a> {
    w: &'a mut W,
}
impl<'a> MSOFS_W<'a> {
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
#[doc = "Write proxy for field `SOFS`"]
pub struct SOFS_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFS_W<'a> {
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
#[doc = "Write proxy for field `EORSTS`"]
pub struct EORSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSTS_W<'a> {
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
#[doc = "Write proxy for field `WAKEUPS`"]
pub struct WAKEUPS_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPS_W<'a> {
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
#[doc = "Write proxy for field `EORSMS`"]
pub struct EORSMS_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSMS_W<'a> {
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
#[doc = "Write proxy for field `UPRSMS`"]
pub struct UPRSMS_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSMS_W<'a> {
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
impl W {
    #[doc = "Bit 0 - SUSP Interrupt Set"]
    #[inline(always)]
    pub fn susps(&mut self) -> SUSPS_W {
        SUSPS_W { w: self }
    }
    #[doc = "Bit 1 - MSOF Interrupt Set"]
    #[inline(always)]
    pub fn msofs(&mut self) -> MSOFS_W {
        MSOFS_W { w: self }
    }
    #[doc = "Bit 2 - SOF Interrupt Set"]
    #[inline(always)]
    pub fn sofs(&mut self) -> SOFS_W {
        SOFS_W { w: self }
    }
    #[doc = "Bit 3 - EORST Interrupt Set"]
    #[inline(always)]
    pub fn eorsts(&mut self) -> EORSTS_W {
        EORSTS_W { w: self }
    }
    #[doc = "Bit 4 - WAKEUP Interrupt Set"]
    #[inline(always)]
    pub fn wakeups(&mut self) -> WAKEUPS_W {
        WAKEUPS_W { w: self }
    }
    #[doc = "Bit 5 - EORSM Interrupt Set"]
    #[inline(always)]
    pub fn eorsms(&mut self) -> EORSMS_W {
        EORSMS_W { w: self }
    }
    #[doc = "Bit 6 - UPRSM Interrupt Set"]
    #[inline(always)]
    pub fn uprsms(&mut self) -> UPRSMS_W {
        UPRSMS_W { w: self }
    }
}
