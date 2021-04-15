#[doc = "Writer for register UDINTESET"]
pub type W = crate::W<u32, super::UDINTESET>;
#[doc = "Register UDINTESET `reset()`'s with value 0"]
impl crate::ResetValue for super::UDINTESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SUSPES`"]
pub struct SUSPES_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPES_W<'a> {
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
#[doc = "Write proxy for field `MSOFES`"]
pub struct MSOFES_W<'a> {
    w: &'a mut W,
}
impl<'a> MSOFES_W<'a> {
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
#[doc = "Write proxy for field `SOFES`"]
pub struct SOFES_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFES_W<'a> {
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
#[doc = "Write proxy for field `EORSTES`"]
pub struct EORSTES_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSTES_W<'a> {
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
#[doc = "Write proxy for field `WAKEUPES`"]
pub struct WAKEUPES_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPES_W<'a> {
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
#[doc = "Write proxy for field `EORSMES`"]
pub struct EORSMES_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSMES_W<'a> {
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
#[doc = "Write proxy for field `UPRSMES`"]
pub struct UPRSMES_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSMES_W<'a> {
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
#[doc = "Write proxy for field `EP0INTES`"]
pub struct EP0INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0INTES_W<'a> {
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
#[doc = "Write proxy for field `EP1INTES`"]
pub struct EP1INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1INTES_W<'a> {
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
#[doc = "Write proxy for field `EP2INTES`"]
pub struct EP2INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2INTES_W<'a> {
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
#[doc = "Write proxy for field `EP3INTES`"]
pub struct EP3INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3INTES_W<'a> {
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
#[doc = "Write proxy for field `EP4INTES`"]
pub struct EP4INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4INTES_W<'a> {
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
#[doc = "Write proxy for field `EP5INTES`"]
pub struct EP5INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5INTES_W<'a> {
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
#[doc = "Write proxy for field `EP6INTES`"]
pub struct EP6INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6INTES_W<'a> {
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
#[doc = "Write proxy for field `EP7INTES`"]
pub struct EP7INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7INTES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - SUSP Interrupt Enable Set"]
    #[inline(always)]
    pub fn suspes(&mut self) -> SUSPES_W {
        SUSPES_W { w: self }
    }
    #[doc = "Bit 1 - MSOF Interrupt Enable Set"]
    #[inline(always)]
    pub fn msofes(&mut self) -> MSOFES_W {
        MSOFES_W { w: self }
    }
    #[doc = "Bit 2 - SOF Interrupt Enable Set"]
    #[inline(always)]
    pub fn sofes(&mut self) -> SOFES_W {
        SOFES_W { w: self }
    }
    #[doc = "Bit 3 - EORST Interrupt Enable Set"]
    #[inline(always)]
    pub fn eorstes(&mut self) -> EORSTES_W {
        EORSTES_W { w: self }
    }
    #[doc = "Bit 4 - WAKEUP Interrupt Enable Set"]
    #[inline(always)]
    pub fn wakeupes(&mut self) -> WAKEUPES_W {
        WAKEUPES_W { w: self }
    }
    #[doc = "Bit 5 - EORSM Interrupt Enable Set"]
    #[inline(always)]
    pub fn eorsmes(&mut self) -> EORSMES_W {
        EORSMES_W { w: self }
    }
    #[doc = "Bit 6 - UPRSM Interrupt Enable Set"]
    #[inline(always)]
    pub fn uprsmes(&mut self) -> UPRSMES_W {
        UPRSMES_W { w: self }
    }
    #[doc = "Bit 12 - EP0INT Interrupt Enable Set"]
    #[inline(always)]
    pub fn ep0intes(&mut self) -> EP0INTES_W {
        EP0INTES_W { w: self }
    }
    #[doc = "Bit 13 - EP1INT Interrupt Enable Set"]
    #[inline(always)]
    pub fn ep1intes(&mut self) -> EP1INTES_W {
        EP1INTES_W { w: self }
    }
    #[doc = "Bit 14 - EP2INT Interrupt Enable Set"]
    #[inline(always)]
    pub fn ep2intes(&mut self) -> EP2INTES_W {
        EP2INTES_W { w: self }
    }
    #[doc = "Bit 15 - EP3INT Interrupt Enable Set"]
    #[inline(always)]
    pub fn ep3intes(&mut self) -> EP3INTES_W {
        EP3INTES_W { w: self }
    }
    #[doc = "Bit 16 - EP4INT Interrupt Enable Set"]
    #[inline(always)]
    pub fn ep4intes(&mut self) -> EP4INTES_W {
        EP4INTES_W { w: self }
    }
    #[doc = "Bit 17 - EP5INT Interrupt Enable Set"]
    #[inline(always)]
    pub fn ep5intes(&mut self) -> EP5INTES_W {
        EP5INTES_W { w: self }
    }
    #[doc = "Bit 18 - EP6INT Interrupt Enable Set"]
    #[inline(always)]
    pub fn ep6intes(&mut self) -> EP6INTES_W {
        EP6INTES_W { w: self }
    }
    #[doc = "Bit 19 - EP7INT Interrupt Enable Set"]
    #[inline(always)]
    pub fn ep7intes(&mut self) -> EP7INTES_W {
        EP7INTES_W { w: self }
    }
}
