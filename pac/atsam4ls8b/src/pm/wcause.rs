#[doc = "Reader of register WCAUSE"]
pub type R = crate::R<u32, super::WCAUSE>;
#[doc = "Reader of field `TWI_SLAVE_0`"]
pub type TWI_SLAVE_0_R = crate::R<bool, bool>;
#[doc = "Reader of field `TWI_SLAVE_1`"]
pub type TWI_SLAVE_1_R = crate::R<bool, bool>;
#[doc = "Reader of field `USBC`"]
pub type USBC_R = crate::R<bool, bool>;
#[doc = "Reader of field `PSOK`"]
pub type PSOK_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD18_IRQ`"]
pub type BOD18_IRQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `BOD33_IRQ`"]
pub type BOD33_IRQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `PICOUART`"]
pub type PICOUART_R = crate::R<bool, bool>;
#[doc = "Reader of field `LCDCA`"]
pub type LCDCA_R = crate::R<bool, bool>;
#[doc = "Reader of field `EIC`"]
pub type EIC_R = crate::R<bool, bool>;
#[doc = "Reader of field `AST`"]
pub type AST_R = crate::R<bool, bool>;
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
