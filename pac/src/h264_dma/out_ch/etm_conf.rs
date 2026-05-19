#[doc = "Register `ETM_CONF` reader"]
pub type R = crate::R<EtmConfSpec>;
#[doc = "Register `ETM_CONF` writer"]
pub type W = crate::W<EtmConfSpec>;
#[doc = "Field `OUT_ETM_EN` reader - Set this bit to 1 to enable ETM task function"]
pub type OutEtmEnR = crate::BitReader;
#[doc = "Field `OUT_ETM_EN` writer - Set this bit to 1 to enable ETM task function"]
pub type OutEtmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ETM_LOOP_EN` reader - when this bit is 1, dscr can be processed after receiving a task"]
pub type OutEtmLoopEnR = crate::BitReader;
#[doc = "Field `OUT_ETM_LOOP_EN` writer - when this bit is 1, dscr can be processed after receiving a task"]
pub type OutEtmLoopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_TASK_MAK` reader - ETM dscr_ready maximum cache numbers"]
pub type OutDscrTaskMakR = crate::FieldReader;
#[doc = "Field `OUT_DSCR_TASK_MAK` writer - ETM dscr_ready maximum cache numbers"]
pub type OutDscrTaskMakW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Set this bit to 1 to enable ETM task function"]
    #[inline(always)]
    pub fn out_etm_en(&self) -> OutEtmEnR {
        OutEtmEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when this bit is 1, dscr can be processed after receiving a task"]
    #[inline(always)]
    pub fn out_etm_loop_en(&self) -> OutEtmLoopEnR {
        OutEtmLoopEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - ETM dscr_ready maximum cache numbers"]
    #[inline(always)]
    pub fn out_dscr_task_mak(&self) -> OutDscrTaskMakR {
        OutDscrTaskMakR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to enable ETM task function"]
    #[inline(always)]
    pub fn out_etm_en(&mut self) -> OutEtmEnW<'_, EtmConfSpec> {
        OutEtmEnW::new(self, 0)
    }
    #[doc = "Bit 1 - when this bit is 1, dscr can be processed after receiving a task"]
    #[inline(always)]
    pub fn out_etm_loop_en(&mut self) -> OutEtmLoopEnW<'_, EtmConfSpec> {
        OutEtmLoopEnW::new(self, 1)
    }
    #[doc = "Bits 2:3 - ETM dscr_ready maximum cache numbers"]
    #[inline(always)]
    pub fn out_dscr_task_mak(&mut self) -> OutDscrTaskMakW<'_, EtmConfSpec> {
        OutDscrTaskMakW::new(self, 2)
    }
}
#[doc = "TX CHx ETM config register\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
