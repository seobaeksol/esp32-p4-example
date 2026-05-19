#[doc = "Register `PWR_UP` reader"]
pub type R = crate::R<PwrUpSpec>;
#[doc = "Register `PWR_UP` writer"]
pub type W = crate::W<PwrUpSpec>;
#[doc = "Field `SHUTDOWNZ` reader - NA"]
pub type ShutdownzR = crate::BitReader;
#[doc = "Field `SHUTDOWNZ` writer - NA"]
pub type ShutdownzW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn shutdownz(&self) -> ShutdownzR {
        ShutdownzR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn shutdownz(&mut self) -> ShutdownzW<'_, PwrUpSpec> {
        ShutdownzW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_up::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_up::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrUpSpec;
impl crate::RegisterSpec for PwrUpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_up::R`](R) reader structure"]
impl crate::Readable for PwrUpSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_up::W`](W) writer structure"]
impl crate::Writable for PwrUpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWR_UP to value 0"]
impl crate::Resettable for PwrUpSpec {}
