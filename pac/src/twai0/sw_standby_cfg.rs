#[doc = "Register `SW_STANDBY_CFG` reader"]
pub type R = crate::R<SwStandbyCfgSpec>;
#[doc = "Register `SW_STANDBY_CFG` writer"]
pub type W = crate::W<SwStandbyCfgSpec>;
#[doc = "Field `SW_STANDBY_EN` reader - Enable standby pin."]
pub type SwStandbyEnR = crate::BitReader;
#[doc = "Field `SW_STANDBY_EN` writer - Enable standby pin."]
pub type SwStandbyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_STANDBY_CLR` reader - Clear standby pin."]
pub type SwStandbyClrR = crate::BitReader;
#[doc = "Field `SW_STANDBY_CLR` writer - Clear standby pin."]
pub type SwStandbyClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable standby pin."]
    #[inline(always)]
    pub fn sw_standby_en(&self) -> SwStandbyEnR {
        SwStandbyEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear standby pin."]
    #[inline(always)]
    pub fn sw_standby_clr(&self) -> SwStandbyClrR {
        SwStandbyClrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable standby pin."]
    #[inline(always)]
    pub fn sw_standby_en(&mut self) -> SwStandbyEnW<'_, SwStandbyCfgSpec> {
        SwStandbyEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear standby pin."]
    #[inline(always)]
    pub fn sw_standby_clr(&mut self) -> SwStandbyClrW<'_, SwStandbyCfgSpec> {
        SwStandbyClrW::new(self, 1)
    }
}
#[doc = "Software configure standby pin directly.\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_standby_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_standby_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwStandbyCfgSpec;
impl crate::RegisterSpec for SwStandbyCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_standby_cfg::R`](R) reader structure"]
impl crate::Readable for SwStandbyCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_standby_cfg::W`](W) writer structure"]
impl crate::Writable for SwStandbyCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SW_STANDBY_CFG to value 0x02"]
impl crate::Resettable for SwStandbyCfgSpec {
    const RESET_VALUE: u32 = 0x02;
}
