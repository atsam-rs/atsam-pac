#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<CONFIG_SPEC>> for R {
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PBA` reader - APBA Implemented"]
pub struct PBA_R(crate::FieldReader<bool, bool>);
impl PBA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBB` reader - APBB Implemented"]
pub struct PBB_R(crate::FieldReader<bool, bool>);
impl PBB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBC` reader - APBC Implemented"]
pub struct PBC_R(crate::FieldReader<bool, bool>);
impl PBC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PBD` reader - APBD Implemented"]
pub struct PBD_R(crate::FieldReader<bool, bool>);
impl PBD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PBD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PBD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSBPEVC` reader - HSB PEVC Clock Implemented"]
pub struct HSBPEVC_R(crate::FieldReader<bool, bool>);
impl HSBPEVC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSBPEVC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSBPEVC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
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
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](index.html) module"]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config::R](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CONFIG to value 0x0f"]
impl crate::Resettable for CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
