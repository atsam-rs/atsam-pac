#[doc = "Register `FWPSAVEPS` reader"]
pub struct R(crate::R<FWPSAVEPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FWPSAVEPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FWPSAVEPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FWPSAVEPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WREGLEVEL` reader - Wait mode Regulator Level"]
pub struct WREGLEVEL_R(crate::FieldReader<u8, u8>);
impl WREGLEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        WREGLEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WREGLEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WBIAS` reader - Bias in wait mode"]
pub struct WBIAS_R(crate::FieldReader<u8, u8>);
impl WBIAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        WBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WBIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WLATDEL` reader - Flash Latdel in wait mode"]
pub struct WLATDEL_R(crate::FieldReader<u8, u8>);
impl WLATDEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        WLATDEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WLATDEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RREGLEVEL` reader - Retention mode Regulator Level"]
pub struct RREGLEVEL_R(crate::FieldReader<u8, u8>);
impl RREGLEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RREGLEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RREGLEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBIAS` reader - Bias in Retention mode"]
pub struct RBIAS_R(crate::FieldReader<u8, u8>);
impl RBIAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RLATDEL` reader - Flash Latdel in Retention mode"]
pub struct RLATDEL_R(crate::FieldReader<u8, u8>);
impl RLATDEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RLATDEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RLATDEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BREGLEVEL` reader - Backup mode Regulator Level"]
pub struct BREGLEVEL_R(crate::FieldReader<u8, u8>);
impl BREGLEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        BREGLEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BREGLEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POR18DIS` reader - POR 18 Disable"]
pub struct POR18DIS_R(crate::FieldReader<bool, bool>);
impl POR18DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        POR18DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POR18DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FWSAS` reader - Flash Wait State Automatic Switching"]
pub struct FWSAS_R(crate::FieldReader<bool, bool>);
impl FWSAS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FWSAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FWSAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Wait mode Regulator Level"]
    #[inline(always)]
    pub fn wreglevel(&self) -> WREGLEVEL_R {
        WREGLEVEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Bias in wait mode"]
    #[inline(always)]
    pub fn wbias(&self) -> WBIAS_R {
        WBIAS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Flash Latdel in wait mode"]
    #[inline(always)]
    pub fn wlatdel(&self) -> WLATDEL_R {
        WLATDEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:16 - Retention mode Regulator Level"]
    #[inline(always)]
    pub fn rreglevel(&self) -> RREGLEVEL_R {
        RREGLEVEL_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:20 - Bias in Retention mode"]
    #[inline(always)]
    pub fn rbias(&self) -> RBIAS_R {
        RBIAS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:25 - Flash Latdel in Retention mode"]
    #[inline(always)]
    pub fn rlatdel(&self) -> RLATDEL_R {
        RLATDEL_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:29 - Backup mode Regulator Level"]
    #[inline(always)]
    pub fn breglevel(&self) -> BREGLEVEL_R {
        BREGLEVEL_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - POR 18 Disable"]
    #[inline(always)]
    pub fn por18dis(&self) -> POR18DIS_R {
        POR18DIS_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Flash Wait State Automatic Switching"]
    #[inline(always)]
    pub fn fwsas(&self) -> FWSAS_R {
        FWSAS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Factory Word Power Save PS Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwpsaveps](index.html) module"]
pub struct FWPSAVEPS_SPEC;
impl crate::RegisterSpec for FWPSAVEPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fwpsaveps::R](R) reader structure"]
impl crate::Readable for FWPSAVEPS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FWPSAVEPS to value 0"]
impl crate::Resettable for FWPSAVEPS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
