#[doc = "Register `CONFIG4` reader"]
pub type R = crate::R<Config4Spec>;
#[doc = "Register `CONFIG4` writer"]
pub type W = crate::W<Config4Spec>;
#[doc = "Field `WDT_STG3_HOLD` reader - need_des"]
pub type WdtStg3HoldR = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG3_HOLD` writer - need_des"]
pub type WdtStg3HoldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wdt_stg3_hold(&self) -> WdtStg3HoldR {
        WdtStg3HoldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wdt_stg3_hold(&mut self) -> WdtStg3HoldW<'_, Config4Spec> {
        WdtStg3HoldW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`config4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config4Spec;
impl crate::RegisterSpec for Config4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config4::R`](R) reader structure"]
impl crate::Readable for Config4Spec {}
#[doc = "`write(|w| ..)` method takes [`config4::W`](W) writer structure"]
impl crate::Writable for Config4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG4 to value 0x0fff"]
impl crate::Resettable for Config4Spec {
    const RESET_VALUE: u32 = 0x0fff;
}
