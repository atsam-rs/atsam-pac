#[doc = "Register `WCAUSE` reader"]
pub struct R(crate::R<WCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::convert::From<crate::R<WCAUSE_SPEC>> for R {
    fn from(reader: crate::R<WCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TWI_SLAVE_0` reader - Two-wire Slave Interface 0"]
pub struct TWI_SLAVE_0_R(crate::FieldReader<bool, bool>);
impl TWI_SLAVE_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWI_SLAVE_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWI_SLAVE_0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TWI_SLAVE_1` reader - Two-wire Slave Interface 1"]
pub struct TWI_SLAVE_1_R(crate::FieldReader<bool, bool>);
impl TWI_SLAVE_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TWI_SLAVE_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TWI_SLAVE_1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBC` reader - USB Device and Embedded Host Interface"]
pub struct USBC_R(crate::FieldReader<bool, bool>);
impl USBC_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSOK` reader - Power Scaling OK"]
pub struct PSOK_R(crate::FieldReader<bool, bool>);
impl PSOK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSOK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSOK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD18_IRQ` reader - BOD18 Interrupt"]
pub struct BOD18_IRQ_R(crate::FieldReader<bool, bool>);
impl BOD18_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOD18_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD18_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOD33_IRQ` reader - BOD33 Interrupt"]
pub struct BOD33_IRQ_R(crate::FieldReader<bool, bool>);
impl BOD33_IRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOD33_IRQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOD33_IRQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PICOUART` reader - Picopower UART"]
pub struct PICOUART_R(crate::FieldReader<bool, bool>);
impl PICOUART_R {
    pub(crate) fn new(bits: bool) -> Self {
        PICOUART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PICOUART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LCDCA` reader - LCD Controller"]
pub struct LCDCA_R(crate::FieldReader<bool, bool>);
impl LCDCA_R {
    pub(crate) fn new(bits: bool) -> Self {
        LCDCA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LCDCA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIC` reader - External Interrupt Controller"]
pub struct EIC_R(crate::FieldReader<bool, bool>);
impl EIC_R {
    pub(crate) fn new(bits: bool) -> Self {
        EIC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AST` reader - Asynchronous Timer"]
pub struct AST_R(crate::FieldReader<bool, bool>);
impl AST_R {
    pub(crate) fn new(bits: bool) -> Self {
        AST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Two-wire Slave Interface 0"]
    #[inline(always)]
    pub fn twi_slave_0(&self) -> TWI_SLAVE_0_R {
        TWI_SLAVE_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Two-wire Slave Interface 1"]
    #[inline(always)]
    pub fn twi_slave_1(&self) -> TWI_SLAVE_1_R {
        TWI_SLAVE_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - USB Device and Embedded Host Interface"]
    #[inline(always)]
    pub fn usbc(&self) -> USBC_R {
        USBC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Power Scaling OK"]
    #[inline(always)]
    pub fn psok(&self) -> PSOK_R {
        PSOK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - BOD18 Interrupt"]
    #[inline(always)]
    pub fn bod18_irq(&self) -> BOD18_IRQ_R {
        BOD18_IRQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BOD33 Interrupt"]
    #[inline(always)]
    pub fn bod33_irq(&self) -> BOD33_IRQ_R {
        BOD33_IRQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Picopower UART"]
    #[inline(always)]
    pub fn picouart(&self) -> PICOUART_R {
        PICOUART_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LCD Controller"]
    #[inline(always)]
    pub fn lcdca(&self) -> LCDCA_R {
        LCDCA_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - External Interrupt Controller"]
    #[inline(always)]
    pub fn eic(&self) -> EIC_R {
        EIC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Asynchronous Timer"]
    #[inline(always)]
    pub fn ast(&self) -> AST_R {
        AST_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
#[doc = "Wake Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcause](index.html) module"]
pub struct WCAUSE_SPEC;
impl crate::RegisterSpec for WCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wcause::R](R) reader structure"]
impl crate::Readable for WCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WCAUSE to value 0"]
impl crate::Resettable for WCAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
