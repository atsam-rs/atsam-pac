#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NMI` reader - External Non Maskable CPU interrupt"]
pub type NMI_R = crate::BitReader<bool>;
#[doc = "Field `INT1` reader - External Interrupt 1"]
pub type INT1_R = crate::BitReader<bool>;
#[doc = "Field `INT2` reader - External Interrupt 2"]
pub type INT2_R = crate::BitReader<bool>;
#[doc = "Field `INT3` reader - External Interrupt 3"]
pub type INT3_R = crate::BitReader<bool>;
#[doc = "Field `INT4` reader - External Interrupt 4"]
pub type INT4_R = crate::BitReader<bool>;
#[doc = "Field `INT5` reader - External Interrupt 5"]
pub type INT5_R = crate::BitReader<bool>;
#[doc = "Field `INT6` reader - External Interrupt 6"]
pub type INT6_R = crate::BitReader<bool>;
#[doc = "Field `INT7` reader - External Interrupt 7"]
pub type INT7_R = crate::BitReader<bool>;
#[doc = "Field `INT8` reader - External Interrupt 8"]
pub type INT8_R = crate::BitReader<bool>;
#[doc = "Field `INT9` reader - External Interrupt 9"]
pub type INT9_R = crate::BitReader<bool>;
#[doc = "Field `INT10` reader - External Interrupt 10"]
pub type INT10_R = crate::BitReader<bool>;
#[doc = "Field `INT11` reader - External Interrupt 11"]
pub type INT11_R = crate::BitReader<bool>;
#[doc = "Field `INT12` reader - External Interrupt 12"]
pub type INT12_R = crate::BitReader<bool>;
#[doc = "Field `INT13` reader - External Interrupt 13"]
pub type INT13_R = crate::BitReader<bool>;
#[doc = "Field `INT14` reader - External Interrupt 14"]
pub type INT14_R = crate::BitReader<bool>;
#[doc = "Field `INT15` reader - External Interrupt 15"]
pub type INT15_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - External Non Maskable CPU interrupt"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Interrupt 1"]
    #[inline(always)]
    pub fn int1(&self) -> INT1_R {
        INT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Interrupt 2"]
    #[inline(always)]
    pub fn int2(&self) -> INT2_R {
        INT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Interrupt 3"]
    #[inline(always)]
    pub fn int3(&self) -> INT3_R {
        INT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Interrupt 4"]
    #[inline(always)]
    pub fn int4(&self) -> INT4_R {
        INT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Interrupt 5"]
    #[inline(always)]
    pub fn int5(&self) -> INT5_R {
        INT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Interrupt 6"]
    #[inline(always)]
    pub fn int6(&self) -> INT6_R {
        INT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - External Interrupt 7"]
    #[inline(always)]
    pub fn int7(&self) -> INT7_R {
        INT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - External Interrupt 8"]
    #[inline(always)]
    pub fn int8(&self) -> INT8_R {
        INT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - External Interrupt 9"]
    #[inline(always)]
    pub fn int9(&self) -> INT9_R {
        INT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - External Interrupt 10"]
    #[inline(always)]
    pub fn int10(&self) -> INT10_R {
        INT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - External Interrupt 11"]
    #[inline(always)]
    pub fn int11(&self) -> INT11_R {
        INT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - External Interrupt 12"]
    #[inline(always)]
    pub fn int12(&self) -> INT12_R {
        INT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Interrupt 13"]
    #[inline(always)]
    pub fn int13(&self) -> INT13_R {
        INT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - External Interrupt 14"]
    #[inline(always)]
    pub fn int14(&self) -> INT14_R {
        INT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External Interrupt 15"]
    #[inline(always)]
    pub fn int15(&self) -> INT15_R {
        INT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
