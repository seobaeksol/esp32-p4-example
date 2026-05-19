#[doc = "Register `WR_TIM_CONF2` reader"]
pub type R = crate::R<WrTimConf2Spec>;
#[doc = "Register `WR_TIM_CONF2` writer"]
pub type W = crate::W<WrTimConf2Spec>;
#[doc = "Field `PWR_OFF_NUM` reader - Configures the power outage time for VDDQ."]
pub type PwrOffNumR = crate::FieldReader<u16>;
#[doc = "Field `PWR_OFF_NUM` writer - Configures the power outage time for VDDQ."]
pub type PwrOffNumW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TPGM` reader - Configures the active programming time."]
pub type TpgmR = crate::FieldReader<u16>;
#[doc = "Field `TPGM` writer - Configures the active programming time."]
pub type TpgmW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the power outage time for VDDQ."]
    #[inline(always)]
    pub fn pwr_off_num(&self) -> PwrOffNumR {
        PwrOffNumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Configures the active programming time."]
    #[inline(always)]
    pub fn tpgm(&self) -> TpgmR {
        TpgmR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the power outage time for VDDQ."]
    #[inline(always)]
    pub fn pwr_off_num(&mut self) -> PwrOffNumW<'_, WrTimConf2Spec> {
        PwrOffNumW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Configures the active programming time."]
    #[inline(always)]
    pub fn tpgm(&mut self) -> TpgmW<'_, WrTimConf2Spec> {
        TpgmW::new(self, 16)
    }
}
#[doc = "Configurarion register 2 of eFuse programming timing parameters.\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_tim_conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_tim_conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrTimConf2Spec;
impl crate::RegisterSpec for WrTimConf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_tim_conf2::R`](R) reader structure"]
impl crate::Readable for WrTimConf2Spec {}
#[doc = "`write(|w| ..)` method takes [`wr_tim_conf2::W`](W) writer structure"]
impl crate::Writable for WrTimConf2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WR_TIM_CONF2 to value 0x00a0_0140"]
impl crate::Resettable for WrTimConf2Spec {
    const RESET_VALUE: u32 = 0x00a0_0140;
}
