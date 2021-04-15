#[doc = "Writer for register DFLL0SSG"]
pub type W = crate::W<u32, super::DFLL0SSG>;
#[doc = "Register DFLL0SSG `reset()`'s with value 0"]
impl crate::ResetValue for super::DFLL0SSG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Write proxy for field `PRBS`"]
pub struct PRBS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRBS_W<'a> {
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
#[doc = "Write proxy for field `AMPLITUDE`"]
pub struct AMPLITUDE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMPLITUDE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Write proxy for field `STEPSIZE`"]
pub struct STEPSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> STEPSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Pseudo Random Bit Sequence"]
    #[inline(always)]
    pub fn prbs(&mut self) -> PRBS_W {
        PRBS_W { w: self }
    }
    #[doc = "Bits 8:12 - SSG Amplitude"]
    #[inline(always)]
    pub fn amplitude(&mut self) -> AMPLITUDE_W {
        AMPLITUDE_W { w: self }
    }
    #[doc = "Bits 16:20 - SSG Step Size"]
    #[inline(always)]
    pub fn stepsize(&mut self) -> STEPSIZE_W {
        STEPSIZE_W { w: self }
    }
}
