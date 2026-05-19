#[doc = "Register `RNG_CFG` reader"]
pub type R = crate::R<RngCfgSpec>;
#[doc = "Register `RNG_CFG` writer"]
pub type W = crate::W<RngCfgSpec>;
#[doc = "Field `RNG_TIMER_EN` reader - enable rng timer"]
pub type RngTimerEnR = crate::BitReader;
#[doc = "Field `RNG_TIMER_EN` writer - enable rng timer"]
pub type RngTimerEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_TIMER_PSCALE` reader - configure ng timer pscale"]
pub type RngTimerPscaleR = crate::FieldReader;
#[doc = "Field `RNG_TIMER_PSCALE` writer - configure ng timer pscale"]
pub type RngTimerPscaleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RNG_SAR_ENABLE` reader - enable rng_saradc"]
pub type RngSarEnableR = crate::BitReader;
#[doc = "Field `RNG_SAR_ENABLE` writer - enable rng_saradc"]
pub type RngSarEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNG_SAR_DATA` reader - debug rng sar sample cnt"]
pub type RngSarDataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - enable rng timer"]
    #[inline(always)]
    pub fn rng_timer_en(&self) -> RngTimerEnR {
        RngTimerEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - configure ng timer pscale"]
    #[inline(always)]
    pub fn rng_timer_pscale(&self) -> RngTimerPscaleR {
        RngTimerPscaleR::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bit 9 - enable rng_saradc"]
    #[inline(always)]
    pub fn rng_sar_enable(&self) -> RngSarEnableR {
        RngSarEnableR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:28 - debug rng sar sample cnt"]
    #[inline(always)]
    pub fn rng_sar_data(&self) -> RngSarDataR {
        RngSarDataR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - enable rng timer"]
    #[inline(always)]
    pub fn rng_timer_en(&mut self) -> RngTimerEnW<'_, RngCfgSpec> {
        RngTimerEnW::new(self, 0)
    }
    #[doc = "Bits 1:8 - configure ng timer pscale"]
    #[inline(always)]
    pub fn rng_timer_pscale(&mut self) -> RngTimerPscaleW<'_, RngCfgSpec> {
        RngTimerPscaleW::new(self, 1)
    }
    #[doc = "Bit 9 - enable rng_saradc"]
    #[inline(always)]
    pub fn rng_sar_enable(&mut self) -> RngSarEnableW<'_, RngCfgSpec> {
        RngSarEnableW::new(self, 9)
    }
}
#[doc = "rng cfg register\n\nYou can [`read`](crate::Reg::read) this register and get [`rng_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rng_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RngCfgSpec;
impl crate::RegisterSpec for RngCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rng_cfg::R`](R) reader structure"]
impl crate::Readable for RngCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`rng_cfg::W`](W) writer structure"]
impl crate::Writable for RngCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RNG_CFG to value 0x03"]
impl crate::Resettable for RngCfgSpec {
    const RESET_VALUE: u32 = 0x03;
}
