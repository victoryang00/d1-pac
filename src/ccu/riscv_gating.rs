#[doc = "Register `RISCV_GATING` reader"]
pub struct R(crate::R<RISCV_GATING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISCV_GATING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISCV_GATING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISCV_GATING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RISCV_GATING` writer"]
pub struct W(crate::W<RISCV_GATING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RISCV_GATING_SPEC>;
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
impl From<crate::W<RISCV_GATING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RISCV_GATING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Gating Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GATING_A {
    #[doc = "0: `0`"]
    MASK = 0,
    #[doc = "1: `1`"]
    PASS = 1,
}
impl From<GATING_A> for bool {
    #[inline(always)]
    fn from(variant: GATING_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GATING` reader - Gating Clock"]
pub struct GATING_R(crate::FieldReader<bool, GATING_A>);
impl GATING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GATING_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GATING_A {
        match self.bits {
            false => GATING_A::MASK,
            true => GATING_A::PASS,
        }
    }
    #[doc = "Checks if the value of the field is `MASK`"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        **self == GATING_A::MASK
    }
    #[doc = "Checks if the value of the field is `PASS`"]
    #[inline(always)]
    pub fn is_pass(&self) -> bool {
        **self == GATING_A::PASS
    }
}
impl core::ops::Deref for GATING_R {
    type Target = crate::FieldReader<bool, GATING_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GATING` writer - Gating Clock"]
pub struct GATING_W<'a> {
    w: &'a mut W,
}
impl<'a> GATING_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GATING_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut W {
        self.variant(GATING_A::MASK)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pass(self) -> &'a mut W {
        self.variant(GATING_A::PASS)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Field `GATING_FIELD` reader - "]
pub struct GATING_FIELD_R(crate::FieldReader<u16, u16>);
impl GATING_FIELD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        GATING_FIELD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GATING_FIELD_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GATING_FIELD` writer - "]
pub struct GATING_FIELD_W<'a> {
    w: &'a mut W,
}
impl<'a> GATING_FIELD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn gating(&self) -> GATING_R {
        GATING_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gating_field(&self) -> GATING_FIELD_R {
        GATING_FIELD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Gating Clock"]
    #[inline(always)]
    pub fn gating(&mut self) -> GATING_W {
        GATING_W { w: self }
    }
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn gating_field(&mut self) -> GATING_FIELD_W {
        GATING_FIELD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RISC-V GATING Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [riscv_gating](index.html) module"]
pub struct RISCV_GATING_SPEC;
impl crate::RegisterSpec for RISCV_GATING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [riscv_gating::R](R) reader structure"]
impl crate::Readable for RISCV_GATING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [riscv_gating::W](W) writer structure"]
impl crate::Writable for RISCV_GATING_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RISCV_GATING to value 0"]
impl crate::Resettable for RISCV_GATING_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
