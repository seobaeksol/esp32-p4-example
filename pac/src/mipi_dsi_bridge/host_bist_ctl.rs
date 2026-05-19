#[doc = "Register `HOST_BIST_CTL` reader"]
pub type R = crate::R<HostBistCtlSpec>;
#[doc = "Register `HOST_BIST_CTL` writer"]
pub type W = crate::W<HostBistCtlSpec>;
#[doc = "Field `BISTOK` reader - bistok"]
pub type BistokR = crate::BitReader;
#[doc = "Field `BISTON` reader - biston"]
pub type BistonR = crate::BitReader;
#[doc = "Field `BISTON` writer - biston"]
pub type BistonW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - bistok"]
    #[inline(always)]
    pub fn bistok(&self) -> BistokR {
        BistokR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - biston"]
    #[inline(always)]
    pub fn biston(&self) -> BistonR {
        BistonR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - biston"]
    #[inline(always)]
    pub fn biston(&mut self) -> BistonW<'_, HostBistCtlSpec> {
        BistonW::new(self, 1)
    }
}
#[doc = "dsi_bridge host bist control register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_bist_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_bist_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostBistCtlSpec;
impl crate::RegisterSpec for HostBistCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_bist_ctl::R`](R) reader structure"]
impl crate::Readable for HostBistCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`host_bist_ctl::W`](W) writer structure"]
impl crate::Writable for HostBistCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_BIST_CTL to value 0"]
impl crate::Resettable for HostBistCtlSpec {}
