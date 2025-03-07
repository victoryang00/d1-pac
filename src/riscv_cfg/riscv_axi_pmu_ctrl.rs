#[doc = "Register `RISCV_AXI_PMU_CTRL` reader"]
pub struct R(crate::R<RISCV_AXI_PMU_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISCV_AXI_PMU_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISCV_AXI_PMU_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISCV_AXI_PMU_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RISCV_AXI_PMU_CTRL` writer"]
pub struct W(crate::W<RISCV_AXI_PMU_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RISCV_AXI_PMU_CTRL_SPEC>;
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
impl From<crate::W<RISCV_AXI_PMU_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RISCV_AXI_PMU_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PMU Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMU_CLR_A {
    #[doc = "0: `0`"]
    NO_OPERATION = 0,
    #[doc = "1: `1`"]
    CLEARED = 1,
}
impl From<PMU_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: PMU_CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMU_CLR` reader - PMU Clear"]
pub struct PMU_CLR_R(crate::FieldReader<bool, PMU_CLR_A>);
impl PMU_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMU_CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMU_CLR_A {
        match self.bits {
            false => PMU_CLR_A::NO_OPERATION,
            true => PMU_CLR_A::CLEARED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OPERATION`"]
    #[inline(always)]
    pub fn is_no_operation(&self) -> bool {
        **self == PMU_CLR_A::NO_OPERATION
    }
    #[doc = "Checks if the value of the field is `CLEARED`"]
    #[inline(always)]
    pub fn is_cleared(&self) -> bool {
        **self == PMU_CLR_A::CLEARED
    }
}
impl core::ops::Deref for PMU_CLR_R {
    type Target = crate::FieldReader<bool, PMU_CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMU_CLR` writer - PMU Clear"]
pub struct PMU_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> PMU_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMU_CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_operation(self) -> &'a mut W {
        self.variant(PMU_CLR_A::NO_OPERATION)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn cleared(self) -> &'a mut W {
        self.variant(PMU_CLR_A::CLEARED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "PMU Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMU_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<PMU_EN_A> for bool {
    #[inline(always)]
    fn from(variant: PMU_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMU_EN` reader - PMU Enable"]
pub struct PMU_EN_R(crate::FieldReader<bool, PMU_EN_A>);
impl PMU_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMU_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMU_EN_A {
        match self.bits {
            false => PMU_EN_A::DISABLED,
            true => PMU_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == PMU_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == PMU_EN_A::ENABLED
    }
}
impl core::ops::Deref for PMU_EN_R {
    type Target = crate::FieldReader<bool, PMU_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMU_EN` writer - PMU Enable"]
pub struct PMU_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PMU_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMU_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PMU_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PMU_EN_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - PMU Clear"]
    #[inline(always)]
    pub fn pmu_clr(&self) -> PMU_CLR_R {
        PMU_CLR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - PMU Enable"]
    #[inline(always)]
    pub fn pmu_en(&self) -> PMU_EN_R {
        PMU_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PMU Clear"]
    #[inline(always)]
    pub fn pmu_clr(&mut self) -> PMU_CLR_W {
        PMU_CLR_W { w: self }
    }
    #[doc = "Bit 0 - PMU Enable"]
    #[inline(always)]
    pub fn pmu_en(&mut self) -> PMU_EN_W {
        PMU_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RISCV AXI PMU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [riscv_axi_pmu_ctrl](index.html) module"]
pub struct RISCV_AXI_PMU_CTRL_SPEC;
impl crate::RegisterSpec for RISCV_AXI_PMU_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [riscv_axi_pmu_ctrl::R](R) reader structure"]
impl crate::Readable for RISCV_AXI_PMU_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [riscv_axi_pmu_ctrl::W](W) writer structure"]
impl crate::Writable for RISCV_AXI_PMU_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RISCV_AXI_PMU_CTRL to value 0"]
impl crate::Resettable for RISCV_AXI_PMU_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
