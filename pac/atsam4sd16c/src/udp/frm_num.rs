#[doc = "Reader of register FRM_NUM"]
pub type R = crate::R<u32, super::FRM_NUM>;
#[doc = "Reader of field `FRM_NUM`"]
pub type FRM_NUM_R = crate::R<u16, u16>;
#[doc = "Reader of field `FRM_ERR`"]
pub type FRM_ERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `FRM_OK`"]
pub type FRM_OK_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:10 - Frame Number as Defined in the Packet Field Formats"]
    #[inline(always)]
    pub fn frm_num(&self) -> FRM_NUM_R {
        FRM_NUM_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16 - Frame Error"]
    #[inline(always)]
    pub fn frm_err(&self) -> FRM_ERR_R {
        FRM_ERR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Frame OK"]
    #[inline(always)]
    pub fn frm_ok(&self) -> FRM_OK_R {
        FRM_OK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
