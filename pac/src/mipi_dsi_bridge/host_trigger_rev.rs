#[doc = "Register `HOST_TRIGGER_REV` reader"]
pub type R = crate::R<HostTriggerRevSpec>;
#[doc = "Register `HOST_TRIGGER_REV` writer"]
pub type W = crate::W<HostTriggerRevSpec>;
#[doc = "Field `TX_TRIGGER_REV_EN` reader - tx_trigger reverse. 0: disable, 1: enable"]
pub type TxTriggerRevEnR = crate::BitReader;
#[doc = "Field `TX_TRIGGER_REV_EN` writer - tx_trigger reverse. 0: disable, 1: enable"]
pub type TxTriggerRevEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TRIGGER_REV_EN` reader - rx_trigger reverse. 0: disable, 1: enable"]
pub type RxTriggerRevEnR = crate::BitReader;
#[doc = "Field `RX_TRIGGER_REV_EN` writer - rx_trigger reverse. 0: disable, 1: enable"]
pub type RxTriggerRevEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - tx_trigger reverse. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tx_trigger_rev_en(&self) -> TxTriggerRevEnR {
        TxTriggerRevEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - rx_trigger reverse. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn rx_trigger_rev_en(&self) -> RxTriggerRevEnR {
        RxTriggerRevEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - tx_trigger reverse. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn tx_trigger_rev_en(&mut self) -> TxTriggerRevEnW<'_, HostTriggerRevSpec> {
        TxTriggerRevEnW::new(self, 0)
    }
    #[doc = "Bit 1 - rx_trigger reverse. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn rx_trigger_rev_en(&mut self) -> RxTriggerRevEnW<'_, HostTriggerRevSpec> {
        RxTriggerRevEnW::new(self, 1)
    }
}
#[doc = "dsi_bridge host trigger reverse control register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_trigger_rev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_trigger_rev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostTriggerRevSpec;
impl crate::RegisterSpec for HostTriggerRevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_trigger_rev::R`](R) reader structure"]
impl crate::Readable for HostTriggerRevSpec {}
#[doc = "`write(|w| ..)` method takes [`host_trigger_rev::W`](W) writer structure"]
impl crate::Writable for HostTriggerRevSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_TRIGGER_REV to value 0"]
impl crate::Resettable for HostTriggerRevSpec {}
