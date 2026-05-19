#[doc = "Register `SLEEP_CONF0` reader"]
pub type R = crate::R<SleepConf0Spec>;
#[doc = "Register `SLEEP_CONF0` writer"]
pub type W = crate::W<SleepConf0Spec>;
#[doc = "Field `WK_CHAR1` reader - This register restores the specified wake up char1 to wake up"]
pub type WkChar1R = crate::FieldReader;
#[doc = "Field `WK_CHAR1` writer - This register restores the specified wake up char1 to wake up"]
pub type WkChar1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WK_CHAR2` reader - This register restores the specified wake up char2 to wake up"]
pub type WkChar2R = crate::FieldReader;
#[doc = "Field `WK_CHAR2` writer - This register restores the specified wake up char2 to wake up"]
pub type WkChar2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WK_CHAR3` reader - This register restores the specified wake up char3 to wake up"]
pub type WkChar3R = crate::FieldReader;
#[doc = "Field `WK_CHAR3` writer - This register restores the specified wake up char3 to wake up"]
pub type WkChar3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WK_CHAR4` reader - This register restores the specified wake up char4 to wake up"]
pub type WkChar4R = crate::FieldReader;
#[doc = "Field `WK_CHAR4` writer - This register restores the specified wake up char4 to wake up"]
pub type WkChar4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register restores the specified wake up char1 to wake up"]
    #[inline(always)]
    pub fn wk_char1(&self) -> WkChar1R {
        WkChar1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This register restores the specified wake up char2 to wake up"]
    #[inline(always)]
    pub fn wk_char2(&self) -> WkChar2R {
        WkChar2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - This register restores the specified wake up char3 to wake up"]
    #[inline(always)]
    pub fn wk_char3(&self) -> WkChar3R {
        WkChar3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - This register restores the specified wake up char4 to wake up"]
    #[inline(always)]
    pub fn wk_char4(&self) -> WkChar4R {
        WkChar4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register restores the specified wake up char1 to wake up"]
    #[inline(always)]
    pub fn wk_char1(&mut self) -> WkChar1W<'_, SleepConf0Spec> {
        WkChar1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - This register restores the specified wake up char2 to wake up"]
    #[inline(always)]
    pub fn wk_char2(&mut self) -> WkChar2W<'_, SleepConf0Spec> {
        WkChar2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - This register restores the specified wake up char3 to wake up"]
    #[inline(always)]
    pub fn wk_char3(&mut self) -> WkChar3W<'_, SleepConf0Spec> {
        WkChar3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - This register restores the specified wake up char4 to wake up"]
    #[inline(always)]
    pub fn wk_char4(&mut self) -> WkChar4W<'_, SleepConf0Spec> {
        WkChar4W::new(self, 24)
    }
}
#[doc = "UART sleep configure register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SleepConf0Spec;
impl crate::RegisterSpec for SleepConf0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_conf0::R`](R) reader structure"]
impl crate::Readable for SleepConf0Spec {}
#[doc = "`write(|w| ..)` method takes [`sleep_conf0::W`](W) writer structure"]
impl crate::Writable for SleepConf0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLEEP_CONF0 to value 0"]
impl crate::Resettable for SleepConf0Spec {}
