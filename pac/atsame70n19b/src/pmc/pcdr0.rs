#[doc = "Register `PCDR0` writer"]
pub struct W(crate::W<PCDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCDR0_SPEC>;
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
impl From<crate::W<PCDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID7` writer - Peripheral Clock 7 Disable"]
pub struct PID7_W<'a> {
    w: &'a mut W,
}
impl<'a> PID7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `PID8` writer - Peripheral Clock 8 Disable"]
pub struct PID8_W<'a> {
    w: &'a mut W,
}
impl<'a> PID8_W<'a> {
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
#[doc = "Field `PID10` writer - Peripheral Clock 10 Disable"]
pub struct PID10_W<'a> {
    w: &'a mut W,
}
impl<'a> PID10_W<'a> {
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
#[doc = "Field `PID11` writer - Peripheral Clock 11 Disable"]
pub struct PID11_W<'a> {
    w: &'a mut W,
}
impl<'a> PID11_W<'a> {
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
#[doc = "Field `PID13` writer - Peripheral Clock 13 Disable"]
pub struct PID13_W<'a> {
    w: &'a mut W,
}
impl<'a> PID13_W<'a> {
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
#[doc = "Field `PID14` writer - Peripheral Clock 14 Disable"]
pub struct PID14_W<'a> {
    w: &'a mut W,
}
impl<'a> PID14_W<'a> {
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
#[doc = "Field `PID15` writer - Peripheral Clock 15 Disable"]
pub struct PID15_W<'a> {
    w: &'a mut W,
}
impl<'a> PID15_W<'a> {
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
#[doc = "Field `PID16` writer - Peripheral Clock 16 Disable"]
pub struct PID16_W<'a> {
    w: &'a mut W,
}
impl<'a> PID16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PID18` writer - Peripheral Clock 18 Disable"]
pub struct PID18_W<'a> {
    w: &'a mut W,
}
impl<'a> PID18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PID19` writer - Peripheral Clock 19 Disable"]
pub struct PID19_W<'a> {
    w: &'a mut W,
}
impl<'a> PID19_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `PID20` writer - Peripheral Clock 20 Disable"]
pub struct PID20_W<'a> {
    w: &'a mut W,
}
impl<'a> PID20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PID21` writer - Peripheral Clock 21 Disable"]
pub struct PID21_W<'a> {
    w: &'a mut W,
}
impl<'a> PID21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `PID22` writer - Peripheral Clock 22 Disable"]
pub struct PID22_W<'a> {
    w: &'a mut W,
}
impl<'a> PID22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PID23` writer - Peripheral Clock 23 Disable"]
pub struct PID23_W<'a> {
    w: &'a mut W,
}
impl<'a> PID23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PID24` writer - Peripheral Clock 24 Disable"]
pub struct PID24_W<'a> {
    w: &'a mut W,
}
impl<'a> PID24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PID25` writer - Peripheral Clock 25 Disable"]
pub struct PID25_W<'a> {
    w: &'a mut W,
}
impl<'a> PID25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PID26` writer - Peripheral Clock 26 Disable"]
pub struct PID26_W<'a> {
    w: &'a mut W,
}
impl<'a> PID26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PID27` writer - Peripheral Clock 27 Disable"]
pub struct PID27_W<'a> {
    w: &'a mut W,
}
impl<'a> PID27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PID28` writer - Peripheral Clock 28 Disable"]
pub struct PID28_W<'a> {
    w: &'a mut W,
}
impl<'a> PID28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `PID29` writer - Peripheral Clock 29 Disable"]
pub struct PID29_W<'a> {
    w: &'a mut W,
}
impl<'a> PID29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `PID30` writer - Peripheral Clock 30 Disable"]
pub struct PID30_W<'a> {
    w: &'a mut W,
}
impl<'a> PID30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `PID31` writer - Peripheral Clock 31 Disable"]
pub struct PID31_W<'a> {
    w: &'a mut W,
}
impl<'a> PID31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bit 7 - Peripheral Clock 7 Disable"]
    #[inline(always)]
    pub fn pid7(&mut self) -> PID7_W {
        PID7_W { w: self }
    }
    #[doc = "Bit 8 - Peripheral Clock 8 Disable"]
    #[inline(always)]
    pub fn pid8(&mut self) -> PID8_W {
        PID8_W { w: self }
    }
    #[doc = "Bit 10 - Peripheral Clock 10 Disable"]
    #[inline(always)]
    pub fn pid10(&mut self) -> PID10_W {
        PID10_W { w: self }
    }
    #[doc = "Bit 11 - Peripheral Clock 11 Disable"]
    #[inline(always)]
    pub fn pid11(&mut self) -> PID11_W {
        PID11_W { w: self }
    }
    #[doc = "Bit 13 - Peripheral Clock 13 Disable"]
    #[inline(always)]
    pub fn pid13(&mut self) -> PID13_W {
        PID13_W { w: self }
    }
    #[doc = "Bit 14 - Peripheral Clock 14 Disable"]
    #[inline(always)]
    pub fn pid14(&mut self) -> PID14_W {
        PID14_W { w: self }
    }
    #[doc = "Bit 15 - Peripheral Clock 15 Disable"]
    #[inline(always)]
    pub fn pid15(&mut self) -> PID15_W {
        PID15_W { w: self }
    }
    #[doc = "Bit 16 - Peripheral Clock 16 Disable"]
    #[inline(always)]
    pub fn pid16(&mut self) -> PID16_W {
        PID16_W { w: self }
    }
    #[doc = "Bit 18 - Peripheral Clock 18 Disable"]
    #[inline(always)]
    pub fn pid18(&mut self) -> PID18_W {
        PID18_W { w: self }
    }
    #[doc = "Bit 19 - Peripheral Clock 19 Disable"]
    #[inline(always)]
    pub fn pid19(&mut self) -> PID19_W {
        PID19_W { w: self }
    }
    #[doc = "Bit 20 - Peripheral Clock 20 Disable"]
    #[inline(always)]
    pub fn pid20(&mut self) -> PID20_W {
        PID20_W { w: self }
    }
    #[doc = "Bit 21 - Peripheral Clock 21 Disable"]
    #[inline(always)]
    pub fn pid21(&mut self) -> PID21_W {
        PID21_W { w: self }
    }
    #[doc = "Bit 22 - Peripheral Clock 22 Disable"]
    #[inline(always)]
    pub fn pid22(&mut self) -> PID22_W {
        PID22_W { w: self }
    }
    #[doc = "Bit 23 - Peripheral Clock 23 Disable"]
    #[inline(always)]
    pub fn pid23(&mut self) -> PID23_W {
        PID23_W { w: self }
    }
    #[doc = "Bit 24 - Peripheral Clock 24 Disable"]
    #[inline(always)]
    pub fn pid24(&mut self) -> PID24_W {
        PID24_W { w: self }
    }
    #[doc = "Bit 25 - Peripheral Clock 25 Disable"]
    #[inline(always)]
    pub fn pid25(&mut self) -> PID25_W {
        PID25_W { w: self }
    }
    #[doc = "Bit 26 - Peripheral Clock 26 Disable"]
    #[inline(always)]
    pub fn pid26(&mut self) -> PID26_W {
        PID26_W { w: self }
    }
    #[doc = "Bit 27 - Peripheral Clock 27 Disable"]
    #[inline(always)]
    pub fn pid27(&mut self) -> PID27_W {
        PID27_W { w: self }
    }
    #[doc = "Bit 28 - Peripheral Clock 28 Disable"]
    #[inline(always)]
    pub fn pid28(&mut self) -> PID28_W {
        PID28_W { w: self }
    }
    #[doc = "Bit 29 - Peripheral Clock 29 Disable"]
    #[inline(always)]
    pub fn pid29(&mut self) -> PID29_W {
        PID29_W { w: self }
    }
    #[doc = "Bit 30 - Peripheral Clock 30 Disable"]
    #[inline(always)]
    pub fn pid30(&mut self) -> PID30_W {
        PID30_W { w: self }
    }
    #[doc = "Bit 31 - Peripheral Clock 31 Disable"]
    #[inline(always)]
    pub fn pid31(&mut self) -> PID31_W {
        PID31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Clock Disable Register 0\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdr0](index.html) module"]
pub struct PCDR0_SPEC;
impl crate::RegisterSpec for PCDR0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pcdr0::W](W) writer structure"]
impl crate::Writable for PCDR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCDR0 to value 0"]
impl crate::Resettable for PCDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
