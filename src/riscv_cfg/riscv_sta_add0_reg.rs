#[doc = "Register `RISCV_STA_ADD0_REG` reader"]
pub struct R(crate::R<RISCV_STA_ADD0_REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RISCV_STA_ADD0_REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RISCV_STA_ADD0_REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RISCV_STA_ADD0_REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RISCV_STA_ADD0_REG` writer"]
pub struct W(crate::W<RISCV_STA_ADD0_REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RISCV_STA_ADD0_REG_SPEC>;
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
impl From<crate::W<RISCV_STA_ADD0_REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RISCV_STA_ADD0_REG_SPEC>) -> Self {
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
#[doc = "RISCV Start Address0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [riscv_sta_add0_reg](index.html) module"]
pub struct RISCV_STA_ADD0_REG_SPEC;
impl crate::RegisterSpec for RISCV_STA_ADD0_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [riscv_sta_add0_reg::R](R) reader structure"]
impl crate::Readable for RISCV_STA_ADD0_REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [riscv_sta_add0_reg::W](W) writer structure"]
impl crate::Writable for RISCV_STA_ADD0_REG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RISCV_STA_ADD0_REG to value 0"]
impl crate::Resettable for RISCV_STA_ADD0_REG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
