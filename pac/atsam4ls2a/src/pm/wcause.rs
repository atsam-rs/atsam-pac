#[doc = "Register `WCAUSE` reader"]
pub struct R(crate::R<WCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TWI_SLAVE_0` reader - Two-wire Slave Interface 0"]
pub type TWI_SLAVE_0_R = crate::BitReader<bool>;
#[doc = "Field `TWI_SLAVE_1` reader - Two-wire Slave Interface 1"]
pub type TWI_SLAVE_1_R = crate::BitReader<bool>;
#[doc = "Field `USBC` reader - USB Device and Embedded Host Interface"]
pub type USBC_R = crate::BitReader<bool>;
#[doc = "Field `PSOK` reader - Power Scaling OK"]
pub type PSOK_R = crate::BitReader<bool>;
#[doc = "Field `BOD18_IRQ` reader - BOD18 Interrupt"]
pub type BOD18_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `BOD33_IRQ` reader - BOD33 Interrupt"]
pub type BOD33_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `PICOUART` reader - Picopower UART"]
pub type PICOUART_R = crate::BitReader<bool>;
#[doc = "Field `LCDCA` reader - LCD Controller"]
pub type LCDCA_R = crate::BitReader<bool>;
#[doc = "Field `EIC` reader - External Interrupt Controller"]
pub type EIC_R = crate::BitReader<bool>;
#[doc = "Field `AST` reader - Asynchronous Timer"]
pub type AST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Two-wire Slave Interface 0"]
    #[inline(always)]
    pub fn twi_slave_0(&self) -> TWI_SLAVE_0_R {
        TWI_SLAVE_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Two-wire Slave Interface 1"]
    #[inline(always)]
    pub fn twi_slave_1(&self) -> TWI_SLAVE_1_R {
        TWI_SLAVE_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB Device and Embedded Host Interface"]
    #[inline(always)]
    pub fn usbc(&self) -> USBC_R {
        USBC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Power Scaling OK"]
    #[inline(always)]
    pub fn psok(&self) -> PSOK_R {
        PSOK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BOD18 Interrupt"]
    #[inline(always)]
    pub fn bod18_irq(&self) -> BOD18_IRQ_R {
        BOD18_IRQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BOD33 Interrupt"]
    #[inline(always)]
    pub fn bod33_irq(&self) -> BOD33_IRQ_R {
        BOD33_IRQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Picopower UART"]
    #[inline(always)]
    pub fn picouart(&self) -> PICOUART_R {
        PICOUART_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD Controller"]
    #[inline(always)]
    pub fn lcdca(&self) -> LCDCA_R {
        LCDCA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - External Interrupt Controller"]
    #[inline(always)]
    pub fn eic(&self) -> EIC_R {
        EIC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Asynchronous Timer"]
    #[inline(always)]
    pub fn ast(&self) -> AST_R {
        AST_R::new(((self.bits >> 17) & 1) != 0)
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
    const RESET_VALUE: Self::Ux = 0;
}
