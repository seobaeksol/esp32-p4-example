#[doc = "Register `CONF` reader"]
pub type R = crate::R<ConfSpec>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<ConfSpec>;
#[doc = "Field `MODE` reader - Set this field to choose the huk process. 1: process huk generate mode. 0: process huk recovery mode."]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - Set this field to choose the huk process. 1: process huk generate mode. 0: process huk recovery mode."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this field to choose the huk process. 1: process huk generate mode. 0: process huk recovery mode."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set this field to choose the huk process. 1: process huk generate mode. 0: process huk recovery mode."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ConfSpec> {
        ModeW::new(self, 0)
    }
}
#[doc = "HUK Generator configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfSpec;
impl crate::RegisterSpec for ConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for ConfSpec {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for ConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for ConfSpec {}
