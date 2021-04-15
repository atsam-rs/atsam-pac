#[doc = "Writer for register IDR"]
pub type W = crate::W<u32, super::IDR>;
#[doc = "Register IDR `reset()`'s with value 0"]
impl crate::ResetValue for super::IDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `OSC32RDY`"]
pub struct OSC32RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32RDY_W<'a> {
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
#[doc = "Write proxy for field `RC32KRDY`"]
pub struct RC32KRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32KRDY_W<'a> {
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
#[doc = "Write proxy for field `RC32KLOCK`"]
pub struct RC32KLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32KLOCK_W<'a> {
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
#[doc = "Write proxy for field `RC32KREFE`"]
pub struct RC32KREFE_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32KREFE_W<'a> {
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
#[doc = "Write proxy for field `RC32KSAT`"]
pub struct RC32KSAT_W<'a> {
    w: &'a mut W,
}
impl<'a> RC32KSAT_W<'a> {
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
#[doc = "Write proxy for field `BOD33DET`"]
pub struct BOD33DET_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD33DET_W<'a> {
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
#[doc = "Write proxy for field `BOD18DET`"]
pub struct BOD18DET_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD18DET_W<'a> {
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
#[doc = "Write proxy for field `BOD33SYNRDY`"]
pub struct BOD33SYNRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD33SYNRDY_W<'a> {
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
#[doc = "Write proxy for field `BOD18SYNRDY`"]
pub struct BOD18SYNRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BOD18SYNRDY_W<'a> {
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
#[doc = "Write proxy for field `SSWRDY`"]
pub struct SSWRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SSWRDY_W<'a> {
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
#[doc = "Write proxy for field `VREGOK`"]
pub struct VREGOK_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGOK_W<'a> {
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
#[doc = "Write proxy for field `LPBGRDY`"]
pub struct LPBGRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBGRDY_W<'a> {
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
    #[doc = "Bit 0 - 32kHz Oscillator Ready"]
    #[inline(always)]
    pub fn osc32rdy(&mut self) -> OSC32RDY_W {
        OSC32RDY_W { w: self }
    }
    #[doc = "Bit 1 - 32kHz RC Oscillator Ready"]
    #[inline(always)]
    pub fn rc32krdy(&mut self) -> RC32KRDY_W {
        RC32KRDY_W { w: self }
    }
    #[doc = "Bit 2 - 32kHz RC Oscillator Lock"]
    #[inline(always)]
    pub fn rc32klock(&mut self) -> RC32KLOCK_W {
        RC32KLOCK_W { w: self }
    }
    #[doc = "Bit 3 - 32kHz RC Oscillator Reference Error"]
    #[inline(always)]
    pub fn rc32krefe(&mut self) -> RC32KREFE_W {
        RC32KREFE_W { w: self }
    }
    #[doc = "Bit 4 - 32kHz RC Oscillator Saturation"]
    #[inline(always)]
    pub fn rc32ksat(&mut self) -> RC32KSAT_W {
        RC32KSAT_W { w: self }
    }
    #[doc = "Bit 5 - BOD33 Detected"]
    #[inline(always)]
    pub fn bod33det(&mut self) -> BOD33DET_W {
        BOD33DET_W { w: self }
    }
    #[doc = "Bit 6 - BOD18 Detected"]
    #[inline(always)]
    pub fn bod18det(&mut self) -> BOD18DET_W {
        BOD18DET_W { w: self }
    }
    #[doc = "Bit 7 - BOD33 Synchronization Ready"]
    #[inline(always)]
    pub fn bod33synrdy(&mut self) -> BOD33SYNRDY_W {
        BOD33SYNRDY_W { w: self }
    }
    #[doc = "Bit 8 - BOD18 Synchronization Ready"]
    #[inline(always)]
    pub fn bod18synrdy(&mut self) -> BOD18SYNRDY_W {
        BOD18SYNRDY_W { w: self }
    }
    #[doc = "Bit 9 - VREG Stop Switching Ready"]
    #[inline(always)]
    pub fn sswrdy(&mut self) -> SSWRDY_W {
        SSWRDY_W { w: self }
    }
    #[doc = "Bit 10 - Mai n VREG OK"]
    #[inline(always)]
    pub fn vregok(&mut self) -> VREGOK_W {
        VREGOK_W { w: self }
    }
    #[doc = "Bit 12 - Low Power Bandgap Voltage Reference Ready"]
    #[inline(always)]
    pub fn lpbgrdy(&mut self) -> LPBGRDY_W {
        LPBGRDY_W { w: self }
    }
    #[doc = "Bit 31 - Access Error"]
    #[inline(always)]
    pub fn ae(&mut self) -> AE_W {
        AE_W { w: self }
    }
}
