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
pub struct DCONNI_R(crate::FieldReader<bool, bool>);
impl DCONNI_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCONNI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCONNI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDISCI` reader - Device Disconnection Interrupt"]
pub struct DDISCI_R(crate::FieldReader<bool, bool>);
impl DDISCI_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDISCI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDISCI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTI` reader - USB Reset Sent Interrupt"]
pub struct RSTI_R(crate::FieldReader<bool, bool>);
impl RSTI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSMEDI` reader - Downstream Resume Sent Interrupt"]
pub struct RSMEDI_R(crate::FieldReader<bool, bool>);
impl RSMEDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSMEDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSMEDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRSMI` reader - Upstream Resume Received Interrupt"]
pub struct RXRSMI_R(crate::FieldReader<bool, bool>);
impl RXRSMI_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRSMI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRSMI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSOFI` reader - Host SOF Interrupt"]
pub struct HSOFI_R(crate::FieldReader<bool, bool>);
impl HSOFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSOFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSOFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HWUPI` reader - Host Wake-Up Interrupt"]
pub struct HWUPI_R(crate::FieldReader<bool, bool>);
impl HWUPI_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWUPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HWUPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0INT` reader - Pipe 0 Interrupt"]
pub struct P0INT_R(crate::FieldReader<bool, bool>);
impl P0INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1INT` reader - Pipe 1 Interrupt"]
pub struct P1INT_R(crate::FieldReader<bool, bool>);
impl P1INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2INT` reader - Pipe 2 Interrupt"]
pub struct P2INT_R(crate::FieldReader<bool, bool>);
impl P2INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3INT` reader - Pipe 3 Interrupt"]
pub struct P3INT_R(crate::FieldReader<bool, bool>);
impl P3INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4INT` reader - Pipe 4 Interrupt"]
pub struct P4INT_R(crate::FieldReader<bool, bool>);
impl P4INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5INT` reader - Pipe 5 Interrupt"]
pub struct P5INT_R(crate::FieldReader<bool, bool>);
impl P5INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        P5INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6INT` reader - Pipe 6 Interrupt"]
pub struct P6INT_R(crate::FieldReader<bool, bool>);
impl P6INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        P6INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Device Connection Interrupt"]
    #[inline(always)]
    pub fn dconni(&self) -> DCONNI_R {
        DCONNI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device Disconnection Interrupt"]
    #[inline(always)]
    pub fn ddisci(&self) -> DDISCI_R {
        DDISCI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB Reset Sent Interrupt"]
    #[inline(always)]
    pub fn rsti(&self) -> RSTI_R {
        RSTI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Downstream Resume Sent Interrupt"]
    #[inline(always)]
    pub fn rsmedi(&self) -> RSMEDI_R {
        RSMEDI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Upstream Resume Received Interrupt"]
    #[inline(always)]
    pub fn rxrsmi(&self) -> RXRSMI_R {
        RXRSMI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Host SOF Interrupt"]
    #[inline(always)]
    pub fn hsofi(&self) -> HSOFI_R {
        HSOFI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Host Wake-Up Interrupt"]
    #[inline(always)]
    pub fn hwupi(&self) -> HWUPI_R {
        HWUPI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Pipe 0 Interrupt"]
    #[inline(always)]
    pub fn p0int(&self) -> P0INT_R {
        P0INT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Pipe 1 Interrupt"]
    #[inline(always)]
    pub fn p1int(&self) -> P1INT_R {
        P1INT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Pipe 2 Interrupt"]
    #[inline(always)]
    pub fn p2int(&self) -> P2INT_R {
        P2INT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Pipe 3 Interrupt"]
    #[inline(always)]
    pub fn p3int(&self) -> P3INT_R {
        P3INT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Pipe 4 Interrupt"]
    #[inline(always)]
    pub fn p4int(&self) -> P4INT_R {
        P4INT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pipe 5 Interrupt"]
    #[inline(always)]
    pub fn p5int(&self) -> P5INT_R {
        P5INT_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Pipe 6 Interrupt"]
    #[inline(always)]
    pub fn p6int(&self) -> P6INT_R {
        P6INT_R::new(((self.bits >> 14) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
