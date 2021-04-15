#[doc = "Reader of register UHINTE"]
pub type R = crate::R<u32, super::UHINTE>;
#[doc = "Reader of field `DCONNIE`"]
pub type DCONNIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DDISCIE`"]
pub type DDISCIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSTIE`"]
pub type RSTIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSMEDIE`"]
pub type RSMEDIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRSMIE`"]
pub type RXRSMIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSOFIE`"]
pub type HSOFIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `HWUPIE`"]
pub type HWUPIE_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0INTE`"]
pub type P0INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `P1INTE`"]
pub type P1INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2INTE`"]
pub type P2INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `P3INTE`"]
pub type P3INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `P4INTE`"]
pub type P4INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `P5INTE`"]
pub type P5INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `P6INTE`"]
pub type P6INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `P7INTE`"]
pub type P7INTE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - DCONNI Enable"]
    #[inline(always)]
    pub fn dconnie(&self) -> DCONNIE_R {
        DCONNIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DDISCI Enable"]
    #[inline(always)]
    pub fn ddiscie(&self) -> DDISCIE_R {
        DDISCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RSTI Enable"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RSMEDI Enable"]
    #[inline(always)]
    pub fn rsmedie(&self) -> RSMEDIE_R {
        RSMEDIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RXRSMI Enable"]
    #[inline(always)]
    pub fn rxrsmie(&self) -> RXRSMIE_R {
        RXRSMIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HSOFI Enable"]
    #[inline(always)]
    pub fn hsofie(&self) -> HSOFIE_R {
        HSOFIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HWUPI Enable"]
    #[inline(always)]
    pub fn hwupie(&self) -> HWUPIE_R {
        HWUPIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - P0INT Enable"]
    #[inline(always)]
    pub fn p0inte(&self) -> P0INTE_R {
        P0INTE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - P1INT Enable"]
    #[inline(always)]
    pub fn p1inte(&self) -> P1INTE_R {
        P1INTE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - P2INT Enable"]
    #[inline(always)]
    pub fn p2inte(&self) -> P2INTE_R {
        P2INTE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - P3INT Enable"]
    #[inline(always)]
    pub fn p3inte(&self) -> P3INTE_R {
        P3INTE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - P4INT Enable"]
    #[inline(always)]
    pub fn p4inte(&self) -> P4INTE_R {
        P4INTE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - P5INT Enable"]
    #[inline(always)]
    pub fn p5inte(&self) -> P5INTE_R {
        P5INTE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - P6INT Enable"]
    #[inline(always)]
    pub fn p6inte(&self) -> P6INTE_R {
        P6INTE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - P7INT Enable"]
    #[inline(always)]
    pub fn p7inte(&self) -> P7INTE_R {
        P7INTE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
