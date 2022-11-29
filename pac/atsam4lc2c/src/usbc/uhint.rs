#[doc = "Register `UHINT` reader"]
pub struct R(crate::R<UHINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UHINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UHINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UHINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCONNI` reader - Device Connection Interrupt"]
pub type DCONNI_R = crate::BitReader<bool>;
#[doc = "Field `DDISCI` reader - Device Disconnection Interrupt"]
pub type DDISCI_R = crate::BitReader<bool>;
#[doc = "Field `RSTI` reader - USB Reset Sent Interrupt"]
pub type RSTI_R = crate::BitReader<bool>;
#[doc = "Field `RSMEDI` reader - Downstream Resume Sent Interrupt"]
pub type RSMEDI_R = crate::BitReader<bool>;
#[doc = "Field `RXRSMI` reader - Upstream Resume Received Interrupt"]
pub type RXRSMI_R = crate::BitReader<bool>;
#[doc = "Field `HSOFI` reader - Host SOF Interrupt"]
pub type HSOFI_R = crate::BitReader<bool>;
#[doc = "Field `HWUPI` reader - Host Wake-Up Interrupt"]
pub type HWUPI_R = crate::BitReader<bool>;
#[doc = "Field `P0INT` reader - Pipe 0 Interrupt"]
pub type P0INT_R = crate::BitReader<bool>;
#[doc = "Field `P1INT` reader - Pipe 1 Interrupt"]
pub type P1INT_R = crate::BitReader<bool>;
#[doc = "Field `P2INT` reader - Pipe 2 Interrupt"]
pub type P2INT_R = crate::BitReader<bool>;
#[doc = "Field `P3INT` reader - Pipe 3 Interrupt"]
pub type P3INT_R = crate::BitReader<bool>;
#[doc = "Field `P4INT` reader - Pipe 4 Interrupt"]
pub type P4INT_R = crate::BitReader<bool>;
#[doc = "Field `P5INT` reader - Pipe 5 Interrupt"]
pub type P5INT_R = crate::BitReader<bool>;
#[doc = "Field `P6INT` reader - Pipe 6 Interrupt"]
pub type P6INT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Device Connection Interrupt"]
    #[inline(always)]
    pub fn dconni(&self) -> DCONNI_R {
        DCONNI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt"]
    #[inline(always)]
    pub fn ddisci(&self) -> DDISCI_R {
        DDISCI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt"]
    #[inline(always)]
    pub fn rsti(&self) -> RSTI_R {
        RSTI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt"]
    #[inline(always)]
    pub fn rsmedi(&self) -> RSMEDI_R {
        RSMEDI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt"]
    #[inline(always)]
    pub fn rxrsmi(&self) -> RXRSMI_R {
        RXRSMI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Host SOF Interrupt"]
    #[inline(always)]
    pub fn hsofi(&self) -> HSOFI_R {
        HSOFI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt"]
    #[inline(always)]
    pub fn hwupi(&self) -> HWUPI_R {
        HWUPI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt"]
    #[inline(always)]
    pub fn p0int(&self) -> P0INT_R {
        P0INT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt"]
    #[inline(always)]
    pub fn p1int(&self) -> P1INT_R {
        P1INT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt"]
    #[inline(always)]
    pub fn p2int(&self) -> P2INT_R {
        P2INT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt"]
    #[inline(always)]
    pub fn p3int(&self) -> P3INT_R {
        P3INT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt"]
    #[inline(always)]
    pub fn p4int(&self) -> P4INT_R {
        P4INT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt"]
    #[inline(always)]
    pub fn p5int(&self) -> P5INT_R {
        P5INT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt"]
    #[inline(always)]
    pub fn p6int(&self) -> P6INT_R {
        P6INT_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "Host Global Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uhint](index.html) module"]
pub struct UHINT_SPEC;
impl crate::RegisterSpec for UHINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uhint::R](R) reader structure"]
impl crate::Readable for UHINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UHINT to value 0"]
impl crate::Resettable for UHINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
