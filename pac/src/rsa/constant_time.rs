#[doc = "Register `CONSTANT_TIME` reader"]
pub type R = crate::R<ConstantTimeSpec>;
#[doc = "Register `CONSTANT_TIME` writer"]
pub type W = crate::W<ConstantTimeSpec>;
#[doc = "Field `CONSTANT_TIME` reader - Configures the constant_time option. \\\\ 0: Acceleration\\\\ 1: No acceleration (default)\\\\"]
pub type ConstantTimeR = crate::BitReader;
#[doc = "Field `CONSTANT_TIME` writer - Configures the constant_time option. \\\\ 0: Acceleration\\\\ 1: No acceleration (default)\\\\"]
pub type ConstantTimeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the constant_time option. \\\\ 0: Acceleration\\\\ 1: No acceleration (default)\\\\"]
    #[inline(always)]
    pub fn constant_time(&self) -> ConstantTimeR {
        ConstantTimeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the constant_time option. \\\\ 0: Acceleration\\\\ 1: No acceleration (default)\\\\"]
    #[inline(always)]
    pub fn constant_time(&mut self) -> ConstantTimeW<'_, ConstantTimeSpec> {
        ConstantTimeW::new(self, 0)
    }
}
#[doc = "Configures the constant_time option\n\nYou can [`read`](crate::Reg::read) this register and get [`constant_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`constant_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConstantTimeSpec;
impl crate::RegisterSpec for ConstantTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`constant_time::R`](R) reader structure"]
impl crate::Readable for ConstantTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`constant_time::W`](W) writer structure"]
impl crate::Writable for ConstantTimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSTANT_TIME to value 0x01"]
impl crate::Resettable for ConstantTimeSpec {
    const RESET_VALUE: u32 = 0x01;
}
