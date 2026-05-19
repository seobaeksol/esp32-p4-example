#[doc = "Register `DBI_PARTITIONING_EN` reader"]
pub type R = crate::R<DbiPartitioningEnSpec>;
#[doc = "Register `DBI_PARTITIONING_EN` writer"]
pub type W = crate::W<DbiPartitioningEnSpec>;
#[doc = "Field `PARTITIONING_EN` reader - NA"]
pub type PartitioningEnR = crate::BitReader;
#[doc = "Field `PARTITIONING_EN` writer - NA"]
pub type PartitioningEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn partitioning_en(&self) -> PartitioningEnR {
        PartitioningEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn partitioning_en(&mut self) -> PartitioningEnW<'_, DbiPartitioningEnSpec> {
        PartitioningEnW::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_partitioning_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_partitioning_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbiPartitioningEnSpec;
impl crate::RegisterSpec for DbiPartitioningEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_partitioning_en::R`](R) reader structure"]
impl crate::Readable for DbiPartitioningEnSpec {}
#[doc = "`write(|w| ..)` method takes [`dbi_partitioning_en::W`](W) writer structure"]
impl crate::Writable for DbiPartitioningEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBI_PARTITIONING_EN to value 0"]
impl crate::Resettable for DbiPartitioningEnSpec {}
