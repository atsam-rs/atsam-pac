#[doc = "Reader of register UDINTE"]
pub type R = crate::R<u32, super::UDINTE>;
#[doc = "Reader of field `SUSPE`"]
pub type SUSPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `MSOFE`"]
pub type MSOFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOFE`"]
pub type SOFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EORSTE`"]
pub type EORSTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `WAKEUPE`"]
pub type WAKEUPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EORSME`"]
pub type EORSME_R = crate::R<bool, bool>;
#[doc = "Reader of field `UPRSME`"]
pub type UPRSME_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP0INTE`"]
pub type EP0INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP1INTE`"]
pub type EP1INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP2INTE`"]
pub type EP2INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP3INTE`"]
pub type EP3INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP4INTE`"]
pub type EP4INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP5INTE`"]
pub type EP5INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP6INTE`"]
pub type EP6INTE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EP7INTE`"]
pub type EP7INTE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - SUSP Interrupt Enable"]
    #[inline(always)]
    pub fn suspe(&self) -> SUSPE_R {
        SUSPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MSOF Interrupt Enable"]
    #[inline(always)]
    pub fn msofe(&self) -> MSOFE_R {
        MSOFE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SOF Interrupt Enable"]
    #[inline(always)]
    pub fn sofe(&self) -> SOFE_R {
        SOFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EORST Interrupt Enable"]
    #[inline(always)]
    pub fn eorste(&self) -> EORSTE_R {
        EORSTE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WAKEUP Interrupt Enable"]
    #[inline(always)]
    pub fn wakeupe(&self) -> WAKEUPE_R {
        WAKEUPE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - EORSM Interrupt Enable"]
    #[inline(always)]
    pub fn eorsme(&self) -> EORSME_R {
        EORSME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UPRSM Interrupt Enable"]
    #[inline(always)]
    pub fn uprsme(&self) -> UPRSME_R {
        UPRSME_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 12 - EP0INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep0inte(&self) -> EP0INTE_R {
        EP0INTE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - EP1INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep1inte(&self) -> EP1INTE_R {
        EP1INTE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - EP2INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep2inte(&self) -> EP2INTE_R {
        EP2INTE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - EP3INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep3inte(&self) -> EP3INTE_R {
        EP3INTE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - EP4INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep4inte(&self) -> EP4INTE_R {
        EP4INTE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - EP5INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep5inte(&self) -> EP5INTE_R {
        EP5INTE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - EP6INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep6inte(&self) -> EP6INTE_R {
        EP6INTE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - EP7INT Interrupt Enable"]
    #[inline(always)]
    pub fn ep7inte(&self) -> EP7INTE_R {
        EP7INTE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
