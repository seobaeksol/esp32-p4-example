#[doc = "Register `ETM_CONF` reader"]
pub type R = crate::R<EtmConfSpec>;
#[doc = "Register `ETM_CONF` writer"]
pub type W = crate::W<EtmConfSpec>;
#[doc = "Field `IN_ETM_EN` reader - Set this bit to 1 to enable ETM task function"]
pub type InEtmEnR = crate::BitReader;
#[doc = "Field `IN_ETM_EN` writer - Set this bit to 1 to enable ETM task function"]
pub type InEtmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ETM_LOOP_EN` reader - when this bit is 1, dscr can be processed after receiving a task"]
pub type InEtmLoopEnR = crate::BitReader;
#[doc = "Field `IN_ETM_LOOP_EN` writer - when this bit is 1, dscr can be processed after receiving a task"]
pub type InEtmLoopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_TASK_MAK` reader - ETM dscr_ready maximum cache numbers"]
pub type InDscrTaskMakR = crate::FieldReader;
#[doc = "Field `IN_DSCR_TASK_MAK` writer - ETM dscr_ready maximum cache numbers"]
pub type InDscrTaskMakW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Set this bit to 1 to enable ETM task function"]
    #[inline(always)]
    pub fn in_etm_en(&self) -> InEtmEnR {
        InEtmEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when this bit is 1, dscr can be processed after receiving a task"]
    #[inline(always)]
    pub fn in_etm_loop_en(&self) -> InEtmLoopEnR {
        InEtmLoopEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - ETM dscr_ready maximum cache numbers"]
    #[inline(always)]
    pub fn in_dscr_task_mak(&self) -> InDscrTaskMakR {
        InDscrTaskMakR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to enable ETM task function"]
    #[inline(always)]
    pub fn in_etm_en(&mut self) -> InEtmEnW<'_, EtmConfSpec> {
        InEtmEnW::new(self, 0)
    }
    #[doc = "Bit 1 - when this bit is 1, dscr can be processed after receiving a task"]
    #[inline(always)]
    pub fn in_etm_loop_en(&mut self) -> InEtmLoopEnW<'_, EtmConfSpec> {
        InEtmLoopEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - ETM dscr_ready maximum cache numbers"]
    #[inline(always)]
    pub fn in_dscr_task_mak(&mut self) -> InDscrTaskMakW<'_, EtmConfSpec> {
        InDscrTaskMakW::new(self, 2)
    }
}
#[doc = "RX CHx ETM config register\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmConfSpec;
impl crate::RegisterSpec for EtmConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_conf::R`](R) reader structure"]
impl crate::Readable for EtmConfSpec {}
#[doc = "`write(|w| ..)` method takes [`etm_conf::W`](W) writer structure"]
impl crate::Writable for EtmConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_CONF to value 0x04"]
impl crate::Resettable for EtmConfSpec {
    const RESET_VALUE: u32 = 0x04;
}
