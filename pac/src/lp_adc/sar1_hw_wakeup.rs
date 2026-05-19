#[doc = "Register `SAR1_HW_WAKEUP` reader"]
pub type R = crate::R<Sar1HwWakeupSpec>;
#[doc = "Register `SAR1_HW_WAKEUP` writer"]
pub type W = crate::W<Sar1HwWakeupSpec>;
#[doc = "Field `ADC1_HW_READ_EN_I` reader - Enable hardware automatic sampling."]
pub type Adc1HwReadEnIR = crate::BitReader;
#[doc = "Field `ADC1_HW_READ_EN_I` writer - Enable hardware automatic sampling."]
pub type Adc1HwReadEnIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_HW_READ_RATE_I` reader - Hardware automatic sampling rate."]
pub type Adc1HwReadRateIR = crate::FieldReader<u16>;
#[doc = "Field `ADC1_HW_READ_RATE_I` writer - Hardware automatic sampling rate."]
pub type Adc1HwReadRateIW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Enable hardware automatic sampling."]
    #[inline(always)]
    pub fn adc1_hw_read_en_i(&self) -> Adc1HwReadEnIR {
        Adc1HwReadEnIR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:16 - Hardware automatic sampling rate."]
    #[inline(always)]
    pub fn adc1_hw_read_rate_i(&self) -> Adc1HwReadRateIR {
        Adc1HwReadRateIR::new(((self.bits >> 1) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable hardware automatic sampling."]
    #[inline(always)]
    pub fn adc1_hw_read_en_i(&mut self) -> Adc1HwReadEnIW<'_, Sar1HwWakeupSpec> {
        Adc1HwReadEnIW::new(self, 0)
    }
    #[doc = "Bits 1:16 - Hardware automatic sampling rate."]
    #[inline(always)]
    pub fn adc1_hw_read_rate_i(&mut self) -> Adc1HwReadRateIW<'_, Sar1HwWakeupSpec> {
        Adc1HwReadRateIW::new(self, 1)
    }
}
#[doc = "Hardware automatic sampling registers for wakeup function.\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_hw_wakeup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_hw_wakeup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar1HwWakeupSpec;
impl crate::RegisterSpec for Sar1HwWakeupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar1_hw_wakeup::R`](R) reader structure"]
impl crate::Readable for Sar1HwWakeupSpec {}
#[doc = "`write(|w| ..)` method takes [`sar1_hw_wakeup::W`](W) writer structure"]
impl crate::Writable for Sar1HwWakeupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR1_HW_WAKEUP to value 0xc8"]
impl crate::Resettable for Sar1HwWakeupSpec {
    const RESET_VALUE: u32 = 0xc8;
}
