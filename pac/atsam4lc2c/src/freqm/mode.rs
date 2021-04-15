#[doc = "Reader of register MODE"]
pub type R = crate::R<u32, super::MODE>;
#[doc = "Writer for register MODE"]
pub type W = crate::W<u32, super::MODE>;
#[doc = "Register MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `REFNUM`"]
pub type REFNUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REFNUM`"]
pub struct REFNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> REFNUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `CLKSEL`"]
pub type CLKSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLKSEL`"]
pub struct CLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Reader of field `REFCEN`"]
pub type REFCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFCEN`"]
pub struct REFCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCEN_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:15 - Number of Reference CLock Cycles"]
    #[inline(always)]
    pub fn refnum(&self) -> REFNUM_R {
        REFNUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:20 - Clock Source Selection"]
    #[inline(always)]
    pub fn clksel(&self) -> CLKSEL_R {
        CLKSEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Reference Clock Enable"]
    #[inline(always)]
    pub fn refcen(&self) -> REFCEN_R {
        REFCEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reference Clock Selection"]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
    #[doc = "Bits 8:15 - Number of Reference CLock Cycles"]
    #[inline(always)]
    pub fn refnum(&mut self) -> REFNUM_W {
        REFNUM_W { w: self }
    }
    #[doc = "Bits 16:20 - Clock Source Selection"]
    #[inline(always)]
    pub fn clksel(&mut self) -> CLKSEL_W {
        CLKSEL_W { w: self }
    }
    #[doc = "Bit 31 - Reference Clock Enable"]
    #[inline(always)]
    pub fn refcen(&mut self) -> REFCEN_W {
        REFCEN_W { w: self }
    }
}
