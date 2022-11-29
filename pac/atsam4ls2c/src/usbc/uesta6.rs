#[doc = "Register `UESTA6` reader"]
pub struct R(crate::R<UESTA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UESTA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UESTA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UESTA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXINI` reader - Transmitted IN Data Interrupt"]
pub type TXINI_R = crate::BitReader<bool>;
#[doc = "Field `RXOUTI` reader - Received OUT Data Interrupt"]
pub type RXOUTI_R = crate::BitReader<bool>;
#[doc = "Field `RXSTPI` reader - Received SETUP Interrupt"]
pub type RXSTPI_R = crate::BitReader<bool>;
#[doc = "Field `NAKOUTI` reader - NAKed OUT Interrupt"]
pub type NAKOUTI_R = crate::BitReader<bool>;
#[doc = "Field `NAKINI` reader - NAKed IN Interrupt"]
pub type NAKINI_R = crate::BitReader<bool>;
#[doc = "Field `STALLEDI` reader - STALLed Interrupt"]
pub type STALLEDI_R = crate::BitReader<bool>;
#[doc = "Field `DTSEQ` reader - Data Toggle Sequence"]
pub type DTSEQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAMACERI` reader - Ram Access Error Interrupt"]
pub type RAMACERI_R = crate::BitReader<bool>;
#[doc = "Field `NBUSYBK` reader - Number Of Busy Banks"]
pub type NBUSYBK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CURRBK` reader - Current Bank"]
pub type CURRBK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTRLDIR` reader - Control Direction"]
pub type CTRLDIR_R = crate::BitReader<CTRLDIRSELECT_A>;
#[doc = "Control Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTRLDIRSELECT_A {
    #[doc = "0: `0`"]
    OUT = 0,
    #[doc = "1: `1`"]
    IN = 1,
}
impl From<CTRLDIRSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: CTRLDIRSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTRLDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRLDIRSELECT_A {
        match self.bits {
            false => CTRLDIRSELECT_A::OUT,
            true => CTRLDIRSELECT_A::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == CTRLDIRSELECT_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == CTRLDIRSELECT_A::IN
    }
}
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txini(&self) -> TXINI_R {
        TXINI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxouti(&self) -> RXOUTI_R {
        RXOUTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt"]
    #[inline(always)]
    pub fn rxstpi(&self) -> RXSTPI_R {
        RXSTPI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt"]
    #[inline(always)]
    pub fn nakouti(&self) -> NAKOUTI_R {
        NAKOUTI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt"]
    #[inline(always)]
    pub fn nakini(&self) -> NAKINI_R {
        NAKINI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - STALLed Interrupt"]
    #[inline(always)]
    pub fn stalledi(&self) -> STALLEDI_R {
        STALLEDI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DTSEQ_R {
        DTSEQ_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - Ram Access Error Interrupt"]
    #[inline(always)]
    pub fn ramaceri(&self) -> RAMACERI_R {
        RAMACERI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Number Of Busy Banks"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NBUSYBK_R {
        NBUSYBK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CURRBK_R {
        CURRBK_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 17 - Control Direction"]
    #[inline(always)]
    pub fn ctrldir(&self) -> CTRLDIR_R {
        CTRLDIR_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta6](index.html) module"]
pub struct UESTA6_SPEC;
impl crate::RegisterSpec for UESTA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uesta6::R](R) reader structure"]
impl crate::Readable for UESTA6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UESTA6 to value 0x0100"]
impl crate::Resettable for UESTA6_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
