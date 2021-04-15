#[doc = "Reader of register FWPSAVEPS"]
pub type R = crate::R<u32, super::FWPSAVEPS>;
#[doc = "Reader of field `WREGLEVEL`"]
pub type WREGLEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `WBIAS`"]
pub type WBIAS_R = crate::R<u8, u8>;
#[doc = "Reader of field `WLATDEL`"]
pub type WLATDEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `RREGLEVEL`"]
pub type RREGLEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `RBIAS`"]
pub type RBIAS_R = crate::R<u8, u8>;
#[doc = "Reader of field `RLATDEL`"]
pub type RLATDEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `BREGLEVEL`"]
pub type BREGLEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `POR18DIS`"]
pub type POR18DIS_R = crate::R<bool, bool>;
#[doc = "Reader of field `FWSAS`"]
pub type FWSAS_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Wait mode Regulator Level"]
    #[inline(always)]
    pub fn wreglevel(&self) -> WREGLEVEL_R {
        WREGLEVEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Bias in wait mode"]
    #[inline(always)]
    pub fn wbias(&self) -> WBIAS_R {
        WBIAS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Flash Latdel in wait mode"]
    #[inline(always)]
    pub fn wlatdel(&self) -> WLATDEL_R {
        WLATDEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:16 - Retention mode Regulator Level"]
    #[inline(always)]
    pub fn rreglevel(&self) -> RREGLEVEL_R {
        RREGLEVEL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:20 - Bias in Retention mode"]
    #[inline(always)]
    pub fn rbias(&self) -> RBIAS_R {
        RBIAS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:25 - Flash Latdel in Retention mode"]
    #[inline(always)]
    pub fn rlatdel(&self) -> RLATDEL_R {
        RLATDEL_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:29 - Backup mode Regulator Level"]
    #[inline(always)]
    pub fn breglevel(&self) -> BREGLEVEL_R {
        BREGLEVEL_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - POR 18 Disable"]
    #[inline(always)]
    pub fn por18dis(&self) -> POR18DIS_R {
        POR18DIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Flash Wait State Automatic Switching"]
    #[inline(always)]
    pub fn fwsas(&self) -> FWSAS_R {
        FWSAS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
