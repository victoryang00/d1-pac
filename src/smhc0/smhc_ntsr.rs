#[doc = "Register `SMHC_NTSR` reader"]
pub struct R(crate::R<SMHC_NTSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMHC_NTSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMHC_NTSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMHC_NTSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMHC_NTSR` writer"]
pub struct W(crate::W<SMHC_NTSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMHC_NTSR_SPEC>;
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
impl From<crate::W<SMHC_NTSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMHC_NTSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_SELECT_A {
    #[doc = "0: Old mode of Sample/Output Timing"]
    OLD_MODE = 0,
    #[doc = "1: New mode of Sample/Output Timing"]
    NEW_MODE = 1,
}
impl From<MODE_SELECT_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_SELECT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE_SELECT` reader - "]
pub struct MODE_SELECT_R(crate::FieldReader<bool, MODE_SELECT_A>);
impl MODE_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MODE_SELECT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_SELECT_A {
        match self.bits {
            false => MODE_SELECT_A::OLD_MODE,
            true => MODE_SELECT_A::NEW_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `OLD_MODE`"]
    #[inline(always)]
    pub fn is_old_mode(&self) -> bool {
        **self == MODE_SELECT_A::OLD_MODE
    }
    #[doc = "Checks if the value of the field is `NEW_MODE`"]
    #[inline(always)]
    pub fn is_new_mode(&self) -> bool {
        **self == MODE_SELECT_A::NEW_MODE
    }
}
impl core::ops::Deref for MODE_SELECT_R {
    type Target = crate::FieldReader<bool, MODE_SELECT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE_SELECT` writer - "]
pub struct MODE_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_SELECT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_SELECT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Old mode of Sample/Output Timing"]
    #[inline(always)]
    pub fn old_mode(self) -> &'a mut W {
        self.variant(MODE_SELECT_A::OLD_MODE)
    }
    #[doc = "New mode of Sample/Output Timing"]
    #[inline(always)]
    pub fn new_mode(self) -> &'a mut W {
        self.variant(MODE_SELECT_A::NEW_MODE)
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
#[doc = "Clear the input phase of command lines and data lines during the update clock operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_DAT_RX_PHASE_CLR_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CMD_DAT_RX_PHASE_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_DAT_RX_PHASE_CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_DAT_RX_PHASE_CLR` reader - Clear the input phase of command lines and data lines during the update clock operation"]
pub struct CMD_DAT_RX_PHASE_CLR_R(crate::FieldReader<bool, CMD_DAT_RX_PHASE_CLR_A>);
impl CMD_DAT_RX_PHASE_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMD_DAT_RX_PHASE_CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_DAT_RX_PHASE_CLR_A {
        match self.bits {
            false => CMD_DAT_RX_PHASE_CLR_A::DISABLED,
            true => CMD_DAT_RX_PHASE_CLR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CMD_DAT_RX_PHASE_CLR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMD_DAT_RX_PHASE_CLR_A::ENABLED
    }
}
impl core::ops::Deref for CMD_DAT_RX_PHASE_CLR_R {
    type Target = crate::FieldReader<bool, CMD_DAT_RX_PHASE_CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_DAT_RX_PHASE_CLR` writer - Clear the input phase of command lines and data lines during the update clock operation"]
pub struct CMD_DAT_RX_PHASE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_DAT_RX_PHASE_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_DAT_RX_PHASE_CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMD_DAT_RX_PHASE_CLR_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMD_DAT_RX_PHASE_CLR_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Clear the input phase of data lines before receiving the CRC status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAT_CRC_STATUS_RX_PHASE_CLR_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DAT_CRC_STATUS_RX_PHASE_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: DAT_CRC_STATUS_RX_PHASE_CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAT_CRC_STATUS_RX_PHASE_CLR` reader - Clear the input phase of data lines before receiving the CRC status"]
pub struct DAT_CRC_STATUS_RX_PHASE_CLR_R(crate::FieldReader<bool, DAT_CRC_STATUS_RX_PHASE_CLR_A>);
impl DAT_CRC_STATUS_RX_PHASE_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAT_CRC_STATUS_RX_PHASE_CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAT_CRC_STATUS_RX_PHASE_CLR_A {
        match self.bits {
            false => DAT_CRC_STATUS_RX_PHASE_CLR_A::DISABLED,
            true => DAT_CRC_STATUS_RX_PHASE_CLR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DAT_CRC_STATUS_RX_PHASE_CLR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DAT_CRC_STATUS_RX_PHASE_CLR_A::ENABLED
    }
}
impl core::ops::Deref for DAT_CRC_STATUS_RX_PHASE_CLR_R {
    type Target = crate::FieldReader<bool, DAT_CRC_STATUS_RX_PHASE_CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT_CRC_STATUS_RX_PHASE_CLR` writer - Clear the input phase of data lines before receiving the CRC status"]
pub struct DAT_CRC_STATUS_RX_PHASE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_CRC_STATUS_RX_PHASE_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAT_CRC_STATUS_RX_PHASE_CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAT_CRC_STATUS_RX_PHASE_CLR_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAT_CRC_STATUS_RX_PHASE_CLR_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Clear the input phase of data lines before transferring the data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAT_TRANS_RX_PHASE_CLR_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DAT_TRANS_RX_PHASE_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: DAT_TRANS_RX_PHASE_CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAT_TRANS_RX_PHASE_CLR` reader - Clear the input phase of data lines before transferring the data"]
pub struct DAT_TRANS_RX_PHASE_CLR_R(crate::FieldReader<bool, DAT_TRANS_RX_PHASE_CLR_A>);
impl DAT_TRANS_RX_PHASE_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAT_TRANS_RX_PHASE_CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAT_TRANS_RX_PHASE_CLR_A {
        match self.bits {
            false => DAT_TRANS_RX_PHASE_CLR_A::DISABLED,
            true => DAT_TRANS_RX_PHASE_CLR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DAT_TRANS_RX_PHASE_CLR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DAT_TRANS_RX_PHASE_CLR_A::ENABLED
    }
}
impl core::ops::Deref for DAT_TRANS_RX_PHASE_CLR_R {
    type Target = crate::FieldReader<bool, DAT_TRANS_RX_PHASE_CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT_TRANS_RX_PHASE_CLR` writer - Clear the input phase of data lines before transferring the data"]
pub struct DAT_TRANS_RX_PHASE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_TRANS_RX_PHASE_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAT_TRANS_RX_PHASE_CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAT_TRANS_RX_PHASE_CLR_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAT_TRANS_RX_PHASE_CLR_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Clear the input phase of data lines before receiving the data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAT_RECV_RX_PHASE_CLR_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DAT_RECV_RX_PHASE_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: DAT_RECV_RX_PHASE_CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAT_RECV_RX_PHASE_CLR` reader - Clear the input phase of data lines before receiving the data"]
pub struct DAT_RECV_RX_PHASE_CLR_R(crate::FieldReader<bool, DAT_RECV_RX_PHASE_CLR_A>);
impl DAT_RECV_RX_PHASE_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAT_RECV_RX_PHASE_CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAT_RECV_RX_PHASE_CLR_A {
        match self.bits {
            false => DAT_RECV_RX_PHASE_CLR_A::DISABLED,
            true => DAT_RECV_RX_PHASE_CLR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DAT_RECV_RX_PHASE_CLR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DAT_RECV_RX_PHASE_CLR_A::ENABLED
    }
}
impl core::ops::Deref for DAT_RECV_RX_PHASE_CLR_R {
    type Target = crate::FieldReader<bool, DAT_RECV_RX_PHASE_CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT_RECV_RX_PHASE_CLR` writer - Clear the input phase of data lines before receiving the data"]
pub struct DAT_RECV_RX_PHASE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_RECV_RX_PHASE_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAT_RECV_RX_PHASE_CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DAT_RECV_RX_PHASE_CLR_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DAT_RECV_RX_PHASE_CLR_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Clear command rx phase before sending the command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMD_SEND_RX_PHASE_CLR_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<CMD_SEND_RX_PHASE_CLR_A> for bool {
    #[inline(always)]
    fn from(variant: CMD_SEND_RX_PHASE_CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMD_SEND_RX_PHASE_CLR` reader - Clear command rx phase before sending the command"]
pub struct CMD_SEND_RX_PHASE_CLR_R(crate::FieldReader<bool, CMD_SEND_RX_PHASE_CLR_A>);
impl CMD_SEND_RX_PHASE_CLR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMD_SEND_RX_PHASE_CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_SEND_RX_PHASE_CLR_A {
        match self.bits {
            false => CMD_SEND_RX_PHASE_CLR_A::DISABLED,
            true => CMD_SEND_RX_PHASE_CLR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CMD_SEND_RX_PHASE_CLR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CMD_SEND_RX_PHASE_CLR_A::ENABLED
    }
}
impl core::ops::Deref for CMD_SEND_RX_PHASE_CLR_R {
    type Target = crate::FieldReader<bool, CMD_SEND_RX_PHASE_CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_SEND_RX_PHASE_CLR` writer - Clear command rx phase before sending the command"]
pub struct CMD_SEND_RX_PHASE_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_SEND_RX_PHASE_CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_SEND_RX_PHASE_CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMD_SEND_RX_PHASE_CLR_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMD_SEND_RX_PHASE_CLR_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAT_SAMPLE_TIMING_PHASE_A {
    #[doc = "0: Sample timing phase offset 90"]
    O90 = 0,
    #[doc = "1: Sample timing phase offset 180"]
    O180 = 1,
    #[doc = "2: Sample timing phase offset 270"]
    O270 = 2,
    #[doc = "3: Sample timing phase offset 0 (only for SD2 hs400 mode)"]
    O0 = 3,
}
impl From<DAT_SAMPLE_TIMING_PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: DAT_SAMPLE_TIMING_PHASE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DAT_SAMPLE_TIMING_PHASE` reader - "]
pub struct DAT_SAMPLE_TIMING_PHASE_R(crate::FieldReader<u8, DAT_SAMPLE_TIMING_PHASE_A>);
impl DAT_SAMPLE_TIMING_PHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAT_SAMPLE_TIMING_PHASE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAT_SAMPLE_TIMING_PHASE_A {
        match self.bits {
            0 => DAT_SAMPLE_TIMING_PHASE_A::O90,
            1 => DAT_SAMPLE_TIMING_PHASE_A::O180,
            2 => DAT_SAMPLE_TIMING_PHASE_A::O270,
            3 => DAT_SAMPLE_TIMING_PHASE_A::O0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `O90`"]
    #[inline(always)]
    pub fn is_o90(&self) -> bool {
        **self == DAT_SAMPLE_TIMING_PHASE_A::O90
    }
    #[doc = "Checks if the value of the field is `O180`"]
    #[inline(always)]
    pub fn is_o180(&self) -> bool {
        **self == DAT_SAMPLE_TIMING_PHASE_A::O180
    }
    #[doc = "Checks if the value of the field is `O270`"]
    #[inline(always)]
    pub fn is_o270(&self) -> bool {
        **self == DAT_SAMPLE_TIMING_PHASE_A::O270
    }
    #[doc = "Checks if the value of the field is `O0`"]
    #[inline(always)]
    pub fn is_o0(&self) -> bool {
        **self == DAT_SAMPLE_TIMING_PHASE_A::O0
    }
}
impl core::ops::Deref for DAT_SAMPLE_TIMING_PHASE_R {
    type Target = crate::FieldReader<u8, DAT_SAMPLE_TIMING_PHASE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAT_SAMPLE_TIMING_PHASE` writer - "]
pub struct DAT_SAMPLE_TIMING_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_SAMPLE_TIMING_PHASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAT_SAMPLE_TIMING_PHASE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Sample timing phase offset 90"]
    #[inline(always)]
    pub fn o90(self) -> &'a mut W {
        self.variant(DAT_SAMPLE_TIMING_PHASE_A::O90)
    }
    #[doc = "Sample timing phase offset 180"]
    #[inline(always)]
    pub fn o180(self) -> &'a mut W {
        self.variant(DAT_SAMPLE_TIMING_PHASE_A::O180)
    }
    #[doc = "Sample timing phase offset 270"]
    #[inline(always)]
    pub fn o270(self) -> &'a mut W {
        self.variant(DAT_SAMPLE_TIMING_PHASE_A::O270)
    }
    #[doc = "Sample timing phase offset 0 (only for SD2 hs400 mode)"]
    #[inline(always)]
    pub fn o0(self) -> &'a mut W {
        self.variant(DAT_SAMPLE_TIMING_PHASE_A::O0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_SAMPLE_TIMING_PHASE_A {
    #[doc = "0: Sample timing phase offset 90"]
    O90 = 0,
    #[doc = "1: Sample timing phase offset 180"]
    O180 = 1,
    #[doc = "2: Sample timing phase offset 270"]
    O270 = 2,
    #[doc = "3: Ignore"]
    O0 = 3,
}
impl From<CMD_SAMPLE_TIMING_PHASE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_SAMPLE_TIMING_PHASE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD_SAMPLE_TIMING_PHASE` reader - "]
pub struct CMD_SAMPLE_TIMING_PHASE_R(crate::FieldReader<u8, CMD_SAMPLE_TIMING_PHASE_A>);
impl CMD_SAMPLE_TIMING_PHASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CMD_SAMPLE_TIMING_PHASE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD_SAMPLE_TIMING_PHASE_A {
        match self.bits {
            0 => CMD_SAMPLE_TIMING_PHASE_A::O90,
            1 => CMD_SAMPLE_TIMING_PHASE_A::O180,
            2 => CMD_SAMPLE_TIMING_PHASE_A::O270,
            3 => CMD_SAMPLE_TIMING_PHASE_A::O0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `O90`"]
    #[inline(always)]
    pub fn is_o90(&self) -> bool {
        **self == CMD_SAMPLE_TIMING_PHASE_A::O90
    }
    #[doc = "Checks if the value of the field is `O180`"]
    #[inline(always)]
    pub fn is_o180(&self) -> bool {
        **self == CMD_SAMPLE_TIMING_PHASE_A::O180
    }
    #[doc = "Checks if the value of the field is `O270`"]
    #[inline(always)]
    pub fn is_o270(&self) -> bool {
        **self == CMD_SAMPLE_TIMING_PHASE_A::O270
    }
    #[doc = "Checks if the value of the field is `O0`"]
    #[inline(always)]
    pub fn is_o0(&self) -> bool {
        **self == CMD_SAMPLE_TIMING_PHASE_A::O0
    }
}
impl core::ops::Deref for CMD_SAMPLE_TIMING_PHASE_R {
    type Target = crate::FieldReader<u8, CMD_SAMPLE_TIMING_PHASE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD_SAMPLE_TIMING_PHASE` writer - "]
pub struct CMD_SAMPLE_TIMING_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_SAMPLE_TIMING_PHASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_SAMPLE_TIMING_PHASE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Sample timing phase offset 90"]
    #[inline(always)]
    pub fn o90(self) -> &'a mut W {
        self.variant(CMD_SAMPLE_TIMING_PHASE_A::O90)
    }
    #[doc = "Sample timing phase offset 180"]
    #[inline(always)]
    pub fn o180(self) -> &'a mut W {
        self.variant(CMD_SAMPLE_TIMING_PHASE_A::O180)
    }
    #[doc = "Sample timing phase offset 270"]
    #[inline(always)]
    pub fn o270(self) -> &'a mut W {
        self.variant(CMD_SAMPLE_TIMING_PHASE_A::O270)
    }
    #[doc = "Ignore"]
    #[inline(always)]
    pub fn o0(self) -> &'a mut W {
        self.variant(CMD_SAMPLE_TIMING_PHASE_A::O0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HS400_NEW_SAMPLE_EN_A {
    #[doc = "0: Disable hs400 new sample method"]
    DISABLE = 0,
    #[doc = "1: Enable hs400 new sample method"]
    ENABLE = 1,
}
impl From<HS400_NEW_SAMPLE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HS400_NEW_SAMPLE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS400_NEW_SAMPLE_EN` reader - "]
pub struct HS400_NEW_SAMPLE_EN_R(crate::FieldReader<bool, HS400_NEW_SAMPLE_EN_A>);
impl HS400_NEW_SAMPLE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        HS400_NEW_SAMPLE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HS400_NEW_SAMPLE_EN_A {
        match self.bits {
            false => HS400_NEW_SAMPLE_EN_A::DISABLE,
            true => HS400_NEW_SAMPLE_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == HS400_NEW_SAMPLE_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == HS400_NEW_SAMPLE_EN_A::ENABLE
    }
}
impl core::ops::Deref for HS400_NEW_SAMPLE_EN_R {
    type Target = crate::FieldReader<bool, HS400_NEW_SAMPLE_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HS400_NEW_SAMPLE_EN` writer - "]
pub struct HS400_NEW_SAMPLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HS400_NEW_SAMPLE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HS400_NEW_SAMPLE_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable hs400 new sample method"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(HS400_NEW_SAMPLE_EN_A::DISABLE)
    }
    #[doc = "Enable hs400 new sample method"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(HS400_NEW_SAMPLE_EN_A::ENABLE)
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
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn mode_select(&self) -> MODE_SELECT_R {
        MODE_SELECT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Clear the input phase of command lines and data lines during the update clock operation"]
    #[inline(always)]
    pub fn cmd_dat_rx_phase_clr(&self) -> CMD_DAT_RX_PHASE_CLR_R {
        CMD_DAT_RX_PHASE_CLR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Clear the input phase of data lines before receiving the CRC status"]
    #[inline(always)]
    pub fn dat_crc_status_rx_phase_clr(&self) -> DAT_CRC_STATUS_RX_PHASE_CLR_R {
        DAT_CRC_STATUS_RX_PHASE_CLR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Clear the input phase of data lines before transferring the data"]
    #[inline(always)]
    pub fn dat_trans_rx_phase_clr(&self) -> DAT_TRANS_RX_PHASE_CLR_R {
        DAT_TRANS_RX_PHASE_CLR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Clear the input phase of data lines before receiving the data"]
    #[inline(always)]
    pub fn dat_recv_rx_phase_clr(&self) -> DAT_RECV_RX_PHASE_CLR_R {
        DAT_RECV_RX_PHASE_CLR_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Clear command rx phase before sending the command"]
    #[inline(always)]
    pub fn cmd_send_rx_phase_clr(&self) -> CMD_SEND_RX_PHASE_CLR_R {
        CMD_SEND_RX_PHASE_CLR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dat_sample_timing_phase(&self) -> DAT_SAMPLE_TIMING_PHASE_R {
        DAT_SAMPLE_TIMING_PHASE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cmd_sample_timing_phase(&self) -> CMD_SAMPLE_TIMING_PHASE_R {
        CMD_SAMPLE_TIMING_PHASE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hs400_new_sample_en(&self) -> HS400_NEW_SAMPLE_EN_R {
        HS400_NEW_SAMPLE_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn mode_select(&mut self) -> MODE_SELECT_W {
        MODE_SELECT_W { w: self }
    }
    #[doc = "Bit 24 - Clear the input phase of command lines and data lines during the update clock operation"]
    #[inline(always)]
    pub fn cmd_dat_rx_phase_clr(&mut self) -> CMD_DAT_RX_PHASE_CLR_W {
        CMD_DAT_RX_PHASE_CLR_W { w: self }
    }
    #[doc = "Bit 22 - Clear the input phase of data lines before receiving the CRC status"]
    #[inline(always)]
    pub fn dat_crc_status_rx_phase_clr(&mut self) -> DAT_CRC_STATUS_RX_PHASE_CLR_W {
        DAT_CRC_STATUS_RX_PHASE_CLR_W { w: self }
    }
    #[doc = "Bit 21 - Clear the input phase of data lines before transferring the data"]
    #[inline(always)]
    pub fn dat_trans_rx_phase_clr(&mut self) -> DAT_TRANS_RX_PHASE_CLR_W {
        DAT_TRANS_RX_PHASE_CLR_W { w: self }
    }
    #[doc = "Bit 20 - Clear the input phase of data lines before receiving the data"]
    #[inline(always)]
    pub fn dat_recv_rx_phase_clr(&mut self) -> DAT_RECV_RX_PHASE_CLR_W {
        DAT_RECV_RX_PHASE_CLR_W { w: self }
    }
    #[doc = "Bit 16 - Clear command rx phase before sending the command"]
    #[inline(always)]
    pub fn cmd_send_rx_phase_clr(&mut self) -> CMD_SEND_RX_PHASE_CLR_W {
        CMD_SEND_RX_PHASE_CLR_W { w: self }
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn dat_sample_timing_phase(&mut self) -> DAT_SAMPLE_TIMING_PHASE_W {
        DAT_SAMPLE_TIMING_PHASE_W { w: self }
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cmd_sample_timing_phase(&mut self) -> CMD_SAMPLE_TIMING_PHASE_W {
        CMD_SAMPLE_TIMING_PHASE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn hs400_new_sample_en(&mut self) -> HS400_NEW_SAMPLE_EN_W {
        HS400_NEW_SAMPLE_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SD New Timing Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smhc_ntsr](index.html) module"]
pub struct SMHC_NTSR_SPEC;
impl crate::RegisterSpec for SMHC_NTSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smhc_ntsr::R](R) reader structure"]
impl crate::Readable for SMHC_NTSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smhc_ntsr::W](W) writer structure"]
impl crate::Writable for SMHC_NTSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SMHC_NTSR to value 0"]
impl crate::Resettable for SMHC_NTSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
