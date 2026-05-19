#[doc = "Register `TX_PCM2PDM_CONF` reader"]
pub type R = crate::R<TxPcm2pdmConfSpec>;
#[doc = "Register `TX_PCM2PDM_CONF` writer"]
pub type W = crate::W<TxPcm2pdmConfSpec>;
#[doc = "Field `TX_PDM_HP_BYPASS` reader - I2S TX PDM bypass hp filter or not. The option has been removed."]
pub type TxPdmHpBypassR = crate::BitReader;
#[doc = "Field `TX_PDM_HP_BYPASS` writer - I2S TX PDM bypass hp filter or not. The option has been removed."]
pub type TxPdmHpBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PDM_SINC_OSR2` reader - I2S TX PDM OSR2 value"]
pub type TxPdmSincOsr2R = crate::FieldReader;
#[doc = "Field `TX_PDM_SINC_OSR2` writer - I2S TX PDM OSR2 value"]
pub type TxPdmSincOsr2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_PDM_PRESCALE` reader - I2S TX PDM prescale for sigmadelta"]
pub type TxPdmPrescaleR = crate::FieldReader;
#[doc = "Field `TX_PDM_PRESCALE` writer - I2S TX PDM prescale for sigmadelta"]
pub type TxPdmPrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TX_PDM_HP_IN_SHIFT` reader - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TxPdmHpInShiftR = crate::FieldReader;
#[doc = "Field `TX_PDM_HP_IN_SHIFT` writer - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TxPdmHpInShiftW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_PDM_LP_IN_SHIFT` reader - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TxPdmLpInShiftR = crate::FieldReader;
#[doc = "Field `TX_PDM_LP_IN_SHIFT` writer - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TxPdmLpInShiftW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_PDM_SINC_IN_SHIFT` reader - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TxPdmSincInShiftR = crate::FieldReader;
#[doc = "Field `TX_PDM_SINC_IN_SHIFT` writer - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TxPdmSincInShiftW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_PDM_SIGMADELTA_IN_SHIFT` reader - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TxPdmSigmadeltaInShiftR = crate::FieldReader;
#[doc = "Field `TX_PDM_SIGMADELTA_IN_SHIFT` writer - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
pub type TxPdmSigmadeltaInShiftW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_PDM_SIGMADELTA_DITHER2` reader - I2S TX PDM sigmadelta dither2 value"]
pub type TxPdmSigmadeltaDither2R = crate::BitReader;
#[doc = "Field `TX_PDM_SIGMADELTA_DITHER2` writer - I2S TX PDM sigmadelta dither2 value"]
pub type TxPdmSigmadeltaDither2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PDM_SIGMADELTA_DITHER` reader - I2S TX PDM sigmadelta dither value"]
pub type TxPdmSigmadeltaDitherR = crate::BitReader;
#[doc = "Field `TX_PDM_SIGMADELTA_DITHER` writer - I2S TX PDM sigmadelta dither value"]
pub type TxPdmSigmadeltaDitherW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PDM_DAC_2OUT_EN` reader - I2S TX PDM dac mode enable"]
pub type TxPdmDac2outEnR = crate::BitReader;
#[doc = "Field `TX_PDM_DAC_2OUT_EN` writer - I2S TX PDM dac mode enable"]
pub type TxPdmDac2outEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PDM_DAC_MODE_EN` reader - I2S TX PDM dac 2channel enable"]
pub type TxPdmDacModeEnR = crate::BitReader;
#[doc = "Field `TX_PDM_DAC_MODE_EN` writer - I2S TX PDM dac 2channel enable"]
pub type TxPdmDacModeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCM2PDM_CONV_EN` reader - I2S TX PDM Converter enable"]
pub type Pcm2pdmConvEnR = crate::BitReader;
#[doc = "Field `PCM2PDM_CONV_EN` writer - I2S TX PDM Converter enable"]
pub type Pcm2pdmConvEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2S TX PDM bypass hp filter or not. The option has been removed."]
    #[inline(always)]
    pub fn tx_pdm_hp_bypass(&self) -> TxPdmHpBypassR {
        TxPdmHpBypassR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - I2S TX PDM OSR2 value"]
    #[inline(always)]
    pub fn tx_pdm_sinc_osr2(&self) -> TxPdmSincOsr2R {
        TxPdmSincOsr2R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:12 - I2S TX PDM prescale for sigmadelta"]
    #[inline(always)]
    pub fn tx_pdm_prescale(&self) -> TxPdmPrescaleR {
        TxPdmPrescaleR::new(((self.bits >> 5) & 0xff) as u8)
    }
    #[doc = "Bits 13:14 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    pub fn tx_pdm_hp_in_shift(&self) -> TxPdmHpInShiftR {
        TxPdmHpInShiftR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:16 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    pub fn tx_pdm_lp_in_shift(&self) -> TxPdmLpInShiftR {
        TxPdmLpInShiftR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:18 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    pub fn tx_pdm_sinc_in_shift(&self) -> TxPdmSincInShiftR {
        TxPdmSincInShiftR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_in_shift(&self) -> TxPdmSigmadeltaInShiftR {
        TxPdmSigmadeltaInShiftR::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21 - I2S TX PDM sigmadelta dither2 value"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither2(&self) -> TxPdmSigmadeltaDither2R {
        TxPdmSigmadeltaDither2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2S TX PDM sigmadelta dither value"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither(&self) -> TxPdmSigmadeltaDitherR {
        TxPdmSigmadeltaDitherR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2S TX PDM dac mode enable"]
    #[inline(always)]
    pub fn tx_pdm_dac_2out_en(&self) -> TxPdmDac2outEnR {
        TxPdmDac2outEnR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - I2S TX PDM dac 2channel enable"]
    #[inline(always)]
    pub fn tx_pdm_dac_mode_en(&self) -> TxPdmDacModeEnR {
        TxPdmDacModeEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - I2S TX PDM Converter enable"]
    #[inline(always)]
    pub fn pcm2pdm_conv_en(&self) -> Pcm2pdmConvEnR {
        Pcm2pdmConvEnR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S TX PDM bypass hp filter or not. The option has been removed."]
    #[inline(always)]
    pub fn tx_pdm_hp_bypass(&mut self) -> TxPdmHpBypassW<'_, TxPcm2pdmConfSpec> {
        TxPdmHpBypassW::new(self, 0)
    }
    #[doc = "Bits 1:4 - I2S TX PDM OSR2 value"]
    #[inline(always)]
    pub fn tx_pdm_sinc_osr2(&mut self) -> TxPdmSincOsr2W<'_, TxPcm2pdmConfSpec> {
        TxPdmSincOsr2W::new(self, 1)
    }
    #[doc = "Bits 5:12 - I2S TX PDM prescale for sigmadelta"]
    #[inline(always)]
    pub fn tx_pdm_prescale(&mut self) -> TxPdmPrescaleW<'_, TxPcm2pdmConfSpec> {
        TxPdmPrescaleW::new(self, 5)
    }
    #[doc = "Bits 13:14 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    pub fn tx_pdm_hp_in_shift(&mut self) -> TxPdmHpInShiftW<'_, TxPcm2pdmConfSpec> {
        TxPdmHpInShiftW::new(self, 13)
    }
    #[doc = "Bits 15:16 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    pub fn tx_pdm_lp_in_shift(&mut self) -> TxPdmLpInShiftW<'_, TxPcm2pdmConfSpec> {
        TxPdmLpInShiftW::new(self, 15)
    }
    #[doc = "Bits 17:18 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    pub fn tx_pdm_sinc_in_shift(&mut self) -> TxPdmSincInShiftW<'_, TxPcm2pdmConfSpec> {
        TxPdmSincInShiftW::new(self, 17)
    }
    #[doc = "Bits 19:20 - I2S TX PDM sigmadelta scale shift number: 0:/2 , 1:x1 , 2:x2 , 3: x4"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_in_shift(&mut self) -> TxPdmSigmadeltaInShiftW<'_, TxPcm2pdmConfSpec> {
        TxPdmSigmadeltaInShiftW::new(self, 19)
    }
    #[doc = "Bit 21 - I2S TX PDM sigmadelta dither2 value"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither2(&mut self) -> TxPdmSigmadeltaDither2W<'_, TxPcm2pdmConfSpec> {
        TxPdmSigmadeltaDither2W::new(self, 21)
    }
    #[doc = "Bit 22 - I2S TX PDM sigmadelta dither value"]
    #[inline(always)]
    pub fn tx_pdm_sigmadelta_dither(&mut self) -> TxPdmSigmadeltaDitherW<'_, TxPcm2pdmConfSpec> {
        TxPdmSigmadeltaDitherW::new(self, 22)
    }
    #[doc = "Bit 23 - I2S TX PDM dac mode enable"]
    #[inline(always)]
    pub fn tx_pdm_dac_2out_en(&mut self) -> TxPdmDac2outEnW<'_, TxPcm2pdmConfSpec> {
        TxPdmDac2outEnW::new(self, 23)
    }
    #[doc = "Bit 24 - I2S TX PDM dac 2channel enable"]
    #[inline(always)]
    pub fn tx_pdm_dac_mode_en(&mut self) -> TxPdmDacModeEnW<'_, TxPcm2pdmConfSpec> {
        TxPdmDacModeEnW::new(self, 24)
    }
    #[doc = "Bit 25 - I2S TX PDM Converter enable"]
    #[inline(always)]
    pub fn pcm2pdm_conv_en(&mut self) -> Pcm2pdmConvEnW<'_, TxPcm2pdmConfSpec> {
        Pcm2pdmConvEnW::new(self, 25)
    }
}
#[doc = "I2S TX PCM2PDM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_pcm2pdm_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_pcm2pdm_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxPcm2pdmConfSpec;
impl crate::RegisterSpec for TxPcm2pdmConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_pcm2pdm_conf::R`](R) reader structure"]
impl crate::Readable for TxPcm2pdmConfSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_pcm2pdm_conf::W`](W) writer structure"]
impl crate::Writable for TxPcm2pdmConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_PCM2PDM_CONF to value 0x004a_a004"]
impl crate::Resettable for TxPcm2pdmConfSpec {
    const RESET_VALUE: u32 = 0x004a_a004;
}
