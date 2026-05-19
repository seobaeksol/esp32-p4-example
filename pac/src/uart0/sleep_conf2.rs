#[doc = "Register `SLEEP_CONF2` reader"]
pub type R = crate::R<SleepConf2Spec>;
#[doc = "Register `SLEEP_CONF2` writer"]
pub type W = crate::W<SleepConf2Spec>;
#[doc = "Field `ACTIVE_THRESHOLD` reader - The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value."]
pub type ActiveThresholdR = crate::FieldReader<u16>;
#[doc = "Field `ACTIVE_THRESHOLD` writer - The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value."]
pub type ActiveThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `RX_WAKE_UP_THRHD` reader - In wake up mode 1 this field is used to set the received data number threshold to wake up chip."]
pub type RxWakeUpThrhdR = crate::FieldReader;
#[doc = "Field `RX_WAKE_UP_THRHD` writer - In wake up mode 1 this field is used to set the received data number threshold to wake up chip."]
pub type RxWakeUpThrhdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WK_CHAR_NUM` reader - This register is used to select number of wake up char."]
pub type WkCharNumR = crate::FieldReader;
#[doc = "Field `WK_CHAR_NUM` writer - This register is used to select number of wake up char."]
pub type WkCharNumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WK_CHAR_MASK` reader - This register is used to mask wake up char."]
pub type WkCharMaskR = crate::FieldReader;
#[doc = "Field `WK_CHAR_MASK` writer - This register is used to mask wake up char."]
pub type WkCharMaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WK_MODE_SEL` reader - This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than"]
pub type WkModeSelR = crate::FieldReader;
#[doc = "Field `WK_MODE_SEL` writer - This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than"]
pub type WkModeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:9 - The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value."]
    #[inline(always)]
    pub fn active_threshold(&self) -> ActiveThresholdR {
        ActiveThresholdR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:17 - In wake up mode 1 this field is used to set the received data number threshold to wake up chip."]
    #[inline(always)]
    pub fn rx_wake_up_thrhd(&self) -> RxWakeUpThrhdR {
        RxWakeUpThrhdR::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:20 - This register is used to select number of wake up char."]
    #[inline(always)]
    pub fn wk_char_num(&self) -> WkCharNumR {
        WkCharNumR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:25 - This register is used to mask wake up char."]
    #[inline(always)]
    pub fn wk_char_mask(&self) -> WkCharMaskR {
        WkCharMaskR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:27 - This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than"]
    #[inline(always)]
    pub fn wk_mode_sel(&self) -> WkModeSelR {
        WkModeSelR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - The uart is activated from light sleeping mode when the input rxd edge changes more times than this register value."]
    #[inline(always)]
    pub fn active_threshold(&mut self) -> ActiveThresholdW<'_, SleepConf2Spec> {
        ActiveThresholdW::new(self, 0)
    }
    #[doc = "Bits 10:17 - In wake up mode 1 this field is used to set the received data number threshold to wake up chip."]
    #[inline(always)]
    pub fn rx_wake_up_thrhd(&mut self) -> RxWakeUpThrhdW<'_, SleepConf2Spec> {
        RxWakeUpThrhdW::new(self, 10)
    }
    #[doc = "Bits 18:20 - This register is used to select number of wake up char."]
    #[inline(always)]
    pub fn wk_char_num(&mut self) -> WkCharNumW<'_, SleepConf2Spec> {
        WkCharNumW::new(self, 18)
    }
    #[doc = "Bits 21:25 - This register is used to mask wake up char."]
    #[inline(always)]
    pub fn wk_char_mask(&mut self) -> WkCharMaskW<'_, SleepConf2Spec> {
        WkCharMaskW::new(self, 21)
    }
    #[doc = "Bits 26:27 - This register is used to select wake up mode. 0: RXD toggling to wake up. 1: received data number larger than"]
    #[inline(always)]
    pub fn wk_mode_sel(&mut self) -> WkModeSelW<'_, SleepConf2Spec> {
        WkModeSelW::new(self, 26)
    }
}
#[doc = "UART sleep configure register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SleepConf2Spec;
impl crate::RegisterSpec for SleepConf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_conf2::R`](R) reader structure"]
impl crate::Readable for SleepConf2Spec {}
#[doc = "`write(|w| ..)` method takes [`sleep_conf2::W`](W) writer structure"]
impl crate::Writable for SleepConf2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLEEP_CONF2 to value 0x0014_04f0"]
impl crate::Resettable for SleepConf2Spec {
    const RESET_VALUE: u32 = 0x0014_04f0;
}
