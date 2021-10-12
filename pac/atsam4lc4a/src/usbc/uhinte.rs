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
pub struct DCONNIE_R(crate::FieldReader<bool, bool>);
impl DCONNIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCONNIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCONNIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DDISCIE` reader - DDISCI Enable"]
pub struct DDISCIE_R(crate::FieldReader<bool, bool>);
impl DDISCIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DDISCIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DDISCIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTIE` reader - RSTI Enable"]
pub struct RSTIE_R(crate::FieldReader<bool, bool>);
impl RSTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSMEDIE` reader - RSMEDI Enable"]
pub struct RSMEDIE_R(crate::FieldReader<bool, bool>);
impl RSMEDIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSMEDIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSMEDIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRSMIE` reader - RXRSMI Enable"]
pub struct RXRSMIE_R(crate::FieldReader<bool, bool>);
impl RXRSMIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXRSMIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRSMIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSOFIE` reader - HSOFI Enable"]
pub struct HSOFIE_R(crate::FieldReader<bool, bool>);
impl HSOFIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSOFIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSOFIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HWUPIE` reader - HWUPI Enable"]
pub struct HWUPIE_R(crate::FieldReader<bool, bool>);
impl HWUPIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HWUPIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HWUPIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0INTE` reader - P0INT Enable"]
pub struct P0INTE_R(crate::FieldReader<bool, bool>);
impl P0INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1INTE` reader - P1INT Enable"]
pub struct P1INTE_R(crate::FieldReader<bool, bool>);
impl P1INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        P1INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2INTE` reader - P2INT Enable"]
pub struct P2INTE_R(crate::FieldReader<bool, bool>);
impl P2INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3INTE` reader - P3INT Enable"]
pub struct P3INTE_R(crate::FieldReader<bool, bool>);
impl P3INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        P3INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4INTE` reader - P4INT Enable"]
pub struct P4INTE_R(crate::FieldReader<bool, bool>);
impl P4INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        P4INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5INTE` reader - P5INT Enable"]
pub struct P5INTE_R(crate::FieldReader<bool, bool>);
impl P5INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        P5INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6INTE` reader - P6INT Enable"]
pub struct P6INTE_R(crate::FieldReader<bool, bool>);
impl P6INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        P6INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7INTE` reader - P7INT Enable"]
pub struct P7INTE_R(crate::FieldReader<bool, bool>);
impl P7INTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        P7INTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7INTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DCONNI Enable"]
    #[inline(always)]
    pub fn dconnie(&self) -> DCONNIE_R {
        DCONNIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DDISCI Enable"]
    #[inline(always)]
    pub fn ddiscie(&self) -> DDISCIE_R {
        DDISCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RSTI Enable"]
    #[inline(always)]
    pub fn rstie(&self) -> RSTIE_R {
        RSTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RSMEDI Enable"]
    #[inline(always)]
    pub fn rsmedie(&self) -> RSMEDIE_R {
        RSMEDIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RXRSMI Enable"]
    #[inline(always)]
    pub fn rxrsmie(&self) -> RXRSMIE_R {
        RXRSMIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HSOFI Enable"]
    #[inline(always)]
    pub fn hsofie(&self) -> HSOFIE_R {
        HSOFIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - HWUPI Enable"]
    #[inline(always)]
    pub fn hwupie(&self) -> HWUPIE_R {
        HWUPIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - P0INT Enable"]
    #[inline(always)]
    pub fn p0inte(&self) -> P0INTE_R {
        P0INTE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - P1INT Enable"]
    #[inline(always)]
    pub fn p1inte(&self) -> P1INTE_R {
        P1INTE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - P2INT Enable"]
    #[inline(always)]
    pub fn p2inte(&self) -> P2INTE_R {
        P2INTE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - P3INT Enable"]
    #[inline(always)]
    pub fn p3inte(&self) -> P3INTE_R {
        P3INTE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - P4INT Enable"]
    #[inline(always)]
    pub fn p4inte(&self) -> P4INTE_R {
        P4INTE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - P5INT Enable"]
    #[inline(always)]
    pub fn p5inte(&self) -> P5INTE_R {
        P5INTE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - P6INT Enable"]
    #[inline(always)]
    pub fn p6inte(&self) -> P6INTE_R {
        P6INTE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - P7INT Enable"]
    #[inline(always)]
    pub fn p7inte(&self) -> P7INTE_R {
        P7INTE_R::new(((self.bits >> 15) & 0x01) != 0)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
