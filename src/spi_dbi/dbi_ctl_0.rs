#[doc = "Register `DBI_CTL_0` reader"]
pub struct R(crate::R<DBI_CTL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBI_CTL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBI_CTL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBI_CTL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBI_CTL_0` writer"]
pub struct W(crate::W<DBI_CTL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBI_CTL_0_SPEC>;
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
impl From<crate::W<DBI_CTL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBI_CTL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDT_A {
    #[doc = "0: `0`"]
    WRITE = 0,
    #[doc = "1: `1`"]
    READ = 1,
}
impl From<CMDT_A> for bool {
    #[inline(always)]
    fn from(variant: CMDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cmdt` reader - Command Type"]
pub struct CMDT_R(crate::FieldReader<bool, CMDT_A>);
impl CMDT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMDT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDT_A {
        match self.bits {
            false => CMDT_A::WRITE,
            true => CMDT_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        **self == CMDT_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == CMDT_A::READ
    }
}
impl core::ops::Deref for CMDT_R {
    type Target = crate::FieldReader<bool, CMDT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmdt` writer - Command Type"]
pub struct CMDT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(CMDT_A::WRITE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(CMDT_A::READ)
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
#[doc = "Field `wcdc` reader - Write Command Dummy Cycles"]
pub struct WCDC_R(crate::FieldReader<u16, u16>);
impl WCDC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        WCDC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WCDC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wcdc` writer - Write Command Dummy Cycles"]
pub struct WCDC_W<'a> {
    w: &'a mut W,
}
impl<'a> WCDC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 20)) | ((value as u32 & 0x07ff) << 20);
        self.w
    }
}
#[doc = "Output Data Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAT_SEQ_A {
    #[doc = "0: `0`"]
    MSB = 0,
    #[doc = "1: `1`"]
    LSB = 1,
}
impl From<DAT_SEQ_A> for bool {
    #[inline(always)]
    fn from(variant: DAT_SEQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dat_seq` reader - Output Data Sequence"]
pub struct DAT_SEQ_R(crate::FieldReader<bool, DAT_SEQ_A>);
impl DAT_SEQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DAT_SEQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAT_SEQ_A {
        match self.bits {
            false => DAT_SEQ_A::MSB,
            true => DAT_SEQ_A::LSB,
        }
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        **self == DAT_SEQ_A::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        **self == DAT_SEQ_A::LSB
    }
}
impl core::ops::Deref for DAT_SEQ_R {
    type Target = crate::FieldReader<bool, DAT_SEQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dat_seq` writer - Output Data Sequence"]
pub struct DAT_SEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_SEQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAT_SEQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(DAT_SEQ_A::MSB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(DAT_SEQ_A::LSB)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Output RGB Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RGB_SEQ_A {
    #[doc = "0: `0`"]
    RGB = 0,
    #[doc = "1: `1`"]
    RBG = 1,
    #[doc = "2: `10`"]
    GRB = 2,
    #[doc = "3: `11`"]
    GBR = 3,
    #[doc = "4: `100`"]
    BRG = 4,
    #[doc = "5: `101`"]
    BGR = 5,
}
impl From<RGB_SEQ_A> for u8 {
    #[inline(always)]
    fn from(variant: RGB_SEQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `rgb_seq` reader - Output RGB Sequence"]
pub struct RGB_SEQ_R(crate::FieldReader<u8, RGB_SEQ_A>);
impl RGB_SEQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RGB_SEQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RGB_SEQ_A> {
        match self.bits {
            0 => Some(RGB_SEQ_A::RGB),
            1 => Some(RGB_SEQ_A::RBG),
            2 => Some(RGB_SEQ_A::GRB),
            3 => Some(RGB_SEQ_A::GBR),
            4 => Some(RGB_SEQ_A::BRG),
            5 => Some(RGB_SEQ_A::BGR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RGB`"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        **self == RGB_SEQ_A::RGB
    }
    #[doc = "Checks if the value of the field is `RBG`"]
    #[inline(always)]
    pub fn is_rbg(&self) -> bool {
        **self == RGB_SEQ_A::RBG
    }
    #[doc = "Checks if the value of the field is `GRB`"]
    #[inline(always)]
    pub fn is_grb(&self) -> bool {
        **self == RGB_SEQ_A::GRB
    }
    #[doc = "Checks if the value of the field is `GBR`"]
    #[inline(always)]
    pub fn is_gbr(&self) -> bool {
        **self == RGB_SEQ_A::GBR
    }
    #[doc = "Checks if the value of the field is `BRG`"]
    #[inline(always)]
    pub fn is_brg(&self) -> bool {
        **self == RGB_SEQ_A::BRG
    }
    #[doc = "Checks if the value of the field is `BGR`"]
    #[inline(always)]
    pub fn is_bgr(&self) -> bool {
        **self == RGB_SEQ_A::BGR
    }
}
impl core::ops::Deref for RGB_SEQ_R {
    type Target = crate::FieldReader<u8, RGB_SEQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rgb_seq` writer - Output RGB Sequence"]
pub struct RGB_SEQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RGB_SEQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGB_SEQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut W {
        self.variant(RGB_SEQ_A::RGB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rbg(self) -> &'a mut W {
        self.variant(RGB_SEQ_A::RBG)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn grb(self) -> &'a mut W {
        self.variant(RGB_SEQ_A::GRB)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn gbr(self) -> &'a mut W {
        self.variant(RGB_SEQ_A::GBR)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn brg(self) -> &'a mut W {
        self.variant(RGB_SEQ_A::BRG)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn bgr(self) -> &'a mut W {
        self.variant(RGB_SEQ_A::BGR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Transmit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRAN_MOD_A {
    #[doc = "0: `0`"]
    COMMAND_PARAMETER = 0,
    #[doc = "1: `1`"]
    VIDEO = 1,
}
impl From<TRAN_MOD_A> for bool {
    #[inline(always)]
    fn from(variant: TRAN_MOD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tran_mod` reader - Transmit Mode"]
pub struct TRAN_MOD_R(crate::FieldReader<bool, TRAN_MOD_A>);
impl TRAN_MOD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRAN_MOD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRAN_MOD_A {
        match self.bits {
            false => TRAN_MOD_A::COMMAND_PARAMETER,
            true => TRAN_MOD_A::VIDEO,
        }
    }
    #[doc = "Checks if the value of the field is `COMMAND_PARAMETER`"]
    #[inline(always)]
    pub fn is_command_parameter(&self) -> bool {
        **self == TRAN_MOD_A::COMMAND_PARAMETER
    }
    #[doc = "Checks if the value of the field is `VIDEO`"]
    #[inline(always)]
    pub fn is_video(&self) -> bool {
        **self == TRAN_MOD_A::VIDEO
    }
}
impl core::ops::Deref for TRAN_MOD_R {
    type Target = crate::FieldReader<bool, TRAN_MOD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tran_mod` writer - Transmit Mode"]
pub struct TRAN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TRAN_MOD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TRAN_MOD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn command_parameter(self) -> &'a mut W {
        self.variant(TRAN_MOD_A::COMMAND_PARAMETER)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn video(self) -> &'a mut W {
        self.variant(TRAN_MOD_A::VIDEO)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Output Data Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DAT_FMT_A {
    #[doc = "0: `0`"]
    RGB111 = 0,
    #[doc = "1: `1`"]
    RGB444 = 1,
    #[doc = "2: `10`"]
    RGB565 = 2,
    #[doc = "3: `11`"]
    RGB666 = 3,
    #[doc = "4: `100`"]
    RGB888 = 4,
}
impl From<DAT_FMT_A> for u8 {
    #[inline(always)]
    fn from(variant: DAT_FMT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `dat_fmt` reader - Output Data Format"]
pub struct DAT_FMT_R(crate::FieldReader<u8, DAT_FMT_A>);
impl DAT_FMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DAT_FMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DAT_FMT_A> {
        match self.bits {
            0 => Some(DAT_FMT_A::RGB111),
            1 => Some(DAT_FMT_A::RGB444),
            2 => Some(DAT_FMT_A::RGB565),
            3 => Some(DAT_FMT_A::RGB666),
            4 => Some(DAT_FMT_A::RGB888),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RGB111`"]
    #[inline(always)]
    pub fn is_rgb111(&self) -> bool {
        **self == DAT_FMT_A::RGB111
    }
    #[doc = "Checks if the value of the field is `RGB444`"]
    #[inline(always)]
    pub fn is_rgb444(&self) -> bool {
        **self == DAT_FMT_A::RGB444
    }
    #[doc = "Checks if the value of the field is `RGB565`"]
    #[inline(always)]
    pub fn is_rgb565(&self) -> bool {
        **self == DAT_FMT_A::RGB565
    }
    #[doc = "Checks if the value of the field is `RGB666`"]
    #[inline(always)]
    pub fn is_rgb666(&self) -> bool {
        **self == DAT_FMT_A::RGB666
    }
    #[doc = "Checks if the value of the field is `RGB888`"]
    #[inline(always)]
    pub fn is_rgb888(&self) -> bool {
        **self == DAT_FMT_A::RGB888
    }
}
impl core::ops::Deref for DAT_FMT_R {
    type Target = crate::FieldReader<u8, DAT_FMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dat_fmt` writer - Output Data Format"]
pub struct DAT_FMT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT_FMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAT_FMT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rgb111(self) -> &'a mut W {
        self.variant(DAT_FMT_A::RGB111)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rgb444(self) -> &'a mut W {
        self.variant(DAT_FMT_A::RGB444)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn rgb565(self) -> &'a mut W {
        self.variant(DAT_FMT_A::RGB565)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn rgb666(self) -> &'a mut W {
        self.variant(DAT_FMT_A::RGB666)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn rgb888(self) -> &'a mut W {
        self.variant(DAT_FMT_A::RGB888)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBI_INTERFACE_A {
    #[doc = "0: 3 Line Interface I"]
    L3I1 = 0,
    #[doc = "1: 3 Line Interface II"]
    L3I2 = 1,
    #[doc = "2: 4 Line Interface I"]
    L4I1 = 2,
    #[doc = "3: 4 Line Interface II"]
    L4I2 = 3,
    #[doc = "4: 2 Data Lane Interface"]
    D2LI = 4,
}
impl From<DBI_INTERFACE_A> for u8 {
    #[inline(always)]
    fn from(variant: DBI_INTERFACE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `dbi_interface` reader - "]
pub struct DBI_INTERFACE_R(crate::FieldReader<u8, DBI_INTERFACE_A>);
impl DBI_INTERFACE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DBI_INTERFACE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DBI_INTERFACE_A> {
        match self.bits {
            0 => Some(DBI_INTERFACE_A::L3I1),
            1 => Some(DBI_INTERFACE_A::L3I2),
            2 => Some(DBI_INTERFACE_A::L4I1),
            3 => Some(DBI_INTERFACE_A::L4I2),
            4 => Some(DBI_INTERFACE_A::D2LI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `L3I1`"]
    #[inline(always)]
    pub fn is_l3i1(&self) -> bool {
        **self == DBI_INTERFACE_A::L3I1
    }
    #[doc = "Checks if the value of the field is `L3I2`"]
    #[inline(always)]
    pub fn is_l3i2(&self) -> bool {
        **self == DBI_INTERFACE_A::L3I2
    }
    #[doc = "Checks if the value of the field is `L4I1`"]
    #[inline(always)]
    pub fn is_l4i1(&self) -> bool {
        **self == DBI_INTERFACE_A::L4I1
    }
    #[doc = "Checks if the value of the field is `L4I2`"]
    #[inline(always)]
    pub fn is_l4i2(&self) -> bool {
        **self == DBI_INTERFACE_A::L4I2
    }
    #[doc = "Checks if the value of the field is `D2LI`"]
    #[inline(always)]
    pub fn is_d2li(&self) -> bool {
        **self == DBI_INTERFACE_A::D2LI
    }
}
impl core::ops::Deref for DBI_INTERFACE_R {
    type Target = crate::FieldReader<u8, DBI_INTERFACE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dbi_interface` writer - "]
pub struct DBI_INTERFACE_W<'a> {
    w: &'a mut W,
}
impl<'a> DBI_INTERFACE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DBI_INTERFACE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "3 Line Interface I"]
    #[inline(always)]
    pub fn l3i1(self) -> &'a mut W {
        self.variant(DBI_INTERFACE_A::L3I1)
    }
    #[doc = "3 Line Interface II"]
    #[inline(always)]
    pub fn l3i2(self) -> &'a mut W {
        self.variant(DBI_INTERFACE_A::L3I2)
    }
    #[doc = "4 Line Interface I"]
    #[inline(always)]
    pub fn l4i1(self) -> &'a mut W {
        self.variant(DBI_INTERFACE_A::L4I1)
    }
    #[doc = "4 Line Interface II"]
    #[inline(always)]
    pub fn l4i2(self) -> &'a mut W {
        self.variant(DBI_INTERFACE_A::L4I2)
    }
    #[doc = "2 Data Lane Interface"]
    #[inline(always)]
    pub fn d2li(self) -> &'a mut W {
        self.variant(DBI_INTERFACE_A::D2LI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "RGB Source Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RGB_SRC_FMT_A {
    #[doc = "0: `0`"]
    RGB = 0,
    #[doc = "1: `1`"]
    RBG = 1,
    #[doc = "2: `10`"]
    GRB = 2,
    #[doc = "3: `11`"]
    GBR = 3,
    #[doc = "4: `100`"]
    BRG = 4,
    #[doc = "5: `101`"]
    BGR = 5,
    #[doc = "6: `110`"]
    GRBG_0 = 6,
    #[doc = "7: `111`"]
    GBRG_0 = 7,
    #[doc = "8: `1000`"]
    GRBG_1 = 8,
    #[doc = "9: `1001`"]
    GBRG_1 = 9,
}
impl From<RGB_SRC_FMT_A> for u8 {
    #[inline(always)]
    fn from(variant: RGB_SRC_FMT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `rgb_src_fmt` reader - RGB Source Format"]
pub struct RGB_SRC_FMT_R(crate::FieldReader<u8, RGB_SRC_FMT_A>);
impl RGB_SRC_FMT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RGB_SRC_FMT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RGB_SRC_FMT_A> {
        match self.bits {
            0 => Some(RGB_SRC_FMT_A::RGB),
            1 => Some(RGB_SRC_FMT_A::RBG),
            2 => Some(RGB_SRC_FMT_A::GRB),
            3 => Some(RGB_SRC_FMT_A::GBR),
            4 => Some(RGB_SRC_FMT_A::BRG),
            5 => Some(RGB_SRC_FMT_A::BGR),
            6 => Some(RGB_SRC_FMT_A::GRBG_0),
            7 => Some(RGB_SRC_FMT_A::GBRG_0),
            8 => Some(RGB_SRC_FMT_A::GRBG_1),
            9 => Some(RGB_SRC_FMT_A::GBRG_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RGB`"]
    #[inline(always)]
    pub fn is_rgb(&self) -> bool {
        **self == RGB_SRC_FMT_A::RGB
    }
    #[doc = "Checks if the value of the field is `RBG`"]
    #[inline(always)]
    pub fn is_rbg(&self) -> bool {
        **self == RGB_SRC_FMT_A::RBG
    }
    #[doc = "Checks if the value of the field is `GRB`"]
    #[inline(always)]
    pub fn is_grb(&self) -> bool {
        **self == RGB_SRC_FMT_A::GRB
    }
    #[doc = "Checks if the value of the field is `GBR`"]
    #[inline(always)]
    pub fn is_gbr(&self) -> bool {
        **self == RGB_SRC_FMT_A::GBR
    }
    #[doc = "Checks if the value of the field is `BRG`"]
    #[inline(always)]
    pub fn is_brg(&self) -> bool {
        **self == RGB_SRC_FMT_A::BRG
    }
    #[doc = "Checks if the value of the field is `BGR`"]
    #[inline(always)]
    pub fn is_bgr(&self) -> bool {
        **self == RGB_SRC_FMT_A::BGR
    }
    #[doc = "Checks if the value of the field is `GRBG_0`"]
    #[inline(always)]
    pub fn is_grbg_0(&self) -> bool {
        **self == RGB_SRC_FMT_A::GRBG_0
    }
    #[doc = "Checks if the value of the field is `GBRG_0`"]
    #[inline(always)]
    pub fn is_gbrg_0(&self) -> bool {
        **self == RGB_SRC_FMT_A::GBRG_0
    }
    #[doc = "Checks if the value of the field is `GRBG_1`"]
    #[inline(always)]
    pub fn is_grbg_1(&self) -> bool {
        **self == RGB_SRC_FMT_A::GRBG_1
    }
    #[doc = "Checks if the value of the field is `GBRG_1`"]
    #[inline(always)]
    pub fn is_gbrg_1(&self) -> bool {
        **self == RGB_SRC_FMT_A::GBRG_1
    }
}
impl core::ops::Deref for RGB_SRC_FMT_R {
    type Target = crate::FieldReader<u8, RGB_SRC_FMT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rgb_src_fmt` writer - RGB Source Format"]
pub struct RGB_SRC_FMT_W<'a> {
    w: &'a mut W,
}
impl<'a> RGB_SRC_FMT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGB_SRC_FMT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rgb(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::RGB)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rbg(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::RBG)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn grb(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::GRB)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn gbr(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::GBR)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn brg(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::BRG)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn bgr(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::BGR)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn grbg_0(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::GRBG_0)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn gbrg_0(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::GBRG_0)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn grbg_1(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::GRBG_1)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn gbrg_1(self) -> &'a mut W {
        self.variant(RGB_SRC_FMT_A::GBRG_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `dum_val` reader - Dummy Cycle Value"]
pub struct DUM_VAL_R(crate::FieldReader<bool, bool>);
impl DUM_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUM_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUM_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dum_val` writer - Dummy Cycle Value"]
pub struct DUM_VAL_W<'a> {
    w: &'a mut W,
}
impl<'a> DUM_VAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "RGB Bit Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RGB_BO_A {
    #[doc = "0: `0`"]
    DATA = 0,
    #[doc = "1: `1`"]
    SWAP = 1,
}
impl From<RGB_BO_A> for bool {
    #[inline(always)]
    fn from(variant: RGB_BO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rgb_bo` reader - RGB Bit Order"]
pub struct RGB_BO_R(crate::FieldReader<bool, RGB_BO_A>);
impl RGB_BO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RGB_BO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RGB_BO_A {
        match self.bits {
            false => RGB_BO_A::DATA,
            true => RGB_BO_A::SWAP,
        }
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        **self == RGB_BO_A::DATA
    }
    #[doc = "Checks if the value of the field is `SWAP`"]
    #[inline(always)]
    pub fn is_swap(&self) -> bool {
        **self == RGB_BO_A::SWAP
    }
}
impl core::ops::Deref for RGB_BO_R {
    type Target = crate::FieldReader<bool, RGB_BO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rgb_bo` writer - RGB Bit Order"]
pub struct RGB_BO_W<'a> {
    w: &'a mut W,
}
impl<'a> RGB_BO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RGB_BO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(RGB_BO_A::DATA)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn swap(self) -> &'a mut W {
        self.variant(RGB_BO_A::SWAP)
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
#[doc = "Element A Position\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ELEMENT_A_POS_A {
    #[doc = "0: `0`"]
    _31_24 = 0,
    #[doc = "1: `1`"]
    _7_0 = 1,
}
impl From<ELEMENT_A_POS_A> for bool {
    #[inline(always)]
    fn from(variant: ELEMENT_A_POS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `element_a_pos` reader - Element A Position"]
pub struct ELEMENT_A_POS_R(crate::FieldReader<bool, ELEMENT_A_POS_A>);
impl ELEMENT_A_POS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ELEMENT_A_POS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELEMENT_A_POS_A {
        match self.bits {
            false => ELEMENT_A_POS_A::_31_24,
            true => ELEMENT_A_POS_A::_7_0,
        }
    }
    #[doc = "Checks if the value of the field is `_31_24`"]
    #[inline(always)]
    pub fn is_31_24(&self) -> bool {
        **self == ELEMENT_A_POS_A::_31_24
    }
    #[doc = "Checks if the value of the field is `_7_0`"]
    #[inline(always)]
    pub fn is_7_0(&self) -> bool {
        **self == ELEMENT_A_POS_A::_7_0
    }
}
impl core::ops::Deref for ELEMENT_A_POS_R {
    type Target = crate::FieldReader<bool, ELEMENT_A_POS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `element_a_pos` writer - Element A Position"]
pub struct ELEMENT_A_POS_W<'a> {
    w: &'a mut W,
}
impl<'a> ELEMENT_A_POS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ELEMENT_A_POS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn _31_24(self) -> &'a mut W {
        self.variant(ELEMENT_A_POS_A::_31_24)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn _7_0(self) -> &'a mut W {
        self.variant(ELEMENT_A_POS_A::_7_0)
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
#[doc = "Video Source Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VI_SRC_TYPE_A {
    #[doc = "0: `0`"]
    RGB32 = 0,
    #[doc = "1: `1`"]
    RGB16 = 1,
}
impl From<VI_SRC_TYPE_A> for bool {
    #[inline(always)]
    fn from(variant: VI_SRC_TYPE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vi_src_type` reader - Video Source Type"]
pub struct VI_SRC_TYPE_R(crate::FieldReader<bool, VI_SRC_TYPE_A>);
impl VI_SRC_TYPE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VI_SRC_TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VI_SRC_TYPE_A {
        match self.bits {
            false => VI_SRC_TYPE_A::RGB32,
            true => VI_SRC_TYPE_A::RGB16,
        }
    }
    #[doc = "Checks if the value of the field is `RGB32`"]
    #[inline(always)]
    pub fn is_rgb32(&self) -> bool {
        **self == VI_SRC_TYPE_A::RGB32
    }
    #[doc = "Checks if the value of the field is `RGB16`"]
    #[inline(always)]
    pub fn is_rgb16(&self) -> bool {
        **self == VI_SRC_TYPE_A::RGB16
    }
}
impl core::ops::Deref for VI_SRC_TYPE_R {
    type Target = crate::FieldReader<bool, VI_SRC_TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vi_src_type` writer - Video Source Type"]
pub struct VI_SRC_TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> VI_SRC_TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VI_SRC_TYPE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn rgb32(self) -> &'a mut W {
        self.variant(VI_SRC_TYPE_A::RGB32)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn rgb16(self) -> &'a mut W {
        self.variant(VI_SRC_TYPE_A::RGB16)
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
    #[doc = "Bit 31 - Command Type"]
    #[inline(always)]
    pub fn cmdt(&self) -> CMDT_R {
        CMDT_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 20:30 - Write Command Dummy Cycles"]
    #[inline(always)]
    pub fn wcdc(&self) -> WCDC_R {
        WCDC_R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 19 - Output Data Sequence"]
    #[inline(always)]
    pub fn dat_seq(&self) -> DAT_SEQ_R {
        DAT_SEQ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Output RGB Sequence"]
    #[inline(always)]
    pub fn rgb_seq(&self) -> RGB_SEQ_R {
        RGB_SEQ_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Transmit Mode"]
    #[inline(always)]
    pub fn tran_mod(&self) -> TRAN_MOD_R {
        TRAN_MOD_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Output Data Format"]
    #[inline(always)]
    pub fn dat_fmt(&self) -> DAT_FMT_R {
        DAT_FMT_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dbi_interface(&self) -> DBI_INTERFACE_R {
        DBI_INTERFACE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - RGB Source Format"]
    #[inline(always)]
    pub fn rgb_src_fmt(&self) -> RGB_SRC_FMT_R {
        RGB_SRC_FMT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 3 - Dummy Cycle Value"]
    #[inline(always)]
    pub fn dum_val(&self) -> DUM_VAL_R {
        DUM_VAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RGB Bit Order"]
    #[inline(always)]
    pub fn rgb_bo(&self) -> RGB_BO_R {
        RGB_BO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Element A Position"]
    #[inline(always)]
    pub fn element_a_pos(&self) -> ELEMENT_A_POS_R {
        ELEMENT_A_POS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Video Source Type"]
    #[inline(always)]
    pub fn vi_src_type(&self) -> VI_SRC_TYPE_R {
        VI_SRC_TYPE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Command Type"]
    #[inline(always)]
    pub fn cmdt(&mut self) -> CMDT_W {
        CMDT_W { w: self }
    }
    #[doc = "Bits 20:30 - Write Command Dummy Cycles"]
    #[inline(always)]
    pub fn wcdc(&mut self) -> WCDC_W {
        WCDC_W { w: self }
    }
    #[doc = "Bit 19 - Output Data Sequence"]
    #[inline(always)]
    pub fn dat_seq(&mut self) -> DAT_SEQ_W {
        DAT_SEQ_W { w: self }
    }
    #[doc = "Bits 16:18 - Output RGB Sequence"]
    #[inline(always)]
    pub fn rgb_seq(&mut self) -> RGB_SEQ_W {
        RGB_SEQ_W { w: self }
    }
    #[doc = "Bit 15 - Transmit Mode"]
    #[inline(always)]
    pub fn tran_mod(&mut self) -> TRAN_MOD_W {
        TRAN_MOD_W { w: self }
    }
    #[doc = "Bits 12:14 - Output Data Format"]
    #[inline(always)]
    pub fn dat_fmt(&mut self) -> DAT_FMT_W {
        DAT_FMT_W { w: self }
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn dbi_interface(&mut self) -> DBI_INTERFACE_W {
        DBI_INTERFACE_W { w: self }
    }
    #[doc = "Bits 4:7 - RGB Source Format"]
    #[inline(always)]
    pub fn rgb_src_fmt(&mut self) -> RGB_SRC_FMT_W {
        RGB_SRC_FMT_W { w: self }
    }
    #[doc = "Bit 3 - Dummy Cycle Value"]
    #[inline(always)]
    pub fn dum_val(&mut self) -> DUM_VAL_W {
        DUM_VAL_W { w: self }
    }
    #[doc = "Bit 2 - RGB Bit Order"]
    #[inline(always)]
    pub fn rgb_bo(&mut self) -> RGB_BO_W {
        RGB_BO_W { w: self }
    }
    #[doc = "Bit 1 - Element A Position"]
    #[inline(always)]
    pub fn element_a_pos(&mut self) -> ELEMENT_A_POS_W {
        ELEMENT_A_POS_W { w: self }
    }
    #[doc = "Bit 0 - Video Source Type"]
    #[inline(always)]
    pub fn vi_src_type(&mut self) -> VI_SRC_TYPE_W {
        VI_SRC_TYPE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DBI Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbi_ctl_0](index.html) module"]
pub struct DBI_CTL_0_SPEC;
impl crate::RegisterSpec for DBI_CTL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbi_ctl_0::R](R) reader structure"]
impl crate::Readable for DBI_CTL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbi_ctl_0::W](W) writer structure"]
impl crate::Writable for DBI_CTL_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBI_CTL_0 to value 0"]
impl crate::Resettable for DBI_CTL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
