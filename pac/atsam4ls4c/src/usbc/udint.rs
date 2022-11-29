#[doc = "Register `UDINT` reader"]
pub struct R(crate::R<UDINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SUSP` reader - Suspend Interrupt"]
pub type SUSP_R = crate::BitReader<bool>;
#[doc = "Field `MSOF` reader - Micro Start of Frame Interrupt"]
pub type MSOF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` reader - Start of Frame Interrupt"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `EORST` reader - End of Reset Interrupt"]
pub type EORST_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP` reader - Wake-Up Interrupt"]
pub type WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `EORSM` reader - End Of Resume Interrupt"]
pub type EORSM_R = crate::BitReader<bool>;
#[doc = "Field `UPRSM` reader - Upstream Resume Interrupt"]
pub type UPRSM_R = crate::BitReader<bool>;
#[doc = "Field `EP0INT` reader - Endpoint 0 Interrupt"]
pub type EP0INT_R = crate::BitReader<bool>;
#[doc = "Field `EP1INT` reader - Endpoint 1 Interrupt"]
pub type EP1INT_R = crate::BitReader<bool>;
#[doc = "Field `EP2INT` reader - Endpoint 2 Interrupt"]
pub type EP2INT_R = crate::BitReader<bool>;
#[doc = "Field `EP3INT` reader - Endpoint 3 Interrupt"]
pub type EP3INT_R = crate::BitReader<bool>;
#[doc = "Field `EP4INT` reader - Endpoint 4 Interrupt"]
pub type EP4INT_R = crate::BitReader<bool>;
#[doc = "Field `EP5INT` reader - Endpoint 5 Interrupt"]
pub type EP5INT_R = crate::BitReader<bool>;
#[doc = "Field `EP6INT` reader - Endpoint 6 Interrupt"]
pub type EP6INT_R = crate::BitReader<bool>;
#[doc = "Field `EP7INT` reader - Endpoint 7 Interrupt"]
pub type EP7INT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Suspend Interrupt"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Micro Start of Frame Interrupt"]
    #[inline(always)]
    pub fn msof(&self) -> MSOF_R {
        MSOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Start of Frame Interrupt"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - End of Reset Interrupt"]
    #[inline(always)]
    pub fn eorst(&self) -> EORST_R {
        EORST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End Of Resume Interrupt"]
    #[inline(always)]
    pub fn eorsm(&self) -> EORSM_R {
        EORSM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Upstream Resume Interrupt"]
    #[inline(always)]
    pub fn uprsm(&self) -> UPRSM_R {
        UPRSM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Endpoint 0 Interrupt"]
    #[inline(always)]
    pub fn ep0int(&self) -> EP0INT_R {
        EP0INT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Endpoint 1 Interrupt"]
    #[inline(always)]
    pub fn ep1int(&self) -> EP1INT_R {
        EP1INT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Endpoint 2 Interrupt"]
    #[inline(always)]
    pub fn ep2int(&self) -> EP2INT_R {
        EP2INT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Endpoint 3 Interrupt"]
    #[inline(always)]
    pub fn ep3int(&self) -> EP3INT_R {
        EP3INT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Endpoint 4 Interrupt"]
    #[inline(always)]
    pub fn ep4int(&self) -> EP4INT_R {
        EP4INT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Endpoint 5 Interrupt"]
    #[inline(always)]
    pub fn ep5int(&self) -> EP5INT_R {
        EP5INT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Endpoint 6 Interrupt"]
    #[inline(always)]
    pub fn ep6int(&self) -> EP6INT_R {
        EP6INT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Endpoint 7 Interrupt"]
    #[inline(always)]
    pub fn ep7int(&self) -> EP7INT_R {
        EP7INT_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Device Global Interupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udint](index.html) module"]
pub struct UDINT_SPEC;
impl crate::RegisterSpec for UDINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [udint::R](R) reader structure"]
impl crate::Readable for UDINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UDINT to value 0"]
impl crate::Resettable for UDINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
