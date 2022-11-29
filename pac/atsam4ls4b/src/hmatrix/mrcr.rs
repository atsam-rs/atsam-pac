#[doc = "Register `MRCR` reader"]
pub struct R(crate::R<MRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MRCR` writer"]
pub struct W(crate::W<MRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MRCR_SPEC>;
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
impl From<crate::W<MRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCB0` reader - Remap Command bit for Master 0"]
pub type RCB0_R = crate::BitReader<RCB0SELECT_A>;
#[doc = "Remap Command bit for Master 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB0SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB0SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB0SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB0SELECT_A {
        match self.bits {
            false => RCB0SELECT_A::_0,
            true => RCB0SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB0SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB0SELECT_A::_1
    }
}
#[doc = "Field `RCB0` writer - Remap Command bit for Master 0"]
pub type RCB0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB0SELECT_A, O>;
impl<'a, const O: u8> RCB0_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB0SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB0SELECT_A::_1)
    }
}
#[doc = "Field `RCB1` reader - Remap Command bit for Master 1"]
pub type RCB1_R = crate::BitReader<RCB1SELECT_A>;
#[doc = "Remap Command bit for Master 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB1SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB1SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB1SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB1SELECT_A {
        match self.bits {
            false => RCB1SELECT_A::_0,
            true => RCB1SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB1SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB1SELECT_A::_1
    }
}
#[doc = "Field `RCB1` writer - Remap Command bit for Master 1"]
pub type RCB1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB1SELECT_A, O>;
impl<'a, const O: u8> RCB1_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB1SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB1SELECT_A::_1)
    }
}
#[doc = "Field `RCB2` reader - Remap Command bit for Master 2"]
pub type RCB2_R = crate::BitReader<RCB2SELECT_A>;
#[doc = "Remap Command bit for Master 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB2SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB2SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB2SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB2SELECT_A {
        match self.bits {
            false => RCB2SELECT_A::_0,
            true => RCB2SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB2SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB2SELECT_A::_1
    }
}
#[doc = "Field `RCB2` writer - Remap Command bit for Master 2"]
pub type RCB2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB2SELECT_A, O>;
impl<'a, const O: u8> RCB2_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB2SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB2SELECT_A::_1)
    }
}
#[doc = "Field `RCB3` reader - Remap Command bit for Master 3"]
pub type RCB3_R = crate::BitReader<RCB3SELECT_A>;
#[doc = "Remap Command bit for Master 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB3SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB3SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB3SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB3SELECT_A {
        match self.bits {
            false => RCB3SELECT_A::_0,
            true => RCB3SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB3SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB3SELECT_A::_1
    }
}
#[doc = "Field `RCB3` writer - Remap Command bit for Master 3"]
pub type RCB3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB3SELECT_A, O>;
impl<'a, const O: u8> RCB3_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB3SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB3SELECT_A::_1)
    }
}
#[doc = "Field `RCB4` reader - Remap Command bit for Master 4"]
pub type RCB4_R = crate::BitReader<RCB4SELECT_A>;
#[doc = "Remap Command bit for Master 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB4SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB4SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB4SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB4SELECT_A {
        match self.bits {
            false => RCB4SELECT_A::_0,
            true => RCB4SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB4SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB4SELECT_A::_1
    }
}
#[doc = "Field `RCB4` writer - Remap Command bit for Master 4"]
pub type RCB4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB4SELECT_A, O>;
impl<'a, const O: u8> RCB4_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB4SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB4SELECT_A::_1)
    }
}
#[doc = "Field `RCB5` reader - Remap Command bit for Master 5"]
pub type RCB5_R = crate::BitReader<RCB5SELECT_A>;
#[doc = "Remap Command bit for Master 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB5SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB5SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB5SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB5SELECT_A {
        match self.bits {
            false => RCB5SELECT_A::_0,
            true => RCB5SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB5SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB5SELECT_A::_1
    }
}
#[doc = "Field `RCB5` writer - Remap Command bit for Master 5"]
pub type RCB5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB5SELECT_A, O>;
impl<'a, const O: u8> RCB5_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB5SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB5SELECT_A::_1)
    }
}
#[doc = "Field `RCB6` reader - Remap Command bit for Master 6"]
pub type RCB6_R = crate::BitReader<RCB6SELECT_A>;
#[doc = "Remap Command bit for Master 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB6SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB6SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB6SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB6SELECT_A {
        match self.bits {
            false => RCB6SELECT_A::_0,
            true => RCB6SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB6SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB6SELECT_A::_1
    }
}
#[doc = "Field `RCB6` writer - Remap Command bit for Master 6"]
pub type RCB6_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB6SELECT_A, O>;
impl<'a, const O: u8> RCB6_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB6SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB6SELECT_A::_1)
    }
}
#[doc = "Field `RCB7` reader - Remap Command bit for Master 7"]
pub type RCB7_R = crate::BitReader<RCB7SELECT_A>;
#[doc = "Remap Command bit for Master 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB7SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB7SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB7SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB7SELECT_A {
        match self.bits {
            false => RCB7SELECT_A::_0,
            true => RCB7SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB7SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB7SELECT_A::_1
    }
}
#[doc = "Field `RCB7` writer - Remap Command bit for Master 7"]
pub type RCB7_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB7SELECT_A, O>;
impl<'a, const O: u8> RCB7_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB7SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB7SELECT_A::_1)
    }
}
#[doc = "Field `RCB8` reader - Remap Command bit for Master 8"]
pub type RCB8_R = crate::BitReader<RCB8SELECT_A>;
#[doc = "Remap Command bit for Master 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB8SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB8SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB8SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB8SELECT_A {
        match self.bits {
            false => RCB8SELECT_A::_0,
            true => RCB8SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB8SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB8SELECT_A::_1
    }
}
#[doc = "Field `RCB8` writer - Remap Command bit for Master 8"]
pub type RCB8_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB8SELECT_A, O>;
impl<'a, const O: u8> RCB8_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB8SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB8SELECT_A::_1)
    }
}
#[doc = "Field `RCB9` reader - Remap Command bit for Master 9"]
pub type RCB9_R = crate::BitReader<RCB9SELECT_A>;
#[doc = "Remap Command bit for Master 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB9SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB9SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB9SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB9SELECT_A {
        match self.bits {
            false => RCB9SELECT_A::_0,
            true => RCB9SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB9SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB9SELECT_A::_1
    }
}
#[doc = "Field `RCB9` writer - Remap Command bit for Master 9"]
pub type RCB9_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB9SELECT_A, O>;
impl<'a, const O: u8> RCB9_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB9SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB9SELECT_A::_1)
    }
}
#[doc = "Field `RCB10` reader - Remap Command bit for Master 10"]
pub type RCB10_R = crate::BitReader<RCB10SELECT_A>;
#[doc = "Remap Command bit for Master 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB10SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB10SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB10SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB10SELECT_A {
        match self.bits {
            false => RCB10SELECT_A::_0,
            true => RCB10SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB10SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB10SELECT_A::_1
    }
}
#[doc = "Field `RCB10` writer - Remap Command bit for Master 10"]
pub type RCB10_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB10SELECT_A, O>;
impl<'a, const O: u8> RCB10_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB10SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB10SELECT_A::_1)
    }
}
#[doc = "Field `RCB11` reader - Remap Command bit for Master 11"]
pub type RCB11_R = crate::BitReader<RCB11SELECT_A>;
#[doc = "Remap Command bit for Master 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB11SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB11SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB11SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB11SELECT_A {
        match self.bits {
            false => RCB11SELECT_A::_0,
            true => RCB11SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB11SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB11SELECT_A::_1
    }
}
#[doc = "Field `RCB11` writer - Remap Command bit for Master 11"]
pub type RCB11_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB11SELECT_A, O>;
impl<'a, const O: u8> RCB11_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB11SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB11SELECT_A::_1)
    }
}
#[doc = "Field `RCB12` reader - Remap Command bit for Master 12"]
pub type RCB12_R = crate::BitReader<RCB12SELECT_A>;
#[doc = "Remap Command bit for Master 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB12SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB12SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB12SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB12SELECT_A {
        match self.bits {
            false => RCB12SELECT_A::_0,
            true => RCB12SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB12SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB12SELECT_A::_1
    }
}
#[doc = "Field `RCB12` writer - Remap Command bit for Master 12"]
pub type RCB12_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB12SELECT_A, O>;
impl<'a, const O: u8> RCB12_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB12SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB12SELECT_A::_1)
    }
}
#[doc = "Field `RCB13` reader - Remap Command bit for Master 13"]
pub type RCB13_R = crate::BitReader<RCB13SELECT_A>;
#[doc = "Remap Command bit for Master 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB13SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB13SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB13SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB13SELECT_A {
        match self.bits {
            false => RCB13SELECT_A::_0,
            true => RCB13SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB13SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB13SELECT_A::_1
    }
}
#[doc = "Field `RCB13` writer - Remap Command bit for Master 13"]
pub type RCB13_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB13SELECT_A, O>;
impl<'a, const O: u8> RCB13_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB13SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB13SELECT_A::_1)
    }
}
#[doc = "Field `RCB14` reader - Remap Command bit for Master 14"]
pub type RCB14_R = crate::BitReader<RCB14SELECT_A>;
#[doc = "Remap Command bit for Master 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB14SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB14SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB14SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB14SELECT_A {
        match self.bits {
            false => RCB14SELECT_A::_0,
            true => RCB14SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB14SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB14SELECT_A::_1
    }
}
#[doc = "Field `RCB14` writer - Remap Command bit for Master 14"]
pub type RCB14_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB14SELECT_A, O>;
impl<'a, const O: u8> RCB14_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB14SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB14SELECT_A::_1)
    }
}
#[doc = "Field `RCB15` reader - Remap Command bit for Master 15"]
pub type RCB15_R = crate::BitReader<RCB15SELECT_A>;
#[doc = "Remap Command bit for Master 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCB15SELECT_A {
    #[doc = "0: Disable remapped address decoding for master"]
    _0 = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    _1 = 1,
}
impl From<RCB15SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: RCB15SELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl RCB15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCB15SELECT_A {
        match self.bits {
            false => RCB15SELECT_A::_0,
            true => RCB15SELECT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCB15SELECT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCB15SELECT_A::_1
    }
}
#[doc = "Field `RCB15` writer - Remap Command bit for Master 15"]
pub type RCB15_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRCR_SPEC, RCB15SELECT_A, O>;
impl<'a, const O: u8> RCB15_W<'a, O> {
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCB15SELECT_A::_0)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCB15SELECT_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Remap Command bit for Master 0"]
    #[inline(always)]
    pub fn rcb0(&self) -> RCB0_R {
        RCB0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Remap Command bit for Master 1"]
    #[inline(always)]
    pub fn rcb1(&self) -> RCB1_R {
        RCB1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Remap Command bit for Master 2"]
    #[inline(always)]
    pub fn rcb2(&self) -> RCB2_R {
        RCB2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Remap Command bit for Master 3"]
    #[inline(always)]
    pub fn rcb3(&self) -> RCB3_R {
        RCB3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Remap Command bit for Master 4"]
    #[inline(always)]
    pub fn rcb4(&self) -> RCB4_R {
        RCB4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Remap Command bit for Master 5"]
    #[inline(always)]
    pub fn rcb5(&self) -> RCB5_R {
        RCB5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Remap Command bit for Master 6"]
    #[inline(always)]
    pub fn rcb6(&self) -> RCB6_R {
        RCB6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Remap Command bit for Master 7"]
    #[inline(always)]
    pub fn rcb7(&self) -> RCB7_R {
        RCB7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Remap Command bit for Master 8"]
    #[inline(always)]
    pub fn rcb8(&self) -> RCB8_R {
        RCB8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remap Command bit for Master 9"]
    #[inline(always)]
    pub fn rcb9(&self) -> RCB9_R {
        RCB9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Remap Command bit for Master 10"]
    #[inline(always)]
    pub fn rcb10(&self) -> RCB10_R {
        RCB10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Remap Command bit for Master 11"]
    #[inline(always)]
    pub fn rcb11(&self) -> RCB11_R {
        RCB11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Remap Command bit for Master 12"]
    #[inline(always)]
    pub fn rcb12(&self) -> RCB12_R {
        RCB12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Remap Command bit for Master 13"]
    #[inline(always)]
    pub fn rcb13(&self) -> RCB13_R {
        RCB13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Remap Command bit for Master 14"]
    #[inline(always)]
    pub fn rcb14(&self) -> RCB14_R {
        RCB14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Remap Command bit for Master 15"]
    #[inline(always)]
    pub fn rcb15(&self) -> RCB15_R {
        RCB15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remap Command bit for Master 0"]
    #[inline(always)]
    #[must_use]
    pub fn rcb0(&mut self) -> RCB0_W<0> {
        RCB0_W::new(self)
    }
    #[doc = "Bit 1 - Remap Command bit for Master 1"]
    #[inline(always)]
    #[must_use]
    pub fn rcb1(&mut self) -> RCB1_W<1> {
        RCB1_W::new(self)
    }
    #[doc = "Bit 2 - Remap Command bit for Master 2"]
    #[inline(always)]
    #[must_use]
    pub fn rcb2(&mut self) -> RCB2_W<2> {
        RCB2_W::new(self)
    }
    #[doc = "Bit 3 - Remap Command bit for Master 3"]
    #[inline(always)]
    #[must_use]
    pub fn rcb3(&mut self) -> RCB3_W<3> {
        RCB3_W::new(self)
    }
    #[doc = "Bit 4 - Remap Command bit for Master 4"]
    #[inline(always)]
    #[must_use]
    pub fn rcb4(&mut self) -> RCB4_W<4> {
        RCB4_W::new(self)
    }
    #[doc = "Bit 5 - Remap Command bit for Master 5"]
    #[inline(always)]
    #[must_use]
    pub fn rcb5(&mut self) -> RCB5_W<5> {
        RCB5_W::new(self)
    }
    #[doc = "Bit 6 - Remap Command bit for Master 6"]
    #[inline(always)]
    #[must_use]
    pub fn rcb6(&mut self) -> RCB6_W<6> {
        RCB6_W::new(self)
    }
    #[doc = "Bit 7 - Remap Command bit for Master 7"]
    #[inline(always)]
    #[must_use]
    pub fn rcb7(&mut self) -> RCB7_W<7> {
        RCB7_W::new(self)
    }
    #[doc = "Bit 8 - Remap Command bit for Master 8"]
    #[inline(always)]
    #[must_use]
    pub fn rcb8(&mut self) -> RCB8_W<8> {
        RCB8_W::new(self)
    }
    #[doc = "Bit 9 - Remap Command bit for Master 9"]
    #[inline(always)]
    #[must_use]
    pub fn rcb9(&mut self) -> RCB9_W<9> {
        RCB9_W::new(self)
    }
    #[doc = "Bit 10 - Remap Command bit for Master 10"]
    #[inline(always)]
    #[must_use]
    pub fn rcb10(&mut self) -> RCB10_W<10> {
        RCB10_W::new(self)
    }
    #[doc = "Bit 11 - Remap Command bit for Master 11"]
    #[inline(always)]
    #[must_use]
    pub fn rcb11(&mut self) -> RCB11_W<11> {
        RCB11_W::new(self)
    }
    #[doc = "Bit 12 - Remap Command bit for Master 12"]
    #[inline(always)]
    #[must_use]
    pub fn rcb12(&mut self) -> RCB12_W<12> {
        RCB12_W::new(self)
    }
    #[doc = "Bit 13 - Remap Command bit for Master 13"]
    #[inline(always)]
    #[must_use]
    pub fn rcb13(&mut self) -> RCB13_W<13> {
        RCB13_W::new(self)
    }
    #[doc = "Bit 14 - Remap Command bit for Master 14"]
    #[inline(always)]
    #[must_use]
    pub fn rcb14(&mut self) -> RCB14_W<14> {
        RCB14_W::new(self)
    }
    #[doc = "Bit 15 - Remap Command bit for Master 15"]
    #[inline(always)]
    #[must_use]
    pub fn rcb15(&mut self) -> RCB15_W<15> {
        RCB15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Remap Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrcr](index.html) module"]
pub struct MRCR_SPEC;
impl crate::RegisterSpec for MRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrcr::R](R) reader structure"]
impl crate::Readable for MRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mrcr::W](W) writer structure"]
impl crate::Writable for MRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MRCR to value 0"]
impl crate::Resettable for MRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
