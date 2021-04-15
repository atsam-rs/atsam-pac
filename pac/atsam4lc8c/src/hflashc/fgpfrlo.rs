#[doc = "Reader of register FGPFRLO"]
pub type R = crate::R<u32, super::FGPFRLO>;
#[doc = "Reader of field `LOCK0`"]
pub type LOCK0_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK1`"]
pub type LOCK1_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK2`"]
pub type LOCK2_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK3`"]
pub type LOCK3_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK4`"]
pub type LOCK4_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK5`"]
pub type LOCK5_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK6`"]
pub type LOCK6_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK7`"]
pub type LOCK7_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK8`"]
pub type LOCK8_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK9`"]
pub type LOCK9_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK10`"]
pub type LOCK10_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK11`"]
pub type LOCK11_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK12`"]
pub type LOCK12_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK13`"]
pub type LOCK13_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK14`"]
pub type LOCK14_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCK15`"]
pub type LOCK15_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF16`"]
pub type GPF16_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF17`"]
pub type GPF17_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF18`"]
pub type GPF18_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF19`"]
pub type GPF19_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF20`"]
pub type GPF20_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF21`"]
pub type GPF21_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF22`"]
pub type GPF22_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF23`"]
pub type GPF23_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF24`"]
pub type GPF24_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF25`"]
pub type GPF25_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF26`"]
pub type GPF26_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF27`"]
pub type GPF27_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF28`"]
pub type GPF28_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF29`"]
pub type GPF29_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF30`"]
pub type GPF30_R = crate::R<bool, bool>;
#[doc = "Reader of field `GPF31`"]
pub type GPF31_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Lock Bit 0"]
    #[inline(always)]
    pub fn lock0(&self) -> LOCK0_R {
        LOCK0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Bit 1"]
    #[inline(always)]
    pub fn lock1(&self) -> LOCK1_R {
        LOCK1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Bit 2"]
    #[inline(always)]
    pub fn lock2(&self) -> LOCK2_R {
        LOCK2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock Bit 3"]
    #[inline(always)]
    pub fn lock3(&self) -> LOCK3_R {
        LOCK3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Lock Bit 4"]
    #[inline(always)]
    pub fn lock4(&self) -> LOCK4_R {
        LOCK4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Lock Bit 5"]
    #[inline(always)]
    pub fn lock5(&self) -> LOCK5_R {
        LOCK5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Bit 6"]
    #[inline(always)]
    pub fn lock6(&self) -> LOCK6_R {
        LOCK6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Lock Bit 7"]
    #[inline(always)]
    pub fn lock7(&self) -> LOCK7_R {
        LOCK7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Lock Bit 8"]
    #[inline(always)]
    pub fn lock8(&self) -> LOCK8_R {
        LOCK8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Lock Bit 9"]
    #[inline(always)]
    pub fn lock9(&self) -> LOCK9_R {
        LOCK9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Lock Bit 10"]
    #[inline(always)]
    pub fn lock10(&self) -> LOCK10_R {
        LOCK10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Lock Bit 11"]
    #[inline(always)]
    pub fn lock11(&self) -> LOCK11_R {
        LOCK11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Lock Bit 12"]
    #[inline(always)]
    pub fn lock12(&self) -> LOCK12_R {
        LOCK12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Lock Bit 13"]
    #[inline(always)]
    pub fn lock13(&self) -> LOCK13_R {
        LOCK13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Lock Bit 14"]
    #[inline(always)]
    pub fn lock14(&self) -> LOCK14_R {
        LOCK14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Lock Bit 15"]
    #[inline(always)]
    pub fn lock15(&self) -> LOCK15_R {
        LOCK15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - General Purpose Fuse 16"]
    #[inline(always)]
    pub fn gpf16(&self) -> GPF16_R {
        GPF16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - General Purpose Fuse 17"]
    #[inline(always)]
    pub fn gpf17(&self) -> GPF17_R {
        GPF17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - General Purpose Fuse 18"]
    #[inline(always)]
    pub fn gpf18(&self) -> GPF18_R {
        GPF18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - General Purpose Fuse 19"]
    #[inline(always)]
    pub fn gpf19(&self) -> GPF19_R {
        GPF19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - General Purpose Fuse 20"]
    #[inline(always)]
    pub fn gpf20(&self) -> GPF20_R {
        GPF20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - General Purpose Fuse 21"]
    #[inline(always)]
    pub fn gpf21(&self) -> GPF21_R {
        GPF21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - General Purpose Fuse 22"]
    #[inline(always)]
    pub fn gpf22(&self) -> GPF22_R {
        GPF22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - General Purpose Fuse 23"]
    #[inline(always)]
    pub fn gpf23(&self) -> GPF23_R {
        GPF23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - General Purpose Fuse 24"]
    #[inline(always)]
    pub fn gpf24(&self) -> GPF24_R {
        GPF24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - General Purpose Fuse 25"]
    #[inline(always)]
    pub fn gpf25(&self) -> GPF25_R {
        GPF25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - General Purpose Fuse 26"]
    #[inline(always)]
    pub fn gpf26(&self) -> GPF26_R {
        GPF26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - General Purpose Fuse 27"]
    #[inline(always)]
    pub fn gpf27(&self) -> GPF27_R {
        GPF27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - General Purpose Fuse 28"]
    #[inline(always)]
    pub fn gpf28(&self) -> GPF28_R {
        GPF28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - General Purpose Fuse 29"]
    #[inline(always)]
    pub fn gpf29(&self) -> GPF29_R {
        GPF29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - General Purpose Fuse 30"]
    #[inline(always)]
    pub fn gpf30(&self) -> GPF30_R {
        GPF30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - General Purpose Fuse 31"]
    #[inline(always)]
    pub fn gpf31(&self) -> GPF31_R {
        GPF31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
