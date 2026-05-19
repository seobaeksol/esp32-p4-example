#[doc = "Register `WAKEUP1` reader"]
pub type R = crate::R<Wakeup1Spec>;
#[doc = "Register `WAKEUP1` writer"]
pub type W = crate::W<Wakeup1Spec>;
#[doc = "Field `SAR1_WAKEUP_TH_LOW` reader - Lower threshold."]
pub type Sar1WakeupThLowR = crate::FieldReader<u16>;
#[doc = "Field `SAR1_WAKEUP_TH_LOW` writer - Lower threshold."]
pub type Sar1WakeupThLowW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SAR1_WAKEUP_TH_HIGH` reader - Upper threshold."]
pub type Sar1WakeupThHighR = crate::FieldReader<u16>;
#[doc = "Field `SAR1_WAKEUP_TH_HIGH` writer - Upper threshold."]
pub type Sar1WakeupThHighW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SAR1_WAKEUP_OVER_UPPER_TH` reader - Indicates that this wakeup event arose from exceeding upper threshold."]
pub type Sar1WakeupOverUpperThR = crate::BitReader;
#[doc = "Field `SAR1_WAKEUP_EN` reader - Wakeup function enable."]
pub type Sar1WakeupEnR = crate::BitReader;
#[doc = "Field `SAR1_WAKEUP_EN` writer - Wakeup function enable."]
pub type Sar1WakeupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_WAKEUP_MODE` reader - 0:absolute value comparison mode. 1: relative value comparison mode."]
pub type Sar1WakeupModeR = crate::BitReader;
#[doc = "Field `SAR1_WAKEUP_MODE` writer - 0:absolute value comparison mode. 1: relative value comparison mode."]
pub type Sar1WakeupModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Lower threshold."]
    #[inline(always)]
    pub fn sar1_wakeup_th_low(&self) -> Sar1WakeupThLowR {
        Sar1WakeupThLowR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 14:25 - Upper threshold."]
    #[inline(always)]
    pub fn sar1_wakeup_th_high(&self) -> Sar1WakeupThHighR {
        Sar1WakeupThHighR::new(((self.bits >> 14) & 0x0fff) as u16)
    }
    #[doc = "Bit 29 - Indicates that this wakeup event arose from exceeding upper threshold."]
    #[inline(always)]
    pub fn sar1_wakeup_over_upper_th(&self) -> Sar1WakeupOverUpperThR {
        Sar1WakeupOverUpperThR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Wakeup function enable."]
    #[inline(always)]
    pub fn sar1_wakeup_en(&self) -> Sar1WakeupEnR {
        Sar1WakeupEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0:absolute value comparison mode. 1: relative value comparison mode."]
    #[inline(always)]
    pub fn sar1_wakeup_mode(&self) -> Sar1WakeupModeR {
        Sar1WakeupModeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Lower threshold."]
    #[inline(always)]
    pub fn sar1_wakeup_th_low(&mut self) -> Sar1WakeupThLowW<'_, Wakeup1Spec> {
        Sar1WakeupThLowW::new(self, 0)
    }
    #[doc = "Bits 14:25 - Upper threshold."]
    #[inline(always)]
    pub fn sar1_wakeup_th_high(&mut self) -> Sar1WakeupThHighW<'_, Wakeup1Spec> {
        Sar1WakeupThHighW::new(self, 14)
    }
    #[doc = "Bit 30 - Wakeup function enable."]
    #[inline(always)]
    pub fn sar1_wakeup_en(&mut self) -> Sar1WakeupEnW<'_, Wakeup1Spec> {
        Sar1WakeupEnW::new(self, 30)
    }
    #[doc = "Bit 31 - 0:absolute value comparison mode. 1: relative value comparison mode."]
    #[inline(always)]
    pub fn sar1_wakeup_mode(&mut self) -> Sar1WakeupModeW<'_, Wakeup1Spec> {
        Sar1WakeupModeW::new(self, 31)
    }
}
#[doc = "ADC1 wakeup configuration registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`wakeup1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wakeup1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wakeup1Spec;
impl crate::RegisterSpec for Wakeup1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wakeup1::R`](R) reader structure"]
impl crate::Readable for Wakeup1Spec {}
#[doc = "`write(|w| ..)` method takes [`wakeup1::W`](W) writer structure"]
impl crate::Writable for Wakeup1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WAKEUP1 to value 0x03ff_c000"]
impl crate::Resettable for Wakeup1Spec {
    const RESET_VALUE: u32 = 0x03ff_c000;
}
