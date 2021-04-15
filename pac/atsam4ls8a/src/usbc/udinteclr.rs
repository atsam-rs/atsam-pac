#[doc = "Writer for register UDINTECLR"]
pub type W = crate::W<u32, super::UDINTECLR>;
#[doc = "Register UDINTECLR `reset()`'s with value 0"]
impl crate::ResetValue for super::UDINTECLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `SUSPEC`"]
pub struct SUSPEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEC_W<'a> {
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
#[doc = "Write proxy for field `MSOFEC`"]
pub struct MSOFEC_W<'a> {
    w: &'a mut W,
}
impl<'a> MSOFEC_W<'a> {
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
#[doc = "Write proxy for field `SOFEC`"]
pub struct SOFEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFEC_W<'a> {
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
#[doc = "Write proxy for field `EORSTEC`"]
pub struct EORSTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSTEC_W<'a> {
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
#[doc = "Write proxy for field `WAKEUPEC`"]
pub struct WAKEUPEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUPEC_W<'a> {
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
#[doc = "Write proxy for field `EORSMEC`"]
pub struct EORSMEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EORSMEC_W<'a> {
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
#[doc = "Write proxy for field `UPRSMEC`"]
pub struct UPRSMEC_W<'a> {
    w: &'a mut W,
}
impl<'a> UPRSMEC_W<'a> {
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
#[doc = "Write proxy for field `EP0INTEC`"]
pub struct EP0INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0INTEC_W<'a> {
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
#[doc = "Write proxy for field `EP1INTEC`"]
pub struct EP1INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1INTEC_W<'a> {
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
#[doc = "Write proxy for field `EP2INTEC`"]
pub struct EP2INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2INTEC_W<'a> {
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
#[doc = "Write proxy for field `EP3INTEC`"]
pub struct EP3INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3INTEC_W<'a> {
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
#[doc = "Write proxy for field `EP4INTEC`"]
pub struct EP4INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4INTEC_W<'a> {
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
#[doc = "Write proxy for field `EP5INTEC`"]
pub struct EP5INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5INTEC_W<'a> {
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
#[doc = "Write proxy for field `EP6INTEC`"]
pub struct EP6INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6INTEC_W<'a> {
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
#[doc = "Write proxy for field `EP7INTEC`"]
pub struct EP7INTEC_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7INTEC_W<'a> {
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
    #[doc = "Bit 0 - SUSP Interrupt Enable Clear"]
    #[inline(always)]
    pub fn suspec(&mut self) -> SUSPEC_W {
        SUSPEC_W { w: self }
    }
    #[doc = "Bit 1 - MSOF Interrupt Enable Clear"]
    #[inline(always)]
    pub fn msofec(&mut self) -> MSOFEC_W {
        MSOFEC_W { w: self }
    }
    #[doc = "Bit 2 - SOF Interrupt Enable Clear"]
    #[inline(always)]
    pub fn sofec(&mut self) -> SOFEC_W {
        SOFEC_W { w: self }
    }
    #[doc = "Bit 3 - EORST Interrupt Enable Clear"]
    #[inline(always)]
    pub fn eorstec(&mut self) -> EORSTEC_W {
        EORSTEC_W { w: self }
    }
    #[doc = "Bit 4 - WAKEUP Interrupt Enable Clear"]
    #[inline(always)]
    pub fn wakeupec(&mut self) -> WAKEUPEC_W {
        WAKEUPEC_W { w: self }
    }
    #[doc = "Bit 5 - EORSM Interrupt Enable Clear"]
    #[inline(always)]
    pub fn eorsmec(&mut self) -> EORSMEC_W {
        EORSMEC_W { w: self }
    }
    #[doc = "Bit 6 - UPRSM Interrupt Enable Clear"]
    #[inline(always)]
    pub fn uprsmec(&mut self) -> UPRSMEC_W {
        UPRSMEC_W { w: self }
    }
    #[doc = "Bit 12 - EP0INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep0intec(&mut self) -> EP0INTEC_W {
        EP0INTEC_W { w: self }
    }
    #[doc = "Bit 13 - EP1INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep1intec(&mut self) -> EP1INTEC_W {
        EP1INTEC_W { w: self }
    }
    #[doc = "Bit 14 - EP2INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep2intec(&mut self) -> EP2INTEC_W {
        EP2INTEC_W { w: self }
    }
    #[doc = "Bit 15 - EP3INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep3intec(&mut self) -> EP3INTEC_W {
        EP3INTEC_W { w: self }
    }
    #[doc = "Bit 16 - EP4INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep4intec(&mut self) -> EP4INTEC_W {
        EP4INTEC_W { w: self }
    }
    #[doc = "Bit 17 - EP5INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep5intec(&mut self) -> EP5INTEC_W {
        EP5INTEC_W { w: self }
    }
    #[doc = "Bit 18 - EP6INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep6intec(&mut self) -> EP6INTEC_W {
        EP6INTEC_W { w: self }
    }
    #[doc = "Bit 19 - EP7INT Interrupt Enable Clear"]
    #[inline(always)]
    pub fn ep7intec(&mut self) -> EP7INTEC_W {
        EP7INTEC_W { w: self }
    }
}
