#[doc = "Register `CONFIG` reader"]
pub struct R(crate::R<CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PBA` reader - APBA Implemented"]
pub type PBA_R = crate::BitReader<bool>;
#[doc = "Field `PBB` reader - APBB Implemented"]
pub type PBB_R = crate::BitReader<bool>;
#[doc = "Field `PBC` reader - APBC Implemented"]
pub type PBC_R = crate::BitReader<bool>;
#[doc = "Field `PBD` reader - APBD Implemented"]
pub type PBD_R = crate::BitReader<bool>;
#[doc = "Field `HSBPEVC` reader - HSB PEVC Clock Implemented"]
pub type HSBPEVC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - APBA Implemented"]
    #[inline(always)]
    pub fn pba(&self) -> PBA_R {
        PBA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - APBB Implemented"]
    #[inline(always)]
    pub fn pbb(&self) -> PBB_R {
        PBB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APBC Implemented"]
    #[inline(always)]
    pub fn pbc(&self) -> PBC_R {
        PBC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - APBD Implemented"]
    #[inline(always)]
    pub fn pbd(&self) -> PBD_R {
        PBD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - HSB PEVC Clock Implemented"]
    #[inline(always)]
    pub fn hsbpevc(&self) -> HSBPEVC_R {
        HSBPEVC_R::new(((self.bits >> 7) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0x0f;
}
