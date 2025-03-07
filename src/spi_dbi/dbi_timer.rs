#[doc = "Register `DBI_TIMER` reader"]
pub struct R(crate::R<DBI_TIMER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBI_TIMER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBI_TIMER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBI_TIMER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBI_TIMER` writer"]
pub struct W(crate::W<DBI_TIMER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_TIMER_SPEC>;
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
impl From<crate::W<DBI_TIMER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_TIMER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "DBI Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DBI_TM_EN_A {
    #[doc = "0: `0`"]
    ENABLE = 0,
    #[doc = "1: `1`"]
    DISABLE = 1,
}
impl From<DBI_TM_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DBI_TM_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dbi_tm_en` reader - DBI Timer Enable"]
pub struct DBI_TM_EN_R(crate::FieldReader<bool, DBI_TM_EN_A>);
impl DBI_TM_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DBI_TM_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBI_TM_EN_A {
        match self.bits {
            false => DBI_TM_EN_A::ENABLE,
            true => DBI_TM_EN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == DBI_TM_EN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == DBI_TM_EN_A::DISABLE
    }
}
impl core::ops::Deref for DBI_TM_EN_R {
    type Target = crate::FieldReader<bool, DBI_TM_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_tm_en` writer - DBI Timer Enable"]
pub struct DBI_TM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_TM_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBI_TM_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DBI_TM_EN_A::ENABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DBI_TM_EN_A::DISABLE)
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
#[doc = "Field `dbi_timer_value` reader - "]
pub struct DBI_TIMER_VALUE_R(crate::FieldReader<u32, u32>);
impl DBI_TIMER_VALUE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DBI_TIMER_VALUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBI_TIMER_VALUE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_timer_value` writer - "]
pub struct DBI_TIMER_VALUE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_TIMER_VALUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - DBI Timer Enable"]
    #[inline(always)]
    pub fn dbi_tm_en(&self) -> DBI_TM_EN_R {
        DBI_TM_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn dbi_timer_value(&self) -> DBI_TIMER_VALUE_R {
        DBI_TIMER_VALUE_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 31 - DBI Timer Enable"]
    #[inline(always)]
    pub fn dbi_tm_en(&mut self) -> DBI_TM_EN_W {
        DBI_TM_EN_W { w: self }
    }
    #[doc = "Bits 0:30"]
    #[inline(always)]
    pub fn dbi_timer_value(&mut self) -> DBI_TIMER_VALUE_W {
        DBI_TIMER_VALUE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBI Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_timer](index.html) module"]
pub struct DBI_TIMER_SPEC;
impl crate::RegisterSpec for DBI_TIMER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbi_timer::R](R) reader structure"]
impl crate::Readable for DBI_TIMER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbi_timer::W](W) writer structure"]
impl crate::Writable for DBI_TIMER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBI_TIMER to value 0"]
impl crate::Resettable for DBI_TIMER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
