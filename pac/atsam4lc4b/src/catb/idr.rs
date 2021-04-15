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
#[doc = "Write proxy for field `SAMPLE`"]
pub struct SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_W<'a> {
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
#[doc = "Write proxy for field `INTCH`"]
pub struct INTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCH_W<'a> {
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
#[doc = "Write proxy for field `OUTTCH`"]
pub struct OUTTCH_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTTCH_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Sample Ready Interrupt Disable"]
    #[inline(always)]
    pub fn sample(&mut self) -> SAMPLE_W {
        SAMPLE_W { w: self }
    }
    #[doc = "Bit 1 - In-touch Interrupt Disable"]
    #[inline(always)]
    pub fn intch(&mut self) -> INTCH_W {
        INTCH_W { w: self }
    }
    #[doc = "Bit 2 - Out-of-Touch Interrupt Disable"]
    #[inline(always)]
    pub fn outtch(&mut self) -> OUTTCH_W {
        OUTTCH_W { w: self }
    }
}
