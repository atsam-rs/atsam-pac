#[doc = "Reader of register DELAYR"]
pub type R = crate::R<u32, super::DELAYR>;
#[doc = "Writer for register DELAYR"]
pub type W = crate::W<u32, super::DELAYR>;
#[doc = "Register DELAYR `reset()`'s with value 0"]
impl crate::ResetValue for super::DELAYR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `Delay0`"]
pub type DELAY0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Delay0`"]
pub struct DELAY0_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `Delay1`"]
pub type DELAY1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Delay1`"]
pub struct DELAY1_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `Delay2`"]
pub type DELAY2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Delay2`"]
pub struct DELAY2_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `Delay3`"]
pub type DELAY3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Delay3`"]
pub struct DELAY3_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `Delay4`"]
pub type DELAY4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Delay4`"]
pub struct DELAY4_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `Delay5`"]
pub type DELAY5_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Delay5`"]
pub struct DELAY5_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `Delay6`"]
pub type DELAY6_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Delay6`"]
pub struct DELAY6_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `Delay7`"]
pub type DELAY7_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `Delay7`"]
pub struct DELAY7_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay0(&self) -> DELAY0_R {
        DELAY0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay1(&self) -> DELAY1_R {
        DELAY1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay2(&self) -> DELAY2_R {
        DELAY2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay3(&self) -> DELAY3_R {
        DELAY3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay4(&self) -> DELAY4_R {
        DELAY4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay5(&self) -> DELAY5_R {
        DELAY5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay6(&self) -> DELAY6_R {
        DELAY6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay7(&self) -> DELAY7_R {
        DELAY7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay0(&mut self) -> DELAY0_W {
        DELAY0_W { w: self }
    }
    #[doc = "Bits 4:7 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay1(&mut self) -> DELAY1_W {
        DELAY1_W { w: self }
    }
    #[doc = "Bits 8:11 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay2(&mut self) -> DELAY2_W {
        DELAY2_W { w: self }
    }
    #[doc = "Bits 12:15 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay3(&mut self) -> DELAY3_W {
        DELAY3_W { w: self }
    }
    #[doc = "Bits 16:19 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay4(&mut self) -> DELAY4_W {
        DELAY4_W { w: self }
    }
    #[doc = "Bits 20:23 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay5(&mut self) -> DELAY5_W {
        DELAY5_W { w: self }
    }
    #[doc = "Bits 24:27 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay6(&mut self) -> DELAY6_W {
        DELAY6_W { w: self }
    }
    #[doc = "Bits 28:31 - Delay Control for Simultaneous Switch Reduction"]
    #[inline(always)]
    pub fn delay7(&mut self) -> DELAY7_W {
        DELAY7_W { w: self }
    }
}
