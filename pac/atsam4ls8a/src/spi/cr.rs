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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SPI Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIENSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Enables the SPI to transfer and receive data."]
    _1 = 1,
}
impl From<SPIENSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: SPIENSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIEN` writer - SPI Enable"]
pub type SPIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SPIENSELECT_AW, O>;
impl<'a, const O: u8> SPIEN_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIENSELECT_AW::_0)
    }
    #[doc = "Enables the SPI to transfer and receive data."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIENSELECT_AW::_1)
    }
}
#[doc = "SPI Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIDISSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Disables the SPI.All pins are set in input mode and no data is received or transmitted.If a transfer is in progress, the transfer is finished before the SPI is disabled.If both SPIEN and SPIDIS are equal to one when the control register is written, the SPI is disabled."]
    _1 = 1,
}
impl From<SPIDISSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: SPIDISSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIDIS` writer - SPI Disable"]
pub type SPIDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SPIDISSELECT_AW, O>;
impl<'a, const O: u8> SPIDIS_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIDISSELECT_AW::_0)
    }
    #[doc = "Disables the SPI.All pins are set in input mode and no data is received or transmitted.If a transfer is in progress, the transfer is finished before the SPI is disabled.If both SPIEN and SPIDIS are equal to one when the control register is written, the SPI is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIDISSELECT_AW::_1)
    }
}
#[doc = "SPI Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRSTSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Reset the SPI. A software-triggered hardware reset of the SPI interface is performed."]
    _1 = 1,
}
impl From<SWRSTSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: SWRSTSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` writer - SPI Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SWRSTSELECT_AW, O>;
impl<'a, const O: u8> SWRST_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWRSTSELECT_AW::_0)
    }
    #[doc = "Reset the SPI. A software-triggered hardware reset of the SPI interface is performed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWRSTSELECT_AW::_1)
    }
}
#[doc = "Field `FLUSHFIFO` writer - Flush FIFO command"]
pub type FLUSHFIFO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Last Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LASTXFERSELECT_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: The current NPCS will be deasserted after the character written in TD has been transferred. When CSAAT is set, thisallows to close the communication with the current serial peripheral by raising the corresponding NPCS line as soon as TDtransfer has completed."]
    _1 = 1,
}
impl From<LASTXFERSELECT_AW> for bool {
    #[inline(always)]
    fn from(variant: LASTXFERSELECT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LASTXFER` writer - Last Transfer"]
pub type LASTXFER_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, LASTXFERSELECT_AW, O>;
impl<'a, const O: u8> LASTXFER_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LASTXFERSELECT_AW::_0)
    }
    #[doc = "The current NPCS will be deasserted after the character written in TD has been transferred. When CSAAT is set, thisallows to close the communication with the current serial peripheral by raising the corresponding NPCS line as soon as TDtransfer has completed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LASTXFERSELECT_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spien(&mut self) -> SPIEN_W<0> {
        SPIEN_W::new(self)
    }
    #[doc = "Bit 1 - SPI Disable"]
    #[inline(always)]
    #[must_use]
    pub fn spidis(&mut self) -> SPIDIS_W<1> {
        SPIDIS_W::new(self)
    }
    #[doc = "Bit 7 - SPI Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<7> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 8 - Flush FIFO command"]
    #[inline(always)]
    #[must_use]
    pub fn flushfifo(&mut self) -> FLUSHFIFO_W<8> {
        FLUSHFIFO_W::new(self)
    }
    #[doc = "Bit 24 - Last Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn lastxfer(&mut self) -> LASTXFER_W<24> {
        LASTXFER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
