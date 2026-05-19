#[doc = "Register `ETM_TASK_P1_CFG` reader"]
pub type R = crate::R<EtmTaskP1CfgSpec>;
#[doc = "Register `ETM_TASK_P1_CFG` writer"]
pub type W = crate::W<EtmTaskP1CfgSpec>;
#[doc = "Field `GPIO_EN(4-7)` reader - Enable bit of GPIO response etm task."]
pub type GpioEnR = crate::BitReader;
#[doc = "Field `GPIO_EN(4-7)` writer - Enable bit of GPIO response etm task."]
pub type GpioEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_SEL(4-7)` reader - GPIO choose a etm task channel."]
pub type GpioSelR = crate::FieldReader;
#[doc = "Field `GPIO_SEL(4-7)` writer - GPIO choose a etm task channel."]
pub type GpioSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Enable bit of GPIO response etm task."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO4_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&self, n: u8) -> GpioEnR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        GpioEnR::new(((self.bits >> (n * 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio_en_iter(&self) -> impl Iterator<Item = GpioEnR> + '_ {
        (0..4).map(move |n| GpioEnR::new(((self.bits >> (n * 8)) & 1) != 0))
    }
    #[doc = "Bit 0 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio4_en(&self) -> GpioEnR {
        GpioEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio5_en(&self) -> GpioEnR {
        GpioEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio6_en(&self) -> GpioEnR {
        GpioEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio7_en(&self) -> GpioEnR {
        GpioEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "GPIO choose a etm task channel."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO4_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&self, n: u8) -> GpioSelR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        GpioSelR::new(((self.bits >> (n * 8 + 1)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio_sel_iter(&self) -> impl Iterator<Item = GpioSelR> + '_ {
        (0..4).map(move |n| GpioSelR::new(((self.bits >> (n * 8 + 1)) & 7) as u8))
    }
    #[doc = "Bits 1:3 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio4_sel(&self) -> GpioSelR {
        GpioSelR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio5_sel(&self) -> GpioSelR {
        GpioSelR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio6_sel(&self) -> GpioSelR {
        GpioSelR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio7_sel(&self) -> GpioSelR {
        GpioSelR::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Enable bit of GPIO response etm task."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO4_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&mut self, n: u8) -> GpioEnW<'_, EtmTaskP1CfgSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        GpioEnW::new(self, n * 8)
    }
    #[doc = "Bit 0 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio4_en(&mut self) -> GpioEnW<'_, EtmTaskP1CfgSpec> {
        GpioEnW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio5_en(&mut self) -> GpioEnW<'_, EtmTaskP1CfgSpec> {
        GpioEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio6_en(&mut self) -> GpioEnW<'_, EtmTaskP1CfgSpec> {
        GpioEnW::new(self, 16)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio7_en(&mut self) -> GpioEnW<'_, EtmTaskP1CfgSpec> {
        GpioEnW::new(self, 24)
    }
    #[doc = "GPIO choose a etm task channel."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO4_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&mut self, n: u8) -> GpioSelW<'_, EtmTaskP1CfgSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        GpioSelW::new(self, n * 8 + 1)
    }
    #[doc = "Bits 1:3 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio4_sel(&mut self) -> GpioSelW<'_, EtmTaskP1CfgSpec> {
        GpioSelW::new(self, 1)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio5_sel(&mut self) -> GpioSelW<'_, EtmTaskP1CfgSpec> {
        GpioSelW::new(self, 9)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio6_sel(&mut self) -> GpioSelW<'_, EtmTaskP1CfgSpec> {
        GpioSelW::new(self, 17)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio7_sel(&mut self) -> GpioSelW<'_, EtmTaskP1CfgSpec> {
        GpioSelW::new(self, 25)
    }
}
#[doc = "Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p1_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p1_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmTaskP1CfgSpec;
impl crate::RegisterSpec for EtmTaskP1CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p1_cfg::R`](R) reader structure"]
impl crate::Readable for EtmTaskP1CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p1_cfg::W`](W) writer structure"]
impl crate::Writable for EtmTaskP1CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_TASK_P1_CFG to value 0"]
impl crate::Resettable for EtmTaskP1CfgSpec {}
