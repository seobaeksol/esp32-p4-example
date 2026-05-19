#[doc = "Register `L1_ICACHE0_PRELOCK_CONF` reader"]
pub type R = crate::R<L1Icache0PrelockConfSpec>;
#[doc = "Register `L1_ICACHE0_PRELOCK_CONF` writer"]
pub type W = crate::W<L1Icache0PrelockConfSpec>;
#[doc = "Field `L1_ICACHE0_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function on L1-ICache0."]
pub type L1Icache0PrelockSct0EnR = crate::BitReader;
#[doc = "Field `L1_ICACHE0_PRELOCK_SCT0_EN` writer - The bit is used to enable the first section of prelock function on L1-ICache0."]
pub type L1Icache0PrelockSct0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE0_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function on L1-ICache0."]
pub type L1Icache0PrelockSct1EnR = crate::BitReader;
#[doc = "Field `L1_ICACHE0_PRELOCK_SCT1_EN` writer - The bit is used to enable the second section of prelock function on L1-ICache0."]
pub type L1Icache0PrelockSct1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1_ICACHE0_PRELOCK_RGID` reader - The bit is used to set the gid of l1 icache0 prelock."]
pub type L1Icache0PrelockRgidR = crate::FieldReader;
#[doc = "Field `L1_ICACHE0_PRELOCK_RGID` writer - The bit is used to set the gid of l1 icache0 prelock."]
pub type L1Icache0PrelockRgidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_prelock_sct0_en(&self) -> L1Icache0PrelockSct0EnR {
        L1Icache0PrelockSct0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_prelock_sct1_en(&self) -> L1Icache0PrelockSct1EnR {
        L1Icache0PrelockSct1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of l1 icache0 prelock."]
    #[inline(always)]
    pub fn l1_icache0_prelock_rgid(&self) -> L1Icache0PrelockRgidR {
        L1Icache0PrelockRgidR::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable the first section of prelock function on L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_prelock_sct0_en(
        &mut self,
    ) -> L1Icache0PrelockSct0EnW<'_, L1Icache0PrelockConfSpec> {
        L1Icache0PrelockSct0EnW::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to enable the second section of prelock function on L1-ICache0."]
    #[inline(always)]
    pub fn l1_icache0_prelock_sct1_en(
        &mut self,
    ) -> L1Icache0PrelockSct1EnW<'_, L1Icache0PrelockConfSpec> {
        L1Icache0PrelockSct1EnW::new(self, 1)
    }
    #[doc = "Bits 2:5 - The bit is used to set the gid of l1 icache0 prelock."]
    #[inline(always)]
    pub fn l1_icache0_prelock_rgid(
        &mut self,
    ) -> L1Icache0PrelockRgidW<'_, L1Icache0PrelockConfSpec> {
        L1Icache0PrelockRgidW::new(self, 2)
    }
}
#[doc = "L1 instruction Cache 0 prelock configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_icache0_prelock_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1_icache0_prelock_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Icache0PrelockConfSpec;
impl crate::RegisterSpec for L1Icache0PrelockConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_icache0_prelock_conf::R`](R) reader structure"]
impl crate::Readable for L1Icache0PrelockConfSpec {}
#[doc = "`write(|w| ..)` method takes [`l1_icache0_prelock_conf::W`](W) writer structure"]
impl crate::Writable for L1Icache0PrelockConfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1_ICACHE0_PRELOCK_CONF to value 0"]
impl crate::Resettable for L1Icache0PrelockConfSpec {}
