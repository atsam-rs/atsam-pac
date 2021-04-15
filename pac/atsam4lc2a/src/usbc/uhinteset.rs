#[doc = "Register `UHINTESET` writer"]
pub struct W(crate::W<UHINTESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UHINTESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl core::convert::From<crate::W<UHINTESET_SPEC>> for W {
    fn from(writer: crate::W<UHINTESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCONNIES` writer - DCONNIE Set"]
pub struct DCONNIES_W<'a> {
    w: &'a mut W,
}
impl<'a> DCONNIES_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `DDISCIES` writer - DDISCIE Set"]
pub struct DDISCIES_W<'a> {
    w: &'a mut W,
}
impl<'a> DDISCIES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `RSTIES` writer - RSTIE Set"]
pub struct RSTIES_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTIES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RSMEDIES` writer - RSMEDIE Set"]
pub struct RSMEDIES_W<'a> {
    w: &'a mut W,
}
impl<'a> RSMEDIES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `RXRSMIES` writer - RXRSMIE Set"]
pub struct RXRSMIES_W<'a> {
    w: &'a mut W,
}
impl<'a> RXRSMIES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `HSOFIES` writer - HSOFIE Set"]
pub struct HSOFIES_W<'a> {
    w: &'a mut W,
}
impl<'a> HSOFIES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `HWUPIES` writer - HWUPIE Set"]
pub struct HWUPIES_W<'a> {
    w: &'a mut W,
}
impl<'a> HWUPIES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `P0INTES` writer - P0INTE Set"]
pub struct P0INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P0INTES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `P1INTES` writer - P1INTE Set"]
pub struct P1INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P1INTES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `P2INTES` writer - P2INTE Set"]
pub struct P2INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P2INTES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `P3INTES` writer - P3INTE Set"]
pub struct P3INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P3INTES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `P4INTES` writer - P4INTE Set"]
pub struct P4INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P4INTES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `P5INTES` writer - P5INTE Set"]
pub struct P5INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P5INTES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `P6INTES` writer - P6INTE Set"]
pub struct P6INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P6INTES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `P7INTES` writer - P7INTE Set"]
pub struct P7INTES_W<'a> {
    w: &'a mut W,
}
impl<'a> P7INTES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - DCONNIE Set"]
    #[inline(always)]
    pub fn dconnies(&mut self) -> DCONNIES_W {
        DCONNIES_W { w: self }
    }
    #[doc = "Bit 1 - DDISCIE Set"]
    #[inline(always)]
    pub fn ddiscies(&mut self) -> DDISCIES_W {
        DDISCIES_W { w: self }
    }
    #[doc = "Bit 2 - RSTIE Set"]
    #[inline(always)]
    pub fn rsties(&mut self) -> RSTIES_W {
        RSTIES_W { w: self }
    }
    #[doc = "Bit 3 - RSMEDIE Set"]
    #[inline(always)]
    pub fn rsmedies(&mut self) -> RSMEDIES_W {
        RSMEDIES_W { w: self }
    }
    #[doc = "Bit 4 - RXRSMIE Set"]
    #[inline(always)]
    pub fn rxrsmies(&mut self) -> RXRSMIES_W {
        RXRSMIES_W { w: self }
    }
    #[doc = "Bit 5 - HSOFIE Set"]
    #[inline(always)]
    pub fn hsofies(&mut self) -> HSOFIES_W {
        HSOFIES_W { w: self }
    }
    #[doc = "Bit 6 - HWUPIE Set"]
    #[inline(always)]
    pub fn hwupies(&mut self) -> HWUPIES_W {
        HWUPIES_W { w: self }
    }
    #[doc = "Bit 8 - P0INTE Set"]
    #[inline(always)]
    pub fn p0intes(&mut self) -> P0INTES_W {
        P0INTES_W { w: self }
    }
    #[doc = "Bit 9 - P1INTE Set"]
    #[inline(always)]
    pub fn p1intes(&mut self) -> P1INTES_W {
        P1INTES_W { w: self }
    }
    #[doc = "Bit 10 - P2INTE Set"]
    #[inline(always)]
    pub fn p2intes(&mut self) -> P2INTES_W {
        P2INTES_W { w: self }
    }
    #[doc = "Bit 11 - P3INTE Set"]
    #[inline(always)]
    pub fn p3intes(&mut self) -> P3INTES_W {
        P3INTES_W { w: self }
    }
    #[doc = "Bit 12 - P4INTE Set"]
    #[inline(always)]
    pub fn p4intes(&mut self) -> P4INTES_W {
        P4INTES_W { w: self }
    }
    #[doc = "Bit 13 - P5INTE Set"]
    #[inline(always)]
    pub fn p5intes(&mut self) -> P5INTES_W {
        P5INTES_W { w: self }
    }
    #[doc = "Bit 14 - P6INTE Set"]
    #[inline(always)]
    pub fn p6intes(&mut self) -> P6INTES_W {
        P6INTES_W { w: self }
    }
    #[doc = "Bit 15 - P7INTE Set"]
    #[inline(always)]
    pub fn p7intes(&mut self) -> P7INTES_W {
        P7INTES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Global Interrupt Enable Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhinteset](index.html) module"]
pub struct UHINTESET_SPEC;
impl crate::RegisterSpec for UHINTESET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [uhinteset::W](W) writer structure"]
impl crate::Writable for UHINTESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UHINTESET to value 0"]
impl crate::Resettable for UHINTESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
