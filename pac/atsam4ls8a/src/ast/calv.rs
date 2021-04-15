#[doc = "Reader of register CALV"]
pub type R = crate::R<u32, super::CALV>;
#[doc = "Writer for register CALV"]
pub type W = crate::W<u32, super::CALV>;
#[doc = "Register CALV `reset()`'s with value 0"]
impl crate::ResetValue for super::CALV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SEC`"]
pub type SEC_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SEC`"]
pub struct SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `MIN`"]
pub type MIN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MIN`"]
pub struct MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `HOUR`"]
pub type HOUR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HOUR`"]
pub struct HOUR_W<'a> {
    w: &'a mut W,
}
impl<'a> HOUR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | (((value as u32) & 0x1f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DAY`"]
pub type DAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAY`"]
pub struct DAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | (((value as u32) & 0x1f) << 17);
        self.w
    }
}
#[doc = "Reader of field `MONTH`"]
pub type MONTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MONTH`"]
pub struct MONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> MONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 22)) | (((value as u32) & 0x0f) << 22);
        self.w
    }
}
#[doc = "Reader of field `YEAR`"]
pub type YEAR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `YEAR`"]
pub struct YEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> YEAR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | (((value as u32) & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Second"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline(always)]
    pub fn min(&self) -> MIN_R {
        MIN_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:16 - Hour"]
    #[inline(always)]
    pub fn hour(&self) -> HOUR_R {
        HOUR_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline(always)]
    pub fn day(&self) -> DAY_R {
        DAY_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline(always)]
    pub fn month(&self) -> MONTH_R {
        MONTH_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline(always)]
    pub fn year(&self) -> YEAR_R {
        YEAR_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Second"]
    #[inline(always)]
    pub fn sec(&mut self) -> SEC_W {
        SEC_W { w: self }
    }
    #[doc = "Bits 6:11 - Minute"]
    #[inline(always)]
    pub fn min(&mut self) -> MIN_W {
        MIN_W { w: self }
    }
    #[doc = "Bits 12:16 - Hour"]
    #[inline(always)]
    pub fn hour(&mut self) -> HOUR_W {
        HOUR_W { w: self }
    }
    #[doc = "Bits 17:21 - Day"]
    #[inline(always)]
    pub fn day(&mut self) -> DAY_W {
        DAY_W { w: self }
    }
    #[doc = "Bits 22:25 - Month"]
    #[inline(always)]
    pub fn month(&mut self) -> MONTH_W {
        MONTH_W { w: self }
    }
    #[doc = "Bits 26:31 - Year"]
    #[inline(always)]
    pub fn year(&mut self) -> YEAR_W {
        YEAR_W { w: self }
    }
}
