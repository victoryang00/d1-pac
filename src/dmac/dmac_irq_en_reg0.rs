#[doc = "Register `DMAC_IRQ_EN_REG0` reader"]
pub struct R(crate::R<DMAC_IRQ_EN_REG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAC_IRQ_EN_REG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAC_IRQ_EN_REG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAC_IRQ_EN_REG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAC_IRQ_EN_REG0` writer"]
pub struct W(crate::W<DMAC_IRQ_EN_REG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAC_IRQ_EN_REG0_SPEC>;
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
impl From<crate::W<DMAC_IRQ_EN_REG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAC_IRQ_EN_REG0_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC IRQ Enable Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmac_irq_en_reg0](index.html) module"]
pub struct DMAC_IRQ_EN_REG0_SPEC;
impl crate::RegisterSpec for DMAC_IRQ_EN_REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmac_irq_en_reg0::R](R) reader structure"]
impl crate::Readable for DMAC_IRQ_EN_REG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmac_irq_en_reg0::W](W) writer structure"]
impl crate::Writable for DMAC_IRQ_EN_REG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAC_IRQ_EN_REG0 to value 0"]
impl crate::Resettable for DMAC_IRQ_EN_REG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
