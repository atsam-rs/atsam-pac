#[doc = "Reader of register TIM"]
pub type R = crate::R<u32, super::TIM>;
#[doc = "Writer for register TIM"]
pub type W = crate::W<u32, super::TIM>;
#[doc = "Register TIM `reset()`'s with value 0"]
impl crate::ResetValue for super::TIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRESC`"]
pub type PRESC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRESC`"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
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
#[doc = "Reader of field `CLKDIV`"]
pub type CLKDIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKDIV`"]
pub struct CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | (((value as u32) & 0x07) << 1);
        self.w
    }
}
#[doc = "Reader of field `FC0`"]
pub type FC0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FC0`"]
pub struct FC0_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `FC0PB`"]
pub type FC0PB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FC0PB`"]
pub struct FC0PB_W<'a> {
    w: &'a mut W,
}
impl<'a> FC0PB_W<'a> {
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
#[doc = "Reader of field `FC1`"]
pub type FC1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FC1`"]
pub struct FC1_W<'a> {
    w: &'a mut W,
}
impl<'a> FC1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `FC2`"]
pub type FC2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FC2`"]
pub struct FC2_W<'a> {
    w: &'a mut W,
}
impl<'a> FC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LCD Prescaler Select"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - LCD Clock Division"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bits 8:12 - Frame Counter 0"]
    #[inline(always)]
    pub fn fc0(&self) -> FC0_R {
        FC0_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Frame Counter 0 Prescaler Bypass"]
    #[inline(always)]
    pub fn fc0pb(&self) -> FC0PB_R {
        FC0PB_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Frame Counter 1"]
    #[inline(always)]
    pub fn fc1(&self) -> FC1_R {
        FC1_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Frame Counter 2"]
    #[inline(always)]
    pub fn fc2(&self) -> FC2_R {
        FC2_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Prescaler Select"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bits 1:3 - LCD Clock Division"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> CLKDIV_W {
        CLKDIV_W { w: self }
    }
    #[doc = "Bits 8:12 - Frame Counter 0"]
    #[inline(always)]
    pub fn fc0(&mut self) -> FC0_W {
        FC0_W { w: self }
    }
    #[doc = "Bit 13 - Frame Counter 0 Prescaler Bypass"]
    #[inline(always)]
    pub fn fc0pb(&mut self) -> FC0PB_W {
        FC0PB_W { w: self }
    }
    #[doc = "Bits 16:20 - Frame Counter 1"]
    #[inline(always)]
    pub fn fc1(&mut self) -> FC1_W {
        FC1_W { w: self }
    }
    #[doc = "Bits 24:28 - Frame Counter 2"]
    #[inline(always)]
    pub fn fc2(&mut self) -> FC2_W {
        FC2_W { w: self }
    }
}
