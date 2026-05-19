#[doc = "Register `L2_DBUS3_ACS_HIT_CNT` reader"]
pub type R = crate::R<L2Dbus3AcsHitCntSpec>;
#[doc = "Field `L2_DBUS3_HIT_CNT` reader - The register records the number of hits when L1-DCache accesses L2-Cache due to bus3 accessing L1-DCache."]
pub type L2Dbus3HitCntR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The register records the number of hits when L1-DCache accesses L2-Cache due to bus3 accessing L1-DCache."]
    #[inline(always)]
    pub fn l2_dbus3_hit_cnt(&self) -> L2Dbus3HitCntR {
        L2Dbus3HitCntR::new(self.bits)
    }
}
#[doc = "L2-Cache bus3 Hit-Access Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2_dbus3_acs_hit_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2Dbus3AcsHitCntSpec;
impl crate::RegisterSpec for L2Dbus3AcsHitCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2_dbus3_acs_hit_cnt::R`](R) reader structure"]
impl crate::Readable for L2Dbus3AcsHitCntSpec {}
#[doc = "`reset()` method sets L2_DBUS3_ACS_HIT_CNT to value 0"]
impl crate::Resettable for L2Dbus3AcsHitCntSpec {}
