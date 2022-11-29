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
pub type WREGLEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WBIAS` reader - Bias in wait mode"]
pub type WBIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WLATDEL` reader - Flash Latdel in wait mode"]
pub type WLATDEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RREGLEVEL` reader - Retention mode Regulator Level"]
pub type RREGLEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RBIAS` reader - Bias in Retention mode"]
pub type RBIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RLATDEL` reader - Flash Latdel in Retention mode"]
pub type RLATDEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BREGLEVEL` reader - Backup mode Regulator Level"]
pub type BREGLEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `POR18DIS` reader - POR 18 Disable"]
pub type POR18DIS_R = crate::BitReader<bool>;
#[doc = "Field `FWSAS` reader - Flash Wait State Automatic Switching"]
pub type FWSAS_R = crate::BitReader<bool>;
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
        POR18DIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Flash Wait State Automatic Switching"]
    #[inline(always)]
    pub fn fwsas(&self) -> FWSAS_R {
        FWSAS_R::new(((self.bits >> 31) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
