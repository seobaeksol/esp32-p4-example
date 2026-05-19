#[doc = "Register `CONFIG1` reader"]
pub type R = crate::R<Config1Spec>;
#[doc = "Register `CONFIG1` writer"]
pub type W = crate::W<Config1Spec>;
#[doc = "Field `WDT_STG0_HOLD` reader - need_des"]
pub type WdtStg0HoldR = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG0_HOLD` writer - need_des"]
pub type WdtStg0HoldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wdt_stg0_hold(&self) -> WdtStg0HoldR {
        WdtStg0HoldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wdt_stg0_hold(&mut self) -> WdtStg0HoldW<'_, Config1Spec> {
        WdtStg0HoldW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`config1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config1Spec;
impl crate::RegisterSpec for Config1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config1::R`](R) reader structure"]
impl crate::Readable for Config1Spec {}
#[doc = "`write(|w| ..)` method takes [`config1::W`](W) writer structure"]
impl crate::Writable for Config1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG1 to value 0x0003_0d40"]
impl crate::Resettable for Config1Spec {
    const RESET_VALUE: u32 = 0x0003_0d40;
}
