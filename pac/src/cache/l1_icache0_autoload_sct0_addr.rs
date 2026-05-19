#[doc = "Register `L1_ICACHE0_AUTOLOAD_SCT0_ADDR` reader"]
pub type R = crate::R<L1Icache0AutoloadSct0AddrSpec>;
#[doc = "Register `L1_ICACHE0_AUTOLOAD_SCT0_ADDR` writer"]
pub type W = crate::W<L1Icache0AutoloadSct0AddrSpec>;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_SCT0_ADDR` reader - Those bits are used to configure the start virtual address of the first section for autoload operation on L1-ICache0. Note that it should be used together with L1_ICACHE0_AUTOLOAD_SCT0_SIZE and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
pub type L1Icache0AutoloadSct0AddrR = crate::FieldReader<u32>;
#[doc = "Field `L1_ICACHE0_AUTOLOAD_SCT0_ADDR` writer - Those bits are used to configure the start virtual address of the first section for autoload operation on L1-ICache0. Note that it should be used together with L1_ICACHE0_AUTOLOAD_SCT0_SIZE and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
pub type L1Icache0AutoloadSct0AddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the first section for autoload operation on L1-ICache0. Note that it should be used together with L1_ICACHE0_AUTOLOAD_SCT0_SIZE and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
    #[inline(always)]
    pub fn l1_icache0_autoload_sct0_addr(&self) -> L1Icache0AutoloadSct0AddrR {
        L1Icache0AutoloadSct0AddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Those bits are used to configure the start virtual address of the first section for autoload operation on L1-ICache0. Note that it should be used together with L1_ICACHE0_AUTOLOAD_SCT0_SIZE and L1_ICACHE_AUTOLOAD_SCT0_ENA."]
    #[inline(always)]
    pub fn l1_icache0_autoload_sct0_addr(
        &mut self,
    ) -> L1Icache0AutoloadSct0AddrW<'_, L1Icache0AutoloadSct0AddrSpec> {
        L1Icache0AutoloadSct0AddrW::new(self, 0)
    }
}
#[doc = "L1 instruction Cache 0 autoload section 0 address configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache0_autoload_sct0_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_icache0_autoload_sct0_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache0AutoloadSct0AddrSpec;
impl crate::RegisterSpec for L1Icache0AutoloadSct0AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache0_autoload_sct0_addr::R`](R) reader structure"]
impl crate::Readable for L1Icache0AutoloadSct0AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_icache0_autoload_sct0_addr::W`](W) writer structure"]
impl crate::Writable for L1Icache0AutoloadSct0AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_ICACHE0_AUTOLOAD_SCT0_ADDR to value 0"]
impl crate::Resettable for L1Icache0AutoloadSct0AddrSpec {}
