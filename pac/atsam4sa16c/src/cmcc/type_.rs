#[doc = "Register `TYPE` reader"]
pub struct R(crate::R<TYPE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TYPE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TYPE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TYPE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AP` reader - Access Port Access Allowed"]
pub type AP_R = crate::BitReader<bool>;
#[doc = "Field `GCLK` reader - Dynamic Clock Gating Supported"]
pub type GCLK_R = crate::BitReader<bool>;
#[doc = "Field `RANDP` reader - Random Selection Policy Supported"]
pub type RANDP_R = crate::BitReader<bool>;
#[doc = "Field `LRUP` reader - Least Recently Used Policy Supported"]
pub type LRUP_R = crate::BitReader<bool>;
#[doc = "Field `RRP` reader - Random Selection Policy Supported"]
pub type RRP_R = crate::BitReader<bool>;
#[doc = "Field `WAYNUM` reader - Number of Way"]
pub type WAYNUM_R = crate::FieldReader<u8, WAYNUM_A>;
#[doc = "Number of Way"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WAYNUM_A {
    #[doc = "0: Direct Mapped Cache"]
    DMAPPED = 0,
    #[doc = "1: 2-WAY set associative"]
    ARCH2WAY = 1,
    #[doc = "2: 4-WAY set associative"]
    ARCH4WAY = 2,
    #[doc = "3: 8-WAY set associative"]
    ARCH8WAY = 3,
}
impl From<WAYNUM_A> for u8 {
    #[inline(always)]
    fn from(variant: WAYNUM_A) -> Self {
        variant as _
    }
}
impl WAYNUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAYNUM_A {
        match self.bits {
            0 => WAYNUM_A::DMAPPED,
            1 => WAYNUM_A::ARCH2WAY,
            2 => WAYNUM_A::ARCH4WAY,
            3 => WAYNUM_A::ARCH8WAY,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DMAPPED`"]
    #[inline(always)]
    pub fn is_dmapped(&self) -> bool {
        *self == WAYNUM_A::DMAPPED
    }
    #[doc = "Checks if the value of the field is `ARCH2WAY`"]
    #[inline(always)]
    pub fn is_arch2way(&self) -> bool {
        *self == WAYNUM_A::ARCH2WAY
    }
    #[doc = "Checks if the value of the field is `ARCH4WAY`"]
    #[inline(always)]
    pub fn is_arch4way(&self) -> bool {
        *self == WAYNUM_A::ARCH4WAY
    }
    #[doc = "Checks if the value of the field is `ARCH8WAY`"]
    #[inline(always)]
    pub fn is_arch8way(&self) -> bool {
        *self == WAYNUM_A::ARCH8WAY
    }
}
#[doc = "Field `LCKDOWN` reader - Lock Down Supported"]
pub type LCKDOWN_R = crate::BitReader<bool>;
#[doc = "Field `CSIZE` reader - Cache Size"]
pub type CSIZE_R = crate::FieldReader<u8, CSIZE_A>;
#[doc = "Cache Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CSIZE_A {
    #[doc = "0: Cache Size 1 Kbytes"]
    CSIZE_1KB = 0,
    #[doc = "1: Cache Size 2 Kbytes"]
    CSIZE_2KB = 1,
    #[doc = "2: Cache Size 4 Kbytes"]
    CSIZE_4KB = 2,
    #[doc = "3: Cache Size 8 Kbytes"]
    CSIZE_8KB = 3,
}
impl From<CSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CSIZE_A) -> Self {
        variant as _
    }
}
impl CSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CSIZE_A> {
        match self.bits {
            0 => Some(CSIZE_A::CSIZE_1KB),
            1 => Some(CSIZE_A::CSIZE_2KB),
            2 => Some(CSIZE_A::CSIZE_4KB),
            3 => Some(CSIZE_A::CSIZE_8KB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CSIZE_1KB`"]
    #[inline(always)]
    pub fn is_csize_1kb(&self) -> bool {
        *self == CSIZE_A::CSIZE_1KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_2KB`"]
    #[inline(always)]
    pub fn is_csize_2kb(&self) -> bool {
        *self == CSIZE_A::CSIZE_2KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_4KB`"]
    #[inline(always)]
    pub fn is_csize_4kb(&self) -> bool {
        *self == CSIZE_A::CSIZE_4KB
    }
    #[doc = "Checks if the value of the field is `CSIZE_8KB`"]
    #[inline(always)]
    pub fn is_csize_8kb(&self) -> bool {
        *self == CSIZE_A::CSIZE_8KB
    }
}
#[doc = "Field `CLSIZE` reader - Cache Size"]
pub type CLSIZE_R = crate::FieldReader<u8, CLSIZE_A>;
#[doc = "Cache Size"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLSIZE_A {
    #[doc = "0: 4 Bytes"]
    CLSIZE_1KB = 0,
    #[doc = "1: 8 Bytes"]
    CLSIZE_2KB = 1,
    #[doc = "2: 16 Bytes"]
    CLSIZE_4KB = 2,
    #[doc = "3: 32 Bytes"]
    CLSIZE_8KB = 3,
}
impl From<CLSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: CLSIZE_A) -> Self {
        variant as _
    }
}
impl CLSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLSIZE_A> {
        match self.bits {
            0 => Some(CLSIZE_A::CLSIZE_1KB),
            1 => Some(CLSIZE_A::CLSIZE_2KB),
            2 => Some(CLSIZE_A::CLSIZE_4KB),
            3 => Some(CLSIZE_A::CLSIZE_8KB),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLSIZE_1KB`"]
    #[inline(always)]
    pub fn is_clsize_1kb(&self) -> bool {
        *self == CLSIZE_A::CLSIZE_1KB
    }
    #[doc = "Checks if the value of the field is `CLSIZE_2KB`"]
    #[inline(always)]
    pub fn is_clsize_2kb(&self) -> bool {
        *self == CLSIZE_A::CLSIZE_2KB
    }
    #[doc = "Checks if the value of the field is `CLSIZE_4KB`"]
    #[inline(always)]
    pub fn is_clsize_4kb(&self) -> bool {
        *self == CLSIZE_A::CLSIZE_4KB
    }
    #[doc = "Checks if the value of the field is `CLSIZE_8KB`"]
    #[inline(always)]
    pub fn is_clsize_8kb(&self) -> bool {
        *self == CLSIZE_A::CLSIZE_8KB
    }
}
impl R {
    #[doc = "Bit 0 - Access Port Access Allowed"]
    #[inline(always)]
    pub fn ap(&self) -> AP_R {
        AP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dynamic Clock Gating Supported"]
    #[inline(always)]
    pub fn gclk(&self) -> GCLK_R {
        GCLK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Random Selection Policy Supported"]
    #[inline(always)]
    pub fn randp(&self) -> RANDP_R {
        RANDP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Least Recently Used Policy Supported"]
    #[inline(always)]
    pub fn lrup(&self) -> LRUP_R {
        LRUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Random Selection Policy Supported"]
    #[inline(always)]
    pub fn rrp(&self) -> RRP_R {
        RRP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Number of Way"]
    #[inline(always)]
    pub fn waynum(&self) -> WAYNUM_R {
        WAYNUM_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Lock Down Supported"]
    #[inline(always)]
    pub fn lckdown(&self) -> LCKDOWN_R {
        LCKDOWN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Cache Size"]
    #[inline(always)]
    pub fn csize(&self) -> CSIZE_R {
        CSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Cache Size"]
    #[inline(always)]
    pub fn clsize(&self) -> CLSIZE_R {
        CLSIZE_R::new(((self.bits >> 11) & 7) as u8)
    }
}
#[doc = "Cache Type Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [type_](index.html) module"]
pub struct TYPE_SPEC;
impl crate::RegisterSpec for TYPE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [type_::R](R) reader structure"]
impl crate::Readable for TYPE_SPEC {
    type Reader = R;
}
