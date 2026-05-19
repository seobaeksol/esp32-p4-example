#[doc = "Register `AHB_CONFIG` reader"]
pub type R = crate::R<AhbConfigSpec>;
#[doc = "Register `AHB_CONFIG` writer"]
pub type W = crate::W<AhbConfigSpec>;
#[doc = "Field `HBURST` reader - set hburst"]
pub type HburstR = crate::FieldReader;
#[doc = "Field `HBURST` writer - set hburst"]
pub type HburstW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MAX_INCR` reader - set max continuous access for incr mode"]
pub type MaxIncrR = crate::FieldReader;
#[doc = "Field `MAX_INCR` writer - set max continuous access for incr mode"]
pub type MaxIncrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - set hburst"]
    #[inline(always)]
    pub fn hburst(&self) -> HburstR {
        HburstR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - set max continuous access for incr mode"]
    #[inline(always)]
    pub fn max_incr(&self) -> MaxIncrR {
        MaxIncrR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - set hburst"]
    #[inline(always)]
    pub fn hburst(&mut self) -> HburstW<'_, AhbConfigSpec> {
        HburstW::new(self, 0)
    }
    #[doc = "Bits 3:5 - set max continuous access for incr mode"]
    #[inline(always)]
    pub fn max_incr(&mut self) -> MaxIncrW<'_, AhbConfigSpec> {
        MaxIncrW::new(self, 3)
    }
}
#[doc = "AHB config register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbConfigSpec;
impl crate::RegisterSpec for AhbConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_config::R`](R) reader structure"]
impl crate::Readable for AhbConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb_config::W`](W) writer structure"]
impl crate::Writable for AhbConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_CONFIG to value 0"]
impl crate::Resettable for AhbConfigSpec {}
