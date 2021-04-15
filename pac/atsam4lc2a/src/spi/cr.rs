#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl core::convert::From<crate::W<CR_SPEC>> for W {
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SPI Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIEN_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the SPI to transfer and receive data."]
    _1 = 1,
}
impl From<SPIEN_AW> for bool {
    #[inline(always)]
    fn from(variant: SPIEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIEN` writer - SPI Enable"]
pub struct SPIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIEN_AW::_0)
    }
    #[doc = "Enables the SPI to transfer and receive data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIEN_AW::_1)
    }
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
#[doc = "SPI Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPIDIS_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Disables the SPI.All pins are set in input mode and no data is received or transmitted.If a transfer is in progress, the transfer is finished before the SPI is disabled.If both SPIEN and SPIDIS are equal to one when the control register is written, the SPI is disabled."]
    _1 = 1,
}
impl From<SPIDIS_AW> for bool {
    #[inline(always)]
    fn from(variant: SPIDIS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIDIS` writer - SPI Disable"]
pub struct SPIDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIDIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPIDIS_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIDIS_AW::_0)
    }
    #[doc = "Disables the SPI.All pins are set in input mode and no data is received or transmitted.If a transfer is in progress, the transfer is finished before the SPI is disabled.If both SPIEN and SPIDIS are equal to one when the control register is written, the SPI is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIDIS_AW::_1)
    }
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
#[doc = "SPI Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWRST_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Reset the SPI. A software-triggered hardware reset of the SPI interface is performed."]
    _1 = 1,
}
impl From<SWRST_AW> for bool {
    #[inline(always)]
    fn from(variant: SWRST_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` writer - SPI Software Reset"]
pub struct SWRST_W<'a> {
    w: &'a mut W,
}
impl<'a> SWRST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWRST_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRST_AW::_0)
    }
    #[doc = "Reset the SPI. A software-triggered hardware reset of the SPI interface is performed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRST_AW::_1)
    }
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
#[doc = "Field `FLUSHFIFO` writer - Flush FIFO command"]
pub struct FLUSHFIFO_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSHFIFO_W<'a> {
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
#[doc = "Last Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LASTXFER_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: The current NPCS will be deasserted after the character written in TD has been transferred. When CSAAT is set, thisallows to close the communication with the current serial peripheral by raising the corresponding NPCS line as soon as TDtransfer has completed."]
    _1 = 1,
}
impl From<LASTXFER_AW> for bool {
    #[inline(always)]
    fn from(variant: LASTXFER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub struct LASTXFER_W<'a> {
    w: &'a mut W,
}
impl<'a> LASTXFER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LASTXFER_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LASTXFER_AW::_0)
    }
    #[doc = "The current NPCS will be deasserted after the character written in TD has been transferred. When CSAAT is set, thisallows to close the communication with the current serial peripheral by raising the corresponding NPCS line as soon as TDtransfer has completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LASTXFER_AW::_1)
    }
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
impl W {
    #[doc = "Bit 0 - SPI Enable"]
    #[inline(always)]
    pub fn spien(&mut self) -> SPIEN_W {
        SPIEN_W { w: self }
    }
    #[doc = "Bit 1 - SPI Disable"]
    #[inline(always)]
    pub fn spidis(&mut self) -> SPIDIS_W {
        SPIDIS_W { w: self }
    }
    #[doc = "Bit 7 - SPI Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W {
        SWRST_W { w: self }
    }
    #[doc = "Bit 8 - Flush FIFO command"]
    #[inline(always)]
    pub fn flushfifo(&mut self) -> FLUSHFIFO_W {
        FLUSHFIFO_W { w: self }
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    pub fn lastxfer(&mut self) -> LASTXFER_W {
        LASTXFER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
