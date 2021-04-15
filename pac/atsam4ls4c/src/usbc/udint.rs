#[doc = "Reader of register UDINT"]
pub type R = crate::R<u32, super::UDINT>;
#[doc = "Reader of field `SUSP`"]
pub type SUSP_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSOF`"]
pub type MSOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOF`"]
pub type SOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `EORST`"]
pub type EORST_R = crate::R<bool, bool>;
#[doc = "Reader of field `WAKEUP`"]
pub type WAKEUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `EORSM`"]
pub type EORSM_R = crate::R<bool, bool>;
#[doc = "Reader of field `UPRSM`"]
pub type UPRSM_R = crate::R<bool, bool>;
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
impl R {
    #[doc = "Bit 0 - Suspend Interrupt"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt"]
    #[inline(always)]
    pub fn msof(&self) -> MSOF_R {
        MSOF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End of Reset Interrupt"]
    #[inline(always)]
    pub fn eorst(&self) -> EORST_R {
        EORST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End Of Resume Interrupt"]
    #[inline(always)]
    pub fn eorsm(&self) -> EORSM_R {
        EORSM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn ep0int(&self) -> EP0INT_R {
        EP0INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn ep1int(&self) -> EP1INT_R {
        EP1INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn ep2int(&self) -> EP2INT_R {
        EP2INT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn ep3int(&self) -> EP3INT_R {
        EP3INT_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn ep4int(&self) -> EP4INT_R {
        EP4INT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn ep5int(&self) -> EP5INT_R {
        EP5INT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn ep6int(&self) -> EP6INT_R {
        EP6INT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn ep7int(&self) -> EP7INT_R {
        EP7INT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
