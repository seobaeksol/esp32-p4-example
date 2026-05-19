#[doc = "Register `GPIO%s` reader"]
pub type R = crate::R<GpioSpec>;
#[doc = "Register `GPIO%s` writer"]
pub type W = crate::W<GpioSpec>;
#[doc = "Field `MCU_OE` reader - Configures whether or not to enable the output of GPIOn in sleep mode. 0: Disable 1: Enable"]
pub type McuOeR = crate::BitReader;
#[doc = "Field `MCU_OE` writer - Configures whether or not to enable the output of GPIOn in sleep mode. 0: Disable 1: Enable"]
pub type McuOeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLP_SEL` reader - Configures whether or not to enter sleep mode for GPIOn. 0: Not enter 1: Enter"]
pub type SlpSelR = crate::BitReader;
#[doc = "Field `SLP_SEL` writer - Configures whether or not to enter sleep mode for GPIOn. 0: Not enter 1: Enter"]
pub type SlpSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_WPD` reader - Configure whether or not to enable pull-down resistor of GPIOn during sleep mode. 0: Disable 1: Enable"]
pub type McuWpdR = crate::BitReader;
#[doc = "Field `MCU_WPD` writer - Configure whether or not to enable pull-down resistor of GPIOn during sleep mode. 0: Disable 1: Enable"]
pub type McuWpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_WPU` reader - Configures whether or not to enable pull-up resistor of GPIOn during sleep mode. 0: Disable 1: Enable"]
pub type McuWpuR = crate::BitReader;
#[doc = "Field `MCU_WPU` writer - Configures whether or not to enable pull-up resistor of GPIOn during sleep mode. 0: Disable 1: Enable"]
pub type McuWpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_IE` reader - Configures whether or not to enable the input of GPIOn during sleep mode. 0: Disable 1: Enable"]
pub type McuIeR = crate::BitReader;
#[doc = "Field `MCU_IE` writer - Configures whether or not to enable the input of GPIOn during sleep mode. 0: Disable 1: Enable"]
pub type McuIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_DRV` reader - Configures the drive strength of GPIOn during sleep mode. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA"]
pub type McuDrvR = crate::FieldReader;
#[doc = "Field `MCU_DRV` writer - Configures the drive strength of GPIOn during sleep mode. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA"]
pub type McuDrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FUN_WPD` reader - Configures whether or not to enable pull-down resistor of GPIOn. 0: Disable 1: Enable"]
pub type FunWpdR = crate::BitReader;
#[doc = "Field `FUN_WPD` writer - Configures whether or not to enable pull-down resistor of GPIOn. 0: Disable 1: Enable"]
pub type FunWpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_WPU` reader - Configures whether or not enable pull-up resistor of GPIOn. 0: Disable 1: Enable"]
pub type FunWpuR = crate::BitReader;
#[doc = "Field `FUN_WPU` writer - Configures whether or not enable pull-up resistor of GPIOn. 0: Disable 1: Enable"]
pub type FunWpuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_IE` reader - Configures whether or not to enable input of GPIOn. 0: Disable 1: Enable"]
pub type FunIeR = crate::BitReader;
#[doc = "Field `FUN_IE` writer - Configures whether or not to enable input of GPIOn. 0: Disable 1: Enable"]
pub type FunIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUN_DRV` reader - Configures the drive strength of GPIOn. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA"]
pub type FunDrvR = crate::FieldReader;
#[doc = "Field `FUN_DRV` writer - Configures the drive strength of GPIOn. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA"]
pub type FunDrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCU_SEL` reader - Configures to select IO MUX function for this pin. 0: Select Function 0 1: Select Function 1 ......"]
pub type McuSelR = crate::FieldReader;
#[doc = "Field `MCU_SEL` writer - Configures to select IO MUX function for this pin. 0: Select Function 0 1: Select Function 1 ......"]
pub type McuSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FILTER_EN` reader - Configures whether or not to enable filter for pin input signals. 0: Disable 1: Enable"]
pub type FilterEnR = crate::BitReader;
#[doc = "Field `FILTER_EN` writer - Configures whether or not to enable filter for pin input signals. 0: Disable 1: Enable"]
pub type FilterEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable the output of GPIOn in sleep mode. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn mcu_oe(&self) -> McuOeR {
        McuOeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enter sleep mode for GPIOn. 0: Not enter 1: Enter"]
    #[inline(always)]
    pub fn slp_sel(&self) -> SlpSelR {
        SlpSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configure whether or not to enable pull-down resistor of GPIOn during sleep mode. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn mcu_wpd(&self) -> McuWpdR {
        McuWpdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to enable pull-up resistor of GPIOn during sleep mode. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn mcu_wpu(&self) -> McuWpuR {
        McuWpuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to enable the input of GPIOn during sleep mode. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn mcu_ie(&self) -> McuIeR {
        McuIeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Configures the drive strength of GPIOn during sleep mode. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA"]
    #[inline(always)]
    pub fn mcu_drv(&self) -> McuDrvR {
        McuDrvR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Configures whether or not to enable pull-down resistor of GPIOn. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn fun_wpd(&self) -> FunWpdR {
        FunWpdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures whether or not enable pull-up resistor of GPIOn. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn fun_wpu(&self) -> FunWpuR {
        FunWpuR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Configures whether or not to enable input of GPIOn. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn fun_ie(&self) -> FunIeR {
        FunIeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Configures the drive strength of GPIOn. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA"]
    #[inline(always)]
    pub fn fun_drv(&self) -> FunDrvR {
        FunDrvR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Configures to select IO MUX function for this pin. 0: Select Function 0 1: Select Function 1 ......"]
    #[inline(always)]
    pub fn mcu_sel(&self) -> McuSelR {
        McuSelR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Configures whether or not to enable filter for pin input signals. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn filter_en(&self) -> FilterEnR {
        FilterEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable the output of GPIOn in sleep mode. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn mcu_oe(&mut self) -> McuOeW<'_, GpioSpec> {
        McuOeW::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enter sleep mode for GPIOn. 0: Not enter 1: Enter"]
    #[inline(always)]
    pub fn slp_sel(&mut self) -> SlpSelW<'_, GpioSpec> {
        SlpSelW::new(self, 1)
    }
    #[doc = "Bit 2 - Configure whether or not to enable pull-down resistor of GPIOn during sleep mode. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn mcu_wpd(&mut self) -> McuWpdW<'_, GpioSpec> {
        McuWpdW::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable pull-up resistor of GPIOn during sleep mode. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn mcu_wpu(&mut self) -> McuWpuW<'_, GpioSpec> {
        McuWpuW::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable the input of GPIOn during sleep mode. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn mcu_ie(&mut self) -> McuIeW<'_, GpioSpec> {
        McuIeW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Configures the drive strength of GPIOn during sleep mode. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA"]
    #[inline(always)]
    pub fn mcu_drv(&mut self) -> McuDrvW<'_, GpioSpec> {
        McuDrvW::new(self, 5)
    }
    #[doc = "Bit 7 - Configures whether or not to enable pull-down resistor of GPIOn. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn fun_wpd(&mut self) -> FunWpdW<'_, GpioSpec> {
        FunWpdW::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not enable pull-up resistor of GPIOn. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn fun_wpu(&mut self) -> FunWpuW<'_, GpioSpec> {
        FunWpuW::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to enable input of GPIOn. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn fun_ie(&mut self) -> FunIeW<'_, GpioSpec> {
        FunIeW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Configures the drive strength of GPIOn. 0: ~5 mA 1: ~10 mA 2: ~20 mA 3: ~40 mA"]
    #[inline(always)]
    pub fn fun_drv(&mut self) -> FunDrvW<'_, GpioSpec> {
        FunDrvW::new(self, 10)
    }
    #[doc = "Bits 12:14 - Configures to select IO MUX function for this pin. 0: Select Function 0 1: Select Function 1 ......"]
    #[inline(always)]
    pub fn mcu_sel(&mut self) -> McuSelW<'_, GpioSpec> {
        McuSelW::new(self, 12)
    }
    #[doc = "Bit 15 - Configures whether or not to enable filter for pin input signals. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn filter_en(&mut self) -> FilterEnW<'_, GpioSpec> {
        FilterEnW::new(self, 15)
    }
}
#[doc = "IO_MUX Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioSpec;
impl crate::RegisterSpec for GpioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio::R`](R) reader structure"]
impl crate::Readable for GpioSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio::W`](W) writer structure"]
impl crate::Writable for GpioSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO%s to value 0"]
impl crate::Resettable for GpioSpec {}
