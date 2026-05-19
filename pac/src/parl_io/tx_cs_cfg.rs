#[doc = "Register `TX_CS_CFG` reader"]
pub type R = crate::R<TxCsCfgSpec>;
#[doc = "Register `TX_CS_CFG` writer"]
pub type W = crate::W<TxCsCfgSpec>;
#[doc = "Field `TX_CS_STOP_DELAY` reader - configure the delay between data tx end and tx_cs_o posedge"]
pub type TxCsStopDelayR = crate::FieldReader<u16>;
#[doc = "Field `TX_CS_STOP_DELAY` writer - configure the delay between data tx end and tx_cs_o posedge"]
pub type TxCsStopDelayW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TX_CS_START_DELAY` reader - configure the delay between tx_cs_o negedge and data tx start"]
pub type TxCsStartDelayR = crate::FieldReader<u16>;
#[doc = "Field `TX_CS_START_DELAY` writer - configure the delay between tx_cs_o negedge and data tx start"]
pub type TxCsStartDelayW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - configure the delay between data tx end and tx_cs_o posedge"]
    #[inline(always)]
    pub fn tx_cs_stop_delay(&self) -> TxCsStopDelayR {
        TxCsStopDelayR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - configure the delay between tx_cs_o negedge and data tx start"]
    #[inline(always)]
    pub fn tx_cs_start_delay(&self) -> TxCsStartDelayR {
        TxCsStartDelayR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - configure the delay between data tx end and tx_cs_o posedge"]
    #[inline(always)]
    pub fn tx_cs_stop_delay(&mut self) -> TxCsStopDelayW<'_, TxCsCfgSpec> {
        TxCsStopDelayW::new(self, 0)
    }
    #[doc = "Bits 16:31 - configure the delay between tx_cs_o negedge and data tx start"]
    #[inline(always)]
    pub fn tx_cs_start_delay(&mut self) -> TxCsStartDelayW<'_, TxCsCfgSpec> {
        TxCsStartDelayW::new(self, 16)
    }
}
#[doc = "Parallel IO tx_cs_o generate configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_cs_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_cs_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxCsCfgSpec;
impl crate::RegisterSpec for TxCsCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_cs_cfg::R`](R) reader structure"]
impl crate::Readable for TxCsCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_cs_cfg::W`](W) writer structure"]
impl crate::Writable for TxCsCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CS_CFG to value 0"]
impl crate::Resettable for TxCsCfgSpec {}
