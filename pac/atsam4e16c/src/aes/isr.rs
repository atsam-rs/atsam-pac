#[doc = "Reader of register ISR"]
pub type R = crate::R<u32, super::ISR>;
#[doc = "Reader of field `DATRDY`"]
pub type DATRDY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENDRX`"]
pub type ENDRX_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENDTX`"]
pub type ENDTX_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXBUFF`"]
pub type RXBUFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TXBUFE`"]
pub type TXBUFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `URAD`"]
pub type URAD_R = crate::R<bool, bool>;
#[doc = "Unspecified Register Access:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum URAT_A {
    #[doc = "0: Input Data Register written during the data processing when SMOD=0x2 mode."]
    IDR_WR_PROCESSING = 0,
    #[doc = "1: Output Data Register read during the data processing."]
    ODR_RD_PROCESSING = 1,
    #[doc = "2: Mode Register written during the data processing."]
    MR_WR_PROCESSING = 2,
    #[doc = "3: Output Data Register read during the sub-keys generation."]
    ODR_RD_SUBKGEN = 3,
    #[doc = "4: Mode Register written during the sub-keys generation."]
    MR_WR_SUBKGEN = 4,
    #[doc = "5: Write-only register read access."]
    WOR_RD_ACCESS = 5,
}
impl From<URAT_A> for u8 {
    #[inline(always)]
    fn from(variant: URAT_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `URAT`"]
pub type URAT_R = crate::R<u8, URAT_A>;
impl URAT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, URAT_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(URAT_A::IDR_WR_PROCESSING),
            1 => Val(URAT_A::ODR_RD_PROCESSING),
            2 => Val(URAT_A::MR_WR_PROCESSING),
            3 => Val(URAT_A::ODR_RD_SUBKGEN),
            4 => Val(URAT_A::MR_WR_SUBKGEN),
            5 => Val(URAT_A::WOR_RD_ACCESS),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `IDR_WR_PROCESSING`"]
    #[inline(always)]
    pub fn is_idr_wr_processing(&self) -> bool {
        *self == URAT_A::IDR_WR_PROCESSING
    }
    #[doc = "Checks if the value of the field is `ODR_RD_PROCESSING`"]
    #[inline(always)]
    pub fn is_odr_rd_processing(&self) -> bool {
        *self == URAT_A::ODR_RD_PROCESSING
    }
    #[doc = "Checks if the value of the field is `MR_WR_PROCESSING`"]
    #[inline(always)]
    pub fn is_mr_wr_processing(&self) -> bool {
        *self == URAT_A::MR_WR_PROCESSING
    }
    #[doc = "Checks if the value of the field is `ODR_RD_SUBKGEN`"]
    #[inline(always)]
    pub fn is_odr_rd_subkgen(&self) -> bool {
        *self == URAT_A::ODR_RD_SUBKGEN
    }
    #[doc = "Checks if the value of the field is `MR_WR_SUBKGEN`"]
    #[inline(always)]
    pub fn is_mr_wr_subkgen(&self) -> bool {
        *self == URAT_A::MR_WR_SUBKGEN
    }
    #[doc = "Checks if the value of the field is `WOR_RD_ACCESS`"]
    #[inline(always)]
    pub fn is_wor_rd_access(&self) -> bool {
        *self == URAT_A::WOR_RD_ACCESS
    }
}
impl R {
    #[doc = "Bit 0 - Data Ready"]
    #[inline(always)]
    pub fn datrdy(&self) -> DATRDY_R {
        DATRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End of RX Buffer"]
    #[inline(always)]
    pub fn endrx(&self) -> ENDRX_R {
        ENDRX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End of TX Buffer"]
    #[inline(always)]
    pub fn endtx(&self) -> ENDTX_R {
        ENDTX_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX Buffer Full"]
    #[inline(always)]
    pub fn rxbuff(&self) -> RXBUFF_R {
        RXBUFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TX Buffer Empty"]
    #[inline(always)]
    pub fn txbufe(&self) -> TXBUFE_R {
        TXBUFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Unspecified Register Access Detection Status"]
    #[inline(always)]
    pub fn urad(&self) -> URAD_R {
        URAD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Unspecified Register Access:"]
    #[inline(always)]
    pub fn urat(&self) -> URAT_R {
        URAT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
