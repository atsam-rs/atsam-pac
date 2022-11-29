#[doc = "Register `FGPFRLO` reader"]
pub struct R(crate::R<FGPFRLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FGPFRLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FGPFRLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FGPFRLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FGPFRLO` writer"]
pub struct W(crate::W<FGPFRLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FGPFRLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<FGPFRLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FGPFRLO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK0` reader - Lock Bit 0"]
pub type LOCK0_R = crate::BitReader<bool>;
#[doc = "Field `LOCK1` reader - Lock Bit 1"]
pub type LOCK1_R = crate::BitReader<bool>;
#[doc = "Field `LOCK2` reader - Lock Bit 2"]
pub type LOCK2_R = crate::BitReader<bool>;
#[doc = "Field `LOCK3` reader - Lock Bit 3"]
pub type LOCK3_R = crate::BitReader<bool>;
#[doc = "Field `LOCK4` reader - Lock Bit 4"]
pub type LOCK4_R = crate::BitReader<bool>;
#[doc = "Field `LOCK5` reader - Lock Bit 5"]
pub type LOCK5_R = crate::BitReader<bool>;
#[doc = "Field `LOCK6` reader - Lock Bit 6"]
pub type LOCK6_R = crate::BitReader<bool>;
#[doc = "Field `LOCK7` reader - Lock Bit 7"]
pub type LOCK7_R = crate::BitReader<bool>;
#[doc = "Field `LOCK8` reader - Lock Bit 8"]
pub type LOCK8_R = crate::BitReader<bool>;
#[doc = "Field `LOCK9` reader - Lock Bit 9"]
pub type LOCK9_R = crate::BitReader<bool>;
#[doc = "Field `LOCK10` reader - Lock Bit 10"]
pub type LOCK10_R = crate::BitReader<bool>;
#[doc = "Field `LOCK11` reader - Lock Bit 11"]
pub type LOCK11_R = crate::BitReader<bool>;
#[doc = "Field `LOCK12` reader - Lock Bit 12"]
pub type LOCK12_R = crate::BitReader<bool>;
#[doc = "Field `LOCK13` reader - Lock Bit 13"]
pub type LOCK13_R = crate::BitReader<bool>;
#[doc = "Field `LOCK14` reader - Lock Bit 14"]
pub type LOCK14_R = crate::BitReader<bool>;
#[doc = "Field `LOCK15` reader - Lock Bit 15"]
pub type LOCK15_R = crate::BitReader<bool>;
#[doc = "Field `GPF16` reader - General Purpose Fuse 16"]
pub type GPF16_R = crate::BitReader<bool>;
#[doc = "Field `GPF17` reader - General Purpose Fuse 17"]
pub type GPF17_R = crate::BitReader<bool>;
#[doc = "Field `GPF18` reader - General Purpose Fuse 18"]
pub type GPF18_R = crate::BitReader<bool>;
#[doc = "Field `GPF19` reader - General Purpose Fuse 19"]
pub type GPF19_R = crate::BitReader<bool>;
#[doc = "Field `GPF20` reader - General Purpose Fuse 20"]
pub type GPF20_R = crate::BitReader<bool>;
#[doc = "Field `GPF21` reader - General Purpose Fuse 21"]
pub type GPF21_R = crate::BitReader<bool>;
#[doc = "Field `GPF22` reader - General Purpose Fuse 22"]
pub type GPF22_R = crate::BitReader<bool>;
#[doc = "Field `GPF23` reader - General Purpose Fuse 23"]
pub type GPF23_R = crate::BitReader<bool>;
#[doc = "Field `GPF24` reader - General Purpose Fuse 24"]
pub type GPF24_R = crate::BitReader<bool>;
#[doc = "Field `GPF25` reader - General Purpose Fuse 25"]
pub type GPF25_R = crate::BitReader<bool>;
#[doc = "Field `GPF26` reader - General Purpose Fuse 26"]
pub type GPF26_R = crate::BitReader<bool>;
#[doc = "Field `GPF27` reader - General Purpose Fuse 27"]
pub type GPF27_R = crate::BitReader<bool>;
#[doc = "Field `GPF28` reader - General Purpose Fuse 28"]
pub type GPF28_R = crate::BitReader<bool>;
#[doc = "Field `GPF29` reader - General Purpose Fuse 29"]
pub type GPF29_R = crate::BitReader<bool>;
#[doc = "Field `GPF30` reader - General Purpose Fuse 30"]
pub type GPF30_R = crate::BitReader<bool>;
#[doc = "Field `GPF31` reader - General Purpose Fuse 31"]
pub type GPF31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Lock Bit 0"]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Lock Bit 1"]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock Bit 2"]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Lock Bit 3"]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Lock Bit 4"]
    #[inline(always)]
    pub fn lock4(&self) -> LOCK4_R {
        LOCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Lock Bit 5"]
    #[inline(always)]
    pub fn lock5(&self) -> LOCK5_R {
        LOCK5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock Bit 6"]
    #[inline(always)]
    pub fn lock6(&self) -> LOCK6_R {
        LOCK6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Lock Bit 7"]
    #[inline(always)]
    pub fn lock7(&self) -> LOCK7_R {
        LOCK7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Lock Bit 8"]
    #[inline(always)]
    pub fn lock8(&self) -> LOCK8_R {
        LOCK8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Lock Bit 9"]
    #[inline(always)]
    pub fn lock9(&self) -> LOCK9_R {
        LOCK9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Lock Bit 10"]
    #[inline(always)]
    pub fn lock10(&self) -> LOCK10_R {
        LOCK10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Lock Bit 11"]
    #[inline(always)]
    pub fn lock11(&self) -> LOCK11_R {
        LOCK11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Lock Bit 12"]
    #[inline(always)]
    pub fn lock12(&self) -> LOCK12_R {
        LOCK12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Lock Bit 13"]
    #[inline(always)]
    pub fn lock13(&self) -> LOCK13_R {
        LOCK13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Lock Bit 14"]
    #[inline(always)]
    pub fn lock14(&self) -> LOCK14_R {
        LOCK14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Lock Bit 15"]
    #[inline(always)]
    pub fn lock15(&self) -> LOCK15_R {
        LOCK15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - General Purpose Fuse 16"]
    #[inline(always)]
    pub fn gpf16(&self) -> GPF16_R {
        GPF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - General Purpose Fuse 17"]
    #[inline(always)]
    pub fn gpf17(&self) -> GPF17_R {
        GPF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - General Purpose Fuse 18"]
    #[inline(always)]
    pub fn gpf18(&self) -> GPF18_R {
        GPF18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - General Purpose Fuse 19"]
    #[inline(always)]
    pub fn gpf19(&self) -> GPF19_R {
        GPF19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - General Purpose Fuse 20"]
    #[inline(always)]
    pub fn gpf20(&self) -> GPF20_R {
        GPF20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - General Purpose Fuse 21"]
    #[inline(always)]
    pub fn gpf21(&self) -> GPF21_R {
        GPF21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - General Purpose Fuse 22"]
    #[inline(always)]
    pub fn gpf22(&self) -> GPF22_R {
        GPF22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - General Purpose Fuse 23"]
    #[inline(always)]
    pub fn gpf23(&self) -> GPF23_R {
        GPF23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - General Purpose Fuse 24"]
    #[inline(always)]
    pub fn gpf24(&self) -> GPF24_R {
        GPF24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - General Purpose Fuse 25"]
    #[inline(always)]
    pub fn gpf25(&self) -> GPF25_R {
        GPF25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - General Purpose Fuse 26"]
    #[inline(always)]
    pub fn gpf26(&self) -> GPF26_R {
        GPF26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - General Purpose Fuse 27"]
    #[inline(always)]
    pub fn gpf27(&self) -> GPF27_R {
        GPF27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - General Purpose Fuse 28"]
    #[inline(always)]
    pub fn gpf28(&self) -> GPF28_R {
        GPF28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - General Purpose Fuse 29"]
    #[inline(always)]
    pub fn gpf29(&self) -> GPF29_R {
        GPF29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - General Purpose Fuse 30"]
    #[inline(always)]
    pub fn gpf30(&self) -> GPF30_R {
        GPF30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - General Purpose Fuse 31"]
    #[inline(always)]
    pub fn gpf31(&self) -> GPF31_R {
        GPF31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Controller General Purpose Fuse Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fgpfrlo](index.html) module"]
pub struct FGPFRLO_SPEC;
impl crate::RegisterSpec for FGPFRLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fgpfrlo::R](R) reader structure"]
impl crate::Readable for FGPFRLO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fgpfrlo::W](W) writer structure"]
impl crate::Writable for FGPFRLO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FGPFRLO to value 0"]
impl crate::Resettable for FGPFRLO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
