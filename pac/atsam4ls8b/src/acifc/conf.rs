#[doc = "Reader of register CONF%s"]
pub type R = crate::R<u32, super::CONF>;
#[doc = "Writer for register CONF%s"]
pub type W = crate::W<u32, super::CONF>;
#[doc = "Register CONF%s `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IS`"]
pub type IS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IS`"]
pub struct IS_W<'a> {
    w: &'a mut W,
}
impl<'a> IS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `INSELN`"]
pub type INSELN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INSELN`"]
pub struct INSELN_W<'a> {
    w: &'a mut W,
}
impl<'a> INSELN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `EVENN`"]
pub type EVENN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENN`"]
pub struct EVENN_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENN_W<'a> {
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
#[doc = "Reader of field `EVENP`"]
pub type EVENP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVENP`"]
pub struct EVENP_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENP_W<'a> {
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
#[doc = "Reader of field `HYS`"]
pub type HYS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HYS`"]
pub struct HYS_W<'a> {
    w: &'a mut W,
}
impl<'a> HYS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Reader of field `FAST`"]
pub type FAST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FAST`"]
pub struct FAST_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `ALWAYSON`"]
pub type ALWAYSON_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALWAYSON`"]
pub struct ALWAYSON_W<'a> {
    w: &'a mut W,
}
impl<'a> ALWAYSON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Interupt Settings"]
    #[inline(always)]
    pub fn is(&self) -> IS_R {
        IS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Analog Comparator Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Negative Input Select"]
    #[inline(always)]
    pub fn inseln(&self) -> INSELN_R {
        INSELN_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Peripheral Event Enable Negative"]
    #[inline(always)]
    pub fn evenn(&self) -> EVENN_R {
        EVENN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Peripheral Event Enable Positive"]
    #[inline(always)]
    pub fn evenp(&self) -> EVENP_R {
        EVENP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Hysteresis Voltage Value"]
    #[inline(always)]
    pub fn hys(&self) -> HYS_R {
        HYS_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - Fast Mode Enable"]
    #[inline(always)]
    pub fn fast(&self) -> FAST_R {
        FAST_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Always On"]
    #[inline(always)]
    pub fn alwayson(&self) -> ALWAYSON_R {
        ALWAYSON_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Interupt Settings"]
    #[inline(always)]
    pub fn is(&mut self) -> IS_W {
        IS_W { w: self }
    }
    #[doc = "Bits 4:5 - Analog Comparator Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 8:9 - Negative Input Select"]
    #[inline(always)]
    pub fn inseln(&mut self) -> INSELN_W {
        INSELN_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral Event Enable Negative"]
    #[inline(always)]
    pub fn evenn(&mut self) -> EVENN_W {
        EVENN_W { w: self }
    }
    #[doc = "Bit 17 - Peripheral Event Enable Positive"]
    #[inline(always)]
    pub fn evenp(&mut self) -> EVENP_W {
        EVENP_W { w: self }
    }
    #[doc = "Bits 24:25 - Hysteresis Voltage Value"]
    #[inline(always)]
    pub fn hys(&mut self) -> HYS_W {
        HYS_W { w: self }
    }
    #[doc = "Bit 26 - Fast Mode Enable"]
    #[inline(always)]
    pub fn fast(&mut self) -> FAST_W {
        FAST_W { w: self }
    }
    #[doc = "Bit 27 - Always On"]
    #[inline(always)]
    pub fn alwayson(&mut self) -> ALWAYSON_W {
        ALWAYSON_W { w: self }
    }
}
