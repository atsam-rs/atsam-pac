#[doc = "Register `QISR` reader"]
pub struct R(crate::R<QISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDX` reader - InDeX"]
pub struct IDX_R(crate::FieldReader<bool, bool>);
impl IDX_R {
    pub(crate) fn new(bits: bool) -> Self {
        IDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIRCHG` reader - DIRection CHanGe"]
pub struct DIRCHG_R(crate::FieldReader<bool, bool>);
impl DIRCHG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIRCHG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIRCHG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QERR` reader - Quadrature ERRor"]
pub struct QERR_R(crate::FieldReader<bool, bool>);
impl QERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        QERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIR` reader - DIRection"]
pub struct DIR_R(crate::FieldReader<bool, bool>);
impl DIR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - InDeX"]
    #[inline(always)]
    pub fn idx(&self) -> IDX_R {
        IDX_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DIRection CHanGe"]
    #[inline(always)]
    pub fn dirchg(&self) -> DIRCHG_R {
        DIRCHG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Quadrature ERRor"]
    #[inline(always)]
    pub fn qerr(&self) -> QERR_R {
        QERR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DIRection"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "QDEC Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [qisr](index.html) module"]
pub struct QISR_SPEC;
impl crate::RegisterSpec for QISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [qisr::R](R) reader structure"]
impl crate::Readable for QISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets QISR to value 0"]
impl crate::Resettable for QISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
