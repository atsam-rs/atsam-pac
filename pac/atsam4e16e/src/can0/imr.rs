#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MB0` reader - Mailbox 0 Interrupt Mask"]
pub type MB0_R = crate::BitReader<bool>;
#[doc = "Field `MB1` reader - Mailbox 1 Interrupt Mask"]
pub type MB1_R = crate::BitReader<bool>;
#[doc = "Field `MB2` reader - Mailbox 2 Interrupt Mask"]
pub type MB2_R = crate::BitReader<bool>;
#[doc = "Field `MB3` reader - Mailbox 3 Interrupt Mask"]
pub type MB3_R = crate::BitReader<bool>;
#[doc = "Field `MB4` reader - Mailbox 4 Interrupt Mask"]
pub type MB4_R = crate::BitReader<bool>;
#[doc = "Field `MB5` reader - Mailbox 5 Interrupt Mask"]
pub type MB5_R = crate::BitReader<bool>;
#[doc = "Field `MB6` reader - Mailbox 6 Interrupt Mask"]
pub type MB6_R = crate::BitReader<bool>;
#[doc = "Field `MB7` reader - Mailbox 7 Interrupt Mask"]
pub type MB7_R = crate::BitReader<bool>;
#[doc = "Field `ERRA` reader - Error Active Mode Interrupt Mask"]
pub type ERRA_R = crate::BitReader<bool>;
#[doc = "Field `WARN` reader - Warning Limit Interrupt Mask"]
pub type WARN_R = crate::BitReader<bool>;
#[doc = "Field `ERRP` reader - Error Passive Mode Interrupt Mask"]
pub type ERRP_R = crate::BitReader<bool>;
#[doc = "Field `BOFF` reader - Bus Off Mode Interrupt Mask"]
pub type BOFF_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP` reader - Sleep Interrupt Mask"]
pub type SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP` reader - Wakeup Interrupt Mask"]
pub type WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `TOVF` reader - Timer Overflow Interrupt Mask"]
pub type TOVF_R = crate::BitReader<bool>;
#[doc = "Field `TSTP` reader - Timestamp Interrupt Mask"]
pub type TSTP_R = crate::BitReader<bool>;
#[doc = "Field `CERR` reader - CRC Error Interrupt Mask"]
pub type CERR_R = crate::BitReader<bool>;
#[doc = "Field `SERR` reader - Stuffing Error Interrupt Mask"]
pub type SERR_R = crate::BitReader<bool>;
#[doc = "Field `AERR` reader - Acknowledgment Error Interrupt Mask"]
pub type AERR_R = crate::BitReader<bool>;
#[doc = "Field `FERR` reader - Form Error Interrupt Mask"]
pub type FERR_R = crate::BitReader<bool>;
#[doc = "Field `BERR` reader - Bit Error Interrupt Mask"]
pub type BERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Mailbox 0 Interrupt Mask"]
    #[inline(always)]
    pub fn mb0(&self) -> MB0_R {
        MB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox 1 Interrupt Mask"]
    #[inline(always)]
    pub fn mb1(&self) -> MB1_R {
        MB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox 2 Interrupt Mask"]
    #[inline(always)]
    pub fn mb2(&self) -> MB2_R {
        MB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox 3 Interrupt Mask"]
    #[inline(always)]
    pub fn mb3(&self) -> MB3_R {
        MB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mailbox 4 Interrupt Mask"]
    #[inline(always)]
    pub fn mb4(&self) -> MB4_R {
        MB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mailbox 5 Interrupt Mask"]
    #[inline(always)]
    pub fn mb5(&self) -> MB5_R {
        MB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mailbox 6 Interrupt Mask"]
    #[inline(always)]
    pub fn mb6(&self) -> MB6_R {
        MB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Mailbox 7 Interrupt Mask"]
    #[inline(always)]
    pub fn mb7(&self) -> MB7_R {
        MB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Error Active Mode Interrupt Mask"]
    #[inline(always)]
    pub fn erra(&self) -> ERRA_R {
        ERRA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Warning Limit Interrupt Mask"]
    #[inline(always)]
    pub fn warn(&self) -> WARN_R {
        WARN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Error Passive Mode Interrupt Mask"]
    #[inline(always)]
    pub fn errp(&self) -> ERRP_R {
        ERRP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bus Off Mode Interrupt Mask"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Sleep Interrupt Mask"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wakeup Interrupt Mask"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Timer Overflow Interrupt Mask"]
    #[inline(always)]
    pub fn tovf(&self) -> TOVF_R {
        TOVF_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Timestamp Interrupt Mask"]
    #[inline(always)]
    pub fn tstp(&self) -> TSTP_R {
        TSTP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - CRC Error Interrupt Mask"]
    #[inline(always)]
    pub fn cerr(&self) -> CERR_R {
        CERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Stuffing Error Interrupt Mask"]
    #[inline(always)]
    pub fn serr(&self) -> SERR_R {
        SERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Acknowledgment Error Interrupt Mask"]
    #[inline(always)]
    pub fn aerr(&self) -> AERR_R {
        AERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Form Error Interrupt Mask"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Bit Error Interrupt Mask"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
