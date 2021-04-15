#[doc = "Reader of register DATABUFPTR"]
pub type R = crate::R<u32, super::DATABUFPTR>;
#[doc = "Writer for register DATABUFPTR"]
pub type W = crate::W<u32, super::DATABUFPTR>;
#[doc = "Register DATABUFPTR `reset()`'s with value 0"]
impl crate::ResetValue for super::DATABUFPTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IDATAW`"]
pub type IDATAW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IDATAW`"]
pub struct IDATAW_W<'a> {
    w: &'a mut W,
}
impl<'a> IDATAW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `ODATAW`"]
pub type ODATAW_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ODATAW`"]
pub struct ODATAW_W<'a> {
    w: &'a mut W,
}
impl<'a> ODATAW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Input Data Word"]
    #[inline(always)]
    pub fn idataw(&self) -> IDATAW_R {
        IDATAW_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Output Data Word"]
    #[inline(always)]
    pub fn odataw(&self) -> ODATAW_R {
        ODATAW_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Data Word"]
    #[inline(always)]
    pub fn idataw(&mut self) -> IDATAW_W {
        IDATAW_W { w: self }
    }
    #[doc = "Bits 4:5 - Output Data Word"]
    #[inline(always)]
    pub fn odataw(&mut self) -> ODATAW_W {
        ODATAW_W { w: self }
    }
}
