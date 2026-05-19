#[doc = "Register `GPIO_O_HYS_CTRL0` reader"]
pub type R = crate::R<GpioOHysCtrl0Spec>;
#[doc = "Register `GPIO_O_HYS_CTRL0` writer"]
pub type W = crate::W<GpioOHysCtrl0Spec>;
#[doc = "Field `REG_GPIO_0_HYS_LOW` reader - hys control for gpio47~16"]
pub type RegGpio0HysLowR = crate::FieldReader<u32>;
#[doc = "Field `REG_GPIO_0_HYS_LOW` writer - hys control for gpio47~16"]
pub type RegGpio0HysLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - hys control for gpio47~16"]
    #[inline(always)]
    pub fn reg_gpio_0_hys_low(&self) -> RegGpio0HysLowR {
        RegGpio0HysLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - hys control for gpio47~16"]
    #[inline(always)]
    pub fn reg_gpio_0_hys_low(&mut self) -> RegGpio0HysLowW<'_, GpioOHysCtrl0Spec> {
        RegGpio0HysLowW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio_o_hys_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio_o_hys_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioOHysCtrl0Spec;
impl crate::RegisterSpec for GpioOHysCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_o_hys_ctrl0::R`](R) reader structure"]
impl crate::Readable for GpioOHysCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio_o_hys_ctrl0::W`](W) writer structure"]
impl crate::Writable for GpioOHysCtrl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO_O_HYS_CTRL0 to value 0"]
impl crate::Resettable for GpioOHysCtrl0Spec {}
