#[doc = "Register `ETM_TASK_P12_CFG` reader"]
pub type R = crate::R<EtmTaskP12CfgSpec>;
#[doc = "Register `ETM_TASK_P12_CFG` writer"]
pub type W = crate::W<EtmTaskP12CfgSpec>;
#[doc = "Field `GPIO_EN(48-51)` reader - Enable bit of GPIO response etm task."]
pub type GpioEnR = crate::BitReader;
#[doc = "Field `GPIO_EN(48-51)` writer - Enable bit of GPIO response etm task."]
pub type GpioEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO_SEL(48-51)` reader - GPIO choose a etm task channel."]
pub type GpioSelR = crate::FieldReader;
#[doc = "Field `GPIO_SEL(48-51)` writer - GPIO choose a etm task channel."]
pub type GpioSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Enable bit of GPIO response etm task."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO48_EN` field.</div>"]
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
    pub fn gpio48_en(&self) -> GpioEnR {
        GpioEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio49_en(&self) -> GpioEnR {
        GpioEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio50_en(&self) -> GpioEnR {
        GpioEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio51_en(&self) -> GpioEnR {
        GpioEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "GPIO choose a etm task channel."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO48_SEL` field.</div>"]
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
    pub fn gpio48_sel(&self) -> GpioSelR {
        GpioSelR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio49_sel(&self) -> GpioSelR {
        GpioSelR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio50_sel(&self) -> GpioSelR {
        GpioSelR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio51_sel(&self) -> GpioSelR {
        GpioSelR::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Enable bit of GPIO response etm task."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO48_EN` field.</div>"]
    #[inline(always)]
    pub fn gpio_en(&mut self, n: u8) -> GpioEnW<'_, EtmTaskP12CfgSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        GpioEnW::new(self, n * 8)
    }
    #[doc = "Bit 0 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio48_en(&mut self) -> GpioEnW<'_, EtmTaskP12CfgSpec> {
        GpioEnW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio49_en(&mut self) -> GpioEnW<'_, EtmTaskP12CfgSpec> {
        GpioEnW::new(self, 8)
    }
    #[doc = "Bit 16 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio50_en(&mut self) -> GpioEnW<'_, EtmTaskP12CfgSpec> {
        GpioEnW::new(self, 16)
    }
    #[doc = "Bit 24 - Enable bit of GPIO response etm task."]
    #[inline(always)]
    pub fn gpio51_en(&mut self) -> GpioEnW<'_, EtmTaskP12CfgSpec> {
        GpioEnW::new(self, 24)
    }
    #[doc = "GPIO choose a etm task channel."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `GPIO48_SEL` field.</div>"]
    #[inline(always)]
    pub fn gpio_sel(&mut self, n: u8) -> GpioSelW<'_, EtmTaskP12CfgSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        GpioSelW::new(self, n * 8 + 1)
    }
    #[doc = "Bits 1:3 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio48_sel(&mut self) -> GpioSelW<'_, EtmTaskP12CfgSpec> {
        GpioSelW::new(self, 1)
    }
    #[doc = "Bits 9:11 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio49_sel(&mut self) -> GpioSelW<'_, EtmTaskP12CfgSpec> {
        GpioSelW::new(self, 9)
    }
    #[doc = "Bits 17:19 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio50_sel(&mut self) -> GpioSelW<'_, EtmTaskP12CfgSpec> {
        GpioSelW::new(self, 17)
    }
    #[doc = "Bits 25:27 - GPIO choose a etm task channel."]
    #[inline(always)]
    pub fn gpio51_sel(&mut self) -> GpioSelW<'_, EtmTaskP12CfgSpec> {
        GpioSelW::new(self, 25)
    }
}
#[doc = "Etm Configure Register to decide which GPIO been chosen\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_task_p12_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_task_p12_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmTaskP12CfgSpec;
impl crate::RegisterSpec for EtmTaskP12CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_task_p12_cfg::R`](R) reader structure"]
impl crate::Readable for EtmTaskP12CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`etm_task_p12_cfg::W`](W) writer structure"]
impl crate::Writable for EtmTaskP12CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_TASK_P12_CFG to value 0"]
impl crate::Resettable for EtmTaskP12CfgSpec {}
