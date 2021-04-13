#[doc = "Reader of register GCFG"]
pub type R = crate::R<u32, super::GCFG>;
#[doc = "Reader of field `CGDISREG`"]
pub type CGDISREG_R = crate::R<bool, bool>;
#[doc = "Reader of field `CGDISPIPE`"]
pub type CGDISPIPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CGDISFIFO`"]
pub type CGDISFIFO_R = crate::R<bool, bool>;
#[doc = "Reader of field `CGDISIF`"]
pub type CGDISIF_R = crate::R<bool, bool>;
#[doc = "Reader of field `BXKBEN`"]
pub type BXKBEN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Configuration Registers Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisreg(&self) -> CGDISREG_R {
        CGDISREG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pipeline Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdispipe(&self) -> CGDISPIPE_R {
        CGDISPIPE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FIFO Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisfifo(&self) -> CGDISFIFO_R {
        CGDISFIFO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bus Interface Clock Gating Disable"]
    #[inline(always)]
    pub fn cgdisif(&self) -> CGDISIF_R {
        CGDISIF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Boundary X Kilobyte Enable"]
    #[inline(always)]
    pub fn bxkben(&self) -> BXKBEN_R {
        BXKBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
