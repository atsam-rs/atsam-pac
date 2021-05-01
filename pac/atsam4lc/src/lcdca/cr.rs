#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DIS`"]
pub struct DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_W<'a> {
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
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Write proxy for field `FC0DIS`"]
pub struct FC0DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0DIS_W<'a> {
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
#[doc = "Write proxy for field `FC0EN`"]
pub struct FC0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0EN_W<'a> {
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
#[doc = "Write proxy for field `FC1DIS`"]
pub struct FC1DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FC1DIS_W<'a> {
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
#[doc = "Write proxy for field `FC1EN`"]
pub struct FC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FC1EN_W<'a> {
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
#[doc = "Write proxy for field `FC2DIS`"]
pub struct FC2DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> FC2DIS_W<'a> {
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
#[doc = "Write proxy for field `FC2EN`"]
pub struct FC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FC2EN_W<'a> {
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
#[doc = "Write proxy for field `CDM`"]
pub struct CDM_W<'a> {
    w: &'a mut W,
}
impl<'a> CDM_W<'a> {
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
#[doc = "Write proxy for field `WDIS`"]
pub struct WDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDIS_W<'a> {
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
#[doc = "Write proxy for field `WEN`"]
pub struct WEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WEN_W<'a> {
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
#[doc = "Write proxy for field `BSTART`"]
pub struct BSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> BSTART_W<'a> {
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
#[doc = "Write proxy for field `BSTOP`"]
pub struct BSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> BSTOP_W<'a> {
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
#[doc = "Write proxy for field `CSTART`"]
pub struct CSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTART_W<'a> {
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
#[doc = "Write proxy for field `CSTOP`"]
pub struct CSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTOP_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Disable"]
    #[inline(always)]
    pub fn dis(&mut self) -> DIS_W {
        DIS_W { w: self }
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 2 - Frame Counter 0 Disable"]
    #[inline(always)]
    pub fn fc0dis(&mut self) -> FC0DIS_W {
        FC0DIS_W { w: self }
    }
    #[doc = "Bit 3 - Frame Counter 0 Enable"]
    #[inline(always)]
    pub fn fc0en(&mut self) -> FC0EN_W {
        FC0EN_W { w: self }
    }
    #[doc = "Bit 4 - Frame Counter 1 Disable"]
    #[inline(always)]
    pub fn fc1dis(&mut self) -> FC1DIS_W {
        FC1DIS_W { w: self }
    }
    #[doc = "Bit 5 - Frame Counter 1 Enable"]
    #[inline(always)]
    pub fn fc1en(&mut self) -> FC1EN_W {
        FC1EN_W { w: self }
    }
    #[doc = "Bit 6 - Frame Counter 2 Disable"]
    #[inline(always)]
    pub fn fc2dis(&mut self) -> FC2DIS_W {
        FC2DIS_W { w: self }
    }
    #[doc = "Bit 7 - Frame Counter 2 Enable"]
    #[inline(always)]
    pub fn fc2en(&mut self) -> FC2EN_W {
        FC2EN_W { w: self }
    }
    #[doc = "Bit 8 - Clear Display Memory"]
    #[inline(always)]
    pub fn cdm(&mut self) -> CDM_W {
        CDM_W { w: self }
    }
    #[doc = "Bit 9 - Wake up Disable"]
    #[inline(always)]
    pub fn wdis(&mut self) -> WDIS_W {
        WDIS_W { w: self }
    }
    #[doc = "Bit 10 - Wake up Enable"]
    #[inline(always)]
    pub fn wen(&mut self) -> WEN_W {
        WEN_W { w: self }
    }
    #[doc = "Bit 11 - Blinking Start"]
    #[inline(always)]
    pub fn bstart(&mut self) -> BSTART_W {
        BSTART_W { w: self }
    }
    #[doc = "Bit 12 - Blinking Stop"]
    #[inline(always)]
    pub fn bstop(&mut self) -> BSTOP_W {
        BSTOP_W { w: self }
    }
    #[doc = "Bit 13 - Circular Shift Start"]
    #[inline(always)]
    pub fn cstart(&mut self) -> CSTART_W {
        CSTART_W { w: self }
    }
    #[doc = "Bit 14 - Circular Shift Stop"]
    #[inline(always)]
    pub fn cstop(&mut self) -> CSTOP_W {
        CSTOP_W { w: self }
    }
}
