#[doc = "Reader of register GTYPE"]
pub type R = crate::R<u32, super::GTYPE>;
#[doc = "Writer for register GTYPE"]
pub type W = crate::W<u32, super::GTYPE>;
#[doc = "Register GTYPE `reset()`'s with value 0"]
impl crate::ResetValue for super::GTYPE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `NB_CH`"]
pub type NB_CH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NB_CH`"]
pub struct NB_CH_W<'a> {
    w: &'a mut W,
}
impl<'a> NB_CH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `FIFO_SZ`"]
pub type FIFO_SZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FIFO_SZ`"]
pub struct FIFO_SZ_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_SZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 5)) | (((value as u32) & 0x07ff) << 5);
        self.w
    }
}
#[doc = "Reader of field `NB_REQ`"]
pub type NB_REQ_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NB_REQ`"]
pub struct NB_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> NB_REQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Number of Channels Minus One"]
    #[inline(always)]
    pub fn nb_ch(&self) -> NB_CH_R {
        NB_CH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:15 - Number of Bytes"]
    #[inline(always)]
    pub fn fifo_sz(&self) -> FIFO_SZ_R {
        FIFO_SZ_R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:22 - Number of Peripheral Requests Minus One"]
    #[inline(always)]
    pub fn nb_req(&self) -> NB_REQ_R {
        NB_REQ_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of Channels Minus One"]
    #[inline(always)]
    pub fn nb_ch(&mut self) -> NB_CH_W {
        NB_CH_W { w: self }
    }
    #[doc = "Bits 5:15 - Number of Bytes"]
    #[inline(always)]
    pub fn fifo_sz(&mut self) -> FIFO_SZ_W {
        FIFO_SZ_W { w: self }
    }
    #[doc = "Bits 16:22 - Number of Peripheral Requests Minus One"]
    #[inline(always)]
    pub fn nb_req(&mut self) -> NB_REQ_W {
        NB_REQ_W { w: self }
    }
}
