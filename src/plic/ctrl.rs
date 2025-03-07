#[doc = "Register `ctrl` reader"]
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
#[doc = "Register `ctrl` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "PLIC Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTRL_A {
    #[doc = "0: Only the machine mode can access to all registers in PLIC. Supervisor mode can only access the interrupt threshold register and the interrupt response/completion register."]
    M = 0,
    #[doc = "1: The machine mode and the supervisor mode can access all registers. CTRL is accessible only in the machine mode."]
    MS = 1,
}
impl From<CTRL_A> for bool {
    #[inline(always)]
    fn from(variant: CTRL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ctrl` reader - PLIC Control"]
pub struct CTRL_R(crate::FieldReader<bool, CTRL_A>);
impl CTRL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTRL_A {
        match self.bits {
            false => CTRL_A::M,
            true => CTRL_A::MS,
        }
    }
    #[doc = "Checks if the value of the field is `M`"]
    #[inline(always)]
    pub fn is_m(&self) -> bool {
        **self == CTRL_A::M
    }
    #[doc = "Checks if the value of the field is `MS`"]
    #[inline(always)]
    pub fn is_ms(&self) -> bool {
        **self == CTRL_A::MS
    }
}
impl core::ops::Deref for CTRL_R {
    type Target = crate::FieldReader<bool, CTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ctrl` writer - PLIC Control"]
pub struct CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTRL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Only the machine mode can access to all registers in PLIC. Supervisor mode can only access the interrupt threshold register and the interrupt response/completion register."]
    #[inline(always)]
    pub fn m(self) -> &'a mut W {
        self.variant(CTRL_A::M)
    }
    #[doc = "The machine mode and the supervisor mode can access all registers. CTRL is accessible only in the machine mode."]
    #[inline(always)]
    pub fn ms(self) -> &'a mut W {
        self.variant(CTRL_A::MS)
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
    #[doc = "Bit 0 - PLIC Control"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLIC Control"]
    #[inline(always)]
    pub fn ctrl(&mut self) -> CTRL_W {
        CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ctrl to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
