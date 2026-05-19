#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `USE_XTAL` reader - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type UseXtalR = crate::BitReader;
#[doc = "Field `USE_XTAL` writer - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
pub type UseXtalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALARM_EN` reader - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
pub type AlarmEnR = crate::BitReader;
#[doc = "Field `ALARM_EN` writer - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
pub type AlarmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVCNT_RST` writer - When set, Timer %s 's clock divider counter will be reset."]
pub type DivcntRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVIDER` reader - Timer %s clock (T%s_clk) prescaler value."]
pub type DividerR = crate::FieldReader<u16>;
#[doc = "Field `DIVIDER` writer - Timer %s clock (T%s_clk) prescaler value."]
pub type DividerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AUTORELOAD` reader - When set, timer %s auto-reload at alarm is enabled."]
pub type AutoreloadR = crate::BitReader;
#[doc = "Field `AUTORELOAD` writer - When set, timer %s auto-reload at alarm is enabled."]
pub type AutoreloadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCREASE` reader - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
pub type IncreaseR = crate::BitReader;
#[doc = "Field `INCREASE` writer - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
pub type IncreaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN` reader - When set, the timer %s time-base counter is enabled."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - When set, the timer %s time-base counter is enabled."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 9 - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
    #[inline(always)]
    pub fn use_xtal(&self) -> UseXtalR {
        UseXtalR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
    #[inline(always)]
    pub fn alarm_en(&self) -> AlarmEnR {
        AlarmEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:28 - Timer %s clock (T%s_clk) prescaler value."]
    #[inline(always)]
    pub fn divider(&self) -> DividerR {
        DividerR::new(((self.bits >> 13) & 0xffff) as u16)
    }
    #[doc = "Bit 29 - When set, timer %s auto-reload at alarm is enabled."]
    #[inline(always)]
    pub fn autoreload(&self) -> AutoreloadR {
        AutoreloadR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
    #[inline(always)]
    pub fn increase(&self) -> IncreaseR {
        IncreaseR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When set, the timer %s time-base counter is enabled."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - 1: Use XTAL_CLK as the source clock of timer group. 0: Use APB_CLK as the source clock of timer group."]
    #[inline(always)]
    pub fn use_xtal(&mut self) -> UseXtalW<'_, ConfigSpec> {
        UseXtalW::new(self, 9)
    }
    #[doc = "Bit 10 - When set, the alarm is enabled. This bit is automatically cleared once an alarm occurs."]
    #[inline(always)]
    pub fn alarm_en(&mut self) -> AlarmEnW<'_, ConfigSpec> {
        AlarmEnW::new(self, 10)
    }
    #[doc = "Bit 12 - When set, Timer %s 's clock divider counter will be reset."]
    #[inline(always)]
    pub fn divcnt_rst(&mut self) -> DivcntRstW<'_, ConfigSpec> {
        DivcntRstW::new(self, 12)
    }
    #[doc = "Bits 13:28 - Timer %s clock (T%s_clk) prescaler value."]
    #[inline(always)]
    pub fn divider(&mut self) -> DividerW<'_, ConfigSpec> {
        DividerW::new(self, 13)
    }
    #[doc = "Bit 29 - When set, timer %s auto-reload at alarm is enabled."]
    #[inline(always)]
    pub fn autoreload(&mut self) -> AutoreloadW<'_, ConfigSpec> {
        AutoreloadW::new(self, 29)
    }
    #[doc = "Bit 30 - When set, the timer %s time-base counter will increment every clock tick. When cleared, the timer %s time-base counter will decrement."]
    #[inline(always)]
    pub fn increase(&mut self) -> IncreaseW<'_, ConfigSpec> {
        IncreaseW::new(self, 30)
    }
    #[doc = "Bit 31 - When set, the timer %s time-base counter is enabled."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<'_, ConfigSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "Timer 0 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0x6000_2000"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x6000_2000;
}
