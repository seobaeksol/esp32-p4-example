#[doc = "Register `RX_PDM_CONF` reader"]
pub type R = crate::R<RxPdmConfSpec>;
#[doc = "Register `RX_PDM_CONF` writer"]
pub type W = crate::W<RxPdmConfSpec>;
#[doc = "Field `RX_PDM2PCM_EN` reader - 1: Enable PDM2PCM RX mode. 0: DIsable."]
pub type RxPdm2pcmEnR = crate::BitReader;
#[doc = "Field `RX_PDM2PCM_EN` writer - 1: Enable PDM2PCM RX mode. 0: DIsable."]
pub type RxPdm2pcmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PDM_SINC_DSR_16_EN` reader - Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64."]
pub type RxPdmSincDsr16EnR = crate::BitReader;
#[doc = "Field `RX_PDM_SINC_DSR_16_EN` writer - Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64."]
pub type RxPdmSincDsr16EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PDM2PCM_AMPLIFY_NUM` reader - Configure PDM RX amplify number."]
pub type RxPdm2pcmAmplifyNumR = crate::FieldReader;
#[doc = "Field `RX_PDM2PCM_AMPLIFY_NUM` writer - Configure PDM RX amplify number."]
pub type RxPdm2pcmAmplifyNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_PDM_HP_BYPASS` reader - I2S PDM RX bypass hp filter or not."]
pub type RxPdmHpBypassR = crate::BitReader;
#[doc = "Field `RX_PDM_HP_BYPASS` writer - I2S PDM RX bypass hp filter or not."]
pub type RxPdmHpBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_IIR_HP_MULT12_5` reader - The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5\\[2:0\\])"]
pub type RxIirHpMult12_5R = crate::FieldReader;
#[doc = "Field `RX_IIR_HP_MULT12_5` writer - The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5\\[2:0\\])"]
pub type RxIirHpMult12_5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RX_IIR_HP_MULT12_0` reader - The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0\\[2:0\\])"]
pub type RxIirHpMult12_0R = crate::FieldReader;
#[doc = "Field `RX_IIR_HP_MULT12_0` writer - The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0\\[2:0\\])"]
pub type RxIirHpMult12_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 19 - 1: Enable PDM2PCM RX mode. 0: DIsable."]
    #[inline(always)]
    pub fn rx_pdm2pcm_en(&self) -> RxPdm2pcmEnR {
        RxPdm2pcmEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64."]
    #[inline(always)]
    pub fn rx_pdm_sinc_dsr_16_en(&self) -> RxPdmSincDsr16EnR {
        RxPdmSincDsr16EnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:24 - Configure PDM RX amplify number."]
    #[inline(always)]
    pub fn rx_pdm2pcm_amplify_num(&self) -> RxPdm2pcmAmplifyNumR {
        RxPdm2pcmAmplifyNumR::new(((self.bits >> 21) & 0x0f) as u8)
    }
    #[doc = "Bit 25 - I2S PDM RX bypass hp filter or not."]
    #[inline(always)]
    pub fn rx_pdm_hp_bypass(&self) -> RxPdmHpBypassR {
        RxPdmHpBypassR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:28 - The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5\\[2:0\\])"]
    #[inline(always)]
    pub fn rx_iir_hp_mult12_5(&self) -> RxIirHpMult12_5R {
        RxIirHpMult12_5R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0\\[2:0\\])"]
    #[inline(always)]
    pub fn rx_iir_hp_mult12_0(&self) -> RxIirHpMult12_0R {
        RxIirHpMult12_0R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 19 - 1: Enable PDM2PCM RX mode. 0: DIsable."]
    #[inline(always)]
    pub fn rx_pdm2pcm_en(&mut self) -> RxPdm2pcmEnW<'_, RxPdmConfSpec> {
        RxPdm2pcmEnW::new(self, 19)
    }
    #[doc = "Bit 20 - Configure the down sampling rate of PDM RX filter group1 module. 1: The down sampling rate is 128. 0: down sampling rate is 64."]
    #[inline(always)]
    pub fn rx_pdm_sinc_dsr_16_en(&mut self) -> RxPdmSincDsr16EnW<'_, RxPdmConfSpec> {
        RxPdmSincDsr16EnW::new(self, 20)
    }
    #[doc = "Bits 21:24 - Configure PDM RX amplify number."]
    #[inline(always)]
    pub fn rx_pdm2pcm_amplify_num(&mut self) -> RxPdm2pcmAmplifyNumW<'_, RxPdmConfSpec> {
        RxPdm2pcmAmplifyNumW::new(self, 21)
    }
    #[doc = "Bit 25 - I2S PDM RX bypass hp filter or not."]
    #[inline(always)]
    pub fn rx_pdm_hp_bypass(&mut self) -> RxPdmHpBypassW<'_, RxPdmConfSpec> {
        RxPdmHpBypassW::new(self, 25)
    }
    #[doc = "Bits 26:28 - The fourth parameter of PDM RX IIR_HP filter stage 2 is (504 + LP_I2S_RX_IIR_HP_MULT12_5\\[2:0\\])"]
    #[inline(always)]
    pub fn rx_iir_hp_mult12_5(&mut self) -> RxIirHpMult12_5W<'_, RxPdmConfSpec> {
        RxIirHpMult12_5W::new(self, 26)
    }
    #[doc = "Bits 29:31 - The fourth parameter of PDM RX IIR_HP filter stage 1 is (504 + LP_I2S_RX_IIR_HP_MULT12_0\\[2:0\\])"]
    #[inline(always)]
    pub fn rx_iir_hp_mult12_0(&mut self) -> RxIirHpMult12_0W<'_, RxPdmConfSpec> {
        RxIirHpMult12_0W::new(self, 29)
    }
}
#[doc = "I2S RX configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_pdm_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_pdm_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxPdmConfSpec;
impl crate::RegisterSpec for RxPdmConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_pdm_conf::R`](R) reader structure"]
impl crate::Readable for RxPdmConfSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_pdm_conf::W`](W) writer structure"]
impl crate::Writable for RxPdmConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_PDM_CONF to value 0xf820_0000"]
impl crate::Resettable for RxPdmConfSpec {
    const RESET_VALUE: u32 = 0xf820_0000;
}
