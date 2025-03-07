#[doc = "Register `CE_ICR` reader"]
pub struct R(crate::R<CE_ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CE_ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CE_ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CE_ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CE_ICR` writer"]
pub struct W(crate::W<CE_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CE_ICR_SPEC>;
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
impl From<crate::W<CE_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CE_ICR_SPEC>) -> Self {
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
#[doc = "Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ce_icr](index.html) module"]
pub struct CE_ICR_SPEC;
impl crate::RegisterSpec for CE_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ce_icr::R](R) reader structure"]
impl crate::Readable for CE_ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ce_icr::W](W) writer structure"]
impl crate::Writable for CE_ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CE_ICR to value 0"]
impl crate::Resettable for CE_ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
