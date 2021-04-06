#[doc = "Writer for register IDR"]
pub type W = crate::W<u32, super::IDR>;
#[doc = "Write proxy for field `EP0INT`"]
pub struct EP0INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP0INT_W<'a> {
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
#[doc = "Write proxy for field `EP1INT`"]
pub struct EP1INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1INT_W<'a> {
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
#[doc = "Write proxy for field `EP2INT`"]
pub struct EP2INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2INT_W<'a> {
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
#[doc = "Write proxy for field `EP3INT`"]
pub struct EP3INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3INT_W<'a> {
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
#[doc = "Write proxy for field `EP4INT`"]
pub struct EP4INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4INT_W<'a> {
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
#[doc = "Write proxy for field `EP5INT`"]
pub struct EP5INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5INT_W<'a> {
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
#[doc = "Write proxy for field `EP6INT`"]
pub struct EP6INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6INT_W<'a> {
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
#[doc = "Write proxy for field `EP7INT`"]
pub struct EP7INT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7INT_W<'a> {
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
#[doc = "Write proxy for field `RXSUSP`"]
pub struct RXSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSUSP_W<'a> {
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
#[doc = "Write proxy for field `RXRSM`"]
pub struct RXRSM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRSM_W<'a> {
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
#[doc = "Write proxy for field `EXTRSM`"]
pub struct EXTRSM_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTRSM_W<'a> {
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
#[doc = "Write proxy for field `SOFINT`"]
pub struct SOFINT_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFINT_W<'a> {
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
#[doc = "Write proxy for field `WAKEUP`"]
pub struct WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKEUP_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Disable Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn ep0int(&mut self) -> EP0INT_W {
        EP0INT_W { w: self }
    }
    #[doc = "Bit 1 - Disable Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn ep1int(&mut self) -> EP1INT_W {
        EP1INT_W { w: self }
    }
    #[doc = "Bit 2 - Disable Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn ep2int(&mut self) -> EP2INT_W {
        EP2INT_W { w: self }
    }
    #[doc = "Bit 3 - Disable Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn ep3int(&mut self) -> EP3INT_W {
        EP3INT_W { w: self }
    }
    #[doc = "Bit 4 - Disable Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn ep4int(&mut self) -> EP4INT_W {
        EP4INT_W { w: self }
    }
    #[doc = "Bit 5 - Disable Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn ep5int(&mut self) -> EP5INT_W {
        EP5INT_W { w: self }
    }
    #[doc = "Bit 6 - Disable Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn ep6int(&mut self) -> EP6INT_W {
        EP6INT_W { w: self }
    }
    #[doc = "Bit 7 - Disable Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn ep7int(&mut self) -> EP7INT_W {
        EP7INT_W { w: self }
    }
    #[doc = "Bit 8 - Disable UDP Suspend Interrupt"]
    #[inline(always)]
    pub fn rxsusp(&mut self) -> RXSUSP_W {
        RXSUSP_W { w: self }
    }
    #[doc = "Bit 9 - Disable UDP Resume Interrupt"]
    #[inline(always)]
    pub fn rxrsm(&mut self) -> RXRSM_W {
        RXRSM_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn extrsm(&mut self) -> EXTRSM_W {
        EXTRSM_W { w: self }
    }
    #[doc = "Bit 11 - Disable Start Of Frame Interrupt"]
    #[inline(always)]
    pub fn sofint(&mut self) -> SOFINT_W {
        SOFINT_W { w: self }
    }
    #[doc = "Bit 13 - Disable USB Bus Interrupt"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WAKEUP_W {
        WAKEUP_W { w: self }
    }
}
