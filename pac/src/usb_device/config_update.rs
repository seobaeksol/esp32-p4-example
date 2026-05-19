#[doc = "Register `CONFIG_UPDATE` writer"]
pub type W = crate::W<ConfigUpdateSpec>;
#[doc = "Field `CONFIG_UPDATE` writer - Write 1 to this register would update the value of configure registers from APB clock domain to 48MHz clock domain."]
pub type ConfigUpdateW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Write 1 to this register would update the value of configure registers from APB clock domain to 48MHz clock domain."]
    #[inline(always)]
    pub fn config_update(&mut self) -> ConfigUpdateW<'_, ConfigUpdateSpec> {
        ConfigUpdateW::new(self, 0)
    }
}
#[doc = "Configuration registers' value update\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config_update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigUpdateSpec;
impl crate::RegisterSpec for ConfigUpdateSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`config_update::W`](W) writer structure"]
impl crate::Writable for ConfigUpdateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG_UPDATE to value 0"]
impl crate::Resettable for ConfigUpdateSpec {}
