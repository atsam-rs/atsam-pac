#[doc = "Reader of register RC%s"]
pub type R = crate::R<u32, super::RC>;
#[doc = "Writer for register RC%s"]
pub type W = crate::W<u32, super::RC>;
#[doc = "Register RC%s `reset()`'s with value 0"]
impl crate::ResetValue for super::RC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RC`"]
pub type RC_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RC`"]
pub struct RC_W<'a> {
    w: &'a mut W,
}
impl<'a> RC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Register C"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Register C"]
    #[inline(always)]
    pub fn rc(&mut self) -> RC_W {
        RC_W { w: self }
    }
}
