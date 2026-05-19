#[doc = "Register `L1_DBUS3_ACS_HIT_CNT` reader"]
pub type R = crate::R<L1Dbus3AcsHitCntSpec>;
#[doc = "Field `L1_DBUS3_HIT_CNT` reader - The register records the number of hits when bus3 accesses L1-DCache."]
pub type L1Dbus3HitCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of hits when bus3 accesses L1-DCache."]
    #[inline(always)]
    pub fn l1_dbus3_hit_cnt(&self) -> L1Dbus3HitCntR {
        L1Dbus3HitCntR::new(self.bits)
    }
}
#[doc = "L1-DCache bus3 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1_dbus3_acs_hit_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1Dbus3AcsHitCntSpec;
impl crate::RegisterSpec for L1Dbus3AcsHitCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_dbus3_acs_hit_cnt::R`](R) reader structure"]
impl crate::Readable for L1Dbus3AcsHitCntSpec {}
#[doc = "`reset()` method sets L1_DBUS3_ACS_HIT_CNT to value 0"]
impl crate::Resettable for L1Dbus3AcsHitCntSpec {}
