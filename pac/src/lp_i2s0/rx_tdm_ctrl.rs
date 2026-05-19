#[doc = "Register `RX_TDM_CTRL` reader"]
pub type R = crate::R<RxTdmCtrlSpec>;
#[doc = "Register `RX_TDM_CTRL` writer"]
pub type W = crate::W<RxTdmCtrlSpec>;
#[doc = "Field `RX_TDM_PDM_CHAN0_EN` reader - 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel."]
pub type RxTdmPdmChan0EnR = crate::BitReader;
#[doc = "Field `RX_TDM_PDM_CHAN0_EN` writer - 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel."]
pub type RxTdmPdmChan0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TDM_PDM_CHAN1_EN` reader - 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel."]
pub type RxTdmPdmChan1EnR = crate::BitReader;
#[doc = "Field `RX_TDM_PDM_CHAN1_EN` writer - 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel."]
pub type RxTdmPdmChan1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TDM_TOT_CHAN_NUM` reader - The total channel number of I2S TX TDM mode."]
pub type RxTdmTotChanNumR = crate::FieldReader;
#[doc = "Field `RX_TDM_TOT_CHAN_NUM` writer - The total channel number of I2S TX TDM mode."]
pub type RxTdmTotChanNumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan0_en(&self) -> RxTdmPdmChan0EnR {
        RxTdmPdmChan0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan1_en(&self) -> RxTdmPdmChan1EnR {
        RxTdmPdmChan1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:19 - The total channel number of I2S TX TDM mode."]
    #[inline(always)]
    pub fn rx_tdm_tot_chan_num(&self) -> RxTdmTotChanNumR {
        RxTdmTotChanNumR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 1: Enable the valid data input of I2S RX TDM or PDM channel 0. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan0_en(&mut self) -> RxTdmPdmChan0EnW<'_, RxTdmCtrlSpec> {
        RxTdmPdmChan0EnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1: Enable the valid data input of I2S RX TDM or PDM channel 1. 0: Disable, just input 0 in this channel."]
    #[inline(always)]
    pub fn rx_tdm_pdm_chan1_en(&mut self) -> RxTdmPdmChan1EnW<'_, RxTdmCtrlSpec> {
        RxTdmPdmChan1EnW::new(self, 1)
    }
    #[doc = "Bits 16:19 - The total channel number of I2S TX TDM mode."]
    #[inline(always)]
    pub fn rx_tdm_tot_chan_num(&mut self) -> RxTdmTotChanNumW<'_, RxTdmCtrlSpec> {
        RxTdmTotChanNumW::new(self, 16)
    }
}
#[doc = "I2S TX TDM mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_tdm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_tdm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxTdmCtrlSpec;
impl crate::RegisterSpec for RxTdmCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_tdm_ctrl::R`](R) reader structure"]
impl crate::Readable for RxTdmCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_tdm_ctrl::W`](W) writer structure"]
impl crate::Writable for RxTdmCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_TDM_CTRL to value 0x03"]
impl crate::Resettable for RxTdmCtrlSpec {
    const RESET_VALUE: u32 = 0x03;
}
