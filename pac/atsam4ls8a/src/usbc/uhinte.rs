#[doc = "Register `UHINTE` reader"]
pub struct R(crate::R<UHINTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHINTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHINTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHINTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCONNIE` reader - DCONNI Enable"]
pub type DCONNIE_R = crate::BitReader<bool>;
#[doc = "Field `DDISCIE` reader - DDISCI Enable"]
pub type DDISCIE_R = crate::BitReader<bool>;
#[doc = "Field `RSTIE` reader - RSTI Enable"]
pub type RSTIE_R = crate::BitReader<bool>;
#[doc = "Field `RSMEDIE` reader - RSMEDI Enable"]
pub type RSMEDIE_R = crate::BitReader<bool>;
#[doc = "Field `RXRSMIE` reader - RXRSMI Enable"]
pub type RXRSMIE_R = crate::BitReader<bool>;
#[doc = "Field `HSOFIE` reader - HSOFI Enable"]
pub type HSOFIE_R = crate::BitReader<bool>;
#[doc = "Field `HWUPIE` reader - HWUPI Enable"]
pub type HWUPIE_R = crate::BitReader<bool>;
#[doc = "Field `P0INTE` reader - P0INT Enable"]
pub type P0INTE_R = crate::BitReader<bool>;
#[doc = "Field `P1INTE` reader - P1INT Enable"]
pub type P1INTE_R = crate::BitReader<bool>;
#[doc = "Field `P2INTE` reader - P2INT Enable"]
pub type P2INTE_R = crate::BitReader<bool>;
#[doc = "Field `P3INTE` reader - P3INT Enable"]
pub type P3INTE_R = crate::BitReader<bool>;
#[doc = "Field `P4INTE` reader - P4INT Enable"]
pub type P4INTE_R = crate::BitReader<bool>;
#[doc = "Field `P5INTE` reader - P5INT Enable"]
pub type P5INTE_R = crate::BitReader<bool>;
#[doc = "Field `P6INTE` reader - P6INT Enable"]
pub type P6INTE_R = crate::BitReader<bool>;
#[doc = "Field `P7INTE` reader - P7INT Enable"]
pub type P7INTE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - DCONNI Enable"]
    #[inline(always)]
    pub fn dconnie(&self) -> DCONNIE_R {
        DCONNIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DDISCI Enable"]
    #[inline(always)]
    pub fn ddiscie(&self) -> DDISCIE_R {
        DDISCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RSTI Enable"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RSMEDI Enable"]
    #[inline(always)]
    pub fn rsmedie(&self) -> RSMEDIE_R {
        RSMEDIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXRSMI Enable"]
    #[inline(always)]
    pub fn rxrsmie(&self) -> RXRSMIE_R {
        RXRSMIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSOFI Enable"]
    #[inline(always)]
    pub fn hsofie(&self) -> HSOFIE_R {
        HSOFIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HWUPI Enable"]
    #[inline(always)]
    pub fn hwupie(&self) -> HWUPIE_R {
        HWUPIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - P0INT Enable"]
    #[inline(always)]
    pub fn p0inte(&self) -> P0INTE_R {
        P0INTE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - P1INT Enable"]
    #[inline(always)]
    pub fn p1inte(&self) -> P1INTE_R {
        P1INTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - P2INT Enable"]
    #[inline(always)]
    pub fn p2inte(&self) -> P2INTE_R {
        P2INTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - P3INT Enable"]
    #[inline(always)]
    pub fn p3inte(&self) -> P3INTE_R {
        P3INTE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - P4INT Enable"]
    #[inline(always)]
    pub fn p4inte(&self) -> P4INTE_R {
        P4INTE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - P5INT Enable"]
    #[inline(always)]
    pub fn p5inte(&self) -> P5INTE_R {
        P5INTE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - P6INT Enable"]
    #[inline(always)]
    pub fn p6inte(&self) -> P6INTE_R {
        P6INTE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - P7INT Enable"]
    #[inline(always)]
    pub fn p7inte(&self) -> P7INTE_R {
        P7INTE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Host Global Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhinte](index.html) module"]
pub struct UHINTE_SPEC;
impl crate::RegisterSpec for UHINTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhinte::R](R) reader structure"]
impl crate::Readable for UHINTE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UHINTE to value 0"]
impl crate::Resettable for UHINTE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
