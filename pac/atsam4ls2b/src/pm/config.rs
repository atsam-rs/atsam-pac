#[doc = "Reader of register CONFIG"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Reader of field `PBA`"]
pub type PBA_R = crate::R<bool, bool>;
#[doc = "Reader of field `PBB`"]
pub type PBB_R = crate::R<bool, bool>;
#[doc = "Reader of field `PBC`"]
pub type PBC_R = crate::R<bool, bool>;
#[doc = "Reader of field `PBD`"]
pub type PBD_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSBPEVC`"]
pub type HSBPEVC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - APBA Implemented"]
    #[inline(always)]
    pub fn pba(&self) -> PBA_R {
        PBA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - APBB Implemented"]
    #[inline(always)]
    pub fn pbb(&self) -> PBB_R {
        PBB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - APBC Implemented"]
    #[inline(always)]
    pub fn pbc(&self) -> PBC_R {
        PBC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - APBD Implemented"]
    #[inline(always)]
    pub fn pbd(&self) -> PBD_R {
        PBD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - HSB PEVC Clock Implemented"]
    #[inline(always)]
    pub fn hsbpevc(&self) -> HSBPEVC_R {
        HSBPEVC_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
