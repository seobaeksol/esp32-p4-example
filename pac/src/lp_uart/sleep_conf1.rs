#[doc = "Register `SLEEP_CONF1` reader"]
pub type R = crate::R<SleepConf1Spec>;
#[doc = "Register `SLEEP_CONF1` writer"]
pub type W = crate::W<SleepConf1Spec>;
#[doc = "Field `WK_CHAR0` reader - This register restores the specified char0 to wake up"]
pub type WkChar0R = crate::FieldReader;
#[doc = "Field `WK_CHAR0` writer - This register restores the specified char0 to wake up"]
pub type WkChar0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register restores the specified char0 to wake up"]
    #[inline(always)]
    pub fn wk_char0(&self) -> WkChar0R {
        WkChar0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register restores the specified char0 to wake up"]
    #[inline(always)]
    pub fn wk_char0(&mut self) -> WkChar0W<'_, SleepConf1Spec> {
        WkChar0W::new(self, 0)
    }
}
#[doc = "UART sleep configure register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SleepConf1Spec;
impl crate::RegisterSpec for SleepConf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_conf1::R`](R) reader structure"]
impl crate::Readable for SleepConf1Spec {}
#[doc = "`write(|w| ..)` method takes [`sleep_conf1::W`](W) writer structure"]
impl crate::Writable for SleepConf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLEEP_CONF1 to value 0"]
impl crate::Resettable for SleepConf1Spec {}
