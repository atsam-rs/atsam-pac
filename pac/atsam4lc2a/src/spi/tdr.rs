#[doc = "Register `TDR` writer"]
pub struct W(crate::W<TDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDR_SPEC>;
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
impl From<crate::W<TDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TD` writer - Transmit Data"]
pub type TD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDR_SPEC, u16, u16, 16, O>;
#[doc = "Field `PCS` writer - Peripheral Chip Select"]
pub type PCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDR_SPEC, u8, u8, 4, O>;
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
pub type LASTXFER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TDR_SPEC, LASTXFERSELECT_AW, O>;
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
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    #[must_use]
    pub fn td(&mut self) -> TD_W<0> {
        TD_W::new(self)
    }
    #[doc = "Bits 16:19 - Peripheral Chip Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcs(&mut self) -> PCS_W<16> {
        PCS_W::new(self)
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
#[doc = "Transmit Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdr](index.html) module"]
pub struct TDR_SPEC;
impl crate::RegisterSpec for TDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tdr::W](W) writer structure"]
impl crate::Writable for TDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
