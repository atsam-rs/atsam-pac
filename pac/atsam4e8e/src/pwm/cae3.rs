#[doc = "Reader of register CAE3"]
pub type R = crate::R<u32, super::CAE3>;
#[doc = "Writer for register CAE3"]
pub type W = crate::W<u32, super::CAE3>;
#[doc = "Register CAE3 `reset()`'s with value 0"]
impl crate::ResetValue for super::CAE3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADEDGV`"]
pub type ADEDGV_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `ADEDGV`"]
pub struct ADEDGV_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEDGV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
#[doc = "Channel Additional Edge Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADEDGM_A {
    #[doc = "0: The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGV and the counter of the channel x is incrementing"]
    INC = 0,
    #[doc = "1: The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGV and the counter of the channel x is incrementing"]
    DEC = 1,
    #[doc = "2: The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGV, whether the counter is incrementing or not"]
    BOTH = 2,
}
impl From<ADEDGM_A> for u8 {
    #[inline(always)]
    fn from(variant: ADEDGM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `ADEDGM`"]
pub type ADEDGM_R = crate::R<u8, ADEDGM_A>;
impl ADEDGM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADEDGM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADEDGM_A::INC),
            1 => Val(ADEDGM_A::DEC),
            2 => Val(ADEDGM_A::BOTH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `INC`"]
    #[inline(always)]
    pub fn is_inc(&self) -> bool {
        *self == ADEDGM_A::INC
    }
    #[doc = "Checks if the value of the field is `DEC`"]
    #[inline(always)]
    pub fn is_dec(&self) -> bool {
        *self == ADEDGM_A::DEC
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == ADEDGM_A::BOTH
    }
}
#[doc = "Write proxy for field `ADEDGM`"]
pub struct ADEDGM_W<'a> {
    w: &'a mut W,
}
impl<'a> ADEDGM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADEDGM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGV and the counter of the channel x is incrementing"]
    #[inline(always)]
    pub fn inc(self) -> &'a mut W {
        self.variant(ADEDGM_A::INC)
    }
    #[doc = "The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGV and the counter of the channel x is incrementing"]
    #[inline(always)]
    pub fn dec(self) -> &'a mut W {
        self.variant(ADEDGM_A::DEC)
    }
    #[doc = "The additional edge of the channel x output waveform occurs when CCNTx reaches ADEDGV, whether the counter is incrementing or not"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(ADEDGM_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Channel Additional Edge Value"]
    #[inline(always)]
    pub fn adedgv(&self) -> ADEDGV_R {
        ADEDGV_R::new((self.bits & 0x00ff_ffff) as u32)
    }
    #[doc = "Bits 24:25 - Channel Additional Edge Mode"]
    #[inline(always)]
    pub fn adedgm(&self) -> ADEDGM_R {
        ADEDGM_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Channel Additional Edge Value"]
    #[inline(always)]
    pub fn adedgv(&mut self) -> ADEDGV_W {
        ADEDGV_W { w: self }
    }
    #[doc = "Bits 24:25 - Channel Additional Edge Mode"]
    #[inline(always)]
    pub fn adedgm(&mut self) -> ADEDGM_W {
        ADEDGM_W { w: self }
    }
}
