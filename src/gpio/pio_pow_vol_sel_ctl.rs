#[doc = "Register `pio_pow_vol_sel_ctl` reader"]
pub struct R(crate::R<PIO_POW_VOL_SEL_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIO_POW_VOL_SEL_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIO_POW_VOL_SEL_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIO_POW_VOL_SEL_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pio_pow_vol_sel_ctl` writer"]
pub struct W(crate::W<PIO_POW_VOL_SEL_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIO_POW_VOL_SEL_CTL_SPEC>;
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
impl From<crate::W<PIO_POW_VOL_SEL_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIO_POW_VOL_SEL_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "VCC_PF Power Voltage Select Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VCC_PF_PWR_VOL_SEL_A {
    #[doc = "0: 1.8V"]
    V18 = 0,
    #[doc = "1: 3.3V"]
    V33 = 1,
}
impl From<VCC_PF_PWR_VOL_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: VCC_PF_PWR_VOL_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VCC_PF_PWR_VOL_SEL` reader - VCC_PF Power Voltage Select Control"]
pub struct VCC_PF_PWR_VOL_SEL_R(crate::FieldReader<bool, VCC_PF_PWR_VOL_SEL_A>);
impl VCC_PF_PWR_VOL_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VCC_PF_PWR_VOL_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCC_PF_PWR_VOL_SEL_A {
        match self.bits {
            false => VCC_PF_PWR_VOL_SEL_A::V18,
            true => VCC_PF_PWR_VOL_SEL_A::V33,
        }
    }
    #[doc = "Checks if the value of the field is `V18`"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        **self == VCC_PF_PWR_VOL_SEL_A::V18
    }
    #[doc = "Checks if the value of the field is `V33`"]
    #[inline(always)]
    pub fn is_v33(&self) -> bool {
        **self == VCC_PF_PWR_VOL_SEL_A::V33
    }
}
impl core::ops::Deref for VCC_PF_PWR_VOL_SEL_R {
    type Target = crate::FieldReader<bool, VCC_PF_PWR_VOL_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCC_PF_PWR_VOL_SEL` writer - VCC_PF Power Voltage Select Control"]
pub struct VCC_PF_PWR_VOL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VCC_PF_PWR_VOL_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCC_PF_PWR_VOL_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "1.8V"]
    #[inline(always)]
    pub fn v18(self) -> &'a mut W {
        self.variant(VCC_PF_PWR_VOL_SEL_A::V18)
    }
    #[doc = "3.3V"]
    #[inline(always)]
    pub fn v33(self) -> &'a mut W {
        self.variant(VCC_PF_PWR_VOL_SEL_A::V33)
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
    #[doc = "Bit 0 - VCC_PF Power Voltage Select Control"]
    #[inline(always)]
    pub fn vcc_pf_pwr_vol_sel(&self) -> VCC_PF_PWR_VOL_SEL_R {
        VCC_PF_PWR_VOL_SEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VCC_PF Power Voltage Select Control"]
    #[inline(always)]
    pub fn vcc_pf_pwr_vol_sel(&mut self) -> VCC_PF_PWR_VOL_SEL_W {
        VCC_PF_PWR_VOL_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PIO Group Power Voltage Select Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pio_pow_vol_sel_ctl](index.html) module"]
pub struct PIO_POW_VOL_SEL_CTL_SPEC;
impl crate::RegisterSpec for PIO_POW_VOL_SEL_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pio_pow_vol_sel_ctl::R](R) reader structure"]
impl crate::Readable for PIO_POW_VOL_SEL_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pio_pow_vol_sel_ctl::W](W) writer structure"]
impl crate::Writable for PIO_POW_VOL_SEL_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets pio_pow_vol_sel_ctl to value 0"]
impl crate::Resettable for PIO_POW_VOL_SEL_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
