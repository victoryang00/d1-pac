#[doc = "Register `SMHC_FUNS` reader"]
pub struct R(crate::R<SMHC_FUNS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_FUNS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_FUNS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_FUNS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_FUNS` writer"]
pub struct W(crate::W<SMHC_FUNS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_FUNS_SPEC>;
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
impl From<crate::W<SMHC_FUNS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_FUNS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Abort Read Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ABT_RDATA_A {
    #[doc = "0: `0`"]
    IGNORED = 0,
    #[doc = "1: `1`"]
    ABORT = 1,
}
impl From<ABT_RDATA_A> for bool {
    #[inline(always)]
    fn from(variant: ABT_RDATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABT_RDATA` reader - Abort Read Data"]
pub struct ABT_RDATA_R(crate::FieldReader<bool, ABT_RDATA_A>);
impl ABT_RDATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ABT_RDATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ABT_RDATA_A {
        match self.bits {
            false => ABT_RDATA_A::IGNORED,
            true => ABT_RDATA_A::ABORT,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORED`"]
    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        **self == ABT_RDATA_A::IGNORED
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        **self == ABT_RDATA_A::ABORT
    }
}
impl core::ops::Deref for ABT_RDATA_R {
    type Target = crate::FieldReader<bool, ABT_RDATA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ABT_RDATA` writer - Abort Read Data"]
pub struct ABT_RDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> ABT_RDATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ABT_RDATA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(ABT_RDATA_A::IGNORED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut W {
        self.variant(ABT_RDATA_A::ABORT)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Read Wait\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READ_WAIT_A {
    #[doc = "0: Clear SDIO read wait"]
    CLEAR = 0,
    #[doc = "1: Assert SDIO read wait"]
    ASSERT = 1,
}
impl From<READ_WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: READ_WAIT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READ_WAIT` reader - Read Wait"]
pub struct READ_WAIT_R(crate::FieldReader<bool, READ_WAIT_A>);
impl READ_WAIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READ_WAIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READ_WAIT_A {
        match self.bits {
            false => READ_WAIT_A::CLEAR,
            true => READ_WAIT_A::ASSERT,
        }
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == READ_WAIT_A::CLEAR
    }
    #[doc = "Checks if the value of the field is `ASSERT`"]
    #[inline(always)]
    pub fn is_assert(&self) -> bool {
        **self == READ_WAIT_A::ASSERT
    }
}
impl core::ops::Deref for READ_WAIT_R {
    type Target = crate::FieldReader<bool, READ_WAIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READ_WAIT` writer - Read Wait"]
pub struct READ_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_WAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READ_WAIT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clear SDIO read wait"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(READ_WAIT_A::CLEAR)
    }
    #[doc = "Assert SDIO read wait"]
    #[inline(always)]
    pub fn assert(self) -> &'a mut W {
        self.variant(READ_WAIT_A::ASSERT)
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
#[doc = "Host Send MMC IRQ Response\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_SEND_MIMC_IRQRESQ_A {
    #[doc = "0: `0`"]
    IGNORED = 0,
    #[doc = "1: Send auto IRQ response"]
    SEND = 1,
}
impl From<HOST_SEND_MIMC_IRQRESQ_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_SEND_MIMC_IRQRESQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST_SEND_MIMC_IRQRESQ` reader - Host Send MMC IRQ Response"]
pub struct HOST_SEND_MIMC_IRQRESQ_R(crate::FieldReader<bool, HOST_SEND_MIMC_IRQRESQ_A>);
impl HOST_SEND_MIMC_IRQRESQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HOST_SEND_MIMC_IRQRESQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOST_SEND_MIMC_IRQRESQ_A {
        match self.bits {
            false => HOST_SEND_MIMC_IRQRESQ_A::IGNORED,
            true => HOST_SEND_MIMC_IRQRESQ_A::SEND,
        }
    }
    #[doc = "Checks if the value of the field is `IGNORED`"]
    #[inline(always)]
    pub fn is_ignored(&self) -> bool {
        **self == HOST_SEND_MIMC_IRQRESQ_A::IGNORED
    }
    #[doc = "Checks if the value of the field is `SEND`"]
    #[inline(always)]
    pub fn is_send(&self) -> bool {
        **self == HOST_SEND_MIMC_IRQRESQ_A::SEND
    }
}
impl core::ops::Deref for HOST_SEND_MIMC_IRQRESQ_R {
    type Target = crate::FieldReader<bool, HOST_SEND_MIMC_IRQRESQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_SEND_MIMC_IRQRESQ` writer - Host Send MMC IRQ Response"]
pub struct HOST_SEND_MIMC_IRQRESQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_SEND_MIMC_IRQRESQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOST_SEND_MIMC_IRQRESQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn ignored(self) -> &'a mut W {
        self.variant(HOST_SEND_MIMC_IRQRESQ_A::IGNORED)
    }
    #[doc = "Send auto IRQ response"]
    #[inline(always)]
    pub fn send(self) -> &'a mut W {
        self.variant(HOST_SEND_MIMC_IRQRESQ_A::SEND)
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
    #[doc = "Bit 2 - Abort Read Data"]
    #[inline(always)]
    pub fn abt_rdata(&self) -> ABT_RDATA_R {
        ABT_RDATA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Read Wait"]
    #[inline(always)]
    pub fn read_wait(&self) -> READ_WAIT_R {
        READ_WAIT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Host Send MMC IRQ Response"]
    #[inline(always)]
    pub fn host_send_mimc_irqresq(&self) -> HOST_SEND_MIMC_IRQRESQ_R {
        HOST_SEND_MIMC_IRQRESQ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Abort Read Data"]
    #[inline(always)]
    pub fn abt_rdata(&mut self) -> ABT_RDATA_W {
        ABT_RDATA_W { w: self }
    }
    #[doc = "Bit 1 - Read Wait"]
    #[inline(always)]
    pub fn read_wait(&mut self) -> READ_WAIT_W {
        READ_WAIT_W { w: self }
    }
    #[doc = "Bit 0 - Host Send MMC IRQ Response"]
    #[inline(always)]
    pub fn host_send_mimc_irqresq(&mut self) -> HOST_SEND_MIMC_IRQRESQ_W {
        HOST_SEND_MIMC_IRQRESQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Function Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_funs](index.html) module"]
pub struct SMHC_FUNS_SPEC;
impl crate::RegisterSpec for SMHC_FUNS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_funs::R](R) reader structure"]
impl crate::Readable for SMHC_FUNS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_funs::W](W) writer structure"]
impl crate::Writable for SMHC_FUNS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_FUNS to value 0"]
impl crate::Resettable for SMHC_FUNS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
