#[doc = "Register `TX_START_CFG` reader"]
pub type R = crate::R<TxStartCfgSpec>;
#[doc = "Register `TX_START_CFG` writer"]
pub type W = crate::W<TxStartCfgSpec>;
#[doc = "Field `TX_START` reader - Write 1 to start tx data transmit."]
pub type TxStartR = crate::BitReader;
#[doc = "Field `TX_START` writer - Write 1 to start tx data transmit."]
pub type TxStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Write 1 to start tx data transmit."]
    #[inline(always)]
    pub fn tx_start(&self) -> TxStartR {
        TxStartR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Write 1 to start tx data transmit."]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TxStartW<'_, TxStartCfgSpec> {
        TxStartW::new(self, 31)
    }
}
#[doc = "Parallel TX Start configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_start_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_start_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxStartCfgSpec;
impl crate::RegisterSpec for TxStartCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_start_cfg::R`](R) reader structure"]
impl crate::Readable for TxStartCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_start_cfg::W`](W) writer structure"]
impl crate::Writable for TxStartCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_START_CFG to value 0"]
impl crate::Resettable for TxStartCfgSpec {}
