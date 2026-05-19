#[doc = "Register `BUS_FREE_TIME` reader"]
pub type R = crate::R<BusFreeTimeSpec>;
#[doc = "Register `BUS_FREE_TIME` writer"]
pub type W = crate::W<BusFreeTimeSpec>;
#[doc = "Field `REG_BUS_FREE_TIME` reader - I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing."]
pub type RegBusFreeTimeR = crate::FieldReader<u16>;
#[doc = "Field `REG_BUS_FREE_TIME` writer - I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing."]
pub type RegBusFreeTimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing."]
    #[inline(always)]
    pub fn reg_bus_free_time(&self) -> RegBusFreeTimeR {
        RegBusFreeTimeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing."]
    #[inline(always)]
    pub fn reg_bus_free_time(&mut self) -> RegBusFreeTimeW<'_, BusFreeTimeSpec> {
        RegBusFreeTimeW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_free_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_free_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusFreeTimeSpec;
impl crate::RegisterSpec for BusFreeTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_free_time::R`](R) reader structure"]
impl crate::Readable for BusFreeTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`bus_free_time::W`](W) writer structure"]
impl crate::Writable for BusFreeTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUS_FREE_TIME to value 0x05"]
impl crate::Resettable for BusFreeTimeSpec {
    const RESET_VALUE: u32 = 0x05;
}
