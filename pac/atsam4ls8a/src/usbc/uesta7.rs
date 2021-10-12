#[doc = "Register `UESTA7` reader"]
pub struct R(crate::R<UESTA7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UESTA7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UESTA7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UESTA7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXINI` reader - Transmitted IN Data Interrupt"]
pub struct TXINI_R(crate::FieldReader<bool, bool>);
impl TXINI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXINI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXINI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXOUTI` reader - Received OUT Data Interrupt"]
pub struct RXOUTI_R(crate::FieldReader<bool, bool>);
impl RXOUTI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXOUTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXOUTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXSTPI` reader - Received SETUP Interrupt"]
pub struct RXSTPI_R(crate::FieldReader<bool, bool>);
impl RXSTPI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXSTPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXSTPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKOUTI` reader - NAKed OUT Interrupt"]
pub struct NAKOUTI_R(crate::FieldReader<bool, bool>);
impl NAKOUTI_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKOUTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKOUTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKINI` reader - NAKed IN Interrupt"]
pub struct NAKINI_R(crate::FieldReader<bool, bool>);
impl NAKINI_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKINI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKINI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STALLEDI` reader - STALLed Interrupt"]
pub struct STALLEDI_R(crate::FieldReader<bool, bool>);
impl STALLEDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        STALLEDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STALLEDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTSEQ` reader - Data Toggle Sequence"]
pub struct DTSEQ_R(crate::FieldReader<u8, u8>);
impl DTSEQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTSEQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTSEQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RAMACERI` reader - Ram Access Error Interrupt"]
pub struct RAMACERI_R(crate::FieldReader<bool, bool>);
impl RAMACERI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAMACERI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAMACERI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NBUSYBK` reader - Number Of Busy Banks"]
pub struct NBUSYBK_R(crate::FieldReader<u8, u8>);
impl NBUSYBK_R {
    pub(crate) fn new(bits: u8) -> Self {
        NBUSYBK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NBUSYBK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CURRBK` reader - Current Bank"]
pub struct CURRBK_R(crate::FieldReader<u8, u8>);
impl CURRBK_R {
    pub(crate) fn new(bits: u8) -> Self {
        CURRBK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CURRBK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Control Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRLDIR_A {
    #[doc = "0: `0`"]
    OUT = 0,
    #[doc = "1: `1`"]
    IN = 1,
}
impl From<CTRLDIR_A> for bool {
    #[inline(always)]
    fn from(variant: CTRLDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTRLDIR` reader - Control Direction"]
pub struct CTRLDIR_R(crate::FieldReader<bool, CTRLDIR_A>);
impl CTRLDIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRLDIR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRLDIR_A {
        match self.bits {
            false => CTRLDIR_A::OUT,
            true => CTRLDIR_A::IN,
        }
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        **self == CTRLDIR_A::OUT
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        **self == CTRLDIR_A::IN
    }
}
impl core::ops::Deref for CTRLDIR_R {
    type Target = crate::FieldReader<bool, CTRLDIR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Transmitted IN Data Interrupt"]
    #[inline(always)]
    pub fn txini(&self) -> TXINI_R {
        TXINI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Received OUT Data Interrupt"]
    #[inline(always)]
    pub fn rxouti(&self) -> RXOUTI_R {
        RXOUTI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Received SETUP Interrupt"]
    #[inline(always)]
    pub fn rxstpi(&self) -> RXSTPI_R {
        RXSTPI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NAKed OUT Interrupt"]
    #[inline(always)]
    pub fn nakouti(&self) -> NAKOUTI_R {
        NAKOUTI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NAKed IN Interrupt"]
    #[inline(always)]
    pub fn nakini(&self) -> NAKINI_R {
        NAKINI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - STALLed Interrupt"]
    #[inline(always)]
    pub fn stalledi(&self) -> STALLEDI_R {
        STALLEDI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Data Toggle Sequence"]
    #[inline(always)]
    pub fn dtseq(&self) -> DTSEQ_R {
        DTSEQ_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 11 - Ram Access Error Interrupt"]
    #[inline(always)]
    pub fn ramaceri(&self) -> RAMACERI_R {
        RAMACERI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Number Of Busy Banks"]
    #[inline(always)]
    pub fn nbusybk(&self) -> NBUSYBK_R {
        NBUSYBK_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Current Bank"]
    #[inline(always)]
    pub fn currbk(&self) -> CURRBK_R {
        CURRBK_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 17 - Control Direction"]
    #[inline(always)]
    pub fn ctrldir(&self) -> CTRLDIR_R {
        CTRLDIR_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "Endpoint Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uesta7](index.html) module"]
pub struct UESTA7_SPEC;
impl crate::RegisterSpec for UESTA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uesta7::R](R) reader structure"]
impl crate::Readable for UESTA7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UESTA7 to value 0x0100"]
impl crate::Resettable for UESTA7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100
    }
}
