#[doc = "Reader of register UHINT"]
pub type R = crate::R<u32, super::UHINT>;
#[doc = "Reader of field `DCONNI`"]
pub type DCONNI_R = crate::R<bool, bool>;
#[doc = "Reader of field `DDISCI`"]
pub type DDISCI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSTI`"]
pub type RSTI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSMEDI`"]
pub type RSMEDI_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRSMI`"]
pub type RXRSMI_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSOFI`"]
pub type HSOFI_R = crate::R<bool, bool>;
#[doc = "Reader of field `HWUPI`"]
pub type HWUPI_R = crate::R<bool, bool>;
#[doc = "Reader of field `P0INT`"]
pub type P0INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `P1INT`"]
pub type P1INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `P2INT`"]
pub type P2INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `P3INT`"]
pub type P3INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `P4INT`"]
pub type P4INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `P5INT`"]
pub type P5INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `P6INT`"]
pub type P6INT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Device Connection Interrupt"]
    #[inline(always)]
    pub fn dconni(&self) -> DCONNI_R {
        DCONNI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt"]
    #[inline(always)]
    pub fn ddisci(&self) -> DDISCI_R {
        DDISCI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt"]
    #[inline(always)]
    pub fn rsti(&self) -> RSTI_R {
        RSTI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt"]
    #[inline(always)]
    pub fn rsmedi(&self) -> RSMEDI_R {
        RSMEDI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt"]
    #[inline(always)]
    pub fn rxrsmi(&self) -> RXRSMI_R {
        RXRSMI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Host SOF Interrupt"]
    #[inline(always)]
    pub fn hsofi(&self) -> HSOFI_R {
        HSOFI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt"]
    #[inline(always)]
    pub fn hwupi(&self) -> HWUPI_R {
        HWUPI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt"]
    #[inline(always)]
    pub fn p0int(&self) -> P0INT_R {
        P0INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt"]
    #[inline(always)]
    pub fn p1int(&self) -> P1INT_R {
        P1INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt"]
    #[inline(always)]
    pub fn p2int(&self) -> P2INT_R {
        P2INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt"]
    #[inline(always)]
    pub fn p3int(&self) -> P3INT_R {
        P3INT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt"]
    #[inline(always)]
    pub fn p4int(&self) -> P4INT_R {
        P4INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt"]
    #[inline(always)]
    pub fn p5int(&self) -> P5INT_R {
        P5INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt"]
    #[inline(always)]
    pub fn p6int(&self) -> P6INT_R {
        P6INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
