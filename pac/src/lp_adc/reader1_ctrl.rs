#[doc = "Register `READER1_CTRL` reader"]
pub type R = crate::R<Reader1CtrlSpec>;
#[doc = "Register `READER1_CTRL` writer"]
pub type W = crate::W<Reader1CtrlSpec>;
#[doc = "Field `SAR1_CLK_DIV` reader - Clock divider."]
pub type Sar1ClkDivR = crate::FieldReader;
#[doc = "Field `SAR1_CLK_DIV` writer - Clock divider."]
pub type Sar1ClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR1_CLK_GATED` reader - N/A"]
pub type Sar1ClkGatedR = crate::BitReader;
#[doc = "Field `SAR1_CLK_GATED` writer - N/A"]
pub type Sar1ClkGatedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_SAMPLE_NUM` reader - N/A"]
pub type Sar1SampleNumR = crate::FieldReader;
#[doc = "Field `SAR1_SAMPLE_NUM` writer - N/A"]
pub type Sar1SampleNumW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR1_DATA_INV` reader - Invert SAR ADC1 data."]
pub type Sar1DataInvR = crate::BitReader;
#[doc = "Field `SAR1_DATA_INV` writer - Invert SAR ADC1 data."]
pub type Sar1DataInvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_INT_EN` reader - Enable saradc1 to send out interrupt."]
pub type Sar1IntEnR = crate::BitReader;
#[doc = "Field `SAR1_INT_EN` writer - Enable saradc1 to send out interrupt."]
pub type Sar1IntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_EN_PAD_FORCE_ENABLE` reader - Force enable adc en_pad to analog circuit 2'b11: force enable ."]
pub type Sar1EnPadForceEnableR = crate::FieldReader;
#[doc = "Field `SAR1_EN_PAD_FORCE_ENABLE` writer - Force enable adc en_pad to analog circuit 2'b11: force enable ."]
pub type Sar1EnPadForceEnableW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Clock divider."]
    #[inline(always)]
    pub fn sar1_clk_div(&self) -> Sar1ClkDivR {
        Sar1ClkDivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn sar1_clk_gated(&self) -> Sar1ClkGatedR {
        Sar1ClkGatedR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:26 - N/A"]
    #[inline(always)]
    pub fn sar1_sample_num(&self) -> Sar1SampleNumR {
        Sar1SampleNumR::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Invert SAR ADC1 data."]
    #[inline(always)]
    pub fn sar1_data_inv(&self) -> Sar1DataInvR {
        Sar1DataInvR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable saradc1 to send out interrupt."]
    #[inline(always)]
    pub fn sar1_int_en(&self) -> Sar1IntEnR {
        Sar1IntEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Force enable adc en_pad to analog circuit 2'b11: force enable ."]
    #[inline(always)]
    pub fn sar1_en_pad_force_enable(&self) -> Sar1EnPadForceEnableR {
        Sar1EnPadForceEnableR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider."]
    #[inline(always)]
    pub fn sar1_clk_div(&mut self) -> Sar1ClkDivW<'_, Reader1CtrlSpec> {
        Sar1ClkDivW::new(self, 0)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn sar1_clk_gated(&mut self) -> Sar1ClkGatedW<'_, Reader1CtrlSpec> {
        Sar1ClkGatedW::new(self, 18)
    }
    #[doc = "Bits 19:26 - N/A"]
    #[inline(always)]
    pub fn sar1_sample_num(&mut self) -> Sar1SampleNumW<'_, Reader1CtrlSpec> {
        Sar1SampleNumW::new(self, 19)
    }
    #[doc = "Bit 28 - Invert SAR ADC1 data."]
    #[inline(always)]
    pub fn sar1_data_inv(&mut self) -> Sar1DataInvW<'_, Reader1CtrlSpec> {
        Sar1DataInvW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable saradc1 to send out interrupt."]
    #[inline(always)]
    pub fn sar1_int_en(&mut self) -> Sar1IntEnW<'_, Reader1CtrlSpec> {
        Sar1IntEnW::new(self, 29)
    }
    #[doc = "Bits 30:31 - Force enable adc en_pad to analog circuit 2'b11: force enable ."]
    #[inline(always)]
    pub fn sar1_en_pad_force_enable(&mut self) -> Sar1EnPadForceEnableW<'_, Reader1CtrlSpec> {
        Sar1EnPadForceEnableW::new(self, 30)
    }
}
#[doc = "Control the read operation of ADC1.\n\nYou can [`read`](crate::Reg::read) this register and get [`reader1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reader1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reader1CtrlSpec;
impl crate::RegisterSpec for Reader1CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reader1_ctrl::R`](R) reader structure"]
impl crate::Readable for Reader1CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reader1_ctrl::W`](W) writer structure"]
impl crate::Writable for Reader1CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets READER1_CTRL to value 0x2004_0002"]
impl crate::Resettable for Reader1CtrlSpec {
    const RESET_VALUE: u32 = 0x2004_0002;
}
