#[doc = "Register `CONFIG2` reader"]
pub type R = crate::R<Config2Spec>;
#[doc = "Register `CONFIG2` writer"]
pub type W = crate::W<Config2Spec>;
#[doc = "Field `WDT_STG1_HOLD` reader - need_des"]
pub type WdtStg1HoldR = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG1_HOLD` writer - need_des"]
pub type WdtStg1HoldW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wdt_stg1_hold(&self) -> WdtStg1HoldR {
        WdtStg1HoldR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wdt_stg1_hold(&mut self) -> WdtStg1HoldW<'_, Config2Spec> {
        WdtStg1HoldW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`config2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Config2Spec;
impl crate::RegisterSpec for Config2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config2::R`](R) reader structure"]
impl crate::Readable for Config2Spec {}
#[doc = "`write(|w| ..)` method takes [`config2::W`](W) writer structure"]
impl crate::Writable for Config2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG2 to value 0x0001_3880"]
impl crate::Resettable for Config2Spec {
    const RESET_VALUE: u32 = 0x0001_3880;
}
