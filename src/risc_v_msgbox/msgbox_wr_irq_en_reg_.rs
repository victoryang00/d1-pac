#[doc = "Register `MSGBOX_WR_IRQ_EN_REG_%s` reader"]
pub struct R(crate::R<MSGBOX_WR_IRQ_EN_REG__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSGBOX_WR_IRQ_EN_REG__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSGBOX_WR_IRQ_EN_REG__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSGBOX_WR_IRQ_EN_REG__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSGBOX_WR_IRQ_EN_REG_%s` writer"]
pub struct W(crate::W<MSGBOX_WR_IRQ_EN_REG__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSGBOX_WR_IRQ_EN_REG__SPEC>;
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
impl From<crate::W<MSGBOX_WR_IRQ_EN_REG__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSGBOX_WR_IRQ_EN_REG__SPEC>) -> Self {
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
#[doc = "MSGBOX Write IRQ Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msgbox_wr_irq_en_reg_](index.html) module"]
pub struct MSGBOX_WR_IRQ_EN_REG__SPEC;
impl crate::RegisterSpec for MSGBOX_WR_IRQ_EN_REG__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msgbox_wr_irq_en_reg_::R](R) reader structure"]
impl crate::Readable for MSGBOX_WR_IRQ_EN_REG__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msgbox_wr_irq_en_reg_::W](W) writer structure"]
impl crate::Writable for MSGBOX_WR_IRQ_EN_REG__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSGBOX_WR_IRQ_EN_REG_%s to value 0"]
impl crate::Resettable for MSGBOX_WR_IRQ_EN_REG__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
