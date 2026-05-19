#[doc = "Register `BOOT_ADDR_HP_LP` reader"]
pub type R = crate::R<BootAddrHpLpSpec>;
#[doc = "Register `BOOT_ADDR_HP_LP` writer"]
pub type W = crate::W<BootAddrHpLpSpec>;
#[doc = "Field `BOOT_ADDR_HP_LP` reader - need_des"]
pub type BootAddrHpLpR = crate::FieldReader<u32>;
#[doc = "Field `BOOT_ADDR_HP_LP` writer - need_des"]
pub type BootAddrHpLpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn boot_addr_hp_lp(&self) -> BootAddrHpLpR {
        BootAddrHpLpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn boot_addr_hp_lp(&mut self) -> BootAddrHpLpW<'_, BootAddrHpLpSpec> {
        BootAddrHpLpW::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_addr_hp_lp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_addr_hp_lp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BootAddrHpLpSpec;
impl crate::RegisterSpec for BootAddrHpLpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_addr_hp_lp::R`](R) reader structure"]
impl crate::Readable for BootAddrHpLpSpec {}
#[doc = "`write(|w| ..)` method takes [`boot_addr_hp_lp::W`](W) writer structure"]
impl crate::Writable for BootAddrHpLpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOOT_ADDR_HP_LP to value 0"]
impl crate::Resettable for BootAddrHpLpSpec {}
