#[doc = "Writer for register CAEUPD2"]
pub type W = crate::W<u32, super::CAEUPD2>;
#[doc = "Write proxy for field `ADEDGVUP`"]
pub struct ADEDGVUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEDGVUP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Channel Additional Edge Mode Update"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADEDGMUP_AW {
    #[doc = "0: The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGVUP and the counter of the channel x is incrementing."]
    INC = 0,
    #[doc = "1: The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGVUP and the counter of the channel x is incrementing."]
    DEC = 1,
    #[doc = "2: The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGVUP, whether the counter is incrementing or not."]
    BOTH = 2,
}
impl From<ADEDGMUP_AW> for u8 {
    #[inline(always)]
    fn from(variant: ADEDGMUP_AW) -> Self {
        variant as _
    }
}
#[doc = "Write proxy for field `ADEDGMUP`"]
pub struct ADEDGMUP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEDGMUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADEDGMUP_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGVUP and the counter of the channel x is incrementing."]
    #[inline(always)]
    pub fn inc(self) -> &'a mut W {
        self.variant(ADEDGMUP_AW::INC)
    }
    #[doc = "The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGVUP and the counter of the channel x is incrementing."]
    #[inline(always)]
    pub fn dec(self) -> &'a mut W {
        self.variant(ADEDGMUP_AW::DEC)
    }
    #[doc = "The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGVUP, whether the counter is incrementing or not."]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(ADEDGMUP_AW::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Additional Edge Value Update"]
    #[inline(always)]
    pub fn adedgvup(&mut self) -> ADEDGVUP_W {
        ADEDGVUP_W { w: self }
    }
    #[doc = "Bits 24:25 - Channel Additional Edge Mode Update"]
    #[inline(always)]
    pub fn adedgmup(&mut self) -> ADEDGMUP_W {
        ADEDGMUP_W { w: self }
    }
}
