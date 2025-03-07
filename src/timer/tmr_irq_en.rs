#[doc = "Register `tmr_irq_en` reader"]
pub struct R(crate::R<TMR_IRQ_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR_IRQ_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR_IRQ_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR_IRQ_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tmr_irq_en` writer"]
pub struct W(crate::W<TMR_IRQ_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR_IRQ_EN_SPEC>;
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
impl From<crate::W<TMR_IRQ_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR_IRQ_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR1_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<TMR1_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMR1_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tmr1_irq_en` reader - "]
pub struct TMR1_IRQ_EN_R(crate::FieldReader<bool, TMR1_IRQ_EN_A>);
impl TMR1_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TMR1_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR1_IRQ_EN_A {
        match self.bits {
            false => TMR1_IRQ_EN_A::DISABLED,
            true => TMR1_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TMR1_IRQ_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TMR1_IRQ_EN_A::ENABLED
    }
}
impl core::ops::Deref for TMR1_IRQ_EN_R {
    type Target = crate::FieldReader<bool, TMR1_IRQ_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmr1_irq_en` writer - "]
pub struct TMR1_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR1_IRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR1_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TMR1_IRQ_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TMR1_IRQ_EN_A::ENABLED)
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
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR0_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLED = 0,
    #[doc = "1: `1`"]
    ENABLED = 1,
}
impl From<TMR0_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMR0_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tmr0_irq_en` reader - "]
pub struct TMR0_IRQ_EN_R(crate::FieldReader<bool, TMR0_IRQ_EN_A>);
impl TMR0_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TMR0_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR0_IRQ_EN_A {
        match self.bits {
            false => TMR0_IRQ_EN_A::DISABLED,
            true => TMR0_IRQ_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TMR0_IRQ_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TMR0_IRQ_EN_A::ENABLED
    }
}
impl core::ops::Deref for TMR0_IRQ_EN_R {
    type Target = crate::FieldReader<bool, TMR0_IRQ_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmr0_irq_en` writer - "]
pub struct TMR0_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR0_IRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR0_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TMR0_IRQ_EN_A::DISABLED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TMR0_IRQ_EN_A::ENABLED)
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
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmr1_irq_en(&self) -> TMR1_IRQ_EN_R {
        TMR1_IRQ_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmr0_irq_en(&self) -> TMR0_IRQ_EN_R {
        TMR0_IRQ_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmr1_irq_en(&mut self) -> TMR1_IRQ_EN_W {
        TMR1_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmr0_irq_en(&mut self) -> TMR0_IRQ_EN_W {
        TMR0_IRQ_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer IRQ Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr_irq_en](index.html) module"]
pub struct TMR_IRQ_EN_SPEC;
impl crate::RegisterSpec for TMR_IRQ_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr_irq_en::R](R) reader structure"]
impl crate::Readable for TMR_IRQ_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr_irq_en::W](W) writer structure"]
impl crate::Writable for TMR_IRQ_EN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tmr_irq_en to value 0"]
impl crate::Resettable for TMR_IRQ_EN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
