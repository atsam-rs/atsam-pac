#[doc = "Register `UPSTA5` reader"]
pub struct R(crate::R<UPSTA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPSTA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPSTA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPSTA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXINI` reader - Received IN Data Interrupt"]
pub type RXINI_R = crate::BitReader<bool>;
#[doc = "Field `TXOUTI` reader - Transmitted OUT Data Interrupt"]
pub type TXOUTI_R = crate::BitReader<bool>;
#[doc = "Field `TXSTPI` reader - Transmitted SETUP Interrupt"]
pub type TXSTPI_R = crate::BitReader<bool>;
#[doc = "Field `PERRI` reader - Pipe Error Interrupt"]
pub type PERRI_R = crate::BitReader<bool>;
#[doc = "Field `NAKEDI` reader - NAKed Interrupt"]
pub type NAKEDI_R = crate::BitReader<bool>;
#[doc = "Field `ERRORFI` reader - Errorflow Interrupt"]
pub type ERRORFI_R = crate::BitReader<bool>;
#[doc = "Field `RXSTALLDI` reader - Received STALLed Interrupt"]
pub type RXSTALLDI_R = crate::BitReader<bool>;
#[doc = "Field `DTSEQ` reader - Data Toggle Sequence"]
pub type DTSEQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAMACERI` reader - Ram Access Error Interrupt"]
pub type RAMACERI_R = crate::BitReader<bool>;
#[doc = "Field `NBUSYBK` reader - Number of Busy Bank"]
pub type NBUSYBK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CURRBK` reader - Current Bank"]
pub type CURRBK_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Received IN Data Interrupt"]
    #[inline(always)]
    pub fn rxini(&self) -> RXINI_R {
        RXINI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitted OUT Data Interrupt"]
    #[inline(always)]
    pub fn txouti(&self) -> TXOUTI_R {
        TXOUTI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitted SETUP Interrupt"]
    #[inline(always)]
    pub fn txstpi(&self) -> TXSTPI_R {
        TXSTPI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Pipe Error Interrupt"]
    #[inline(always)]
    pub fn perri(&self) -> PERRI_R {
        PERRI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKed Interrupt"]
    #[inline(always)]
    pub fn nakedi(&self) -> NAKEDI_R {
        NAKEDI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Errorflow Interrupt"]
    #[inline(always)]
    pub fn errorfi(&self) -> ERRORFI_R {
        ERRORFI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Received STALLed Interrupt"]
    #[inline(always)]
    pub fn rxstalldi(&self) -> RXSTALLDI_R {
        RXSTALLDI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DTSEQ_R {
        DTSEQ_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Ram Access Error Interrupt"]
    #[inline(always)]
    pub fn ramaceri(&self) -> RAMACERI_R {
        RAMACERI_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Number of Busy Bank"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NBUSYBK_R {
        NBUSYBK_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CURRBK_R {
        CURRBK_R::new(((self.bits >> 14) & 3) as u8)
    }
}
#[doc = "Pipe Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upsta5](index.html) module"]
pub struct UPSTA5_SPEC;
impl crate::RegisterSpec for UPSTA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [upsta5::R](R) reader structure"]
impl crate::Readable for UPSTA5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UPSTA5 to value 0"]
impl crate::Resettable for UPSTA5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
