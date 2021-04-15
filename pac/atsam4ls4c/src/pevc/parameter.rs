#[doc = "Reader of register PARAMETER"]
pub type R = crate::R<u32, super::PARAMETER>;
#[doc = "Reader of field `IGF_COUNT`"]
pub type IGF_COUNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `EVS_COUNT`"]
pub type EVS_COUNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `EVIN`"]
pub type EVIN_R = crate::R<u8, u8>;
#[doc = "Reader of field `TRIGOUT`"]
pub type TRIGOUT_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of Input Glitch Filters"]
    #[inline(always)]
    pub fn igf_count(&self) -> IGF_COUNT_R {
        IGF_COUNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of Event Shapers"]
    #[inline(always)]
    pub fn evs_count(&self) -> EVS_COUNT_R {
        EVS_COUNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Number of Event Inputs / Generators"]
    #[inline(always)]
    pub fn evin(&self) -> EVIN_R {
        EVIN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Number of Trigger Outputs / Channels / Users"]
    #[inline(always)]
    pub fn trigout(&self) -> TRIGOUT_R {
        TRIGOUT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
