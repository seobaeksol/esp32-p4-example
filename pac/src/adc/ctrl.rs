#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `START_FORCE` reader - need_des"]
pub type StartForceR = crate::BitReader;
#[doc = "Field `START_FORCE` writer - need_des"]
pub type StartForceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - need_des"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - need_des"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_MODE` reader - 0: single mode, 1: double mode, 2: alternate mode"]
pub type WorkModeR = crate::FieldReader;
#[doc = "Field `WORK_MODE` writer - 0: single mode, 1: double mode, 2: alternate mode"]
pub type WorkModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR_SEL` reader - 0: SAR1, 1: SAR2, only work for single SAR mode"]
pub type SarSelR = crate::BitReader;
#[doc = "Field `SAR_SEL` writer - 0: SAR1, 1: SAR2, only work for single SAR mode"]
pub type SarSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_CLK_GATED` reader - need_des"]
pub type SarClkGatedR = crate::BitReader;
#[doc = "Field `SAR_CLK_GATED` writer - need_des"]
pub type SarClkGatedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_CLK_DIV` reader - SAR clock divider"]
pub type SarClkDivR = crate::FieldReader;
#[doc = "Field `SAR_CLK_DIV` writer - SAR clock divider"]
pub type SarClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR1_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub type Sar1PattLenR = crate::FieldReader;
#[doc = "Field `SAR1_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub type Sar1PattLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAR2_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub type Sar2PattLenR = crate::FieldReader;
#[doc = "Field `SAR2_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub type Sar2PattLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAR1_PATT_P_CLEAR` reader - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub type Sar1PattPClearR = crate::BitReader;
#[doc = "Field `SAR1_PATT_P_CLEAR` writer - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub type Sar1PattPClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_PATT_P_CLEAR` reader - clear the pointer of pattern table for DIG ADC2 CTRL"]
pub type Sar2PattPClearR = crate::BitReader;
#[doc = "Field `SAR2_PATT_P_CLEAR` writer - clear the pointer of pattern table for DIG ADC2 CTRL"]
pub type Sar2PattPClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_SAR_SEL` reader - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
pub type DataSarSelR = crate::BitReader;
#[doc = "Field `DATA_SAR_SEL` writer - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
pub type DataSarSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TO_I2S` reader - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
pub type DataToI2sR = crate::BitReader;
#[doc = "Field `DATA_TO_I2S` writer - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
pub type DataToI2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_SAR1_FORCE` reader - force option to xpd sar1 blocks"]
pub type XpdSar1ForceR = crate::FieldReader;
#[doc = "Field `XPD_SAR1_FORCE` writer - force option to xpd sar1 blocks"]
pub type XpdSar1ForceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `XPD_SAR2_FORCE` reader - force option to xpd sar2 blocks"]
pub type XpdSar2ForceR = crate::FieldReader;
#[doc = "Field `XPD_SAR2_FORCE` writer - force option to xpd sar2 blocks"]
pub type XpdSar2ForceW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAIT_ARB_CYCLE` reader - wait arbit signal stable after sar_done"]
pub type WaitArbCycleR = crate::FieldReader;
#[doc = "Field `WAIT_ARB_CYCLE` writer - wait arbit signal stable after sar_done"]
pub type WaitArbCycleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn start_force(&self) -> StartForceR {
        StartForceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 0: single mode, 1: double mode, 2: alternate mode"]
    #[inline(always)]
    pub fn work_mode(&self) -> WorkModeR {
        WorkModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 0: SAR1, 1: SAR2, only work for single SAR mode"]
    #[inline(always)]
    pub fn sar_sel(&self) -> SarSelR {
        SarSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn sar_clk_gated(&self) -> SarClkGatedR {
        SarClkGatedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:13 - SAR clock divider"]
    #[inline(always)]
    pub fn sar_clk_div(&self) -> SarClkDivR {
        SarClkDivR::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:17 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar1_patt_len(&self) -> Sar1PattLenR {
        Sar1PattLenR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar2_patt_len(&self) -> Sar2PattLenR {
        Sar2PattLenR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn sar1_patt_p_clear(&self) -> Sar1PattPClearR {
        Sar1PattPClearR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC2 CTRL"]
    #[inline(always)]
    pub fn sar2_patt_p_clear(&self) -> Sar2PattPClearR {
        Sar2PattPClearR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn data_sar_sel(&self) -> DataSarSelR {
        DataSarSelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn data_to_i2s(&self) -> DataToI2sR {
        DataToI2sR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - force option to xpd sar1 blocks"]
    #[inline(always)]
    pub fn xpd_sar1_force(&self) -> XpdSar1ForceR {
        XpdSar1ForceR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - force option to xpd sar2 blocks"]
    #[inline(always)]
    pub fn xpd_sar2_force(&self) -> XpdSar2ForceR {
        XpdSar2ForceR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - wait arbit signal stable after sar_done"]
    #[inline(always)]
    pub fn wait_arb_cycle(&self) -> WaitArbCycleR {
        WaitArbCycleR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn start_force(&mut self) -> StartForceW<'_, CtrlSpec> {
        StartForceW::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CtrlSpec> {
        StartW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 0: single mode, 1: double mode, 2: alternate mode"]
    #[inline(always)]
    pub fn work_mode(&mut self) -> WorkModeW<'_, CtrlSpec> {
        WorkModeW::new(self, 2)
    }
    #[doc = "Bit 4 - 0: SAR1, 1: SAR2, only work for single SAR mode"]
    #[inline(always)]
    pub fn sar_sel(&mut self) -> SarSelW<'_, CtrlSpec> {
        SarSelW::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn sar_clk_gated(&mut self) -> SarClkGatedW<'_, CtrlSpec> {
        SarClkGatedW::new(self, 5)
    }
    #[doc = "Bits 6:13 - SAR clock divider"]
    #[inline(always)]
    pub fn sar_clk_div(&mut self) -> SarClkDivW<'_, CtrlSpec> {
        SarClkDivW::new(self, 6)
    }
    #[doc = "Bits 14:17 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar1_patt_len(&mut self) -> Sar1PattLenW<'_, CtrlSpec> {
        Sar1PattLenW::new(self, 14)
    }
    #[doc = "Bits 18:21 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar2_patt_len(&mut self) -> Sar2PattLenW<'_, CtrlSpec> {
        Sar2PattLenW::new(self, 18)
    }
    #[doc = "Bit 22 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn sar1_patt_p_clear(&mut self) -> Sar1PattPClearW<'_, CtrlSpec> {
        Sar1PattPClearW::new(self, 22)
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC2 CTRL"]
    #[inline(always)]
    pub fn sar2_patt_p_clear(&mut self) -> Sar2PattPClearW<'_, CtrlSpec> {
        Sar2PattPClearW::new(self, 23)
    }
    #[doc = "Bit 24 - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn data_sar_sel(&mut self) -> DataSarSelW<'_, CtrlSpec> {
        DataSarSelW::new(self, 24)
    }
    #[doc = "Bit 25 - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn data_to_i2s(&mut self) -> DataToI2sW<'_, CtrlSpec> {
        DataToI2sW::new(self, 25)
    }
    #[doc = "Bits 26:27 - force option to xpd sar1 blocks"]
    #[inline(always)]
    pub fn xpd_sar1_force(&mut self) -> XpdSar1ForceW<'_, CtrlSpec> {
        XpdSar1ForceW::new(self, 26)
    }
    #[doc = "Bits 28:29 - force option to xpd sar2 blocks"]
    #[inline(always)]
    pub fn xpd_sar2_force(&mut self) -> XpdSar2ForceW<'_, CtrlSpec> {
        XpdSar2ForceW::new(self, 28)
    }
    #[doc = "Bits 30:31 - wait arbit signal stable after sar_done"]
    #[inline(always)]
    pub fn wait_arb_cycle(&mut self) -> WaitArbCycleW<'_, CtrlSpec> {
        WaitArbCycleW::new(self, 30)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x403f_c120"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x403f_c120;
}
