#[doc = "Reader of register IMR"]
pub type R = crate::R<u32, super::IMR>;
#[doc = "Reader of field `EP0INT`"]
pub type EP0INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP1INT`"]
pub type EP1INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP2INT`"]
pub type EP2INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP3INT`"]
pub type EP3INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP4INT`"]
pub type EP4INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP5INT`"]
pub type EP5INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP6INT`"]
pub type EP6INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP7INT`"]
pub type EP7INT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXSUSP`"]
pub type RXSUSP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RXRSM`"]
pub type RXRSM_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTRSM`"]
pub type EXTRSM_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOFINT`"]
pub type SOFINT_R = crate::R<bool, bool>;
#[doc = "Reader of field `BIT12`"]
pub type BIT12_R = crate::R<bool, bool>;
#[doc = "Reader of field `WAKEUP`"]
pub type WAKEUP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Mask Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn ep0int(&self) -> EP0INT_R {
        EP0INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Mask Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn ep1int(&self) -> EP1INT_R {
        EP1INT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mask Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn ep2int(&self) -> EP2INT_R {
        EP2INT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mask Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn ep3int(&self) -> EP3INT_R {
        EP3INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn ep4int(&self) -> EP4INT_R {
        EP4INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn ep5int(&self) -> EP5INT_R {
        EP5INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mask Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn ep6int(&self) -> EP6INT_R {
        EP6INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mask Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn ep7int(&self) -> EP7INT_R {
        EP7INT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Mask UDP Suspend Interrupt"]
    #[inline(always)]
    pub fn rxsusp(&self) -> RXSUSP_R {
        RXSUSP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mask UDP Resume Interrupt"]
    #[inline(always)]
    pub fn rxrsm(&self) -> RXRSM_R {
        RXRSM_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn extrsm(&self) -> EXTRSM_R {
        EXTRSM_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Mask Start Of Frame Interrupt"]
    #[inline(always)]
    pub fn sofint(&self) -> SOFINT_R {
        SOFINT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - UDP_IMR Bit 12"]
    #[inline(always)]
    pub fn bit12(&self) -> BIT12_R {
        BIT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - USB Bus WAKEUP Interrupt"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
