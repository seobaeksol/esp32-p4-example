#[doc = "Register `LCD_USER` reader"]
pub type R = crate::R<LcdUserSpec>;
#[doc = "Register `LCD_USER` writer"]
pub type W = crate::W<LcdUserSpec>;
#[doc = "Field `LCD_DOUT_CYCLELEN` reader - The output data cycles minus 1 of LCD module."]
pub type LcdDoutCyclelenR = crate::FieldReader<u16>;
#[doc = "Field `LCD_DOUT_CYCLELEN` writer - The output data cycles minus 1 of LCD module."]
pub type LcdDoutCyclelenW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `LCD_ALWAYS_OUT_EN` reader - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
pub type LcdAlwaysOutEnR = crate::BitReader;
#[doc = "Field `LCD_ALWAYS_OUT_EN` writer - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
pub type LcdAlwaysOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_DOUT_BYTE_SWIZZLE_MODE` reader - 0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA"]
pub type LcdDoutByteSwizzleModeR = crate::FieldReader;
#[doc = "Field `LCD_DOUT_BYTE_SWIZZLE_MODE` writer - 0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA"]
pub type LcdDoutByteSwizzleModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LCD_DOUT_BYTE_SWIZZLE_ENABLE` reader - 1: enable byte swizzle 0: disable"]
pub type LcdDoutByteSwizzleEnableR = crate::BitReader;
#[doc = "Field `LCD_DOUT_BYTE_SWIZZLE_ENABLE` writer - 1: enable byte swizzle 0: disable"]
pub type LcdDoutByteSwizzleEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_DOUT_BIT_ORDER` reader - 1: change bit order in every byte. 0: Not change."]
pub type LcdDoutBitOrderR = crate::BitReader;
#[doc = "Field `LCD_DOUT_BIT_ORDER` writer - 1: change bit order in every byte. 0: Not change."]
pub type LcdDoutBitOrderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_BYTE_MODE` reader - 2: 24bit mode. 1: 16bit mode. 0: 8bit mode"]
pub type LcdByteModeR = crate::FieldReader;
#[doc = "Field `LCD_BYTE_MODE` writer - 2: 24bit mode. 1: 16bit mode. 0: 8bit mode"]
pub type LcdByteModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_UPDATE` reader - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
pub type LcdUpdateR = crate::BitReader;
#[doc = "Field `LCD_UPDATE` writer - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
pub type LcdUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_BIT_ORDER` reader - 1: Change data bit order, change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
pub type LcdBitOrderR = crate::BitReader;
#[doc = "Field `LCD_BIT_ORDER` writer - 1: Change data bit order, change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
pub type LcdBitOrderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_BYTE_ORDER` reader - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
pub type LcdByteOrderR = crate::BitReader;
#[doc = "Field `LCD_BYTE_ORDER` writer - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
pub type LcdByteOrderW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_DOUT` reader - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
pub type LcdDoutR = crate::BitReader;
#[doc = "Field `LCD_DOUT` writer - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
pub type LcdDoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_DUMMY` reader - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
pub type LcdDummyR = crate::BitReader;
#[doc = "Field `LCD_DUMMY` writer - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
pub type LcdDummyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CMD` reader - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
pub type LcdCmdR = crate::BitReader;
#[doc = "Field `LCD_CMD` writer - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
pub type LcdCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_START` reader - LCD start sending data enable signal, valid in high level."]
pub type LcdStartR = crate::BitReader;
#[doc = "Field `LCD_START` writer - LCD start sending data enable signal, valid in high level."]
pub type LcdStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_RESET` writer - The value of command."]
pub type LcdResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_DUMMY_CYCLELEN` reader - The dummy cycle length minus 1."]
pub type LcdDummyCyclelenR = crate::FieldReader;
#[doc = "Field `LCD_DUMMY_CYCLELEN` writer - The dummy cycle length minus 1."]
pub type LcdDummyCyclelenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LCD_CMD_2_CYCLE_EN` reader - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
pub type LcdCmd2CycleEnR = crate::BitReader;
#[doc = "Field `LCD_CMD_2_CYCLE_EN` writer - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
pub type LcdCmd2CycleEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - The output data cycles minus 1 of LCD module."]
    #[inline(always)]
    pub fn lcd_dout_cyclelen(&self) -> LcdDoutCyclelenR {
        LcdDoutCyclelenR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
    #[inline(always)]
    pub fn lcd_always_out_en(&self) -> LcdAlwaysOutEnR {
        LcdAlwaysOutEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:16 - 0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA"]
    #[inline(always)]
    pub fn lcd_dout_byte_swizzle_mode(&self) -> LcdDoutByteSwizzleModeR {
        LcdDoutByteSwizzleModeR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bit 17 - 1: enable byte swizzle 0: disable"]
    #[inline(always)]
    pub fn lcd_dout_byte_swizzle_enable(&self) -> LcdDoutByteSwizzleEnableR {
        LcdDoutByteSwizzleEnableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: change bit order in every byte. 0: Not change."]
    #[inline(always)]
    pub fn lcd_dout_bit_order(&self) -> LcdDoutBitOrderR {
        LcdDoutBitOrderR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - 2: 24bit mode. 1: 16bit mode. 0: 8bit mode"]
    #[inline(always)]
    pub fn lcd_byte_mode(&self) -> LcdByteModeR {
        LcdByteModeR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
    #[inline(always)]
    pub fn lcd_update(&self) -> LcdUpdateR {
        LcdUpdateR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1: Change data bit order, change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_bit_order(&self) -> LcdBitOrderR {
        LcdBitOrderR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_byte_order(&self) -> LcdByteOrderR {
        LcdByteOrderR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_dout(&self) -> LcdDoutR {
        LcdDoutR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_dummy(&self) -> LcdDummyR {
        LcdDummyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_cmd(&self) -> LcdCmdR {
        LcdCmdR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - LCD start sending data enable signal, valid in high level."]
    #[inline(always)]
    pub fn lcd_start(&self) -> LcdStartR {
        LcdStartR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 29:30 - The dummy cycle length minus 1."]
    #[inline(always)]
    pub fn lcd_dummy_cyclelen(&self) -> LcdDummyCyclelenR {
        LcdDummyCyclelenR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
    #[inline(always)]
    pub fn lcd_cmd_2_cycle_en(&self) -> LcdCmd2CycleEnR {
        LcdCmd2CycleEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - The output data cycles minus 1 of LCD module."]
    #[inline(always)]
    pub fn lcd_dout_cyclelen(&mut self) -> LcdDoutCyclelenW<'_, LcdUserSpec> {
        LcdDoutCyclelenW::new(self, 0)
    }
    #[doc = "Bit 13 - LCD always output when LCD is in LCD_DOUT state, unless reg_lcd_start is cleared or reg_lcd_reset is set."]
    #[inline(always)]
    pub fn lcd_always_out_en(&mut self) -> LcdAlwaysOutEnW<'_, LcdUserSpec> {
        LcdAlwaysOutEnW::new(self, 13)
    }
    #[doc = "Bits 14:16 - 0: ABAB->BABA. 1: ABC->ACB. 2: ABC->BAC. 3: ABC->BCA. 4:ABC->CAB. 5:ABC->CBA"]
    #[inline(always)]
    pub fn lcd_dout_byte_swizzle_mode(&mut self) -> LcdDoutByteSwizzleModeW<'_, LcdUserSpec> {
        LcdDoutByteSwizzleModeW::new(self, 14)
    }
    #[doc = "Bit 17 - 1: enable byte swizzle 0: disable"]
    #[inline(always)]
    pub fn lcd_dout_byte_swizzle_enable(&mut self) -> LcdDoutByteSwizzleEnableW<'_, LcdUserSpec> {
        LcdDoutByteSwizzleEnableW::new(self, 17)
    }
    #[doc = "Bit 18 - 1: change bit order in every byte. 0: Not change."]
    #[inline(always)]
    pub fn lcd_dout_bit_order(&mut self) -> LcdDoutBitOrderW<'_, LcdUserSpec> {
        LcdDoutBitOrderW::new(self, 18)
    }
    #[doc = "Bits 19:20 - 2: 24bit mode. 1: 16bit mode. 0: 8bit mode"]
    #[inline(always)]
    pub fn lcd_byte_mode(&mut self) -> LcdByteModeW<'_, LcdUserSpec> {
        LcdByteModeW::new(self, 19)
    }
    #[doc = "Bit 21 - 1: Update LCD registers, will be cleared by hardware. 0 : Not care."]
    #[inline(always)]
    pub fn lcd_update(&mut self) -> LcdUpdateW<'_, LcdUserSpec> {
        LcdUpdateW::new(self, 21)
    }
    #[doc = "Bit 22 - 1: Change data bit order, change LCD_DATA_out\\[7:0\\] to LCD_DATA_out\\[0:7\\] in one byte mode, and bits\\[15:0\\] to bits\\[0:15\\] in two byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_bit_order(&mut self) -> LcdBitOrderW<'_, LcdUserSpec> {
        LcdBitOrderW::new(self, 22)
    }
    #[doc = "Bit 23 - 1: invert data byte order, only valid in 2 byte mode. 0: Not change."]
    #[inline(always)]
    pub fn lcd_byte_order(&mut self) -> LcdByteOrderW<'_, LcdUserSpec> {
        LcdByteOrderW::new(self, 23)
    }
    #[doc = "Bit 24 - 1: Be able to send data out in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_dout(&mut self) -> LcdDoutW<'_, LcdUserSpec> {
        LcdDoutW::new(self, 24)
    }
    #[doc = "Bit 25 - 1: Enable DUMMY phase in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_dummy(&mut self) -> LcdDummyW<'_, LcdUserSpec> {
        LcdDummyW::new(self, 25)
    }
    #[doc = "Bit 26 - 1: Be able to send command in LCD sequence when LCD starts. 0: Disable."]
    #[inline(always)]
    pub fn lcd_cmd(&mut self) -> LcdCmdW<'_, LcdUserSpec> {
        LcdCmdW::new(self, 26)
    }
    #[doc = "Bit 27 - LCD start sending data enable signal, valid in high level."]
    #[inline(always)]
    pub fn lcd_start(&mut self) -> LcdStartW<'_, LcdUserSpec> {
        LcdStartW::new(self, 27)
    }
    #[doc = "Bit 28 - The value of command."]
    #[inline(always)]
    pub fn lcd_reset(&mut self) -> LcdResetW<'_, LcdUserSpec> {
        LcdResetW::new(self, 28)
    }
    #[doc = "Bits 29:30 - The dummy cycle length minus 1."]
    #[inline(always)]
    pub fn lcd_dummy_cyclelen(&mut self) -> LcdDummyCyclelenW<'_, LcdUserSpec> {
        LcdDummyCyclelenW::new(self, 29)
    }
    #[doc = "Bit 31 - The cycle length of command phase. 1: 2 cycles. 0: 1 cycle."]
    #[inline(always)]
    pub fn lcd_cmd_2_cycle_en(&mut self) -> LcdCmd2CycleEnW<'_, LcdUserSpec> {
        LcdCmd2CycleEnW::new(self, 31)
    }
}
#[doc = "LCD config register.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_user::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_user::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdUserSpec;
impl crate::RegisterSpec for LcdUserSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_user::R`](R) reader structure"]
impl crate::Readable for LcdUserSpec {}
#[doc = "`write(|w| ..)` method takes [`lcd_user::W`](W) writer structure"]
impl crate::Writable for LcdUserSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCD_USER to value 0x01"]
impl crate::Resettable for LcdUserSpec {
    const RESET_VALUE: u32 = 0x01;
}
